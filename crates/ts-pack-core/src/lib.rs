//! # tree-sitter-language-pack
//!
//! Pre-compiled tree-sitter grammars for 372 programming languages with
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
//! - [`parsing`] - Ownership-safe tree-sitter parser, tree, node, and cursor wrappers
//! - [`process_config`] - Configuration for the [`process`] pipeline
//! - [`pack_config`] - Configuration for the language pack (cache dir, languages to download)
//! - [`error`] - Error types
//!
//! Source-code intelligence and the syntax-aware text splitter are exposed via re-exports
//! from the crate root (for example [`process`] and [`StructureItem`]); their backing
//! modules are internal implementation details.

/// Error types for tree-sitter language pack operations.
pub mod error;
pub(crate) mod extensions;
pub(crate) mod intel;
#[cfg(feature = "serde")]
#[doc(hidden)]
pub mod json_utils;
/// Configuration for the language pack (cache directory, languages to pre-download).
pub mod pack_config;
pub(crate) mod parse;
/// Ownership-safe tree-sitter parser, tree, node, and cursor wrappers for FFI.
pub mod parsing;
/// Configuration for the `process()` pipeline (which analysis features to enable).
pub mod process_config;
pub(crate) mod queries;
/// Process-wide cache of compiled tree-sitter queries (`Arc<Query>`), keyed by language + kind.
pub mod query_cache;
/// Thread-safe language registry mapping names to compiled tree-sitter parsers.
pub mod registry;
pub(crate) mod text_splitter;

#[cfg(feature = "config")]
pub(crate) mod definitions;
/// Download manager for fetching pre-built parser shared libraries from GitHub releases.
#[cfg(feature = "download")]
pub mod download;

pub use error::Error;
#[cfg(feature = "serde")]
pub use extensions::extension_ambiguity_json;
pub use extensions::{
    detect_language_from_content, detect_language_from_extension, detect_language_from_path, extension_ambiguity,
};
pub use intel::types::{
    ChunkContext, CodeChunk, CommentInfo, CommentKind, DataAttribute, DataNode, DataNodeKind, Diagnostic,
    DiagnosticSeverity, DocSection, DocstringFormat, DocstringInfo, ExportInfo, ExportKind, FileMetrics, ImportInfo,
    ProcessResult, Span, StructureItem, StructureKind, SymbolInfo, SymbolKind,
};
pub use pack_config::{PackConfig, TlsRootsMode};
pub use parsing::{ByteRange, Node, Parser, Point, Tree, TreeCursor};
pub use process_config::ProcessConfig;
pub use queries::{
    get_folds_query, get_highlights_query, get_indents_query, get_injections_query, get_locals_query, get_tags_query,
};
pub use query_cache::{QueryKind, get_query};
pub use registry::LanguageRegistry;
pub use tree_sitter::Language;

#[cfg(feature = "download")]
pub use download::DownloadManager;

use std::sync::LazyLock;
#[cfg(feature = "download")]
use std::sync::{Mutex, RwLock};

static REGISTRY: LazyLock<LanguageRegistry> = LazyLock::new(LanguageRegistry::new);

/// Recover a poisoned `Mutex<()>` guard, logging a WARN instead of propagating.
///
/// Restricted to locks proven to guard no data — `DOWNLOAD_CACHE_LOCK`,
/// `LANGUAGE_LOAD_LOCK`, and the per-language locks in `parse::STATEFUL_SCANNER_LOCKS`
/// (one `Mutex<()>` per grammar whose external scanner keeps mutable process-global
/// state, e.g. `properties`; see `parse::lock_for_stateful_scanner`) all serialize
/// side effects only, so a panic while one is held can never leave protected state
/// half-written. Poisoning there buys nothing but an unrecoverable failure: every
/// later call through the same lock — including `clean_cache`, the natural recovery
/// path — would return `LockPoisoned` forever, requiring a process restart. The
/// `Mutex<()>` parameter type is deliberate: it is a compile error to reuse this on
/// a lock that guards real data. ~keep
pub(crate) fn recover_poisoned_lock<'a>(
    lock_name: &str,
    poisoned: std::sync::PoisonError<std::sync::MutexGuard<'a, ()>>,
) -> std::sync::MutexGuard<'a, ()> {
    tracing::warn!(
        lock = lock_name,
        "recovered a poisoned lock after a panicking critical section"
    );
    poisoned.into_inner()
}

