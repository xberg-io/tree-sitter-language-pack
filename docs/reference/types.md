---
title: "Types Reference"
---

## Types Reference

All types defined by the library, grouped by category. Types are shown using Rust as the canonical representation.

### Result Types

#### ProcessResult

Complete analysis result from processing a source file.

Contains metrics, structural analysis, imports/exports, comments,
docstrings, symbols, diagnostics, and optionally chunked code segments.
Fields are populated based on the `ProcessConfig` flags.


| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `language` | `String` | — | Language |
| `metrics` | `FileMetrics` | — | Metrics (file metrics) |
| `structure` | `Vec<StructureItem>` | `vec![]` | Structure |
| `imports` | `Vec<ImportInfo>` | `vec![]` | Imports |
| `exports` | `Vec<ExportInfo>` | `vec![]` | Exports |
| `comments` | `Vec<CommentInfo>` | `vec![]` | Comments |
| `docstrings` | `Vec<DocstringInfo>` | `vec![]` | Docstrings |
| `symbols` | `Vec<SymbolInfo>` | `vec![]` | Symbols |
| `diagnostics` | `Vec<Diagnostic>` | `vec![]` | Diagnostics |
| `chunks` | `Vec<CodeChunk>` | `vec![]` | Text chunks for chunking/embedding |

---

### Configuration Types

See [Configuration Reference](configuration.md) for detailed defaults and language-specific representations.

#### Span

Byte and line/column range in source code.

Represents both byte offsets (for slicing) and human-readable line/column
positions (for display and diagnostics).


| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `start_byte` | `usize` | — | Start byte |
| `end_byte` | `usize` | — | End byte |
| `start_line` | `usize` | — | Start line |
| `start_column` | `usize` | — | Start column |
| `end_line` | `usize` | — | End line |
| `end_column` | `usize` | — | End column |

---

#### FileMetrics

Aggregate metrics for a source file.


| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `total_lines` | `usize` | — | Total lines |
| `code_lines` | `usize` | — | Code lines |
| `comment_lines` | `usize` | — | Comment lines |
| `blank_lines` | `usize` | — | Blank lines |
| `total_bytes` | `usize` | — | Total bytes |
| `node_count` | `usize` | — | Number of nodes |
| `error_count` | `usize` | — | Number of errors |
| `max_depth` | `usize` | — | Maximum depth |

---

#### StructureItem

A structural item (function, class, struct, etc.) in source code.


| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `kind` | `StructureKind` | `StructureKind::Function` | Kind (structure kind) |
| `name` | `Option<String>` | `Default::default()` | The name |
| `visibility` | `Option<String>` | `Default::default()` | Visibility |
| `span` | `Span` | — | Span (span) |
| `children` | `Vec<StructureItem>` | `vec![]` | Children |
| `decorators` | `Vec<String>` | `vec![]` | Decorators |
| `doc_comment` | `Option<String>` | `Default::default()` | Doc comment |
| `signature` | `Option<String>` | `Default::default()` | Signature |
| `body_span` | `Option<Span>` | `Default::default()` | Body span (span) |

---

#### CommentInfo

A comment extracted from source code.


| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `text` | `String` | — | Text |
| `kind` | `CommentKind` | `CommentKind::Line` | Kind (comment kind) |
| `span` | `Span` | — | Span (span) |
| `associated_node` | `Option<String>` | `Default::default()` | Associated node |

---

#### DocstringInfo

A docstring extracted from source code.


| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `text` | `String` | — | Text |
| `format` | `DocstringFormat` | `DocstringFormat::PythonTripleQuote` | Format (docstring format) |
| `span` | `Span` | — | Span (span) |
| `associated_item` | `Option<String>` | `Default::default()` | Associated item |
| `parsed_sections` | `Vec<DocSection>` | `vec![]` | Parsed sections |

---

#### DocSection

A section within a docstring (e.g., Args, Returns, Raises).


| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `kind` | `String` | — | Kind |
| `name` | `Option<String>` | `Default::default()` | The name |
| `description` | `String` | — | Human-readable description |

