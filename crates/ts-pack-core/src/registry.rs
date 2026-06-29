use ahash::{AHashMap, AHashSet};
#[cfg(feature = "dynamic-loading")]
use std::path::PathBuf;
#[cfg(feature = "dynamic-loading")]
use std::sync::{Arc, Mutex};
use tree_sitter::Language;

use crate::error::Error;

// Include the build.rs-generated language table
include!(concat!(env!("OUT_DIR"), "/registry_generated.rs"));

// Serializes only the not-yet-loaded dynamic-library *mutation* path; the static
// fast path and the already-loaded dynamic read are lock-free.
#[cfg(feature = "dynamic-loading")]
static LANGUAGE_LOAD_LOCK: Mutex<()> = Mutex::new(());

/// Alternative names that resolve to an existing grammar.
const LANGUAGE_ALIASES: &[(&str, &str)] = &[
    ("bazel", "starlark"),
    ("gradle", "groovy"),
    ("ignorefile", "gitignore"),
    ("lisp", "commonlisp"),
    ("makefile", "make"),
    ("shell", "bash"),
];

/// Resolve a language name to its C symbol name (e.g. "csharp" -> "c_sharp").
/// Falls back to the language name itself if no override exists.
#[cfg(any(feature = "dynamic-loading", feature = "download"))]
#[inline(always)]
pub(crate) fn c_symbol_for(name: &str) -> &str {
    for &(lang, sym) in C_SYMBOL_OVERRIDES {
        if lang == name {
            return sym;
        }
    }
    name
}

/// Reverse lookup: given a c_symbol (e.g. "c_sharp"), return the language name ("csharp").
/// If no override matches, returns the input as-is.
#[cfg(any(all(feature = "dynamic-loading", not(target_arch = "wasm32")), feature = "download",))]
#[inline(always)]
pub(crate) fn lang_name_for_symbol(symbol: &str) -> &str {
    for &(lang, sym) in C_SYMBOL_OVERRIDES {
        if sym == symbol {
            return lang;
        }
    }
    symbol
}

#[inline(always)]
pub(crate) fn resolve_alias(name: &str) -> &str {
    for &(alias, target) in LANGUAGE_ALIASES {
        if name == alias {
            return target;
        }
    }
    name
}

#[cfg(feature = "dynamic-loading")]
fn lib_path_in(dir: &std::path::Path, name: &str) -> PathBuf {
    let lib_name = format!("tree_sitter_{}", c_symbol_for(name));
    let (prefix, ext) = if cfg!(target_os = "macos") {
        ("lib", "dylib")
    } else if cfg!(target_os = "windows") {
        ("", "dll")
    } else {
        ("lib", "so")
    };
    dir.join(format!("{prefix}{lib_name}.{ext}"))
}

#[cfg(all(feature = "dynamic-loading", target_arch = "wasm32"))]
mod dynamic {
    //! wasm32 stub: shared-library loading is unsupported on wasm32-unknown-unknown.
    //! The public API mirrors the native impl but every load attempt returns an error.
    use std::path::PathBuf;
    use tree_sitter::Language;

    use crate::error::Error;

    pub(crate) struct DynamicLoader {
        pub(crate) libs_dir: PathBuf,
        pub(crate) dynamic_names: Vec<&'static str>,
    }

    impl DynamicLoader {
        pub(crate) fn new(libs_dir: PathBuf, dynamic_names: Vec<&'static str>) -> Self {
            Self {
                libs_dir,
                dynamic_names,
            }
        }

        pub(crate) fn get_cached(&self, _name: &str) -> Result<Option<Language>, Error> {
            Ok(None)
        }

        pub(crate) fn cached_names(&self) -> Vec<String> {
            Vec::new()
        }

        pub(crate) fn lib_file_exists(&self, _name: &str) -> bool {
            false
        }

        pub(crate) fn load_from_dir(&self, name: &str, _dir: &std::path::Path) -> Result<Language, Error> {
            Err(Error::DynamicLoad(format!(
                "Dynamic loading is unsupported on wasm32 (requested '{name}')"
            )))
        }

        pub(crate) fn load(&self, name: &str) -> Result<Language, Error> {
            Err(Error::DynamicLoad(format!(
                "Dynamic loading is unsupported on wasm32 (requested '{name}')"
            )))
        }
    }
}

