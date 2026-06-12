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
| `language` | `String` | — | The language name used to parse the source file. |
| `metrics` | `FileMetrics` | — | File-level metrics (line counts, byte size, error count). |
| `structure` | `Vec<StructureItem>` | `vec![]` | Top-level structural items (functions, classes, etc.). |
| `imports` | `Vec<ImportInfo>` | `vec![]` | Import statements extracted from the source. |
| `exports` | `Vec<ExportInfo>` | `vec![]` | Export statements extracted from the source. |
| `comments` | `Vec<CommentInfo>` | `vec![]` | Comments extracted from the source. |
| `docstrings` | `Vec<DocstringInfo>` | `vec![]` | Docstrings extracted from the source. |
| `symbols` | `Vec<SymbolInfo>` | `vec![]` | Symbol definitions (variables, types, functions) extracted from the source. |
| `diagnostics` | `Vec<Diagnostic>` | `vec![]` | Parse diagnostics (syntax errors, missing nodes) from tree-sitter. |
| `chunks` | `Vec<CodeChunk>` | `vec![]` | Syntax-aware code chunks produced when chunking is enabled. |

---

### Configuration Types

See [Configuration Reference](configuration.md) for detailed defaults and language-specific representations.

#### Span

Byte and line/column range in source code.

Represents both byte offsets (for slicing) and human-readable line/column
positions (for display and diagnostics).

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `start_byte` | `usize` | — | Inclusive start byte offset in the source. |
| `end_byte` | `usize` | — | Exclusive end byte offset in the source. |
| `start_line` | `usize` | — | Zero-indexed line number of the span's start. |
| `start_column` | `usize` | — | Zero-indexed column number of the span's start. |
| `end_line` | `usize` | — | Zero-indexed line number of the span's end. |
| `end_column` | `usize` | — | Zero-indexed column number of the span's end. |

---

#### FileMetrics

Aggregate metrics for a source file.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `total_lines` | `usize` | — | Total number of lines (including blank and comment lines). |
| `code_lines` | `usize` | — | Number of lines containing non-blank, non-comment source code. |
| `comment_lines` | `usize` | — | Number of lines that are entirely comments. |
| `blank_lines` | `usize` | — | Number of blank (whitespace-only) lines. |
| `total_bytes` | `usize` | — | Total byte length of the source file. |
| `node_count` | `usize` | — | Total number of nodes in the syntax tree. |
| `error_count` | `usize` | — | Number of error nodes in the syntax tree (parse errors). |
| `max_depth` | `usize` | — | Maximum nesting depth reached in the syntax tree. |

---

#### StructureItem

A structural item (function, class, struct, etc.) in source code.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `kind` | `StructureKind` | `StructureKind::Function` | The kind of structural item. |
| `name` | `Option<String>` | `Default::default()` | The declared name of the item, if present. |
| `visibility` | `Option<String>` | `Default::default()` | Visibility modifier (e.g., `"pub"`, `"public"`, `"private"`). |
| `span` | `Span` | — | Source span covering the entire item declaration. |
| `children` | `Vec<StructureItem>` | `vec![]` | Nested structural items (e.g., methods within a class). |
| `decorators` | `Vec<String>` | `vec![]` | Decorator or attribute names applied to the item. |
| `doc_comment` | `Option<String>` | `Default::default()` | Documentation comment attached to the item, if any. |
| `signature` | `Option<String>` | `Default::default()` | Full signature text of the item (e.g., function parameters and return type). |
| `body_span` | `Option<Span>` | `Default::default()` | Source span covering only the body of the item, if distinct from the declaration. |

---

#### CommentInfo

A comment extracted from source code.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `text` | `String` | — | The raw text content of the comment. |
| `kind` | `CommentKind` | `CommentKind::Line` | The kind of comment (line, block, or doc). |
| `span` | `Span` | — | Source span covering the comment. |
| `associated_node` | `Option<String>` | `Default::default()` | Name of the syntax node this comment is directly associated with. |

---

#### DocstringInfo

A docstring extracted from source code.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `text` | `String` | — | The raw text of the docstring. |
| `format` | `DocstringFormat` | `DocstringFormat::PythonTripleQuote` | The docstring format (Python, JSDoc, Rustdoc, etc.). |
| `span` | `Span` | — | Source span covering the docstring. |
| `associated_item` | `Option<String>` | `Default::default()` | Name of the item this docstring documents. |
| `parsed_sections` | `Vec<DocSection>` | `vec![]` | Parsed sections of the docstring (Args, Returns, Raises, etc.). |

---

#### DocSection

A section within a docstring (e.g., Args, Returns, Raises).

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `kind` | `String` | — | Section kind (e.g., `"args"`, `"returns"`, `"raises"`). |
| `name` | `Option<String>` | `Default::default()` | Parameter or return value name, if applicable. |
| `description` | `String` | — | Description text for this section. |

---

#### ImportInfo

An import statement extracted from source code.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `source` | `String` | — | The module or path being imported from. |
| `items` | `Vec<String>` | `vec![]` | Specific names imported from the source module. |
| `alias` | `Option<String>` | `Default::default()` | Alias assigned to the import (e.g., `import numpy as np`). |
| `is_wildcard` | `bool` | — | Whether this is a wildcard import (e.g., `import *` or `use foo.*`). |
| `span` | `Span` | — | Source span covering the import statement. |

---

#### ExportInfo

An export statement extracted from source code.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `name` | `String` | — | The exported name. |
| `kind` | `ExportKind` | `ExportKind::Named` | The kind of export (named, default, or re-export). |
| `span` | `Span` | — | Source span covering the export statement. |

---

#### SymbolInfo