---

#### ImportInfo

An import statement extracted from source code.


| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `source` | `String` | — | Source |
| `items` | `Vec<String>` | `vec![]` | Items |
| `alias` | `Option<String>` | `Default::default()` | Alias |
| `is_wildcard` | `bool` | — | Whether wildcard |
| `span` | `Span` | — | Span (span) |

---

#### ExportInfo

An export statement extracted from source code.


| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `name` | `String` | — | The name |
| `kind` | `ExportKind` | `ExportKind::Named` | Kind (export kind) |
| `span` | `Span` | — | Span (span) |

---

#### SymbolInfo

A symbol (variable, function, type, etc.) extracted from source code.


| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `name` | `String` | — | The name |
| `kind` | `SymbolKind` | `SymbolKind::Variable` | Kind (symbol kind) |
| `span` | `Span` | — | Span (span) |
| `type_annotation` | `Option<String>` | `Default::default()` | Type annotation |
| `doc` | `Option<String>` | `Default::default()` | Doc |

---

#### Diagnostic

A diagnostic (syntax error, missing node, etc.) from parsing.


| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `message` | `String` | — | Message |
| `severity` | `DiagnosticSeverity` | `DiagnosticSeverity::Error` | Severity (diagnostic severity) |
| `span` | `Span` | — | Span (span) |

---

#### CodeChunk

A chunk of source code with rich metadata.


| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `content` | `String` | — | The extracted text content |
| `start_byte` | `usize` | — | Start byte |
| `end_byte` | `usize` | — | End byte |
| `start_line` | `usize` | — | Start line |
| `end_line` | `usize` | — | End line |
| `metadata` | `ChunkContext` | — | Document metadata |

---

#### ChunkContext

Metadata for a single chunk of source code.


| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `language` | `String` | — | Language |
| `chunk_index` | `usize` | — | Chunk index |
| `total_chunks` | `usize` | — | Total chunks |
| `node_types` | `Vec<String>` | `vec![]` | Node types |
| `context_path` | `Vec<String>` | `vec![]` | Context path |
| `symbols_defined` | `Vec<String>` | `vec![]` | Symbols defined |
| `comments` | `Vec<CommentInfo>` | `vec![]` | Comments |
| `docstrings` | `Vec<DocstringInfo>` | `vec![]` | Docstrings |
| `has_error_nodes` | `bool` | — | Whether error nodes |

---

#### PackConfig

Configuration for the tree-sitter language pack.

Controls cache directory and which languages to pre-download.
Can be loaded from a TOML file, constructed programmatically,
or passed as a dict/object from language bindings.


| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `cache_dir` | `Option<PathBuf>` | `Default::default()` | Override default cache directory. Default: `~/.cache/tree-sitter-language-pack/v{version}/libs/` |
| `languages` | `Vec<String>` | `vec![]` | Languages to pre-download on init. Each entry is a language name (e.g. `"python"`, `"rust"`). |
| `groups` | `Vec<String>` | `vec![]` | Language groups to pre-download (e.g. `"web"`, `"systems"`, `"scripting"`). |

---

#### Parser

A tree-sitter parser configured for one language at a time.

*Opaque type — fields are not directly accessible.*

---

#### ProcessConfig

Configuration for the `process()` function.

Controls which analysis features are enabled and whether chunking is performed.


| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `language` | `String` | — | Language name (required). |
| `structure` | `bool` | `true` | Extract structural items (functions, classes, etc.). Default: true. |
| `imports` | `bool` | `true` | Extract import statements. Default: true. |
| `exports` | `bool` | `true` | Extract export statements. Default: true. |
| `comments` | `bool` | `false` | Extract comments. Default: false. |
| `docstrings` | `bool` | `false` | Extract docstrings. Default: false. |
| `symbols` | `bool` | `false` | Extract symbol definitions. Default: false. |
| `diagnostics` | `bool` | `false` | Include parse diagnostics. Default: false. |
| `chunk_max_size` | `Option<usize>` | `None` | Maximum chunk size in bytes. `None` disables chunking. |

