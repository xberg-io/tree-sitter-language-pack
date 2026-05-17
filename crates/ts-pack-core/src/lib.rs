//! # tree-sitter-language-pack
//!
//! Pre-compiled tree-sitter grammars for 305 programming languages with
//! a unified API for parsing, analysis, and intelligent code chunking.
//!
//! ## Quick Start
//!
//! ```no_run
//! use tree_sitter_language_pack::{ProcessConfig, available_languages, has_language, process};
//!
//! // Check available languages
//! let langs = available_languages();
//! assert!(has_language("python"));
//!
//! // Process source code
//! let config = ProcessConfig::new("python").all();
//! let result = process("def hello(): pass", &config).unwrap();
//! println!("Language: {}", result.language);
//! println!("Functions: {}", result.structure.len());
//! ```
//!
//! ## Parser API
//!
//! ```no_run
//! use tree_sitter_language_pack::get_parser;
//!
//! let mut parser = get_parser("python")?;
//! let tree = parser.parse("def foo(): pass").expect("parse failed");
//! assert_eq!(tree.root_node().kind(), "module");
//! # Ok::<(), tree_sitter_language_pack::Error>(())
//! ```
//!
//! ## Modules
//!
//! - [`registry`] - Thread-safe language registry for parser lookup
//! - [`intel`] - Source code intelligence extraction (structure, imports, exports, etc.)
//! - [`parse`] - Low-level tree-sitter parsing utilities
//! - [`text_splitter`] - Syntax-aware code chunking
//! - [`process_config`] - Configuration for the `process` pipeline
//! - [`pack_config`] - Configuration for the language pack (cache dir, languages to download)
//! - [`error`] - Error types

pub mod error;
pub(crate) mod extensions;
pub(crate) mod intel;
#[cfg(feature = "serde")]
pub(crate) mod json_utils;
pub mod pack_config;
pub(crate) mod parse;
pub mod parsing;
pub mod process_config;
pub(crate) mod queries;
pub mod registry;
pub(crate) mod text_splitter;

#[cfg(feature = "config")]
pub(crate) mod config;
#[cfg(feature = "config")]
pub(crate) mod definitions;
#[cfg(feature = "download")]
pub mod download;

// ── Public API re-exports ────────────────────────────────────────────────────
// Only items that are part of the polyglot binding surface are re-exported here.
// Internal helpers, tuple-returning functions, and implementation details stay
// behind pub(crate) modules.

pub use error::Error;
pub use extensions::{detect_language_from_content, detect_language_from_extension, detect_language_from_path};
pub use intel::types::{
    ChunkContext, CodeChunk, CommentInfo, CommentKind, Diagnostic, DiagnosticSeverity, DocSection, DocstringFormat,
    DocstringInfo, ExportInfo, ExportKind, FileMetrics, ImportInfo, ProcessResult, Span, StructureItem, StructureKind,
    SymbolInfo, SymbolKind,
};
pub use pack_config::{PackConfig, TlsRootsMode};
pub use parsing::{ByteRange, Node, Parser, Point, Tree, TreeCursor};
pub use process_config::ProcessConfig;
pub use queries::{get_highlights_query, get_injections_query, get_locals_query};
pub use registry::LanguageRegistry;
pub use tree_sitter::Language;

#[cfg(feature = "download")]
pub use download::DownloadManager;

use std::sync::LazyLock;
#[cfg(feature = "download")]
use std::sync::{Mutex, RwLock};

static REGISTRY: LazyLock<LanguageRegistry> = LazyLock::new(LanguageRegistry::new);

#[cfg(feature = "download")]
static CACHE_REGISTERED: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

#[cfg(feature = "download")]
static CUSTOM_CACHE_DIR: LazyLock<RwLock<Option<std::path::PathBuf>>> = LazyLock::new(|| RwLock::new(None));

#[cfg(feature = "download")]
static DOWNLOAD_CACHE_LOCK: Mutex<()> = Mutex::new(());

