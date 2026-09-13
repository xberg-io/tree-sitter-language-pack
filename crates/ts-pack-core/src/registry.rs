use ahash::{AHashMap, AHashSet};
use std::borrow::Cow;
#[cfg(feature = "dynamic-loading")]
use std::path::PathBuf;
#[cfg(feature = "dynamic-loading")]
use std::sync::{Arc, Mutex};
use tree_sitter::Language;

use crate::error::Error;

include!(concat!(env!("OUT_DIR"), "/registry_generated.rs"));

// ~keep Serializes only not-yet-loaded dynamic-library mutation; read/static paths are lock-free.
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
#[cfg(any(feature = "dynamic-loading", feature = "download"))]
#[inline(always)]
pub(crate) fn lang_name_for_symbol(symbol: &str) -> &str {
    for &(lang, sym) in C_SYMBOL_OVERRIDES {
        if sym == symbol {
            return lang;
        }
    }
    symbol
}

/// Platform-specific filename of a language's parser shared library,
/// e.g. `"csharp"` -> `libtree_sitter_c_sharp.dylib` on macOS.
///
/// Applies the `c_symbol` override, so callers must never build this name by
/// interpolating the language id directly — the two differ for the grammars
/// listed in `C_SYMBOL_OVERRIDES`. ~keep
///
/// Rust-only: bindings address parsers by language name, never by file path.
#[cfg(any(feature = "dynamic-loading", feature = "download"))]
#[cfg_attr(alef, alef(skip))]
pub fn library_file_name(language: &str) -> String {
    let symbol = c_symbol_for(language);
    let (prefix, ext) = if cfg!(target_os = "macos") {
        ("lib", "dylib")
    } else if cfg!(target_os = "windows") {
        ("", "dll")
    } else {
        ("lib", "so")
    };
    format!("{prefix}tree_sitter_{symbol}.{ext}")
}

/// Inverse of [`library_file_name`]: recover the language name from a parser
/// shared-library filename, e.g. `libtree_sitter_c_sharp.dylib` -> `"csharp"`.
///
/// Returns `None` when the filename is not a parser library. Accepts both the
/// `tree_sitter_` and `tree-sitter-` infixes because the two producers (build
/// script and release archives) differ. ~keep
#[cfg(any(feature = "dynamic-loading", feature = "download"))]
pub(crate) fn lang_name_from_lib_filename(filename: &str) -> Option<String> {
    let name = filename.strip_prefix("lib").unwrap_or(filename);
    let name = name
        .strip_prefix("tree_sitter_")
        .or_else(|| name.strip_prefix("tree-sitter-"))?;
    let name = name
        .strip_suffix(".so")
        .or_else(|| name.strip_suffix(".dylib"))
        .or_else(|| name.strip_suffix(".dll"))?;
    Some(lang_name_for_symbol(name).to_string())
}

/// Whether the build-time manifest knows this (already alias-resolved) language.
///
/// `KNOWN_LANGUAGES` is emitted from a `BTreeMap` in `build.rs`, so it is sorted
/// by byte order and can be binary-searched; `known_languages_is_sorted` pins
/// that cross-file invariant so the search cannot silently start lying. ~keep
#[cfg(feature = "download")]
#[inline]
fn known_language(name: &str) -> bool {
    KNOWN_LANGUAGES.binary_search(&name).is_ok()
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

/// Reverse lookup: every alias that resolves to `target` (the inverse of
/// [`resolve_alias`]).
///
/// The only entry point for this direction of the lookup, so callers that need
/// to report aliases alongside a canonical name — e.g.
/// [`crate::download::DownloadManager::installed_languages`] — never need their
/// own copy of `LANGUAGE_ALIASES`. There is exactly one alias table; keeping a
/// second copy elsewhere would let the two drift. ~keep
#[cfg(feature = "download")]
pub(crate) fn aliases_for(target: &str) -> Vec<&'static str> {
    LANGUAGE_ALIASES
        .iter()
        .filter_map(|&(alias, canonical)| (canonical == target).then_some(alias))
        .collect()
}

#[cfg(feature = "dynamic-loading")]
fn lib_path_in(dir: &std::path::Path, name: &str) -> PathBuf {
    dir.join(library_file_name(name))
}

/// Run-time override for the directory holding dynamically loadable parsers.
#[cfg(feature = "dynamic-loading")]
const LIBS_DIR_ENV: &str = "TREE_SITTER_LANGUAGE_PACK_LIBS_DIR";