#[cfg(all(feature = "dynamic-loading", not(target_arch = "wasm32")))]
mod dynamic {
    use std::collections::HashMap;
    use std::path::{Path, PathBuf};
    use std::sync::{LazyLock, RwLock};
    use tree_sitter::Language;

    use crate::error::Error;

    static LOADED_LIBRARIES: LazyLock<RwLock<HashMap<PathBuf, libloading::Library>>> =
        LazyLock::new(|| RwLock::new(HashMap::new()));

    /// Holds dynamically loaded libraries to keep them alive.
    /// The Library must outlive the Language since Language references code in the loaded library.
    pub(crate) struct DynamicLibs {
        languages: HashMap<String, Language>,
    }

    pub(crate) struct DynamicLoader {
        inner: RwLock<DynamicLibs>,
        pub(crate) libs_dir: PathBuf,
        pub(crate) dynamic_names: Vec<&'static str>,
    }

    impl DynamicLoader {
        pub(crate) fn new(libs_dir: PathBuf, dynamic_names: Vec<&'static str>) -> Self {
            Self {
                inner: RwLock::new(DynamicLibs {
                    languages: HashMap::new(),
                }),
                libs_dir,
                dynamic_names,
            }
        }

        pub(crate) fn get_cached(&self, name: &str) -> Result<Option<Language>, Error> {
            let dynamic = self.inner.read().map_err(|e| Error::LockPoisoned(e.to_string()))?;
            Ok(dynamic.languages.get(name).cloned())
        }

        pub(crate) fn cached_names(&self) -> Vec<String> {
            if let Ok(dynamic) = self.inner.read() {
                dynamic.languages.keys().cloned().collect()
            } else {
                Vec::new()
            }
        }

        pub(crate) fn lib_file_exists(&self, name: &str) -> bool {
            self.lib_path(name).exists()
        }

        fn lib_path(&self, name: &str) -> PathBuf {
            super::lib_path_in(&self.libs_dir, name)
        }

        /// Load a language from a specific directory (e.g. download cache).
        /// The loaded library is stored in the shared cache.
        pub(crate) fn load_from_dir(&self, name: &str, dir: &std::path::Path) -> Result<Language, Error> {
            let lib_path = super::lib_path_in(dir, name);
            if !lib_path.exists() {
                return Err(Error::LanguageNotFound(format!(
                    "Dynamic library for '{}' not found at {}",
                    name,
                    lib_path.display()
                )));
            }
            self.load_from_path(name, &lib_path)
        }

        pub(crate) fn load(&self, name: &str) -> Result<Language, Error> {
            let lib_path = self.lib_path(name);
            if !lib_path.exists() {
                return Err(Error::LanguageNotFound(format!(
                    "Dynamic library for '{}' not found at {}",
                    name,
                    lib_path.display()
                )));
            }
            self.load_from_path(name, &lib_path)
        }

        fn load_from_path(&self, name: &str, lib_path: &Path) -> Result<Language, Error> {
            let mut dynamic = self.inner.write().map_err(|e| Error::LockPoisoned(e.to_string()))?;

            // Another thread may have loaded it between our read and write lock
            if let Some(lang) = dynamic.languages.get(name) {
                return Ok(lang.clone());
            }

            let func_name = format!("tree_sitter_{}", super::c_symbol_for(name));
            let lib_key = lib_path.canonicalize().unwrap_or_else(|_| lib_path.to_path_buf());
            let language = language_from_process_library(name, &func_name, &lib_key, lib_path)?;

            dynamic.languages.insert(name.to_string(), language.clone());
            Ok(language)
        }
    }