/// Cache directory most recently registered with `REGISTRY.add_extra_libs_dir`.
///
/// Compared by path, not by a boolean flag: `ensure_cache_registered` recomputes
/// `effective_cache_dir()` fresh — once for its lock-free fast-path check, and again
/// after acquiring the write lock — and only skips re-registration when the current
/// value exactly matches what was last registered here. A boolean "have we
/// registered something" flag cannot express that — a racing `configure()` can flip
/// it back to "not registered" and reset `CUSTOM_CACHE_DIR` in between another
/// thread's read of the flag and its read of the cache dir, so that thread registers
/// the OLD path and then marks registration done, permanently hiding the new
/// directory.
///
/// The write-lock re-read is what actually closes that window. An earlier version
/// computed `cache_dir` once, before ever taking the write lock, and the "double
/// check under the write lock" compared against that same pre-lock snapshot — a
/// `configure()` racing in between the initial read and the lock acquisition could
/// still make the write path register a directory that was already stale by the
/// time it ran. Re-reading `effective_cache_dir()` a second time, after the write
/// lock is held, compares against current state instead of a snapshot. This is safe
/// against deadlock: `effective_cache_dir()` takes only `CUSTOM_CACHE_DIR`'s own,
/// separate read lock, and nothing in this crate acquires `CUSTOM_CACHE_DIR` while
/// waiting on `REGISTERED_CACHE_DIR` in the other order (`configure_inner` only ever
/// touches `CUSTOM_CACHE_DIR`), so there is no lock-order cycle between the two.
///
/// `extra_lib_dirs` in `registry.rs` is append-only and never pruned (see its own
/// doc comment), so repeated reconfiguration under concurrent load accumulates one
/// entry per distinct directory ever registered here — harmless for correctness (a
/// stale entry is just an extra, permission-checked-once, miss-on-lookup search
/// path) but worth knowing before treating reconfiguration as free.
///
/// Poisoning is recovered rather than propagated: the only mutation is a full
/// `Option` overwrite performed after `add_extra_libs_dir` returns, so a panic here
/// leaves the previous, still-coherent value in place — propagating would
/// reintroduce the same permanent-failure mode this type replaces `CACHE_REGISTERED`
/// to fix. ~keep
#[cfg(feature = "download")]
static REGISTERED_CACHE_DIR: LazyLock<RwLock<Option<std::path::PathBuf>>> = LazyLock::new(|| RwLock::new(None));

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
#[tracing::instrument(level = "debug", skip_all, fields(language = name))]
pub fn get_language(name: &str) -> Result<Language, Error> {
    #[cfg(feature = "download")]
    {
        // ~keep Lock-free probe first: only take the global download lock on an actual miss.
        if let Ok(lang) = REGISTRY.get_language(name) {
            return Ok(lang);
        }
        let _cache_guard = DOWNLOAD_CACHE_LOCK
            .lock()
            .unwrap_or_else(|poisoned| recover_poisoned_lock("download_cache", poisoned));
        // ~keep Double-check under the lock: another thread may have downloaded it.
        if let Ok(lang) = REGISTRY.get_language(name) {
            return Ok(lang);
        }
        ensure_cache_registered()?;
        let cache_dir = effective_cache_dir()?;
        let dm = DownloadManager::with_cache_dir(env!("CARGO_PKG_VERSION"), cache_dir);
        let resolved = crate::registry::resolve_alias(name);
        dm.ensure_languages(&[resolved])?;
        if let Ok(lang) = REGISTRY.get_language(resolved) {
            return Ok(lang);
        }
        // ~keep Retry once: concurrent cache cleanup can delete the file between extraction and registry lookup.
        // ~keep `ensure_languages` is idempotent and TOCTOU-safe, so the second pass can restore the cache.
        dm.ensure_languages(&[resolved])?;
        REGISTRY.get_language(resolved)
    }

    #[cfg(not(feature = "download"))]
    {
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

/// Check whether this language can be parsed right now, without downloading.
///
/// Answers from the same lookup [`get_parser`] performs — statically compiled
/// grammars, already-loaded dynamic grammars, and parser shared libraries
/// present in the library and download-cache directories — but never fetches
/// anything over the network.
///
/// Use it to distinguish "we recognise this name" ([`has_language`], which is
/// also `true` for a grammar that still has to be downloaded) from "parsing this
/// will work offline, now". It is likewise independent of the
/// extension-to-language mapping: [`detect_language_from_extension`] consults
/// the static ext table for all 372 grammars regardless of what is installed.
///
/// The first `true` answer for a dynamic grammar loads its shared library, since
/// loading is the only way to know it is usable; loads are cached process-wide,
/// so repeat calls are cheap. ~keep
///
/// ```no_run
/// use tree_sitter_language_pack::{detect_language_from_extension, has_parser};
///
/// if let Some(lang) = detect_language_from_extension("feature") {
///     if has_parser(lang) {
///         // the gherkin parser is installed — parse without touching the network
///     } else {
///         // we know it's gherkin, but the parser still has to be downloaded
///     }
/// }
/// ```
pub fn has_parser(name: &str) -> bool {
    // ~keep Register the download cache first, or installed-but-unregistered parsers read as absent.
    #[cfg(feature = "download")]
    let _ = ensure_cache_registered();
    REGISTRY.has_parser(name)
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
/// Returns [`Error::InvalidRange`] if the config carries a zero-valued limit or
/// the source exceeds [`ProcessConfig::max_source_bytes`],
/// [`Error::ParseTimeout`] if the parse exceeds
/// [`ProcessConfig::parse_timeout_ms`], [`Error::LanguageNotFound`] if the
/// language is unknown, or [`Error::ParseFailed`] if parsing yields no tree.
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
#[tracing::instrument(
    level = "debug",
    skip_all,
    fields(language = %config.language, source_bytes = source.len())
)]
pub fn process(source: &str, config: &ProcessConfig) -> Result<ProcessResult, Error> {
    // ~keep Validate before the download below, so a bad config fails without any network I/O.
    config.validate()?;
    config.check_source_size(source.len())?;

    // ~keep Trigger auto-download here; `REGISTRY.process()` does not perform the download fallback.
    #[cfg(feature = "download")]
    get_language(&config.language)?;

    REGISTRY.process(source, config)
}

#[cfg(feature = "download")]
fn ensure_cache_registered() -> Result<(), Error> {
    let cache_dir = effective_cache_dir()?;
    // ~keep Fast path: a read lock is far cheaper than the write lock plus the
    // ~keep `add_extra_libs_dir` call below, and "already registered" is the common case.
    {
        let registered = REGISTERED_CACHE_DIR
            .read()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        if registered.as_deref() == Some(cache_dir.as_path()) {
            return Ok(());
        }
    }
    let mut registered = REGISTERED_CACHE_DIR
        .write()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    // ~keep Re-read rather than reuse the snapshot computed above: a concurrent
    // ~keep `configure()` can change `CUSTOM_CACHE_DIR` between that read and this lock
    // ~keep acquisition, and comparing against the stale value would register the OLD
    // ~keep directory. Safe against deadlock — see `REGISTERED_CACHE_DIR`'s doc comment.
    let cache_dir = effective_cache_dir()?;
    if registered.as_deref() == Some(cache_dir.as_path()) {
        return Ok(());
    }
    // ~keep Verify (never create) before ever registering this path as a dlopen search
    // ~keep directory: a pre-existing directory that is attacker-owned or group/other-
    // ~keep writable must never be trusted just because it sits at the resolved cache
    // ~keep path. See #101 H2. ~keep
    crate::download::verify_cache_dir_if_present(&cache_dir)?;
    REGISTRY.try_add_extra_libs_dir(cache_dir.clone())?;
    *registered = Some(cache_dir);
    Ok(())
}

#[cfg(feature = "download")]
fn effective_cache_dir() -> Result<std::path::PathBuf, Error> {
    let custom = CUSTOM_CACHE_DIR
        .read()
        .map_err(|e| Error::LockPoisoned(e.to_string()))?;
    match custom.as_ref() {
        // ~keep `PackConfig::cache_dir` / `configure()` is a BASE directory, not the
        // ~keep final libs path: apply the same `tree-sitter-language-pack/v{version}/libs`
        // ~keep suffix `default_cache_dir` applies to the platform cache dir, so
        // ~keep `DownloadManager::version_cache_dir()` (`self.cache_dir.parent()`) always
        // ~keep resolves to a directory this crate owns. Before this, a custom `cache_dir`
        // ~keep was used verbatim and `manifest.json`/`bundles/`/`.download.lock` all ended
        // ~keep up in its *parent* instead. See #101 H1. ~keep
        Some(dir) => Ok(DownloadManager::cache_dir_from_base(dir, env!("CARGO_PKG_VERSION"))),
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
#[tracing::instrument(level = "info", skip_all)]
pub fn init(config: &PackConfig) -> Result<(), Error> {
    let _cache_guard = DOWNLOAD_CACHE_LOCK
        .lock()
        .unwrap_or_else(|poisoned| recover_poisoned_lock("download_cache", poisoned));
    configure_inner(config)?;
    if let Some(ref languages) = config.languages {
        let refs: Vec<&str> = languages.iter().map(String::as_str).collect();
        download_inner(&refs)?;
    }
    if let Some(ref groups) = config.groups {
        let cache_dir = effective_cache_dir()?;
        let dm = DownloadManager::with_cache_dir(env!("CARGO_PKG_VERSION"), cache_dir);
        for group in groups {
            dm.ensure_group(group)?;
        }
    }
    ensure_cache_registered()?;
    tracing::info!(
        languages = config.languages.as_ref().map_or(0, Vec::len),
        groups = config.groups.as_ref().map_or(0, Vec::len),
        "language pack initialized"
    );
    Ok(())
}

/// Apply download configuration without downloading anything.
///
/// Use this to set a custom cache directory before the first call to
/// [`get_language`] or any download function. Changing the cache dir
/// after languages have been registered has no effect on already-loaded
/// languages.
///
/// `PackConfig::cache_dir` is a BASE directory, not the final libs path: this
/// crate appends `tree-sitter-language-pack/v{version}/libs` to it, the same
/// suffix applied to the platform default cache directory. In the example below,
/// files actually land under `/tmp/my-parsers/tree-sitter-language-pack/v{version}/libs/`,
/// never directly in `/tmp/my-parsers/`. Call [`cache_dir`] to read back the
/// resolved, fully-suffixed path.
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
#[tracing::instrument(level = "debug", skip_all)]
pub fn configure(config: &PackConfig) -> Result<(), Error> {
    let _cache_guard = DOWNLOAD_CACHE_LOCK
        .lock()
        .unwrap_or_else(|poisoned| recover_poisoned_lock("download_cache", poisoned));
    configure_inner(config)
}

#[cfg(feature = "download")]
fn configure_inner(config: &PackConfig) -> Result<(), Error> {
    if let Some(ref dir) = config.cache_dir {
        let mut custom = CUSTOM_CACHE_DIR
            .write()
            .map_err(|e| Error::LockPoisoned(e.to_string()))?;
        *custom = Some(dir.clone());
        // ~keep No explicit re-registration reset needed: `ensure_cache_registered` always
        // ~keep recomputes `effective_cache_dir()` fresh and compares it against
        // ~keep `REGISTERED_CACHE_DIR` by path, so the very next call anywhere registers this
        // ~keep new directory on its own. Old cache directories remain registered alongside
        // ~keep it; per-path scanning and dedup in `add_extra_libs_dir` make that acceptable.
    }
    Ok(())
}

/// Download specific languages to the local cache.
///
/// Returns the number of distinct languages available after the call. Already
/// compiled or cached languages are included in the count.
///
/// Aliases are resolved before counting, so `["shell", "bash"]` names one
/// language and returns 1.
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
#[tracing::instrument(level = "info", skip_all, fields(requested = names.len()))]
pub fn download(names: &[&str]) -> Result<usize, Error> {
    let _cache_guard = DOWNLOAD_CACHE_LOCK
        .lock()
        .unwrap_or_else(|poisoned| recover_poisoned_lock("download_cache", poisoned));
    let count = download_inner(names)?;
    tracing::info!(count, "ensured languages");
    Ok(count)
}

/// Select the names that are not yet loadable from disk and must therefore be
/// fetched by [`download_inner`].
///
/// Keys on `registry.get_language(name).is_err()`, not `has_language`: in a
/// download-enabled build `has_language` reports every known language as present,
/// including one that has never been fetched, so filtering on it made `download()`
/// a no-op that still returned the requested count. `get_language` fails with
/// `LanguageNotFound` until the parser is actually on disk. See #201. ~keep
#[cfg(feature = "download")]
fn languages_needing_download<'a, I>(registry: &crate::registry::LanguageRegistry, names: I) -> Vec<&'a str>
where
    I: IntoIterator<Item = &'a str>,
{
    names
        .into_iter()
        .filter(|name| registry.get_language(name).is_err())
        .collect()
}

#[cfg(feature = "download")]
fn download_inner(names: &[&str]) -> Result<usize, Error> {
    ensure_cache_registered()?;
    let cache_dir = effective_cache_dir()?;
    let dm = DownloadManager::with_cache_dir(env!("CARGO_PKG_VERSION"), cache_dir);
    // ~keep The manifest is keyed by canonical name only, so an alias like `shell`
    // must be resolved before it reaches `ensure_languages` — otherwise it 404s as
    // an unknown language. `prefetch` already does this; `download` did not.
    let resolved: Vec<&str> = names.iter().map(|n| crate::registry::resolve_alias(n)).collect();
    let needs_download = languages_needing_download(&REGISTRY, resolved.iter().copied());
    dm.ensure_languages(&needs_download)?;
    let unique: std::collections::BTreeSet<&str> = resolved.iter().copied().collect();
    Ok(unique.len())
}

/// Prefetch grammars: download any not already loadable from disk, then load every
/// requested language into the process registry so a subsequent hot loop only parses.
///
/// Unlike [`download()`], this does not trust in-memory availability — it downloads
/// whenever a grammar is not actually loadable from disk (fixing the case where a
/// known-but-not-downloaded grammar is reported present), then resolves and caches
/// every requested language. Call it once, up front, before a parallel workload.
///
/// # Errors
///
/// Returns [`Error::Download`] if a required grammar cannot be fetched, or
/// [`Error::LanguageNotFound`] if a requested name is unknown.
///
/// # Example
///
/// ```no_run
/// use tree_sitter_language_pack::prefetch;
///
/// prefetch(&["rust", "python", "go"])?;
/// # Ok::<(), tree_sitter_language_pack::Error>(())
/// ```
#[cfg(feature = "download")]
#[tracing::instrument(level = "info", skip_all, fields(requested = languages.len()))]
pub fn prefetch(languages: &[&str]) -> Result<(), Error> {
    let _cache_guard = DOWNLOAD_CACHE_LOCK
        .lock()
        .unwrap_or_else(|poisoned| recover_poisoned_lock("download_cache", poisoned));
    ensure_cache_registered()?;

    let resolved: Vec<&str> = languages.iter().map(|n| crate::registry::resolve_alias(n)).collect();

    // ~keep Probe real on-disk loadability; `has_language` reports known-but-not-downloaded grammars as present.
    let needs_download: Vec<&str> = resolved
        .iter()
        .copied()
        .filter(|name| REGISTRY.get_language(name).is_err())
        .collect();

    if !needs_download.is_empty() {
        let cache_dir = effective_cache_dir()?;
        let dm = DownloadManager::with_cache_dir(env!("CARGO_PKG_VERSION"), cache_dir);
        dm.ensure_languages(&needs_download)?;
    }

    // ~keep Load every requested grammar into the process registry cache.
    for name in &resolved {
        REGISTRY.get_language(name)?;
    }
    tracing::info!(loaded = resolved.len(), "prefetched languages");
    Ok(())
}

/// Prefetch grammars by loading each into the registry.
///
/// Without the `download` feature there is no network step — every requested
/// language must already be statically compiled or present on disk.
///
/// # Errors
///
/// Returns [`Error::LanguageNotFound`] if a requested language is not available.
#[cfg(not(feature = "download"))]
#[tracing::instrument(level = "info", skip_all, fields(requested = languages.len()))]
pub fn prefetch(languages: &[&str]) -> Result<(), Error> {
    for raw in languages {
        let name = crate::registry::resolve_alias(raw);
        REGISTRY.get_language(name)?;
    }
    Ok(())
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
#[tracing::instrument(level = "info", skip_all)]
pub fn download_all() -> Result<usize, Error> {
    let _cache_guard = DOWNLOAD_CACHE_LOCK
        .lock()
        .unwrap_or_else(|poisoned| recover_poisoned_lock("download_cache", poisoned));
    ensure_cache_registered()?;
    let cache_dir = effective_cache_dir()?;
    let dm = DownloadManager::with_cache_dir(env!("CARGO_PKG_VERSION"), cache_dir);
    dm.download_all_best_effort()?;
    let count = REGISTRY.language_count();
    tracing::info!(count, "downloaded all available languages");
    Ok(count)
}

/// Download every language in a named group.
///
/// Groups are defined by the remote manifest, not by this crate, and let you
/// ensure a curated set of related grammars in one call instead of listing each
/// name to [`download()`]. Already-cached languages are skipped.
///
/// Call [`manifest_groups`] to discover the group names the manifest actually
/// defines. The published manifest currently defines a single group, `"all"`;
/// earlier revisions of this documentation advertised `"web"`, `"data"`, and
/// `"systems"`, which the manifest has never contained, so every call following
/// that example failed. Do not hardcode a group name without checking. ~keep
///
/// Returns the total number of languages now available (statically compiled
/// plus downloaded and cached).
///
/// # Errors
///
/// Returns [`Error::Download`] if the manifest cannot be fetched, if the group
/// is unknown — the message lists the groups the manifest defines — or if any
/// constituent language fails to download.
///
/// # Example
///
/// ```no_run
/// use tree_sitter_language_pack::{download_group, manifest_groups};
///
/// let group = manifest_groups()?.into_iter().next().expect("manifest defines no groups");
/// let count = download_group(&group)?;
/// println!("{count} languages available");
/// # Ok::<(), tree_sitter_language_pack::Error>(())
/// ```
#[cfg(feature = "download")]
#[tracing::instrument(level = "info", skip_all, fields(group = name))]
pub fn download_group(name: &str) -> Result<usize, Error> {
    let _cache_guard = DOWNLOAD_CACHE_LOCK
        .lock()
        .unwrap_or_else(|poisoned| recover_poisoned_lock("download_cache", poisoned));
    ensure_cache_registered()?;
    let cache_dir = effective_cache_dir()?;
    let dm = DownloadManager::with_cache_dir(env!("CARGO_PKG_VERSION"), cache_dir);
    dm.ensure_group(name)?;
    let count = REGISTRY.language_count();
    tracing::info!(group = name, count, "downloaded language group");
    Ok(count)
}

/// Return all language names available in the remote manifest (372).
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

/// Return the names of every language group the remote manifest defines, sorted.
///
/// Group names are manifest data, not a compile-time constant of this crate, so
/// this is the only reliable way to learn what [`download_group`] and
/// [`PackConfig::groups`] accept. The published manifest currently defines just
/// `"all"`. ~keep
///
/// # Errors
///
/// Returns [`Error::Download`] if the manifest cannot be fetched.
///
/// # Example
///
/// ```no_run
/// use tree_sitter_language_pack::manifest_groups;
///
/// for group in manifest_groups()? {
///     println!("{group}");
/// }
/// # Ok::<(), tree_sitter_language_pack::Error>(())
/// ```
#[cfg(feature = "download")]
pub fn manifest_groups() -> Result<Vec<String>, Error> {
    let cache_dir = effective_cache_dir()?;
    let dm = DownloadManager::with_cache_dir(env!("CARGO_PKG_VERSION"), cache_dir);
    let manifest = dm.fetch_manifest()?;
    let mut groups: Vec<String> = manifest.groups.keys().cloned().collect();
    groups.sort_unstable();
    Ok(groups)
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
#[tracing::instrument(level = "info", skip_all)]
pub fn clean_cache() -> Result<(), Error> {
    let _cache_guard = DOWNLOAD_CACHE_LOCK
        .lock()
        .unwrap_or_else(|poisoned| recover_poisoned_lock("download_cache", poisoned));
    let cache_dir = effective_cache_dir()?;
    let dm = DownloadManager::with_cache_dir(env!("CARGO_PKG_VERSION"), cache_dir);
    dm.clean_cache()?;
    // ~keep No registration reset needed: the cache directory path is unchanged, it is
    // ~keep already registered in `extra_lib_dirs`, and directory contents are scanned
    // ~keep live on every call rather than cached at registration time.
    tracing::info!("cleared parser cache");
    Ok(())
}

/// Return the effective cache directory path.
///
/// This is `{base}/tree-sitter-language-pack/v{version}/libs/`, where `{base}` is
/// either the custom BASE directory set via [`configure`] / [`init`]
/// (`PackConfig::cache_dir`) or the platform default cache directory — both are
/// suffixed identically, so a custom `cache_dir` is never used as the final libs
/// path. The default resolves to `~/.cache/tree-sitter-language-pack/v{version}/libs/`
/// on a typical Unix system.
///
/// # Errors
///
/// Returns an error if no cache directory can be resolved: `version` is somehow
/// invalid, or (with no custom `cache_dir` configured) the platform reports none
/// and `TREE_SITTER_LANGUAGE_PACK_CACHE_DIR` is unset. This crate no longer falls
/// back to the temporary directory for the latter case — see #101 H2.
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
    #[ignore = "loads all 372 dynamic libraries — run with --ignored"]
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

    #[cfg(feature = "download")]
    #[test]
    fn should_select_known_but_not_downloaded_languages_for_download() {
        // ~keep Regression for #201: an empty libs dir makes `get_language` fail for a
        // ~keep known language while `has_language` still reports it present (a download
        // ~keep build recognises the whole manifest), which is exactly the gap that made
        // ~keep `download([...])` a no-op. Selection must key on on-disk loadability.
        let empty = std::env::temp_dir().join("tslp-201-nonexistent-libs");
        let registry = LanguageRegistry::with_libs_dir(empty);
        let is_static = |name: &str| crate::registry::STATIC_LANGUAGES.iter().any(|&(s, _)| s == name);
        if let Some(&(linked, _)) = crate::registry::STATIC_LANGUAGES.first() {
            assert!(
                languages_needing_download(&registry, [linked]).is_empty(),
                "a statically linked language must never be selected for download"
            );
        }
        // ~keep Statically linked grammars are loadable without a download, so the target
        // ~keep must be one this build did not compile in (TSLP_LANGUAGES varies per run).
        // ~keep A build linking every grammar has nothing left to download.
        let Some(target) = crate::registry::KNOWN_LANGUAGES
            .iter()
            .copied()
            .find(|name| !is_static(name))
        else {
            return;
        };
        assert!(
            registry.has_language(target),
            "a download-enabled build must report canonical manifest languages as known"
        );
        let selected = languages_needing_download(&registry, [target]);
        assert_eq!(
            selected,
            vec![target],
            "a known-but-not-on-disk language must be selected for download"
        );
    }

    #[cfg(feature = "download")]
    #[test]
    fn should_keep_configuring_after_a_panic_poisons_the_download_cache_lock() {
        // ~keep DOWNLOAD_CACHE_LOCK guards no data (it only serializes the
        // ~keep read-download-register sequence), so poisoning it should never brick every
        // ~keep subsequent call. Poisoning is process-wide and sticky (std::sync::Mutex never
        // ~keep un-poisons); safe to leave poisoned for the rest of the process here only
        // ~keep because every acquisition site recovers via `recover_poisoned_lock`. This test
        // ~keep does not set `cache_dir`, so unlike a reconfiguration test it cannot corrupt
        // ~keep any sibling test's view of the cache directory.
        let poison_result = std::panic::catch_unwind(|| {
            let _guard = DOWNLOAD_CACHE_LOCK
                .lock()
                .expect("DOWNLOAD_CACHE_LOCK should not already be poisoned");
            panic!("intentional panic to poison DOWNLOAD_CACHE_LOCK for this test");
        });
        assert!(poison_result.is_err(), "the intentional panic should have unwound");
        assert!(
            DOWNLOAD_CACHE_LOCK.is_poisoned(),
            "DOWNLOAD_CACHE_LOCK should be poisoned after the panic"
        );

        let result = configure(&PackConfig {
            cache_dir: None,
            languages: None,
            groups: None,
        });
        assert!(
            result.is_ok(),
            "configure must recover a poisoned DOWNLOAD_CACHE_LOCK instead of failing forever, got: {result:?}"
        );
    }

    #[cfg(feature = "download")]
    #[test]
    #[ignore = "mutates the process-wide CUSTOM_CACHE_DIR/REGISTERED_CACHE_DIR statics; run \
                alone, e.g. `cargo test -- --ignored --test-threads=1`, not inside the default \
                parallel suite"]
    fn should_register_the_new_cache_dir_after_configure_changes_it() {
        // ~keep Regression for item 1: the old `CACHE_REGISTERED` boolean could get stuck
        // ~keep `true` forever after a race with `configure()`, permanently registering only
        // ~keep the OLD cache dir. The path-CAS fix in `ensure_cache_registered` compares the
        // ~keep actual registered path on every call, so the very next call anywhere after a
        // ~keep reconfiguration observes and registers the new path — this asserts exactly
        // ~keep that, deterministically, without needing to reproduce the original race.
        let first_base = std::env::temp_dir().join("tslp-item1-regression-first");
        let second_base = std::env::temp_dir().join("tslp-item1-regression-second");
        // ~keep `cache_dir` is a BASE directory (#101 H1): the registered path is suffixed.
        let first_dir = DownloadManager::cache_dir_from_base(&first_base, env!("CARGO_PKG_VERSION"));
        let second_dir = DownloadManager::cache_dir_from_base(&second_base, env!("CARGO_PKG_VERSION"));

        configure(&PackConfig {
            cache_dir: Some(first_base.clone()),
            languages: None,
            groups: None,
        })
        .expect("configure should accept the first cache dir");
        let _ = has_language("definitely_not_a_real_language_xyz");
        {
            let registered = REGISTERED_CACHE_DIR.read().expect("read lock should not be poisoned");
            assert_eq!(registered.as_deref(), Some(first_dir.as_path()));
        }

        configure(&PackConfig {
            cache_dir: Some(second_base.clone()),
            languages: None,
            groups: None,
        })
        .expect("configure should accept the second cache dir");
        let _ = has_language("definitely_not_a_real_language_xyz");
        {
            let registered = REGISTERED_CACHE_DIR.read().expect("read lock should not be poisoned");
            assert_eq!(
                registered.as_deref(),
                Some(second_dir.as_path()),
                "reconfiguring the cache dir must be observed by the very next call, \
                 not stuck on the old path forever"
            );
        }
    }

    #[cfg(feature = "download")]
    #[test]
    #[ignore = "mutates the process-wide CUSTOM_CACHE_DIR static; run alone, e.g. \
                `cargo test -- --ignored --test-threads=1`, not inside the default parallel suite"]
    fn should_root_effective_cache_dir_under_the_configured_base_when_cache_dir_is_custom() {
        // ~keep Regression for #101 H1: `PackConfig::cache_dir` used to be forwarded
        // ~keep verbatim, so `DownloadManager::version_cache_dir()`
        // ~keep (`self.cache_dir.parent()`) resolved OUTSIDE the caller's configured
        // ~keep directory entirely — `manifest.json`, `bundles/`, and `.download.lock` all
        // ~keep ended up in the *parent* of whatever the caller configured.
        let base = std::env::temp_dir().join("tslp-h1-regression-base");

        configure(&PackConfig {
            cache_dir: Some(base.clone()),
            languages: None,
            groups: None,
        })
        .expect("configure should accept a custom base directory");

        let dir = effective_cache_dir().expect("effective cache dir should resolve");

        assert!(
            dir.starts_with(&base),
            "effective cache dir {} must live under the configured base {}",
            dir.display(),
            base.display()
        );
        assert_eq!(
            dir,
            base.join("tree-sitter-language-pack")
                .join(format!("v{}", env!("CARGO_PKG_VERSION")))
                .join("libs")
        );
    }
}