A symbol (variable, function, type, etc.) extracted from source code.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `name` | `String` | — | The name of the symbol. |
| `kind` | `SymbolKind` | `SymbolKind::Variable` | The kind of symbol (variable, function, class, etc.). |
| `span` | `Span` | — | Source span covering the symbol definition. |
| `type_annotation` | `Option<String>` | `Default::default()` | Explicit type annotation, if present in the source. |
| `doc` | `Option<String>` | `Default::default()` | Documentation comment associated with this symbol. |

---

#### Diagnostic

A diagnostic (syntax error, missing node, etc.) from parsing.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `message` | `String` | — | Human-readable description of the diagnostic. |
| `severity` | `DiagnosticSeverity` | `DiagnosticSeverity::Error` | Severity of the diagnostic. |
| `span` | `Span` | — | Source span where the diagnostic was detected. |

---

#### CodeChunk

A chunk of source code with rich metadata.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `content` | `String` | — | The raw source text of this chunk. |
| `start_byte` | `usize` | — | Inclusive start byte offset of this chunk in the original source. |
| `end_byte` | `usize` | — | Exclusive end byte offset of this chunk in the original source. |
| `start_line` | `usize` | — | Zero-indexed start line of this chunk. |
| `end_line` | `usize` | — | Zero-indexed end line of this chunk. |
| `metadata` | `ChunkContext` | — | Contextual metadata about this chunk. |

---

#### ChunkContext

Metadata for a single chunk of source code.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `language` | `String` | — | Language name used to parse this chunk. |
| `chunk_index` | `usize` | — | Zero-indexed position of this chunk within the file's chunk list. |
| `total_chunks` | `usize` | — | Total number of chunks the file was split into. |
| `node_types` | `Vec<String>` | `vec![]` | Tree-sitter node kinds that appear at the top level of this chunk. |
| `context_path` | `Vec<String>` | `vec![]` | Hierarchical path of enclosing structural items (e.g., `["MyClass", "my_method"]`). |
| `symbols_defined` | `Vec<String>` | `vec![]` | Names of symbols defined within this chunk. |
| `comments` | `Vec<CommentInfo>` | `vec![]` | Comments contained within this chunk. |
| `docstrings` | `Vec<DocstringInfo>` | `vec![]` | Docstrings contained within this chunk. |
| `has_error_nodes` | `bool` | — | Whether this chunk contains any tree-sitter error nodes. |

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

### Structured Data Types

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
| `Line` | A single-line comment (e.g., `// ...` or `# ...`). |
| `Block` | A block or multi-line comment using slash-star delimiters. |
| `Doc` | A documentation comment such as `/// ...` or slash-double-star block. |

---

#### DiagnosticSeverity

Severity level of a diagnostic produced during parsing.

Used to classify parse errors, warnings, and informational messages
found in the syntax tree.

| Variant | Description |
|---------|-------------|
| `Error` | A parse error (e.g., an `ERROR` or `MISSING` node in the tree). |
| `Warning` | A warning-level diagnostic. |
| `Info` | An informational diagnostic. |

---

#### DocstringFormat

The format of a docstring extracted from source code.

Identifies the docstring convention used, which varies by language
(e.g., Python triple-quoted strings, JSDoc, Rustdoc `///` comments).

| Variant | Description |
|---------|-------------|
| `PythonTripleQuote` | Python triple-quoted string docstring (`"""..."""`). |
| `JSDoc` | JavaScript/TypeScript JSDoc comment (`/** ... */`). |
| `Rustdoc` | Rust `///` or `//!` doc comment. |
| `GoDoc` | Go doc comment (a comment block immediately preceding a declaration). |
| `JavaDoc` | Java Javadoc comment (`/** ... */`). |
| `Other` | A language-specific docstring format not covered by the standard variants. — Fields: `value`: `String` |

---

#### ExportKind

The kind of an export statement found in source code.

Covers named exports, default exports, and re-exports from other modules.

| Variant | Description |
|---------|-------------|
| `Named` | A named export (e.g., `export { foo }`). |
| `Default` | A default export (e.g., `export default foo`). |
| `ReExport` | A re-export from another module (e.g., `export { foo } from 'bar'`). |

---

#### StructureKind

The kind of structural item found in source code.

Categorizes top-level and nested declarations such as functions, classes,
structs, enums, traits, and more. Use `Other` for
language-specific constructs that do not fit a standard category.

| Variant | Description |
|---------|-------------|
| `Function` | A free-standing or associated function. |
| `Method` | A method defined inside a class, struct, trait, or impl block. |
| `Class` | A class definition. |
| `Struct` | A struct definition. |
| `Interface` | An interface or protocol definition. |
| `Enum` | An enum definition. |
| `Module` | A module or package declaration. |
| `Trait` | A trait definition. |
| `Impl` | An impl block (Rust) or similar implementation block. |
| `Namespace` | A namespace declaration. |
| `Other` | A language-specific construct that does not fit any standard category. The `value` field carries the language-specific kind label. — Fields: `value`: `String` |

---

#### SymbolKind

The kind of a symbol definition found in source code.

Categorizes symbol definitions such as variables, constants, functions,
classes, types, interfaces, enums, and modules.

| Variant | Description |
|---------|-------------|
| `Variable` | A variable binding. |
| `Constant` | A constant (immutable binding). |
| `Function` | A function definition. |
| `Class` | A class definition. |
| `Type` | A type alias or typedef. |
| `Interface` | An interface definition. |
| `Enum` | An enum definition. |
| `Module` | A module declaration. |
| `Other` | A symbol kind not covered by the standard variants. — Fields: `value`: `String` |

---