    fn language_from_process_library(
        name: &str,
        func_name: &str,
        lib_key: &Path,
        lib_path: &Path,
    ) -> Result<Language, Error> {
        let mut libraries = LOADED_LIBRARIES
            .write()
            .map_err(|e| Error::LockPoisoned(e.to_string()))?;
        if !libraries.contains_key(lib_key) {
            // SAFETY: We load a tree-sitter grammar shared library from an explicit path
            // selected by the registry/download cache and keep it in a process-wide map.
            let lib = unsafe { libloading::Library::new(lib_path) }
                .map_err(|e| Error::DynamicLoad(format!("Failed to load library {}: {}", lib_path.display(), e)))?;
            libraries.insert(lib_key.to_path_buf(), lib);
        }

        let lib = libraries
            .get(lib_key)
            .ok_or_else(|| Error::DynamicLoad(format!("Loaded library {} missing from cache", lib_path.display())))?;
        // SAFETY: The loaded library is kept alive for the process lifetime by LOADED_LIBRARIES,
        // and tree-sitter grammars export `tree_sitter_<name>() -> *const TSLanguage`.
        unsafe {
            let func: libloading::Symbol<unsafe extern "C" fn() -> *const tree_sitter::ffi::TSLanguage> =
                lib.get(func_name.as_bytes()).map_err(|e| {
                    Error::DynamicLoad(format!(
                        "Symbol '{}' not found in {}: {}",
                        func_name,
                        lib_path.display(),
                        e
                    ))
                })?;
            let ptr = func();
            if ptr.is_null() {
                return Err(Error::NullLanguagePointer(name.to_string()));
            }
            Ok(Language::from_raw(ptr))
        }
    }

    #[cfg(test)]
    pub(super) fn loaded_library_count_for_tests() -> usize {
        LOADED_LIBRARIES.read().map(|libs| libs.len()).unwrap_or_default()
    }
}

/// Thread-safe registry of tree-sitter language parsers.
///
/// Manages both statically compiled and dynamically loaded language grammars.
/// Use [`LanguageRegistry::new()`] for the default registry, or access the
/// global instance via the module-level convenience functions
/// ([`crate::get_language`], [`crate::available_languages`], etc.).
///
/// # Example
///
/// ```no_run
/// use tree_sitter_language_pack::{LanguageRegistry, ProcessConfig};
///
/// let registry = LanguageRegistry::new();
/// let langs = registry.available_languages();
/// println!("Available: {:?}", langs);
///
/// let config = ProcessConfig::new("python").all();
/// let result = registry.process("def hello(): pass", &config).unwrap();
/// println!("Structure: {:?}", result.structure);
/// ```
pub struct LanguageRegistry {
    static_lookup: AHashMap<&'static str, fn() -> Language>,
    #[cfg(feature = "dynamic-loading")]
    dynamic_loader: dynamic::DynamicLoader,
    /// Additional library directories to search (e.g. download cache).
    /// Wrapped in Arc<RwLock<...>> so the outer struct is Send+Sync without
    /// requiring &mut self for mutation — interior mutability via the inner lock.
    #[cfg(feature = "dynamic-loading")]
    extra_lib_dirs: Arc<std::sync::RwLock<Arc<Vec<PathBuf>>>>,
}

impl LanguageRegistry {
    /// Create a new registry populated with all statically compiled languages.
    ///
    /// When the `dynamic-loading` feature is enabled, the registry also knows
    /// about dynamically loadable grammars and will load them on demand.
    pub fn new() -> Self {
        let mut static_lookup = AHashMap::with_capacity(STATIC_LANGUAGES.len());
        for &(name, loader) in STATIC_LANGUAGES {
            static_lookup.insert(name, loader);
        }

        Self {
            static_lookup,
            #[cfg(feature = "dynamic-loading")]
            dynamic_loader: dynamic::DynamicLoader::new(PathBuf::from(LIBS_DIR), DYNAMIC_LANGUAGE_NAMES.to_vec()),
            #[cfg(feature = "dynamic-loading")]
            extra_lib_dirs: Arc::new(std::sync::RwLock::new(Arc::new(Vec::new()))),
        }
    }

    /// Create a registry with a custom directory for dynamic libraries.
    ///
    /// Overrides the default build-time library directory. Useful when
    /// dynamic grammar shared libraries are stored in a non-standard location.
    #[cfg_attr(alef, alef(skip))]
    #[cfg(feature = "dynamic-loading")]
    pub fn with_libs_dir(libs_dir: PathBuf) -> Self {
        let mut reg = Self::new();
        reg.dynamic_loader.libs_dir = libs_dir;
        reg
    }