/// Get a tree-sitter [`Language`] by name using the global registry.
///
/// Resolves language aliases (e.g., `"shell"` maps to `"bash"`).
/// When the `download` feature is enabled (default), automatically downloads
/// the parser from GitHub releases if not found locally.
///
/// # Errors
///
/// Returns [`Error::LanguageNotFound`] if the language is not recognized,
/// or [`Error::Download`] if auto-download fails.
///
/// # Example
///
/// ```no_run
/// use tree_sitter_language_pack::{get_language, Parser};
///
/// let _lang = get_language("python")?;
/// let mut parser = Parser::new();
/// parser.set_language("python")?;
/// let tree = parser.parse("x = 1").expect("parse failed");
/// assert_eq!(tree.root_node().kind(), "module");
/// # Ok::<(), tree_sitter_language_pack::Error>(())
/// ```
pub fn get_language(name: &str) -> Result<Language, Error> {
    #[cfg(feature = "download")]
    {
        let _cache_guard = DOWNLOAD_CACHE_LOCK
            .lock()
            .map_err(|e| Error::LockPoisoned(e.to_string()))?;
        if let Ok(lang) = REGISTRY.get_language(name) {
            return Ok(lang);
        }
        ensure_cache_registered()?;
        let cache_dir = effective_cache_dir()?;
        let dm = DownloadManager::with_cache_dir(env!("CARGO_PKG_VERSION"), cache_dir);
        let resolved = crate::registry::resolve_alias(name);
        dm.ensure_languages(&[resolved])?;
        REGISTRY.get_language(resolved)
    }

    #[cfg(not(feature = "download"))]
    {
        // Fast path: check registry directly (no outer lock needed)
        if let Ok(lang) = REGISTRY.get_language(name) {
            return Ok(lang);
        }
        Err(Error::LanguageNotFound(name.to_string()))
    }
}

/// Get a [`Parser`] pre-configured for the given language.
///
/// This is a convenience function that calls [`get_language`] and configures
/// a new parser in one step.
///
/// # Errors
///
/// Returns [`Error::LanguageNotFound`] if the language is not recognized, or
/// [`Error::ParserSetup`] if the language cannot be applied to the parser.
///
/// # Example
///
/// ```no_run
/// use tree_sitter_language_pack::get_parser;
///
/// let mut parser = get_parser("rust")?;
/// let tree = parser.parse("fn main() {}").expect("parse failed");
/// assert!(!tree.root_node().has_error());
/// # Ok::<(), tree_sitter_language_pack::Error>(())
/// ```
pub fn get_parser(name: &str) -> Result<Parser, Error> {
    let mut parser = Parser::new();
    parser.set_language(name)?;
    Ok(parser)
}

/// Detect language name from a file path or extension.
///
/// This compatibility alias matches the pre-Alef Python binding API.
pub fn detect_language(path: &str) -> Option<&'static str> {
    detect_language_from_path(path).or_else(|| detect_language_from_extension(path.trim_start_matches('.')))
}

/// List all available language names (sorted, deduplicated, includes aliases).
///
/// Returns names of both statically compiled and dynamically loadable languages,
/// plus any configured aliases.
///
/// # Example
///
/// ```no_run
/// use tree_sitter_language_pack::available_languages;
///
/// let langs = available_languages();
/// for name in &langs {
///     println!("{}", name);
/// }
/// ```
pub fn available_languages() -> Vec<String> {
    #[cfg(feature = "download")]
    let _ = ensure_cache_registered();
    REGISTRY.available_languages()
}

/// Check if a language is available by name or alias.
///
/// Returns `true` if the language can be loaded (statically compiled,
/// dynamically available, or a known alias for one of these).
///
/// # Example
///
/// ```no_run
/// use tree_sitter_language_pack::has_language;
///
/// assert!(has_language("python"));
/// assert!(has_language("shell")); // alias for "bash"
/// assert!(!has_language("nonexistent_language"));
/// ```
pub fn has_language(name: &str) -> bool {
    #[cfg(feature = "download")]
    let _ = ensure_cache_registered();
    REGISTRY.has_language(name)
}

/// Return the number of available languages.
///
/// Includes statically compiled languages, dynamically loadable languages,
/// and aliases.
///
/// # Example
///
/// ```no_run
/// use tree_sitter_language_pack::language_count;
///
/// let count = language_count();
/// println!("{} languages available", count);
/// ```
pub fn language_count() -> usize {
    #[cfg(feature = "download")]
    let _ = ensure_cache_registered();
    REGISTRY.language_count()
}

