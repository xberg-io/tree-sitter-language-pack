/// Byte and line/column range in source code.
///
/// Represents both byte offsets (for slicing) and human-readable line/column
/// positions (for display and diagnostics).
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Span {
    pub start_byte: usize,
    pub end_byte: usize,
    pub start_line: usize,
    pub start_column: usize,
    pub end_line: usize,
    pub end_column: usize,
}

/// Complete analysis result from processing a source file.
///
/// Contains metrics, structural analysis, imports/exports, comments,
/// docstrings, symbols, diagnostics, and optionally chunked code segments.
/// Fields are populated based on the [`crate::ProcessConfig`] flags.
///
/// # Fields
///
/// - `language` - The language used for parsing
/// - `metrics` - Always computed: line counts, byte sizes, error counts
/// - `structure` - Functions, classes, structs (when `config.structure = true`)
/// - `imports` - Import statements (when `config.imports = true`)
/// - `exports` - Export statements (when `config.exports = true`)
/// - `comments` - Comments (when `config.comments = true`)
/// - `docstrings` - Docstrings (when `config.docstrings = true`)
/// - `symbols` - Symbol definitions (when `config.symbols = true`)
/// - `diagnostics` - Parse errors (when `config.diagnostics = true`)
/// - `chunks` - Chunked code segments (when `config.chunk_max_size` is set)
#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ProcessResult {
    pub language: String,
    pub metrics: FileMetrics,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty", default))]
    pub structure: Vec<StructureItem>,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty", default))]
    pub imports: Vec<ImportInfo>,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty", default))]
    pub exports: Vec<ExportInfo>,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty", default))]
    pub comments: Vec<CommentInfo>,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty", default))]
    pub docstrings: Vec<DocstringInfo>,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty", default))]
    pub symbols: Vec<SymbolInfo>,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty", default))]
    pub diagnostics: Vec<Diagnostic>,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty", default))]
    pub chunks: Vec<CodeChunk>,
}

/// Aggregate metrics for a source file.
#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FileMetrics {
    pub total_lines: usize,
    pub code_lines: usize,
    pub comment_lines: usize,
    pub blank_lines: usize,
    pub total_bytes: usize,
    pub node_count: usize,
    pub error_count: usize,
    pub max_depth: usize,
}

/// The kind of structural item found in source code.
///
/// Categorizes top-level and nested declarations such as functions, classes,
/// structs, enums, traits, and more. Use [`Other`](StructureKind::Other) for
/// language-specific constructs that do not fit a standard category.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum StructureKind {
    #[default]
    Function,
    Method,
    Class,
    Struct,
    Interface,
    Enum,
    Module,
    Trait,
    Impl,
    Namespace,
    Other(String),
}

/// A structural item (function, class, struct, etc.) in source code.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct StructureItem {
    pub kind: StructureKind,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub name: Option<String>,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none", default))]
    pub visibility: Option<String>,
    pub span: Span,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty", default))]
    pub children: Vec<StructureItem>,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty", default))]
    pub decorators: Vec<String>,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none", default))]
    pub doc_comment: Option<String>,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none", default))]
    pub signature: Option<String>,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub body_span: Option<Span>,
}

/// The kind of a comment found in source code.
///
/// Distinguishes between single-line comments, block (multi-line) comments,
/// and documentation comments.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum CommentKind {
    #[default]
    Line,
    Block,
    Doc,
}

/// A comment extracted from source code.
#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CommentInfo {
    pub text: String,
    pub kind: CommentKind,
    pub span: Span,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub associated_node: Option<String>,
}

/// The format of a docstring extracted from source code.
///
/// Identifies the docstring convention used, which varies by language
/// (e.g., Python triple-quoted strings, JSDoc, Rustdoc `///` comments).
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum DocstringFormat {
    #[default]
    PythonTripleQuote,
    JSDoc,
    Rustdoc,
    GoDoc,
    JavaDoc,
    Other(String),
}

/// A docstring extracted from source code.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct DocstringInfo {
    pub text: String,
    pub format: DocstringFormat,
    pub span: Span,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub associated_item: Option<String>,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty", default))]
    pub parsed_sections: Vec<DocSection>,
}

/// A section within a docstring (e.g., Args, Returns, Raises).
#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct DocSection {
    pub kind: String,
    pub name: Option<String>,
    pub description: String,
}

/// An import statement extracted from source code.
#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ImportInfo {
    pub source: String,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty", default))]
    pub items: Vec<String>,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none", default))]
    pub alias: Option<String>,
    pub is_wildcard: bool,
    pub span: Span,
}

/// The kind of an export statement found in source code.
///
/// Covers named exports, default exports, and re-exports from other modules.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ExportKind {
    #[default]
    Named,
    Default,
    ReExport,
}

/// An export statement extracted from source code.
#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ExportInfo {
    pub name: String,
    pub kind: ExportKind,
    pub span: Span,
}

/// The kind of a symbol definition found in source code.
///
/// Categorizes symbol definitions such as variables, constants, functions,
/// classes, types, interfaces, enums, and modules.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum SymbolKind {
    #[default]
    Variable,
    Constant,
    Function,
    Class,
    Type,
    Interface,
    Enum,
    Module,
    Other(String),
}

/// A symbol (variable, function, type, etc.) extracted from source code.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SymbolInfo {
    pub name: String,
    pub kind: SymbolKind,
    pub span: Span,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub type_annotation: Option<String>,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none", default))]
    pub doc: Option<String>,
}

/// Severity level of a diagnostic produced during parsing.
///
/// Used to classify parse errors, warnings, and informational messages
/// found in the syntax tree.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum DiagnosticSeverity {
    #[default]
    Error,
    Warning,
    Info,
}

/// A diagnostic (syntax error, missing node, etc.) from parsing.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Diagnostic {
    pub message: String,
    pub severity: DiagnosticSeverity,
    pub span: Span,
}

/// A chunk of source code with rich metadata.
#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CodeChunk {
    pub content: String,
    pub start_byte: usize,
    pub end_byte: usize,
    pub start_line: usize,
    pub end_line: usize,
    pub metadata: ChunkContext,
}

/// Metadata for a single chunk of source code.
#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ChunkContext {
    pub language: String,
    pub chunk_index: usize,
    pub total_chunks: usize,
    pub node_types: Vec<String>,
    pub context_path: Vec<String>,
    pub symbols_defined: Vec<String>,
    pub comments: Vec<CommentInfo>,
    pub docstrings: Vec<DocstringInfo>,
    pub has_error_nodes: bool,
}