    /// Add an additional directory to search for dynamic libraries.
    ///
    /// When [`get_language`](Self::get_language) cannot find a grammar in the
    /// primary library directory, it searches these extra directories in order.
    /// Typically used by the download system to register its cache directory.
    ///
    /// Takes `&self` (not `&mut self`) because `extra_lib_dirs` uses interior
    /// mutability via an `Arc<RwLock<...>>`, so the outer registry can remain
    /// immutable while the directory list is updated.
    #[cfg_attr(alef, alef(skip))]
    #[cfg(feature = "dynamic-loading")]
    pub fn add_extra_libs_dir(&self, dir: PathBuf) {
        if let Ok(mut dirs) = self.extra_lib_dirs.write()
            && !dirs.contains(&dir)
        {
            let mut new_dirs = (**dirs).clone();
            new_dirs.push(dir);
            *dirs = Arc::new(new_dirs);
        }
    }

    /// Get a tree-sitter [`Language`] by name.
    ///
    /// Resolves aliases (e.g., `"shell"` -> `"bash"`, `"makefile"` -> `"make"`),
    /// then looks up the language in the static table. When the `dynamic-loading`
    /// feature is enabled, falls back to loading a shared library on demand.
    ///
    /// # Errors
    ///
    /// Returns [`Error::LanguageNotFound`] if the name (after alias resolution)
    /// does not match any known grammar.
    pub fn get_language(&self, name: &str) -> Result<Language, Error> {
        let name = resolve_alias(name);

        // Lock-free static fast path: `static_lookup` is built once in `new()` and
        // never mutated, and `AHashMap::get(&self)` performs no interior mutation,
        // so concurrent reads are data-race-free without any lock.
        if let Some(loader) = self.static_lookup.get(name) {
            return Ok(loader());
        }

        #[cfg(feature = "dynamic-loading")]
        {
            // Already-loaded dynamic grammars: guarded by the loader's own RwLock,
            // so this read also needs no outer lock.
            if let Some(lang) = self.dynamic_loader.get_cached(name)? {
                return Ok(lang);
            }

            // Mutation path only: serialize loads of not-yet-loaded libraries.
            let _guard = LANGUAGE_LOAD_LOCK
                .lock()
                .map_err(|e| Error::LockPoisoned(e.to_string()))?;
            // Double-check under the lock — another thread may have loaded it.
            if let Some(lang) = self.dynamic_loader.get_cached(name)? {
                return Ok(lang);
            }

            // Try loading from build-time libs dir
            if self.dynamic_loader.dynamic_names.contains(&name) || self.dynamic_loader.lib_file_exists(name) {
                return self.dynamic_loader.load(name);
            }

            // Try loading from extra dirs (e.g. download cache)
            let extra_dirs: Arc<Vec<PathBuf>> = self
                .extra_lib_dirs
                .read()
                .map(|dirs| Arc::clone(&dirs))
                .unwrap_or_default();
            for extra_dir in extra_dirs.iter() {
                if self.dynamic_loader.load_from_dir(name, extra_dir).is_ok() {
                    // Re-fetch from cache — load_from_dir inserted it
                    if let Some(lang) = self.dynamic_loader.get_cached(name)? {
                        return Ok(lang);
                    }
                }
            }
        }

        Err(Error::LanguageNotFound(name.to_string()))
    }

    /// List all available language names, sorted and deduplicated.
    ///
    /// Includes statically compiled languages, dynamically loadable languages
    /// (if the `dynamic-loading` feature is enabled), and all configured aliases.
    pub fn available_languages(&self) -> Vec<String> {
        let mut seen: AHashSet<&str> = self.static_lookup.keys().copied().collect();

        // Owned strings from dynamic sources; kept alive so we can borrow into `seen`.
        #[cfg(feature = "dynamic-loading")]
        let _owned_names: Vec<String>;

        #[cfg(feature = "dynamic-loading")]
        {
            for name in self.dynamic_loader.dynamic_names.iter() {
                seen.insert(name);
            }

            let mut owned = self.dynamic_loader.cached_names();

            // Scan extra library directories for downloadable/cached libraries
            let extra_dirs: Arc<Vec<PathBuf>> = self
                .extra_lib_dirs
                .read()
                .map(|dirs| Arc::clone(&dirs))
                .unwrap_or_default();
            for extra_dir in extra_dirs.iter() {
                if let Ok(entries) = std::fs::read_dir(extra_dir) {
                    for entry in entries.flatten() {
                        let filename = entry.file_name();
                        let name = filename.to_string_lossy();
                        let stripped = name.strip_prefix("lib").unwrap_or(&name);
                        if let Some(lang) = stripped.strip_prefix("tree_sitter_") {
                            let lang = lang
                                .strip_suffix(".so")
                                .or_else(|| lang.strip_suffix(".dylib"))
                                .or_else(|| lang.strip_suffix(".dll"));
                            if let Some(lang) = lang {
                                owned.push(lang.to_string());
                            }
                        }
                    }
                }
            }

            _owned_names = owned;
            for name in &_owned_names {
                seen.insert(name.as_str());
            }
        }
        for &(alias, target) in LANGUAGE_ALIASES {
            if seen.contains(target) {
                seen.insert(alias);
            }
        }

        let mut langs: Vec<String> = seen.into_iter().map(String::from).collect();
        langs.sort_unstable();
        langs
    }

