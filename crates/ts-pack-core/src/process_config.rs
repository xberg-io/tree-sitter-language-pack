use std::borrow::Cow;

/// Configuration for the `process()` function.
///
/// Controls which analysis features are enabled and whether chunking is performed.
///
/// # Examples
///
/// ```
/// use tree_sitter_language_pack::ProcessConfig;
///
/// // Defaults: structure + imports + exports enabled
/// let config = ProcessConfig::new("python");
///
/// // With chunking
/// let config = ProcessConfig::new("python").with_chunking(1000);
///
/// // Everything enabled
/// let config = ProcessConfig::new("python").all();
/// ```
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ProcessConfig {
    /// Language name (required).
    pub language: Cow<'static, str>,
    /// Extract structural items (functions, classes, etc.). Default: true.
    #[cfg_attr(feature = "serde", serde(default = "default_true"))]
    pub structure: bool,
    /// Extract import statements. Default: true.
    #[cfg_attr(feature = "serde", serde(default = "default_true"))]
    pub imports: bool,
    /// Extract export statements. Default: true.
    #[cfg_attr(feature = "serde", serde(default = "default_true"))]
    pub exports: bool,
    /// Extract comments. Default: false.
    #[cfg_attr(feature = "serde", serde(default))]
    pub comments: bool,
    /// Extract docstrings. Default: false.
    #[cfg_attr(feature = "serde", serde(default))]
    pub docstrings: bool,
    /// Extract symbol definitions. Default: false.
    #[cfg_attr(feature = "serde", serde(default))]
    pub symbols: bool,
    /// Include parse diagnostics. Default: false.
    #[cfg_attr(feature = "serde", serde(default))]
    pub diagnostics: bool,
    /// Maximum chunk size in bytes. `None` disables chunking.
    #[cfg_attr(feature = "serde", serde(default))]
    pub chunk_max_size: Option<usize>,
}

#[cfg(feature = "serde")]
fn default_true() -> bool {
    true
}

impl Default for ProcessConfig {
    fn default() -> Self {
        Self {
            language: Cow::Borrowed(""),
            structure: true,
            imports: true,
            exports: true,
            comments: false,
            docstrings: false,
            symbols: false,
            diagnostics: false,
            chunk_max_size: None,
        }
    }
}

impl ProcessConfig {
    /// Create a new config for the given language with default settings.
    pub fn new(language: impl Into<String>) -> Self {
        Self {
            language: Cow::Owned(language.into()),
            ..Default::default()
        }
    }

    /// Enable chunking with the given maximum chunk size in bytes.
    pub fn with_chunking(mut self, max_size: usize) -> Self {
        self.chunk_max_size = Some(max_size);
        self
    }

    /// Enable all analysis features.
    pub fn all(mut self) -> Self {
        self.structure = true;
        self.imports = true;
        self.exports = true;
        self.comments = true;
        self.docstrings = true;
        self.symbols = true;
        self.diagnostics = true;
        self
    }

    /// Disable all analysis features (only metrics computed).
    pub fn minimal(mut self) -> Self {
        self.structure = false;
        self.imports = false;
        self.exports = false;
        self.comments = false;
        self.docstrings = false;
        self.symbols = false;
        self.diagnostics = false;
        self
    }
}