/// Process source code and extract file intelligence using the global registry.
///
/// Parses the source with tree-sitter and extracts metrics, structure, imports,
/// exports, comments, docstrings, symbols, diagnostics, and/or chunks based on
/// the flags set in [`ProcessConfig`].
///
/// # Errors
///
/// Returns an error if the language is not found or parsing fails.
///
/// # Example
///
/// ```no_run
/// use tree_sitter_language_pack::{ProcessConfig, process};
///
/// let config = ProcessConfig::new("python").all();
/// let result = process("def hello(): pass", &config).unwrap();
/// println!("Language: {}", result.language);
/// println!("Lines: {}", result.metrics.total_lines);
/// println!("Structures: {}", result.structure.len());
/// ```
pub fn process(source: &str, config: &ProcessConfig) -> Result<ProcessResult, Error> {
    // Trigger auto-download for the requested language if not already cached.
    // get_language() contains the download fallback path; REGISTRY.process() does not.
    #[cfg(feature = "download")]
    get_language(&config.language)?;

    REGISTRY.process(source, config)
}

// ──────────────────────────────────────────────────────────────────────────────
// Download feature helpers and public API
// ──────────────────────────────────────────────────────────────────────────────

#[cfg(feature = "download")]
fn ensure_cache_registered() -> Result<(), Error> {
    if CACHE_REGISTERED.load(std::sync::atomic::Ordering::Acquire) {
        return Ok(());
    }
    let cache_dir = effective_cache_dir()?;
    // add_extra_libs_dir uses interior mutability — no outer write lock needed
    REGISTRY.add_extra_libs_dir(cache_dir);
    CACHE_REGISTERED.store(true, std::sync::atomic::Ordering::Release);
    Ok(())
}

#[cfg(feature = "download")]
fn effective_cache_dir() -> Result<std::path::PathBuf, Error> {
    let custom = CUSTOM_CACHE_DIR
        .read()
        .map_err(|e| Error::LockPoisoned(e.to_string()))?;
    match custom.as_ref() {
        Some(dir) => Ok(dir.clone()),
        None => DownloadManager::default_cache_dir(env!("CARGO_PKG_VERSION")),
    }
}

/// Initialize the language pack with the given configuration.
///
/// Applies any custom cache directory, then downloads all languages and groups
/// specified in the config. This is the recommended entry point when you want
/// to pre-warm the cache before use.
///
/// # Errors
///
/// Returns an error if configuration cannot be applied or if downloads fail.
///
/// # Example
///
/// ```no_run
/// use tree_sitter_language_pack::{PackConfig, init};
///
/// let config = PackConfig {
///     cache_dir: None,
///     languages: Some(vec!["python".to_string(), "rust".to_string()]),
///     groups: None,
/// };
/// init(&config).unwrap();
/// ```
#[cfg(feature = "download")]
pub fn init(config: &PackConfig) -> Result<(), Error> {
    let _cache_guard = DOWNLOAD_CACHE_LOCK
        .lock()
        .map_err(|e| Error::LockPoisoned(e.to_string()))?;
    configure_inner(config)?;
    if let Some(ref languages) = config.languages {
        download_inner(languages)?;
    }
    if let Some(ref groups) = config.groups {
        let cache_dir = effective_cache_dir()?;
        let dm = DownloadManager::with_cache_dir(env!("CARGO_PKG_VERSION"), cache_dir);
        for group in groups {
            dm.ensure_group(group)?;
        }
    }
    ensure_cache_registered()?;
    Ok(())
}

/// Apply download configuration without downloading anything.
///
/// Use this to set a custom cache directory before the first call to
/// [`get_language`] or any download function. Changing the cache dir
/// after languages have been registered has no effect on already-loaded
/// languages.
///
/// # Errors
///
/// Returns an error if the lock cannot be acquired.
///
/// # Example
///
/// ```no_run
/// use std::path::PathBuf;
/// use tree_sitter_language_pack::{PackConfig, configure};
///
/// let config = PackConfig {
///     cache_dir: Some(PathBuf::from("/tmp/my-parsers")),
///     languages: None,
///     groups: None,
/// };
/// configure(&config).unwrap();
/// ```
#[cfg(feature = "download")]
pub fn configure(config: &PackConfig) -> Result<(), Error> {
    let _cache_guard = DOWNLOAD_CACHE_LOCK
        .lock()
        .map_err(|e| Error::LockPoisoned(e.to_string()))?;
    configure_inner(config)
}