/// Directory searched for dynamically loadable parser libraries.
///
/// `LIBS_DIR` is baked in by `build.rs` as an absolute `OUT_DIR` path, so it cannot
/// survive the artifact being moved — copied into a container image, shipped inside a
/// wheel or gem, or relocated after `cargo build`. Nothing emitted at build time can
/// resolve at run time, so the override has to come from the environment. ~keep
///
/// Falls back to the baked path, which remains correct for an in-place build tree.
#[cfg(feature = "dynamic-loading")]
fn resolve_libs_dir() -> PathBuf {
    match std::env::var(LIBS_DIR_ENV) {
        Ok(dir) if !dir.trim().is_empty() => PathBuf::from(dir),
        _ => PathBuf::from(LIBS_DIR),
    }
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

    // ~keep INVARIANT: nothing in this crate ever removes or clears an entry from this
    // ~keep map — the only mutation site is the guarded `insert` in
    // ~keep `language_from_process_library` below. That single guarantee is what lets a
    // ~keep `Language` (which borrows function pointers out of the loaded
    // ~keep `libloading::Library`) outlive the `LanguageRegistry` scope that produced it:
    // ~keep the `Library` stays alive in this process-wide map for the remaining life of
    // ~keep the process, so the code a `Language` points into is never unloaded out from
    // ~keep under it. Adding a `remove`/`clear` here would reintroduce a use-after-free.
    // ~keep See `should_keep_loaded_libraries_present_after_the_owning_registry_is_dropped`.
    static LOADED_LIBRARIES: LazyLock<RwLock<HashMap<PathBuf, libloading::Library>>> =
        LazyLock::new(|| RwLock::new(HashMap::new()));

    /// Whether a grammar's ABI version can be used by the tree-sitter runtime linked here.
    ///
    /// The bounds come from the runtime rather than being written down, so a runtime upgrade
    /// moves them automatically instead of leaving a stale literal behind. ~keep
    pub(super) fn is_supported_abi(abi: usize) -> bool {
        (tree_sitter::MIN_COMPATIBLE_LANGUAGE_VERSION..=tree_sitter::LANGUAGE_VERSION).contains(&abi)
    }

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

            // ~keep Another thread may have loaded it between our read and write lock.
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
            tracing::debug!(name, path = %lib_path.display(), "loading grammar shared library");
            // ~keep SAFETY: load grammar shared libraries only from registry/download-cache paths.
            // ~keep The library is retained in a process-wide map for symbol lifetime.
            let lib = unsafe { libloading::Library::new(lib_path) }
                .map_err(|e| Error::DynamicLoad(format!("Failed to load library {}: {}", lib_path.display(), e)))?;
            libraries.insert(lib_key.to_path_buf(), lib);
        }

        let lib = libraries
            .get(lib_key)
            .ok_or_else(|| Error::DynamicLoad(format!("Loaded library {} missing from cache", lib_path.display())))?;
        // ~keep SAFETY: LOADED_LIBRARIES keeps the library alive; grammars export `tree_sitter_<name>()`.
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
            let language = Language::from_raw(ptr);
            // ~keep A downloaded grammar is the one language object this crate does not compile
            // itself, so it is the only one whose ABI can disagree with the linked runtime. Without
            // this check the mismatch surfaces later as a `ts_parser_set_language` failure or, for
            // a grammar just outside the compatible range, as wrong parses -- a load-time error
            // naming both versions is far cheaper to diagnose. `Language` is not refcounted, so
            // returning early here leaks nothing.
            let abi = language.abi_version();
            if !is_supported_abi(abi) {
                return Err(Error::DynamicLoad(format!(
                    "Grammar '{}' in {} has ABI version {}, outside the range {}..={} supported by \
                     the linked tree-sitter runtime",
                    name,
                    lib_path.display(),
                    abi,
                    tree_sitter::MIN_COMPATIBLE_LANGUAGE_VERSION,
                    tree_sitter::LANGUAGE_VERSION,
                )));
            }
            Ok(language)
        }
    }

    #[cfg(test)]
    /// Whether one specific library path is present in the process-wide load cache.
    ///
    /// Scoped to a single path on purpose: the total count is shared with every other
    /// test that loads a grammar, so comparing global snapshots across two points in
    /// time races under `cargo test`'s default parallelism. ~keep
    #[cfg(test)]
    pub(super) fn library_is_loaded_for_tests(path: &Path) -> bool {
        LOADED_LIBRARIES
            .read()
            .map(|libs| libs.contains_key(path))
            .unwrap_or(false)
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
            dynamic_loader: dynamic::DynamicLoader::new(resolve_libs_dir(), DYNAMIC_LANGUAGE_NAMES.to_vec()),
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
    ///
    /// A poisoned lock is reported as a WARN and the directory is not registered.
    /// Callers that must distinguish "registered" from "could not register" — the
    /// download cache registration does, because recording success after a failed
    /// registration hides the cache directory permanently — should use
    /// [`Self::try_add_extra_libs_dir`] instead. This signature stays infallible to
    /// keep the public API source-compatible. ~keep
    #[cfg_attr(alef, alef(skip))]
    #[cfg(feature = "dynamic-loading")]
    pub fn add_extra_libs_dir(&self, dir: PathBuf) {
        if let Err(error) = self.try_add_extra_libs_dir(dir) {
            tracing::warn!(%error, "failed to register an extra library directory; it will not be searched");
        }
    }

    /// Add an additional directory to search for dynamic libraries, reporting failure.
    ///
    /// # Errors
    ///
    /// Returns [`Error::LockPoisoned`] if the write lock is poisoned. Unlike the
    /// read side ([`Self::extra_lib_dirs_snapshot`]), this does not recover and
    /// retry: a caller registering a directory needs to know registration did not
    /// happen, or it can silently believe a cache directory is searched when it
    /// never was. ~keep
    #[cfg(feature = "dynamic-loading")]
    pub(crate) fn try_add_extra_libs_dir(&self, dir: PathBuf) -> Result<(), Error> {
        let mut dirs = self
            .extra_lib_dirs
            .write()
            .map_err(|e| Error::LockPoisoned(e.to_string()))?;
        if !dirs.contains(&dir) {
            let mut new_dirs = (**dirs).clone();
            new_dirs.push(dir);
            *dirs = Arc::new(new_dirs);
        }
        Ok(())
    }

    /// Snapshot the extra library-search directories, recovering a poisoned lock.
    ///
    /// `extra_lib_dirs` only ever grows (see [`Self::add_extra_libs_dir`]) — no site
    /// removes or clears it — so a write-side panic can only interrupt the
    /// clone-then-push before it replaces the shared `Arc`, leaving the previous,
    /// still-coherent list of directories in place. Poisoning therefore carries no
    /// risk of a torn read here, and treating it as fatal would report a language as
    /// absent solely because some earlier, unrelated directory registration
    /// panicked. ~keep
    #[cfg(feature = "dynamic-loading")]
    fn extra_lib_dirs_snapshot(&self) -> Arc<Vec<PathBuf>> {
        let dirs = self.extra_lib_dirs.read().unwrap_or_else(|poisoned| {
            tracing::warn!("recovered a poisoned extra_lib_dirs lock (append-only data, safe to reuse)");
            poisoned.into_inner()
        });
        Arc::clone(&dirs)
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

        // ~keep Static lookup is immutable after `new()`, so concurrent reads require no lock.
        if let Some(loader) = self.static_lookup.get(name) {
            return Ok(loader());
        }

        #[cfg(feature = "dynamic-loading")]
        {
            // ~keep Already-loaded dynamic grammars are guarded by the loader RwLock; no outer lock needed.
            if let Some(lang) = self.dynamic_loader.get_cached(name)? {
                return Ok(lang);
            }

            // ~keep Serialize only loads of not-yet-loaded libraries.
            let _guard = LANGUAGE_LOAD_LOCK
                .lock()
                .unwrap_or_else(|poisoned| crate::recover_poisoned_lock("language_load", poisoned));
            // ~keep Double-check under the lock; another thread may have loaded it.
            if let Some(lang) = self.dynamic_loader.get_cached(name)? {
                return Ok(lang);
            }

            // ~keep A name in the build-time `DYNAMIC_LANGUAGE_NAMES` list may still be absent from
            // ~keep `libs_dir` once the artifact has been relocated. Returning here unconditionally
            // ~keep made that a hard `LanguageNotFound` even when the download cache — registered as
            // ~keep an extra dir below — held that exact parser. Fall through on `LanguageNotFound`
            // ~keep only; any other error is a real load failure and must propagate rather than be
            // ~keep retried against unrelated directories.
            if self.dynamic_loader.dynamic_names.contains(&name) || self.dynamic_loader.lib_file_exists(name) {
                match self.dynamic_loader.load(name) {
                    Ok(language) => return Ok(language),
                    Err(Error::LanguageNotFound(_)) => {}
                    Err(error) => return Err(error),
                }
            }

            let extra_dirs: Arc<Vec<PathBuf>> = self.extra_lib_dirs_snapshot();
            // ~keep A real load failure (truncated, wrong-architecture, or ABI-mismatched library)
            // ~keep must not be flattened into `LanguageNotFound`: that reports "your cached parser
            // ~keep is corrupt" as "no such language" and sends callers after the wrong bug.
            let mut load_failure: Option<Error> = None;
            for extra_dir in extra_dirs.iter() {
                match self.dynamic_loader.load_from_dir(name, extra_dir) {
                    Ok(language) => return Ok(language),
                    // ~keep Simply absent from this directory: keep searching the remaining ones.
                    Err(Error::LanguageNotFound(_)) => {}
                    Err(error) => {
                        tracing::warn!(
                            language = name,
                            dir = %extra_dir.display(),
                            error = %error,
                            "parser library found but could not be loaded"
                        );
                        load_failure = Some(error);
                    }
                }
            }
            if let Some(error) = load_failure {
                return Err(error);
            }
        }

        Err(Error::LanguageNotFound(name.to_string()))
    }

    /// The deduplicated set of every available language name, including aliases.
    ///
    /// Static, dynamic and alias names are borrowed `&'static str`; only names
    /// recovered from a directory listing or from the loaded-grammar map are
    /// owned, so a caller that just wants the count never allocates per name. ~keep
    fn collect_language_names(&self) -> AHashSet<Cow<'static, str>> {
        let mut seen: AHashSet<Cow<'static, str>> =
            self.static_lookup.keys().map(|&name| Cow::Borrowed(name)).collect();

        #[cfg(feature = "dynamic-loading")]
        {
            for &name in self.dynamic_loader.dynamic_names.iter() {
                seen.insert(Cow::Borrowed(name));
            }
            for name in self.dynamic_loader.cached_names() {
                seen.insert(Cow::Owned(name));
            }

            let extra_dirs: Arc<Vec<PathBuf>> = self.extra_lib_dirs_snapshot();
            for extra_dir in extra_dirs.iter() {
                let Ok(entries) = std::fs::read_dir(extra_dir) else {
                    continue;
                };
                for entry in entries.flatten() {
                    // ~keep Must map the C symbol back to the language id, otherwise the
                    // ~keep list reports `c_sharp`/`TSQL`/`apache_avro` instead of the
                    // ~keep documented names and inflates `language_count()`.
                    if let Some(lang) = lang_name_from_lib_filename(&entry.file_name().to_string_lossy()) {
                        seen.insert(Cow::Owned(lang));
                    }
                }
            }
        }

        for &(alias, target) in LANGUAGE_ALIASES {
            if seen.contains(target) {
                seen.insert(Cow::Borrowed(alias));
            }
        }
        seen
    }

    /// List all available language names, sorted and deduplicated.
    ///
    /// Includes statically compiled languages, dynamically loadable languages
    /// (if the `dynamic-loading` feature is enabled), and all configured aliases.
    pub fn available_languages(&self) -> Vec<String> {
        let mut langs: Vec<String> = self.collect_language_names().into_iter().map(Cow::into_owned).collect();
        langs.sort_unstable();
        langs
    }

    /// Check whether this language can be parsed right now, without downloading.
    ///
    /// Resolves aliases, then answers from exactly the lookup
    /// [`get_language`](Self::get_language) performs: the statically compiled
    /// table, the already-loaded dynamic grammars, and the parser shared
    /// libraries present in the primary and extra (download-cache) library
    /// directories. It never performs network I/O.
    ///
    /// A previous implementation consulted only the statically compiled table.
    /// That table is empty in every build that does not set `TSLP_LANGUAGES`,
    /// so the function answered `false` for languages this registry parses
    /// perfectly well. ~keep
    ///
    /// Contrast [`has_language`](Self::has_language), which is also `true` for a
    /// grammar that is merely *known to the manifest* and would have to be
    /// downloaded first. The pair distinguishes "we can parse it offline, now"
    /// from "we recognise the name".
    ///
    /// The first `true` answer for a dynamic grammar loads its shared library:
    /// loading is the only way to know the grammar is usable, since a truncated
    /// or wrong-architecture library exists on disk but cannot parse. Loads are
    /// cached process-wide, so repeat calls are cheap. ~keep
    ///
    /// ```no_run
    /// use tree_sitter_language_pack::{detect_language_from_extension, LanguageRegistry};
    ///
    /// let registry = LanguageRegistry::new();
    /// // Extension detection uses the static ext table for all 371 grammars.
    /// let lang = detect_language_from_extension("feature"); // always returns Some("gherkin")
    /// // Parser availability depends on what is compiled in or cached on disk.
    /// let can_parse = lang.map(|name| registry.has_parser(name)).unwrap_or(false);
    /// ```
    pub fn has_parser(&self, name: &str) -> bool {
        self.get_language(name).is_ok()
    }

    /// Check whether a language is available by name or alias.
    ///
    /// Returns `true` if the language can be loaded, either from the static
    /// table or from a dynamic library on disk.
    ///
    /// Every branch is one more way to answer `true`, so they are ordered
    /// cheapest-first and the filesystem is only consulted once every in-memory
    /// source has said no. Probing the loaded-grammar map and the manifest ahead
    /// of the `stat` calls is what keeps this off the syscall path for the
    /// languages a process actually uses. ~keep
    pub fn has_language(&self, name: &str) -> bool {
        let name = resolve_alias(name);
        if self.static_lookup.contains_key(name) {
            return true;
        }

        #[cfg(feature = "dynamic-loading")]
        {
            if self.dynamic_loader.dynamic_names.contains(&name) {
                return true;
            }
            if matches!(self.dynamic_loader.get_cached(name), Ok(Some(_))) {
                return true;
            }
        }

        // ~keep Download-enabled builds report known languages as available before on-demand fetch.
        #[cfg(feature = "download")]
        {
            if known_language(name) {
                return true;
            }
        }

        #[cfg(feature = "dynamic-loading")]
        {
            if self.dynamic_loader.lib_file_exists(name) {
                return true;
            }

            let extra_dirs: Arc<Vec<PathBuf>> = self.extra_lib_dirs_snapshot();
            for extra_dir in extra_dirs.iter() {
                if lib_path_in(extra_dir, name).exists() {
                    return true;
                }
            }
        }

        false
    }

    /// Return the total number of available languages (including aliases).
    ///
    /// Counts the same set [`available_languages`](Self::available_languages)
    /// lists, without materialising or sorting it. ~keep
    pub fn language_count(&self) -> usize {
        self.collect_language_names().len()
    }

    /// Parse source code and extract file intelligence based on config in a single pass.
    ///
    /// # Errors
    ///
    /// Returns [`Error::InvalidRange`] if the config is invalid (see
    /// [`ProcessConfig::validate`](crate::ProcessConfig::validate)) or if the source
    /// exceeds the configured
    /// [`max_source_bytes`](crate::ProcessConfig::max_source_bytes);
    /// [`Error::LanguageNotFound`] if the language is unknown; or
    /// [`Error::ParseFailed`] if parsing produces no tree.
    pub fn process(
        &self,
        source: &str,
        config: &crate::process_config::ProcessConfig,
    ) -> Result<crate::intel::types::ProcessResult, Error> {
        // ~keep Reject bad limits before parsing: a zero chunk size used to return an empty
        // ~keep chunk list and `Ok(())`, silently dropping the entire source.
        config.validate()?;
        config.check_source_size(source.len())?;

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

    #[cfg(all(feature = "dynamic-loading", not(target_arch = "wasm32")))]
    mod abi_gate {
        use super::super::dynamic::is_supported_abi;

        #[test]
        fn should_accept_every_abi_the_linked_runtime_declares_compatible() {
            for abi in tree_sitter::MIN_COMPATIBLE_LANGUAGE_VERSION..=tree_sitter::LANGUAGE_VERSION {
                assert!(
                    is_supported_abi(abi),
                    "runtime declares {abi} compatible but the gate rejects it"
                );
            }
        }

        #[test]
        fn should_reject_an_abi_below_the_minimum_compatible_version() {
            let below = tree_sitter::MIN_COMPATIBLE_LANGUAGE_VERSION - 1;
            assert!(
                !is_supported_abi(below),
                "ABI {below} is below the runtime minimum and must be rejected"
            );
        }

        #[test]
        fn should_reject_an_abi_above_the_runtime_version() {
            let above = tree_sitter::LANGUAGE_VERSION + 1;
            assert!(
                !is_supported_abi(above),
                "ABI {above} is newer than the runtime and must be rejected"
            );
        }
    }

    #[cfg(any(feature = "dynamic-loading", feature = "download"))]
    #[test]
    fn lib_filename_maps_c_symbol_back_to_language_name() {
        assert_eq!(
            lang_name_from_lib_filename("libtree_sitter_c_sharp.so").as_deref(),
            Some("csharp")
        );
        assert_eq!(
            lang_name_from_lib_filename("libtree_sitter_TSQL.dylib").as_deref(),
            Some("tsql")
        );
        assert_eq!(
            lang_name_from_lib_filename("tree_sitter_apache_avro.dll").as_deref(),
            Some("avro")
        );
        assert_eq!(
            lang_name_from_lib_filename("libtree-sitter-nu.so").as_deref(),
            Some("nushell")
        );
        assert_eq!(
            lang_name_from_lib_filename("libtree_sitter_rust.so").as_deref(),
            Some("rust")
        );
        assert_eq!(lang_name_from_lib_filename("README.md"), None);
        assert_eq!(lang_name_from_lib_filename("libtree_sitter_rust.txt"), None);
    }

    #[cfg(any(feature = "dynamic-loading", feature = "download"))]
    #[test]
    fn library_file_name_applies_the_c_symbol_override() {
        assert!(
            library_file_name("csharp").contains("tree_sitter_c_sharp."),
            "{}",
            library_file_name("csharp")
        );
        assert!(
            library_file_name("rust").contains("tree_sitter_rust."),
            "{}",
            library_file_name("rust")
        );
    }

    #[cfg(feature = "dynamic-loading")]
    #[test]
    fn available_languages_reports_language_names_not_c_symbols_for_extra_dirs() {
        let dir = tempfile::Builder::new()
            .prefix("tslp-extra-libs-")
            .tempdir()
            .expect("temporary libs directory should be created");
        for symbol in ["c_sharp", "TSQL", "apache_avro", "embedded_template", "nu", "vb_dotnet"] {
            std::fs::write(dir.path().join(format!("libtree_sitter_{symbol}.so")), b"")
                .expect("stub parser library should be written");
        }

        let registry = LanguageRegistry::new();
        registry
            .try_add_extra_libs_dir(dir.path().to_path_buf())
            .expect("registering an extra libs dir should succeed");
        let langs = registry.available_languages();

        for (symbol, language) in [
            ("c_sharp", "csharp"),
            ("TSQL", "tsql"),
            ("apache_avro", "avro"),
            ("embedded_template", "embeddedtemplate"),
            ("nu", "nushell"),
            ("vb_dotnet", "vb"),
        ] {
            assert!(langs.iter().any(|l| l == language), "missing language name {language}");
            assert!(
                !langs.iter().any(|l| l == symbol),
                "leaked raw C symbol {symbol} into available_languages()"
            );
        }
        assert_eq!(registry.language_count(), langs.len());
    }

    #[cfg(all(feature = "dynamic-loading", not(target_arch = "wasm32")))]
    #[test]
    fn should_distinguish_an_absent_library_from_an_unloadable_one() {
        // ~keep get_language falls through to the extra lib dirs on LanguageNotFound ONLY, so that a
        // ~keep relocated build can still be satisfied by the download cache. If `load` ever reports an
        // ~keep absent file as DynamicLoad — or an unloadable one as LanguageNotFound — that fallthrough
        // ~keep silently stops working and a corrupt parser starts reading as "no such language".
        let dir = tempfile::Builder::new()
            .prefix("tslp-load-semantics-")
            .tempdir()
            .expect("temporary libs directory should be created");
        let loader = super::dynamic::DynamicLoader::new(dir.path().to_path_buf(), vec!["python"]);

        match loader.load("python") {
            Err(Error::LanguageNotFound(_)) => {}
            Ok(_) => panic!("an absent library must not load successfully"),
            Err(error) => panic!("an absent library must report LanguageNotFound, got: {error}"),
        }

        std::fs::write(super::lib_path_in(dir.path(), "python"), b"not a shared library")
            .expect("stub parser library should be written");

        match loader.load("python") {
            Err(Error::DynamicLoad(_)) => {}
            Ok(_) => panic!("a truncated library must not load successfully"),
            Err(error) => panic!("a present-but-unloadable library must report DynamicLoad, got: {error}"),
        }
    }

    #[cfg(feature = "dynamic-loading")]
    #[test]
    fn should_recover_a_poisoned_read_but_propagate_a_poisoned_write_of_extra_lib_dirs() {
        // ~keep A fresh, local `LanguageRegistry` — this test never touches process-wide
        // ~keep statics, so it is safe to run in any order and in parallel with every
        // ~keep other test in the suite.
        let registry = LanguageRegistry::new();

        // ~keep Poison `extra_lib_dirs` by panicking while holding its write guard.
        let dirs_arc = Arc::clone(&registry.extra_lib_dirs);
        let poison_result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let _guard = dirs_arc.write().expect("lock should not already be poisoned");
            panic!("intentional panic to poison extra_lib_dirs for this test");
        }));
        assert!(poison_result.is_err(), "the intentional panic should have unwound");
        assert!(
            registry.extra_lib_dirs.is_poisoned(),
            "extra_lib_dirs should be poisoned after the panic"
        );

        // ~keep Item 4: the read side recovers — `extra_lib_dirs` only ever grows, so a
        // ~keep poisoned read is still coherent, and reporting a language absent because of
        // ~keep an unrelated panic elsewhere would be a wrong answer, not a safe failure.
        let snapshot = registry.extra_lib_dirs_snapshot();
        assert!(
            snapshot.is_empty(),
            "no directories were ever registered, so the recovered snapshot must be empty"
        );

        // ~keep Item 2: the write side propagates instead of silently doing nothing — a
        // ~keep caller registering a cache directory must be able to tell registration failed.
        let result = registry.try_add_extra_libs_dir(PathBuf::from("/tmp/should-not-be-registered"));
        assert!(
            matches!(result, Err(Error::LockPoisoned(_))),
            "try_add_extra_libs_dir must surface a poisoned lock as an error, got: {result:?}"
        );
    }

    #[cfg(all(feature = "dynamic-loading", not(target_arch = "wasm32")))]
    #[test]
    fn should_keep_loading_dynamic_languages_after_a_panic_poisons_the_language_load_lock() {
        let registry = LanguageRegistry::new();
        let Some(language_name) = registry.dynamic_loader.dynamic_names.first().copied() else {
            return;
        };

        // ~keep LANGUAGE_LOAD_LOCK guards no data (it only serializes not-yet-loaded dynamic
        // ~keep library loads), so poisoning it should never brick `get_language`. Poisoning
        // ~keep is process-wide and sticky, which is safe here only because every acquisition
        // ~keep site recovers via `recover_poisoned_lock`; this test leaves the lock poisoned
        // ~keep for the rest of the process on purpose.
        let poison_result = std::panic::catch_unwind(|| {
            let _guard = LANGUAGE_LOAD_LOCK
                .lock()
                .expect("LANGUAGE_LOAD_LOCK should not already be poisoned");
            panic!("intentional panic to poison LANGUAGE_LOAD_LOCK for this test");
        });
        assert!(poison_result.is_err(), "the intentional panic should have unwound");
        assert!(
            LANGUAGE_LOAD_LOCK.is_poisoned(),
            "LANGUAGE_LOAD_LOCK should be poisoned after the panic"
        );

        let result = registry.get_language(language_name);
        assert!(
            result.is_ok(),
            "get_language must recover a poisoned LANGUAGE_LOAD_LOCK instead of failing forever, got: {result:?}"
        );
    }

    #[cfg(feature = "download")]
    #[test]
    fn known_languages_is_sorted() {
        assert!(
            KNOWN_LANGUAGES.windows(2).all(|pair| pair[0] < pair[1]),
            "KNOWN_LANGUAGES must stay sorted: has_language() binary-searches it"
        );
    }

    #[cfg(feature = "download")]
    #[test]
    fn known_language_matches_linear_scan() {
        for name in KNOWN_LANGUAGES {
            assert!(known_language(name), "binary search missed known language {name}");
        }
        assert!(!known_language("nonexistent_lang_xyz"));
        assert!(!known_language(""));
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

        let first = LanguageRegistry::new();
        let lib_path = lib_path_in(&first.dynamic_loader.libs_dir, language_name);
        let first_language = first
            .get_language(language_name)
            .expect("first registry should load dynamic language");
        assert!(
            dynamic::library_is_loaded_for_tests(&lib_path),
            "first registry should have populated the process-wide cache for {}",
            lib_path.display()
        );

        let second = LanguageRegistry::new();
        let second_language = second
            .get_language(language_name)
            .expect("second registry should reuse process-loaded dynamic library");

        // ~keep Asserted per-path, not on the global library count. LOADED_LIBRARIES is keyed by
        // ~keep path, so a redundant dlopen would REPLACE the same key and leave the count
        // ~keep unchanged — the previous count comparison could not observe the property this test
        // ~keep is named for, and raced with every other test that loads a grammar.
        assert!(
            dynamic::library_is_loaded_for_tests(&lib_path),
            "second registry must reuse the cached library rather than evict it"
        );
        assert_eq!(first_language.abi_version(), second_language.abi_version());
    }

    #[cfg(all(feature = "dynamic-loading", not(target_arch = "wasm32")))]
    #[test]
    fn should_keep_loaded_libraries_present_after_the_owning_registry_is_dropped() {
        // ~keep LOADED_LIBRARIES has exactly one mutation site — the guarded `insert` in
        // ~keep `language_from_process_library` — and no `remove`/`clear` anywhere in the
        // ~keep crate. That absence is the invariant that lets a `Language` (which borrows
        // ~keep code out of the loaded `libloading::Library`) outlive the `LanguageRegistry`
        // ~keep scope that produced it. This test cannot observe the absence of a `remove`
        // ~keep call directly, so it asserts the externally visible consequence instead:
        // ~keep dropping every registry that loaded a library must never shrink the map.
        let registry = LanguageRegistry::new();
        let Some(language_name) = registry.dynamic_loader.dynamic_names.first().copied() else {
            return;
        };
        let lib_path = lib_path_in(&registry.dynamic_loader.libs_dir, language_name);
        let Ok(_language) = registry.get_language(language_name) else {
            return;
        };
        assert!(
            dynamic::library_is_loaded_for_tests(&lib_path),
            "library should be loaded after get_language"
        );

        drop(registry);

        assert!(
            dynamic::library_is_loaded_for_tests(&lib_path),
            "LOADED_LIBRARIES must retain the entry after every owning registry is dropped \
             — nothing in this crate removes from it"
        );
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

    #[cfg(any(feature = "dynamic-loading", feature = "download"))]
    #[test]
    fn should_resolve_every_alias_to_a_stable_canonical_name() {
        // ~keep The download manifest is keyed by canonical name only, so any caller
        // taking user input must resolve before consulting it. `download` shipped
        // without doing so and rejected `shell` as an unknown language.
        for &(alias, target) in LANGUAGE_ALIASES {
            assert_eq!(
                resolve_alias(alias),
                target,
                "alias '{alias}' must resolve to '{target}'"
            );
            assert_eq!(
                resolve_alias(target),
                target,
                "canonical name '{target}' must be a fixed point, or resolving twice would corrupt it"
            );
            assert!(
                !LANGUAGE_ALIASES.iter().any(|&(a, _)| a == target),
                "alias target '{target}' must not itself be an alias — resolution is single-pass"
            );
        }
    }

    #[cfg(any(feature = "dynamic-loading", feature = "download"))]
    #[test]
    fn should_leave_unknown_and_canonical_names_untouched() {
        assert_eq!(resolve_alias("rust"), "rust");
        assert_eq!(resolve_alias("definitely_not_a_language"), "definitely_not_a_language");
        assert_eq!(resolve_alias(""), "");
    }

    #[cfg(feature = "download")]
    #[test]
    fn should_report_every_alias_that_resolves_to_a_canonical_target() {
        assert_eq!(aliases_for("bash"), vec!["shell"]);
        assert_eq!(aliases_for("starlark"), vec!["bazel"]);
        assert_eq!(
            aliases_for("definitely_not_a_language"),
            Vec::<&str>::new(),
            "a target with no alias must report an empty list, not panic"
        );
    }

    #[cfg(feature = "download")]
    #[test]
    fn should_agree_with_resolve_alias_for_every_alias_in_the_table() {
        // ~keep aliases_for is the reverse of resolve_alias; every alias must round-trip.
        for &(alias, target) in LANGUAGE_ALIASES {
            assert!(
                aliases_for(target).contains(&alias),
                "aliases_for({target:?}) must contain {alias:?}"
            );
        }
    }
}