---

#### LanguageRegistry

Thread-safe registry of tree-sitter language parsers.

Manages both statically compiled and dynamically loaded language grammars.
Use `LanguageRegistry.new()` for the default registry, or access the
global instance via the module-level convenience functions
(`get_language`, `available_languages`, etc.).

*Opaque type — fields are not directly accessible.*

---

### Document Structure

#### Node

A single syntax node within a `Tree`.

Nodes hold a strong reference to their parent tree so they remain valid
regardless of how the tree is moved or stored at the FFI boundary.

*Opaque type — fields are not directly accessible.*

---

### Other Types

#### Point

A source position — row + column, zero-indexed.


| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `row` | `usize` | — | Zero-indexed row number. |
| `column` | `usize` | — | Zero-indexed column number, in UTF-16 code units. |

---

#### ByteRange

A byte range — start (inclusive) to end (exclusive).


| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `start` | `usize` | — | Inclusive start byte offset. |
| `end` | `usize` | — | Exclusive end byte offset. |

---

#### Tree

A parsed syntax tree. Cheap to clone (refcount bump).

*Opaque type — fields are not directly accessible.*

---

#### TreeCursor

A cursor for traversing a `Tree`.

*Opaque type — fields are not directly accessible.*

---

#### DownloadManager

Manages downloading and caching of pre-built parser shared libraries.

*Opaque type — fields are not directly accessible.*

---

#### Language

*Opaque type — fields are not directly accessible.*

---

### Enums

#### CommentKind

The kind of a comment found in source code.

Distinguishes between single-line comments, block (multi-line) comments,
and documentation comments.


| Variant | Description |
|---------|-------------|
| `Line` | Line |
| `Block` | Block |
| `Doc` | Doc |

---

#### DiagnosticSeverity

Severity level of a diagnostic produced during parsing.

Used to classify parse errors, warnings, and informational messages
found in the syntax tree.


| Variant | Description |
|---------|-------------|
| `Error` | Error |
| `Warning` | Warning |
| `Info` | Info |

---

#### DocstringFormat

The format of a docstring extracted from source code.

Identifies the docstring convention used, which varies by language
(e.g., Python triple-quoted strings, JSDoc, Rustdoc `///` comments).


| Variant | Description |
|---------|-------------|
| `PythonTripleQuote` | Python triple quote |
| `JSDoc` | J s doc |
| `Rustdoc` | Rustdoc |
| `GoDoc` | Go doc |
| `JavaDoc` | Java doc |
| `Other` | Other — Fields: `_0`: `String` |

---

#### ExportKind

The kind of an export statement found in source code.

Covers named exports, default exports, and re-exports from other modules.


| Variant | Description |
|---------|-------------|
| `Named` | Named |
| `Default` | Default |
| `ReExport` | Re export |

---

#### StructureKind

The kind of structural item found in source code.

Categorizes top-level and nested declarations such as functions, classes,
structs, enums, traits, and more. Use `Other` for
language-specific constructs that do not fit a standard category.


| Variant | Description |
|---------|-------------|
| `Function` | Function |
| `Method` | Method |
| `Class` | Class |
| `Struct` | Struct |
| `Interface` | Interface |
| `Enum` | Enum |
| `Module` | Module |
| `Trait` | Trait |
| `Impl` | Impl |
| `Namespace` | Namespace |
| `Other` | Other — Fields: `_0`: `String` |

---

#### SymbolKind

The kind of a symbol definition found in source code.

Categorizes symbol definitions such as variables, constants, functions,
classes, types, interfaces, enums, and modules.


| Variant | Description |
|---------|-------------|
| `Variable` | Variable |
| `Constant` | Constant |
| `Function` | Function |
| `Class` | Class |
| `Type` | Type |
| `Interface` | Interface |
| `Enum` | Enum |
| `Module` | Module |
| `Other` | Other — Fields: `_0`: `String` |

---