#[cfg(feature = "download")]
fn configure_inner(config: &PackConfig) -> Result<(), Error> {
    if let Some(ref dir) = config.cache_dir {
        let mut custom = CUSTOM_CACHE_DIR
            .write()
            .map_err(|e| Error::LockPoisoned(e.to_string()))?;
        *custom = Some(dir.clone());
        // Reset cache registration so the new directory gets registered on next use.
        // NOTE: Old directories remain in the registry but won't have new files since
        // add_extra_libs_dir deduplicates, and the directory scanning is independent
        // per path. This is acceptable behavior and avoids complex cleanup logic.
        CACHE_REGISTERED.store(false, std::sync::atomic::Ordering::Release);
    }
    Ok(())
}

/// Download specific languages to the local cache.
///
/// Returns the number of requested languages available after the call. Already
/// compiled or cached languages are included in the count.
///
/// # Errors
///
/// Returns an error if any language is not available in the manifest or if
/// the download fails.
///
/// # Example
///
/// ```no_run
/// use tree_sitter_language_pack::download;
///
/// let count = download(&["python", "rust", "typescript"]).unwrap();
/// println!("Ensured {} languages", count);
/// ```
#[cfg(feature = "download")]
pub fn download<S: AsRef<str>>(names: &[S]) -> Result<usize, Error> {
    let _cache_guard = DOWNLOAD_CACHE_LOCK
        .lock()
        .map_err(|e| Error::LockPoisoned(e.to_string()))?;
    download_inner(names)
}

#[cfg(feature = "download")]
fn download_inner<S: AsRef<str>>(names: &[S]) -> Result<usize, Error> {
    ensure_cache_registered()?;
    let cache_dir = effective_cache_dir()?;
    let dm = DownloadManager::with_cache_dir(env!("CARGO_PKG_VERSION"), cache_dir);
    let unavailable: Vec<&str> = names
        .iter()
        .map(|s| s.as_ref())
        .filter(|name| !REGISTRY.has_language(name))
        .collect();
    dm.ensure_languages(&unavailable)?;
    let unique: std::collections::BTreeSet<&str> = names.iter().map(|s| s.as_ref()).collect();
    Ok(unique.len())
}

/// Download all available languages from the remote manifest.
///
/// Downloads the platform bundle and extracts every library it contains.
/// Languages that appear in the manifest but are absent from the bundle
/// (e.g. grammars that failed to compile at release time) are silently
/// skipped — they are not treated as an error.
///
/// Returns the total number of languages now available (statically compiled
/// plus downloaded and cached).
///
/// # Errors
///
/// Returns an error if the manifest cannot be fetched or the bundle download fails.
///
/// # Example
///
/// ```no_run
/// use tree_sitter_language_pack::download_all;
///
/// let count = download_all().unwrap();
/// println!("{} languages available", count);
/// ```
#[cfg(feature = "download")]
pub fn download_all() -> Result<usize, Error> {
    let _cache_guard = DOWNLOAD_CACHE_LOCK
        .lock()
        .map_err(|e| Error::LockPoisoned(e.to_string()))?;
    ensure_cache_registered()?;
    let cache_dir = effective_cache_dir()?;
    let dm = DownloadManager::with_cache_dir(env!("CARGO_PKG_VERSION"), cache_dir);
    dm.download_all_best_effort()?;
    Ok(REGISTRY.language_count())
}

/// Return all language names available in the remote manifest (305).
///
/// Fetches (and caches) the remote manifest to discover the full list of
/// downloadable languages. Use [`downloaded_languages`] to list what is
/// already cached locally.
///
/// # Errors
///
/// Returns an error if the manifest cannot be fetched.
///
/// # Example
///
/// ```no_run
/// use tree_sitter_language_pack::manifest_languages;
///
/// let langs = manifest_languages().unwrap();
/// println!("{} languages available for download", langs.len());
/// ```
#[cfg(feature = "download")]
pub fn manifest_languages() -> Result<Vec<String>, Error> {
    let cache_dir = effective_cache_dir()?;
    let dm = DownloadManager::with_cache_dir(env!("CARGO_PKG_VERSION"), cache_dir);
    let manifest = dm.fetch_manifest()?;
    let mut langs: Vec<String> = manifest.languages.keys().cloned().collect();
    langs.sort_unstable();
    Ok(langs)
}

