/// Byte and line/column range in source code.
///
/// Represents both byte offsets (for slicing) and human-readable line/column
/// positions (for display and diagnostics).
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Span {
    /// Inclusive start byte offset in the source.
    pub start_byte: usize,
    /// Exclusive end byte offset in the source.
    pub end_byte: usize,
    /// Zero-indexed line number of the span's start.
    pub start_line: usize,
    /// Zero-indexed column number of the span's start.
    pub start_column: usize,
    /// Zero-indexed line number of the span's end.
    pub end_line: usize,
    /// Zero-indexed column number of the span's end.
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
    /// The language name used to parse the source file.
    pub language: String,
    /// File-level metrics (line counts, byte size, error count).
    pub metrics: FileMetrics,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty", default))]
    /// Top-level structural items (functions, classes, etc.).
    pub structure: Vec<StructureItem>,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty", default))]
    /// Import statements extracted from the source.
    pub imports: Vec<ImportInfo>,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty", default))]
    /// Export statements extracted from the source.
    pub exports: Vec<ExportInfo>,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty", default))]
    /// Comments extracted from the source.
    pub comments: Vec<CommentInfo>,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty", default))]
    /// Docstrings extracted from the source.
    pub docstrings: Vec<DocstringInfo>,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty", default))]
    /// Symbol definitions (variables, types, functions) extracted from the source.
    pub symbols: Vec<SymbolInfo>,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty", default))]
    /// Parse diagnostics (syntax errors, missing nodes) from tree-sitter.
    pub diagnostics: Vec<Diagnostic>,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty", default))]
    /// Syntax-aware code chunks produced when chunking is enabled.
    pub chunks: Vec<CodeChunk>,
}

/// Aggregate metrics for a source file.
#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FileMetrics {
    /// Total number of lines (including blank and comment lines).
    pub total_lines: usize,
    /// Number of lines containing non-blank, non-comment source code.
    pub code_lines: usize,
    /// Number of lines that are entirely comments.
    pub comment_lines: usize,
    /// Number of blank (whitespace-only) lines.
    pub blank_lines: usize,
    /// Total byte length of the source file.
    pub total_bytes: usize,
    /// Total number of nodes in the syntax tree.
    pub node_count: usize,
    /// Number of error nodes in the syntax tree (parse errors).
    pub error_count: usize,
    /// Maximum nesting depth reached in the syntax tree.
    pub max_depth: usize,
}

/// The kind of structural item found in source code.
///
/// Categorizes top-level and nested declarations such as functions, classes,
/// structs, enums, traits, and more. Use [`Other`](StructureKind::Other) for
/// language-specific constructs that do not fit a standard category.
///
/// # Wire format (public JSON contract)
///
/// Unit variants serialize as a bare string (`"Function"`); the `Other`
/// variant serializes as a single-keyed object (`{"Other": "macro"}`). DO
/// NOT add `#[serde(tag = "...")]` or rename variants — every language
/// binding has a hand-written deserializer matching this exact shape, and
/// any change breaks all bindings' `process()` tests simultaneously.
/// Covered by `tests/wire_format.rs`.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum StructureKind {
    #[default]
    /// A free-standing or associated function.
    Function,
    /// A method defined inside a class, struct, trait, or impl block.
    Method,
    /// A class definition.
    Class,
    /// A struct definition.
    Struct,
    /// An interface or protocol definition.
    Interface,
    /// An enum definition.
    Enum,
    /// A module or package declaration.
    Module,
    /// A trait definition.
    Trait,
    /// An impl block (Rust) or similar implementation block.
    Impl,
    /// A namespace declaration.
    Namespace,
    /// A language-specific construct that does not fit any standard category.
    Other(String),
}

/// A structural item (function, class, struct, etc.) in source code.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct StructureItem {
    /// The kind of structural item.
    pub kind: StructureKind,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    /// The declared name of the item, if present.
    pub name: Option<String>,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none", default))]
    /// Visibility modifier (e.g., `"pub"`, `"public"`, `"private"`).
    pub visibility: Option<String>,
    /// Source span covering the entire item declaration.
    pub span: Span,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty", default))]
    /// Nested structural items (e.g., methods within a class).
    pub children: Vec<StructureItem>,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty", default))]
    /// Decorator or attribute names applied to the item.
    pub decorators: Vec<String>,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none", default))]
    /// Documentation comment attached to the item, if any.
    pub doc_comment: Option<String>,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none", default))]
    /// Full signature text of the item (e.g., function parameters and return type).
    pub signature: Option<String>,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    /// Source span covering only the body of the item, if distinct from the declaration.
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
    /// A single-line comment (e.g., `// ...` or `# ...`).
    Line,
    /// A block or multi-line comment using slash-star delimiters.
    Block,
    /// A documentation comment such as `/// ...` or slash-double-star block.
    Doc,
}

/// A comment extracted from source code.
#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CommentInfo {
    /// The raw text content of the comment.
    pub text: String,
    /// The kind of comment (line, block, or doc).
    pub kind: CommentKind,
    /// Source span covering the comment.
    pub span: Span,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    /// Name of the syntax node this comment is directly associated with.
    pub associated_node: Option<String>,
}

/// The format of a docstring extracted from source code.
///
/// Identifies the docstring convention used, which varies by language
/// (e.g., Python triple-quoted strings, JSDoc, Rustdoc `///` comments).
///
/// # Wire format (public JSON contract)
///
/// Unit variants serialize as a bare string (`"JSDoc"`); the `Other`
/// variant serializes as a single-keyed object (`{"Other": "rst"}`). DO
/// NOT add `#[serde(tag = "...")]`. Covered by `tests/wire_format.rs`.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum DocstringFormat {
    #[default]
    /// Python triple-quoted string docstring (`"""..."""`).
    PythonTripleQuote,
    /// JavaScript/TypeScript JSDoc comment (`/** ... */`).
    JSDoc,
    /// Rust `///` or `//!` doc comment.
    Rustdoc,
    /// Go doc comment (a comment block immediately preceding a declaration).
    GoDoc,
    /// Java Javadoc comment (`/** ... */`).
    JavaDoc,
    /// A language-specific docstring format not covered by the standard variants.
    Other(String),
}

