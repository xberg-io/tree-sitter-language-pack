use thiserror::Error;

/// Errors that can occur when using the tree-sitter language pack.
///
/// Covers language lookup failures, parse errors, query errors, and I/O issues.
/// Feature-gated variants are included when `config`, `download`, or related
/// features are enabled.
#[derive(Debug, Error)]
pub enum Error {
    /// The requested language name (or alias) was not found in the registry.
    #[error("Language '{0}' not found")]
    LanguageNotFound(String),

    /// A dynamic shared library could not be loaded at runtime.
    #[error("Dynamic library load error: {0}")]
    DynamicLoad(String),

    /// The tree-sitter language function returned a null pointer for the given language name.
    #[error("Language function returned null pointer for '{0}'")]
    NullLanguagePointer(String),

    /// The language could not be applied to the parser (e.g., ABI version mismatch).
    #[error("Failed to set parser language: {0}")]
    ParserSetup(String),

    /// An internal `RwLock` or `Mutex` was poisoned by a previous panic.
    #[error("Registry lock poisoned: {0}")]
    LockPoisoned(String),

    /// A configuration file or value was invalid or could not be applied.
    #[error("Configuration error: {0}")]
    Config(String),

    /// The tree-sitter parser returned no tree for the given source input.
    #[error("Parse failed: parsing returned no tree")]
    ParseFailed,

    /// A tree-sitter query could not be compiled or executed.
    #[error("Query error: {0}")]
    QueryError(String),

    /// A byte range was invalid (e.g., end before start, or out of bounds).
    #[error("Invalid byte range: {0}")]
    InvalidRange(String),

    /// A filesystem or network I/O operation failed.
    #[cfg(not(alef))]
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    /// A JSON value could not be parsed (requires `config` or `download` feature).
    #[cfg(all(not(alef), any(feature = "config", feature = "download")))]
    #[error("JSON parse error: {0}")]
    Json(#[from] serde_json::Error),

    /// A TOML configuration file could not be parsed (requires `config` feature).
    #[cfg(all(not(alef), feature = "config"))]
    #[error("TOML parse error: {0}")]
    Toml(#[from] toml::de::Error),

    /// A parser download from GitHub releases failed.
    #[error("Download error: {0}")]
    Download(String),

    /// The downloaded file's SHA-256 digest did not match the manifest's expected value.
    #[error("Checksum mismatch for '{file}': expected {expected}, got {actual}")]
    ChecksumMismatch {
        /// Path of the file whose checksum was verified.
        file: String,
        /// Expected SHA-256 hex digest from the manifest.
        expected: String,
        /// Actual SHA-256 hex digest computed from the downloaded bytes.
        actual: String,
    },

    /// The cross-process download cache lock file could not be acquired or created.
    #[error("Download cache lock error: {0}")]
    CacheLock(String),
}