    /// Check whether a parser is statically compiled into this build.
    ///
    /// Returns `true` only when the grammar was compiled in at build time
    /// (i.e. it appears in the `STATIC_LANGUAGES` table). This is independent
    /// of the extension-to-language mapping: [`crate::detect_language_from_extension`]
    /// consults the static ext table for all 306 grammars regardless of which
    /// parsers are compiled in.
    ///
    /// Use this when you need to distinguish "we know the language name" from
    /// "we can actually parse files in that language right now".
    ///
    /// ```no_run
    /// use tree_sitter_language_pack::{detect_language_from_extension, LanguageRegistry};
    ///
    /// let registry = LanguageRegistry::new();
    /// // Extension detection uses the static table — independent of compiled parsers.
    /// let lang = detect_language_from_extension("feature"); // always returns Some("gherkin")
    /// // Parser availability depends on which grammars were compiled in.
    /// let can_parse = lang.map(|name| registry.has_parser(name)).unwrap_or(false);
    /// ```
    pub fn has_parser(&self, name: &str) -> bool {
        let name = resolve_alias(name);
        self.static_lookup.contains_key(name)
    }

    /// Check whether a language is available by name or alias.
    ///
    /// Returns `true` if the language can be loaded, either from the static
    /// table or from a dynamic library on disk.
    pub fn has_language(&self, name: &str) -> bool {
        let name = resolve_alias(name);
        if self.static_lookup.contains_key(name) {
            return true;
        }

        #[cfg(feature = "dynamic-loading")]
        {
            if self.dynamic_loader.dynamic_names.contains(&name) || self.dynamic_loader.lib_file_exists(name) {
                return true;
            }

            let extra_dirs: Arc<Vec<PathBuf>> = self
                .extra_lib_dirs
                .read()
                .map(|dirs| Arc::clone(&dirs))
                .unwrap_or_default();
            for extra_dir in extra_dirs.iter() {
                if lib_path_in(extra_dir, name).exists() {
                    return true;
                }
            }
        }

        // With the `download` feature, any language present in the language pack's
        // definition table can be fetched on demand. Report it as available even if
        // it hasn't been downloaded yet — `get_language` triggers the fetch.
        #[cfg(feature = "download")]
        {
            if KNOWN_LANGUAGES.contains(&name) {
                return true;
            }
        }

        false
    }

    /// Return the total number of available languages (including aliases).
    pub fn language_count(&self) -> usize {
        self.available_languages().len()
    }

    /// Parse source code and extract file intelligence based on config in a single pass.
    pub fn process(
        &self,
        source: &str,
        config: &crate::process_config::ProcessConfig,
    ) -> Result<crate::intel::types::ProcessResult, Error> {
        let resolved_lang = resolve_alias(&config.language);
        if resolved_lang != config.language.as_ref() {
            let mut resolved_config = config.clone();
            resolved_config.language = std::borrow::Cow::Owned(resolved_lang.to_string());
            crate::intel::process(source, &resolved_config, self)
        } else {
            crate::intel::process(source, config, self)
        }
    }
}

impl Default for LanguageRegistry {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::process_config::ProcessConfig;

    fn first_available_lang() -> Option<String> {
        let registry = LanguageRegistry::new();
        let langs = registry.available_languages();
        langs.into_iter().next()
    }

    #[test]
    fn test_registry_process() {
        let Some(lang) = first_available_lang() else { return };
        let registry = LanguageRegistry::new();
        let config = ProcessConfig::new(&lang);
        let result = registry.process("x", &config);
        assert!(result.is_ok(), "registry.process() should succeed");
        let intel = result.unwrap();
        assert_eq!(intel.language, lang);
        assert!(intel.metrics.total_lines >= 1);
    }