/// A docstring extracted from source code.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct DocstringInfo {
    /// The raw text of the docstring.
    pub text: String,
    /// The docstring format (Python, JSDoc, Rustdoc, etc.).
    pub format: DocstringFormat,
    /// Source span covering the docstring.
    pub span: Span,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    /// Name of the item this docstring documents.
    pub associated_item: Option<String>,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty", default))]
    /// Parsed sections of the docstring (Args, Returns, Raises, etc.).
    pub parsed_sections: Vec<DocSection>,
}

/// A section within a docstring (e.g., Args, Returns, Raises).
#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct DocSection {
    /// Section kind (e.g., `"args"`, `"returns"`, `"raises"`).
    pub kind: String,
    /// Parameter or return value name, if applicable.
    pub name: Option<String>,
    /// Description text for this section.
    pub description: String,
}

/// An import statement extracted from source code.
#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ImportInfo {
    /// The module or path being imported from.
    pub source: String,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty", default))]
    /// Specific names imported from the source module.
    pub items: Vec<String>,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none", default))]
    /// Alias assigned to the import (e.g., `import numpy as np`).
    pub alias: Option<String>,
    /// Whether this is a wildcard import (e.g., `import *` or `use foo::*`).
    pub is_wildcard: bool,
    /// Source span covering the import statement.
    pub span: Span,
}

/// The kind of an export statement found in source code.
///
/// Covers named exports, default exports, and re-exports from other modules.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ExportKind {
    #[default]
    /// A named export (e.g., `export { foo }`).
    Named,
    /// A default export (e.g., `export default foo`).
    Default,
    /// A re-export from another module (e.g., `export { foo } from 'bar'`).
    ReExport,
}

/// An export statement extracted from source code.
#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ExportInfo {
    /// The exported name.
    pub name: String,
    /// The kind of export (named, default, or re-export).
    pub kind: ExportKind,
    /// Source span covering the export statement.
    pub span: Span,
}

/// The kind of a symbol definition found in source code.
///
/// Categorizes symbol definitions such as variables, constants, functions,
/// classes, types, interfaces, enums, and modules.
///
/// # Wire format (public JSON contract)
///
/// Unit variants serialize as a bare string (`"Function"`); the `Other`
/// variant serializes as a single-keyed object (`{"Other": "macro"}`). DO
/// NOT add `#[serde(tag = "...")]`. Covered by `tests/wire_format.rs`.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum SymbolKind {
    #[default]
    /// A variable binding.
    Variable,
    /// A constant (immutable binding).
    Constant,
    /// A function definition.
    Function,
    /// A class definition.
    Class,
    /// A type alias or typedef.
    Type,
    /// An interface definition.
    Interface,
    /// An enum definition.
    Enum,
    /// A module declaration.
    Module,
    /// A symbol kind not covered by the standard variants.
    Other(String),
}

/// A symbol (variable, function, type, etc.) extracted from source code.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SymbolInfo {
    /// The name of the symbol.
    pub name: String,
    /// The kind of symbol (variable, function, class, etc.).
    pub kind: SymbolKind,
    /// Source span covering the symbol definition.
    pub span: Span,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    /// Explicit type annotation, if present in the source.
    pub type_annotation: Option<String>,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none", default))]
    /// Documentation comment associated with this symbol.
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
    /// A parse error (e.g., an `ERROR` or `MISSING` node in the tree).
    Error,
    /// A warning-level diagnostic.
    Warning,
    /// An informational diagnostic.
    Info,
}

/// A diagnostic (syntax error, missing node, etc.) from parsing.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Diagnostic {
    /// Human-readable description of the diagnostic.
    pub message: String,
    /// Severity of the diagnostic.
    pub severity: DiagnosticSeverity,
    /// Source span where the diagnostic was detected.
    pub span: Span,
}

/// A chunk of source code with rich metadata.
#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CodeChunk {
    /// The raw source text of this chunk.
    pub content: String,
    /// Inclusive start byte offset of this chunk in the original source.
    pub start_byte: usize,
    /// Exclusive end byte offset of this chunk in the original source.
    pub end_byte: usize,
    /// Zero-indexed start line of this chunk.
    pub start_line: usize,
    /// Zero-indexed end line of this chunk.
    pub end_line: usize,
    /// Contextual metadata about this chunk.
    pub metadata: ChunkContext,
}

/// Metadata for a single chunk of source code.
#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ChunkContext {
    /// Language name used to parse this chunk.
    pub language: String,
    /// Zero-indexed position of this chunk within the file's chunk list.
    pub chunk_index: usize,
    /// Total number of chunks the file was split into.
    pub total_chunks: usize,
    /// Tree-sitter node kinds that appear at the top level of this chunk.
    pub node_types: Vec<String>,
    /// Hierarchical path of enclosing structural items (e.g., `["MyClass", "my_method"]`).
    pub context_path: Vec<String>,
    /// Names of symbols defined within this chunk.
    pub symbols_defined: Vec<String>,
    /// Comments contained within this chunk.
    pub comments: Vec<CommentInfo>,
    /// Docstrings contained within this chunk.
    pub docstrings: Vec<DocstringInfo>,
    /// Whether this chunk contains any tree-sitter error nodes.
    pub has_error_nodes: bool,
}