/// Return languages that are already downloaded and cached locally.
///
/// Does not perform any network requests. Returns an empty list if the
/// cache directory does not exist or cannot be read.
///
/// # Example
///
/// ```no_run
/// use tree_sitter_language_pack::downloaded_languages;
///
/// let langs = downloaded_languages();
/// println!("{} languages already cached", langs.len());
/// ```
#[cfg(feature = "download")]
pub fn downloaded_languages() -> Vec<String> {
    let cache_dir = match effective_cache_dir() {
        Ok(dir) => dir,
        Err(_) => return Vec::new(),
    };
    let dm = DownloadManager::with_cache_dir(env!("CARGO_PKG_VERSION"), cache_dir);
    dm.installed_languages()
}

/// Delete all cached parser shared libraries.
///
/// Resets the cache registration so the next call to [`get_language`] or
/// a download function will re-register the (now empty) cache directory.
///
/// # Errors
///
/// Returns an error if the cache directory cannot be removed.
///
/// # Example
///
/// ```no_run
/// use tree_sitter_language_pack::clean_cache;
///
/// clean_cache().unwrap();
/// println!("Cache cleared");
/// ```
#[cfg(feature = "download")]
pub fn clean_cache() -> Result<(), Error> {
    let _cache_guard = DOWNLOAD_CACHE_LOCK
        .lock()
        .map_err(|e| Error::LockPoisoned(e.to_string()))?;
    let cache_dir = effective_cache_dir()?;
    let dm = DownloadManager::with_cache_dir(env!("CARGO_PKG_VERSION"), cache_dir);
    dm.clean_cache()?;
    CACHE_REGISTERED.store(false, std::sync::atomic::Ordering::Release);
    Ok(())
}

/// Return the effective cache directory path.
///
/// This is either the custom path set via [`configure`] / [`init`] or the
/// default: `~/.cache/tree-sitter-language-pack/v{version}/libs/`.
///
/// # Errors
///
/// Returns an error if the system cache directory cannot be determined.
///
/// # Example
///
/// ```no_run
/// use tree_sitter_language_pack::cache_dir;
///
/// let dir = cache_dir().unwrap();
/// println!("Cache directory: {dir}");
/// ```
#[cfg(feature = "download")]
pub fn cache_dir() -> Result<String, Error> {
    effective_cache_dir().map(|p| p.to_string_lossy().into_owned())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_available_languages() {
        let langs = available_languages();
        // With zero default parsers, this may be empty unless lang-* features are enabled
        // Verify available_languages doesn't panic; may be empty without lang-* features
        let _ = langs;
    }

    #[test]
    fn test_has_language() {
        let langs = available_languages();
        if !langs.is_empty() {
            assert!(has_language(&langs[0]));
        }
        assert!(!has_language("nonexistent_language_xyz"));
    }

    #[test]
    fn test_get_language_invalid() {
        let result = get_language("nonexistent_language_xyz");
        assert!(result.is_err());
    }

    #[test]
    #[ignore = "loads all 305 dynamic libraries — run with --ignored"]
    fn test_get_language_and_parse() {
        let langs = available_languages();
        for lang_name in &langs {
            let lang = get_language(lang_name.as_str())
                .unwrap_or_else(|e| panic!("Failed to load language '{lang_name}': {e}"));
            let mut parser = tree_sitter::Parser::new();
            parser
                .set_language(&lang)
                .unwrap_or_else(|e| panic!("Failed to set language '{lang_name}': {e}"));
            let tree = parser.parse("x", None);
            assert!(tree.is_some(), "Parser for '{lang_name}' should parse a string");
        }
    }

    #[test]
    fn test_get_parser() {
        let langs = available_languages();
        if let Some(first) = langs.first() {
            let parser = get_parser(first.as_str());
            assert!(parser.is_ok(), "get_parser should succeed for '{first}'");
        }
    }

    #[test]
    fn test_pack_config_default() {
        let config = PackConfig::default();
        assert!(config.cache_dir.is_none());
        assert!(config.languages.is_none());
        assert!(config.groups.is_none());
    }
}
