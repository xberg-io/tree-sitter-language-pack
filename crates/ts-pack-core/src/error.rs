use thiserror::Error;

/// Errors that can occur when using the tree-sitter language pack.
///
/// Covers language lookup failures, parse errors, query errors, and I/O issues.
/// Feature-gated variants are included when `config`, `download`, or related
/// features are enabled.
#[derive(Debug, Error)]
pub enum Error {
    #[error("Language '{0}' not found")]
    LanguageNotFound(String),

    #[error("Dynamic library load error: {0}")]
    DynamicLoad(String),

    #[error("Language function returned null pointer for '{0}'")]
    NullLanguagePointer(String),

    #[error("Failed to set parser language: {0}")]
    ParserSetup(String),

    #[error("Registry lock poisoned: {0}")]
    LockPoisoned(String),

    #[error("Configuration error: {0}")]
    Config(String),

    #[error("Parse failed: parsing returned no tree")]
    ParseFailed,

    #[error("Query error: {0}")]
    QueryError(String),

    #[error("Invalid byte range: {0}")]
    InvalidRange(String),

    #[cfg(not(alef))]
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[cfg(all(not(alef), any(feature = "config", feature = "download")))]
    #[error("JSON parse error: {0}")]
    Json(#[from] serde_json::Error),

    #[cfg(all(not(alef), feature = "config"))]
    #[error("TOML parse error: {0}")]
    Toml(#[from] toml::de::Error),

    #[error("Download error: {0}")]
    Download(String),

    #[error("Checksum mismatch for '{file}': expected {expected}, got {actual}")]
    ChecksumMismatch {
        file: String,
        expected: String,
        actual: String,
    },

    #[error("Download cache lock error: {0}")]
    CacheLock(String),
}