    #[test]
    fn test_registry_process_with_chunking() {
        let Some(lang) = first_available_lang() else { return };
        let registry = LanguageRegistry::new();
        let config = ProcessConfig::new(&lang).with_chunking(1000);
        let result = registry.process("x", &config);
        assert!(result.is_ok(), "registry.process() with chunking should succeed");
        let intel = result.unwrap();
        assert_eq!(intel.language, lang);
        assert!(!intel.chunks.is_empty());
    }

    #[test]
    fn test_registry_process_invalid_language() {
        let registry = LanguageRegistry::new();
        let config = ProcessConfig::new("nonexistent_lang_xyz");
        let result = registry.process("x", &config);
        assert!(result.is_err());
    }

    #[test]
    fn test_registry_has_language_and_count() {
        let registry = LanguageRegistry::new();
        let langs = registry.available_languages();
        assert_eq!(registry.language_count(), langs.len());
        if let Some(lang) = langs.first() {
            assert!(registry.has_language(lang));
        }
        assert!(!registry.has_language("nonexistent_lang_xyz"));
    }

    #[cfg(all(feature = "dynamic-loading", not(target_arch = "wasm32")))]
    #[test]
    fn test_dynamic_language_survives_registry_drop() {
        let language_name = {
            let registry = LanguageRegistry::new();
            let Some(name) = registry.dynamic_loader.dynamic_names.first() else {
                return;
            };
            (*name).to_string()
        };
        let language = {
            let registry = LanguageRegistry::new();
            let Ok(language) = registry.get_language(&language_name) else {
                return;
            };
            language
        };

        let mut parser = tree_sitter::Parser::new();
        parser
            .set_language(&language)
            .expect("language should remain valid after registry drop");
        let tree = parser.parse("x", None).expect("parser should work after registry drop");
        assert!(tree.root_node().end_byte() <= 1);
    }

    #[cfg(all(feature = "dynamic-loading", not(target_arch = "wasm32")))]
    #[test]
    fn test_dynamic_library_is_loaded_once_across_registries() {
        let registry = LanguageRegistry::new();
        let Some(language_name) = registry.dynamic_loader.dynamic_names.first().copied() else {
            return;
        };
        drop(registry);

        let before = dynamic::loaded_library_count_for_tests();
        let first = LanguageRegistry::new();
        let first_language = first
            .get_language(language_name)
            .expect("first registry should load dynamic language");
        let after_first = dynamic::loaded_library_count_for_tests();

        let second = LanguageRegistry::new();
        let second_language = second
            .get_language(language_name)
            .expect("second registry should reuse process-loaded dynamic library");
        let after_second = dynamic::loaded_library_count_for_tests();

        assert!(after_first >= before);
        assert_eq!(after_first, after_second);
        assert_eq!(first_language.abi_version(), second_language.abi_version());
    }

    #[test]
    fn test_global_process_is_safe_from_multiple_threads() {
        let languages: Vec<String> = ["python", "rust", "javascript", "go"]
            .into_iter()
            .filter(|name| crate::has_language(name))
            .map(str::to_string)
            .collect();
        if languages.is_empty() {
            return;
        }

        std::thread::scope(|scope| {
            for language in &languages {
                scope.spawn(move || {
                    for _ in 0..16 {
                        let config = ProcessConfig::new(language);
                        let result = crate::process("x", &config).expect("process should be thread-safe");
                        assert_eq!(result.language, *language);
                    }
                });
            }
        });
    }

    #[cfg(feature = "serde")]
    #[test]
    fn test_process_result_serde_roundtrip() {
        let Some(lang) = first_available_lang() else { return };
        let registry = LanguageRegistry::new();
        let source = "x";
        let config = ProcessConfig::new(&lang);
        let intel = registry.process(source, &config).unwrap();
        let json = serde_json::to_string(&intel).expect("serialize should succeed");
        let deserialized: crate::intel::types::ProcessResult =
            serde_json::from_str(&json).expect("deserialize should succeed");
        assert_eq!(deserialized.language, intel.language);
        assert_eq!(deserialized.metrics.total_lines, intel.metrics.total_lines);
        assert_eq!(deserialized.metrics.total_bytes, intel.metrics.total_bytes);
    }
}
