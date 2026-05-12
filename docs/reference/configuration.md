---
title: "Configuration Reference"
---

## Configuration Reference

This page documents all configuration types and their defaults across all languages.

### Span

Byte and line/column range in source code.

Represents both byte offsets (for slicing) and human-readable line/column
positions (for display and diagnostics).

| Field          | Type  | Default | Description  |
| -------------- | ----- | ------- | ------------ |
| `start_byte`   | `int` | —       | Start byte   |
| `end_byte`     | `int` | —       | End byte     |
| `start_line`   | `int` | —       | Start line   |
| `start_column` | `int` | —       | Start column |
| `end_line`     | `int` | —       | End line     |
| `end_column`   | `int` | —       | End column   |

---

### ProcessResult

Complete analysis result from processing a source file.

Contains metrics, structural analysis, imports/exports, comments,
docstrings, symbols, diagnostics, and optionally chunked code segments.
Fields are populated based on the `ProcessConfig` flags.

| Field         | Type                  | Default | Description                        |
| ------------- | --------------------- | ------- | ---------------------------------- |
| `language`    | `str`                 | —       | Language                           |
| `metrics`     | `FileMetrics`         | —       | Metrics (file metrics)             |
| `structure`   | `list[StructureItem]` | `[]`    | Structure                          |
| `imports`     | `list[ImportInfo]`    | `[]`    | Imports                            |
| `exports`     | `list[ExportInfo]`    | `[]`    | Exports                            |
| `comments`    | `list[CommentInfo]`   | `[]`    | Comments                           |
| `docstrings`  | `list[DocstringInfo]` | `[]`    | Docstrings                         |
| `symbols`     | `list[SymbolInfo]`    | `[]`    | Symbols                            |
| `diagnostics` | `list[Diagnostic]`    | `[]`    | Diagnostics                        |
| `chunks`      | `list[CodeChunk]`     | `[]`    | Text chunks for chunking/embedding |

---

### FileMetrics

Aggregate metrics for a source file.

| Field           | Type  | Default | Description      |
| --------------- | ----- | ------- | ---------------- |
| `total_lines`   | `int` | —       | Total lines      |
| `code_lines`    | `int` | —       | Code lines       |
| `comment_lines` | `int` | —       | Comment lines    |
| `blank_lines`   | `int` | —       | Blank lines      |
| `total_bytes`   | `int` | —       | Total bytes      |
| `node_count`    | `int` | —       | Number of nodes  |
| `error_count`   | `int` | —       | Number of errors |
| `max_depth`     | `int` | —       | Maximum depth    |

---

### StructureItem

A structural item (function, class, struct, etc.) in source code.

| Field         | Type                  | Default                  | Description           |
| ------------- | --------------------- | ------------------------ | --------------------- |
| `kind`        | `StructureKind`       | `StructureKind.FUNCTION` | Kind (structure kind) |
| `name`        | `str                  | None`                    | `None`                | The name         |
| `visibility`  | `str                  | None`                    | `None`                | Visibility       |
| `span`        | `Span`                | —                        | Span (span)           |
| `children`    | `list[StructureItem]` | `[]`                     | Children              |
| `decorators`  | `list[str]`           | `[]`                     | Decorators            |
| `doc_comment` | `str                  | None`                    | `None`                | Doc comment      |
| `signature`   | `str                  | None`                    | `None`                | Signature        |
| `body_span`   | `Span                 | None`                    | `None`                | Body span (span) |

---

### CommentInfo

A comment extracted from source code.

| Field             | Type          | Default            | Description         |
| ----------------- | ------------- | ------------------ | ------------------- |
| `text`            | `str`         | —                  | Text                |
| `kind`            | `CommentKind` | `CommentKind.LINE` | Kind (comment kind) |
| `span`            | `Span`        | —                  | Span (span)         |
| `associated_node` | `str          | None`              | `None`              | Associated node |

---

### DocstringInfo

A docstring extracted from source code.

| Field             | Type               | Default                               | Description               |
| ----------------- | ------------------ | ------------------------------------- | ------------------------- |
| `text`            | `str`              | —                                     | Text                      |
| `format`          | `DocstringFormat`  | `DocstringFormat.PYTHON_TRIPLE_QUOTE` | Format (docstring format) |
| `span`            | `Span`             | —                                     | Span (span)               |
| `associated_item` | `str               | None`                                 | `None`                    | Associated item |
| `parsed_sections` | `list[DocSection]` | `[]`                                  | Parsed sections           |

---

### DocSection

A section within a docstring (e.g., Args, Returns, Raises).

| Field         | Type  | Default | Description                |
| ------------- | ----- | ------- | -------------------------- |
| `kind`        | `str` | —       | Kind                       |
| `name`        | `str  | None`   | `None`                     | The name |
| `description` | `str` | —       | Human-readable description |

---

### ImportInfo

An import statement extracted from source code.

| Field         | Type        | Default | Description      |
| ------------- | ----------- | ------- | ---------------- |
| `source`      | `str`       | —       | Source           |
| `items`       | `list[str]` | `[]`    | Items            |
| `alias`       | `str        | None`   | `None`           | Alias |
| `is_wildcard` | `bool`      | —       | Whether wildcard |
| `span`        | `Span`      | —       | Span (span)      |

---

### ExportInfo

An export statement extracted from source code.

| Field  | Type         | Default            | Description        |
| ------ | ------------ | ------------------ | ------------------ |
| `name` | `str`        | —                  | The name           |
| `kind` | `ExportKind` | `ExportKind.NAMED` | Kind (export kind) |
| `span` | `Span`       | —                  | Span (span)        |

---

### SymbolInfo

A symbol (variable, function, type, etc.) extracted from source code.

| Field             | Type         | Default               | Description        |
| ----------------- | ------------ | --------------------- | ------------------ |
| `name`            | `str`        | —                     | The name           |
| `kind`            | `SymbolKind` | `SymbolKind.VARIABLE` | Kind (symbol kind) |
| `span`            | `Span`       | —                     | Span (span)        |
| `type_annotation` | `str         | None`                 | `None`             | Type annotation |
| `doc`             | `str         | None`                 | `None`             | Doc             |

---

### Diagnostic

A diagnostic (syntax error, missing node, etc.) from parsing.

| Field      | Type                 | Default                    | Description                    |
| ---------- | -------------------- | -------------------------- | ------------------------------ |
| `message`  | `str`                | —                          | Message                        |
| `severity` | `DiagnosticSeverity` | `DiagnosticSeverity.ERROR` | Severity (diagnostic severity) |
| `span`     | `Span`               | —                          | Span (span)                    |

---

### CodeChunk

A chunk of source code with rich metadata.

| Field        | Type           | Default | Description                |
| ------------ | -------------- | ------- | -------------------------- |
| `content`    | `str`          | —       | The extracted text content |
| `start_byte` | `int`          | —       | Start byte                 |
| `end_byte`   | `int`          | —       | End byte                   |
| `start_line` | `int`          | —       | Start line                 |
| `end_line`   | `int`          | —       | End line                   |
| `metadata`   | `ChunkContext` | —       | Document metadata          |

---

### ChunkContext

Metadata for a single chunk of source code.

| Field             | Type                  | Default | Description         |
| ----------------- | --------------------- | ------- | ------------------- |
| `language`        | `str`                 | —       | Language            |
| `chunk_index`     | `int`                 | —       | Chunk index         |
| `total_chunks`    | `int`                 | —       | Total chunks        |
| `node_types`      | `list[str]`           | `[]`    | Node types          |
| `context_path`    | `list[str]`           | `[]`    | Context path        |
| `symbols_defined` | `list[str]`           | `[]`    | Symbols defined     |
| `comments`        | `list[CommentInfo]`   | `[]`    | Comments            |
| `docstrings`      | `list[DocstringInfo]` | `[]`    | Docstrings          |
| `has_error_nodes` | `bool`                | —       | Whether error nodes |

---

### PackConfig

Configuration for the tree-sitter language pack.

Controls cache directory and which languages to pre-download.
Can be loaded from a TOML file, constructed programmatically,
or passed as a dict/object from language bindings.

| Field       | Type       | Default | Description |
| ----------- | ---------- | ------- | ----------- |
| `cache_dir` | `str       | None`   | `None`      | Override default cache directory. Default: `~/.cache/tree-sitter-language-pack/v{version}/libs/` |
| `languages` | `list[str] | None`   | `[]`        | Languages to pre-download on init. Each entry is a language name (e.g. `"python"`, `"rust"`).    |
| `groups`    | `list[str] | None`   | `[]`        | Language groups to pre-download (e.g. `"web"`, `"systems"`, `"scripting"`).                      |

---

### ProcessConfig

Configuration for the `process()` function.

Controls which analysis features are enabled and whether chunking is performed.

| Field            | Type   | Default | Description                                                         |
| ---------------- | ------ | ------- | ------------------------------------------------------------------- |
| `language`       | `str`  | —       | Language name (required).                                           |
| `structure`      | `bool` | `True`  | Extract structural items (functions, classes, etc.). Default: true. |
| `imports`        | `bool` | `True`  | Extract import statements. Default: true.                           |
| `exports`        | `bool` | `True`  | Extract export statements. Default: true.                           |
| `comments`       | `bool` | `False` | Extract comments. Default: false.                                   |
| `docstrings`     | `bool` | `False` | Extract docstrings. Default: false.                                 |
| `symbols`        | `bool` | `False` | Extract symbol definitions. Default: false.                         |
| `diagnostics`    | `bool` | `False` | Include parse diagnostics. Default: false.                          |
| `chunk_max_size` | `int   | None`   | `None`                                                              | Maximum chunk size in bytes. `None` disables chunking. |

---
