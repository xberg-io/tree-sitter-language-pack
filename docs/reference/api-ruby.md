---
title: "Ruby API Reference"
---

## Ruby API Reference <span class="version-badge">v1.8.1</span>

### Functions

#### detect_language_from_extension()

Detect language name from a file extension (without leading dot).

Returns `nil` for unrecognized extensions. The match is case-insensitive.

**Signature:**

```ruby
def self.detect_language_from_extension(ext)
```

**Parameters:**

| Name  | Type     | Required | Description |
| ----- | -------- | -------- | ----------- |
| `ext` | `String` | Yes      | The ext     |

**Returns:** `String?`

---

#### detect_language_from_path()

Detect language name from a file path.

Extracts the file extension and looks it up. Returns `nil` if the
path has no extension or the extension is not recognized.

**Signature:**

```ruby
def self.detect_language_from_path(path)
```

**Parameters:**

| Name   | Type     | Required | Description      |
| ------ | -------- | -------- | ---------------- |
| `path` | `String` | Yes      | Path to the file |

**Returns:** `String?`

---

#### detect_language_from_content()

Detect language name from file content using the shebang line (`#!`).

Inspects only the first line of `content`. If it begins with `#!`, the
interpreter name is extracted and mapped to a language name.

Handles common patterns:

- `#!/usr/bin/env python3` → `"python"`
- `#!/bin/bash` → `"bash"`
- `#!/usr/bin/env node` → `"javascript"`

The `-S` flag accepted by some `env` implementations is skipped automatically.
Version suffixes (e.g. `python3.11`, `ruby3.2`) are stripped before matching.

Returns `nil` when content does not start with `#!`, the shebang is
malformed, or the interpreter is not recognised.

**Signature:**

```ruby
def self.detect_language_from_content(content)
```

**Parameters:**

| Name      | Type     | Required | Description            |
| --------- | -------- | -------- | ---------------------- |
| `content` | `String` | Yes      | The content to process |

**Returns:** `String?`

---

#### get_highlights_query()

Get the highlights query for a language, if bundled.

Returns the contents of `highlights.scm` as a static string, or `nil`
if no highlights query is bundled for this language.

**Signature:**

```ruby
def self.get_highlights_query(language)
```

**Parameters:**

| Name       | Type     | Required | Description  |
| ---------- | -------- | -------- | ------------ |
| `language` | `String` | Yes      | The language |

**Returns:** `String?`

---

#### get_injections_query()

Get the injections query for a language, if bundled.

Returns the contents of `injections.scm` as a static string, or `nil`
if no injections query is bundled for this language.

**Signature:**

```ruby
def self.get_injections_query(language)
```

**Parameters:**

| Name       | Type     | Required | Description  |
| ---------- | -------- | -------- | ------------ |
| `language` | `String` | Yes      | The language |

**Returns:** `String?`

---

#### get_locals_query()

Get the locals query for a language, if bundled.

Returns the contents of `locals.scm` as a static string, or `nil`
if no locals query is bundled for this language.

**Signature:**

```ruby
def self.get_locals_query(language)
```

**Parameters:**

| Name       | Type     | Required | Description  |
| ---------- | -------- | -------- | ------------ |
| `language` | `String` | Yes      | The language |

**Returns:** `String?`

---

#### get_language()

Get a tree-sitter `Language` by name using the global registry.

Resolves language aliases (e.g., `"shell"` maps to `"bash"`).
When the `download` feature is enabled (default), automatically downloads
the parser from GitHub releases if not found locally.

**Errors:**

Returns `Error.LanguageNotFound` if the language is not recognized,
or `Error.Download` if auto-download fails.

**Signature:**

```ruby
def self.get_language(name)
```

**Parameters:**

| Name   | Type     | Required | Description |
| ------ | -------- | -------- | ----------- |
| `name` | `String` | Yes      | The name    |

**Returns:** `Language`
**Errors:** Raises `Error`.

---

#### get_parser()

Get a `Parser` pre-configured for the given language.

This is a convenience function that calls `get_language` and configures
a new parser in one step.

**Errors:**

Returns `Error.LanguageNotFound` if the language is not recognized, or
`Error.ParserSetup` if the language cannot be applied to the parser.

**Signature:**

```ruby
def self.get_parser(name)
```

**Parameters:**

| Name   | Type     | Required | Description |
| ------ | -------- | -------- | ----------- |
| `name` | `String` | Yes      | The name    |

**Returns:** `Parser`
**Errors:** Raises `Error`.

---

#### detect_language()

Detect language name from a file path or extension.

This compatibility alias matches the pre-Alef Python binding API.

**Signature:**

```ruby
def self.detect_language(path)
```

**Parameters:**

| Name   | Type     | Required | Description      |
| ------ | -------- | -------- | ---------------- |
| `path` | `String` | Yes      | Path to the file |

**Returns:** `String?`

---

#### available_languages()

List all available language names (sorted, deduplicated, includes aliases).

Returns names of both statically compiled and dynamically loadable languages,
plus any configured aliases.

**Signature:**

```ruby
def self.available_languages()
```

**Returns:** `Array<String>`

---

#### has_language()

Check if a language is available by name or alias.

Returns `true` if the language can be loaded (statically compiled,
dynamically available, or a known alias for one of these).

**Signature:**

```ruby
def self.has_language(name)
```

**Parameters:**

| Name   | Type     | Required | Description |
| ------ | -------- | -------- | ----------- |
| `name` | `String` | Yes      | The name    |

**Returns:** `Boolean`

---

#### language_count()

Return the number of available languages.

Includes statically compiled languages, dynamically loadable languages,
and aliases.

**Signature:**

```ruby
def self.language_count()
```

**Returns:** `Integer`

---

#### process()

Process source code and extract file intelligence using the global registry.

Parses the source with tree-sitter and extracts metrics, structure, imports,
exports, comments, docstrings, symbols, diagnostics, and/or chunks based on
the flags set in `ProcessConfig`.

**Errors:**

Returns an error if the language is not found or parsing fails.

**Signature:**

```ruby
def self.process(source, config)
```

**Parameters:**

| Name     | Type            | Required | Description               |
| -------- | --------------- | -------- | ------------------------- |
| `source` | `String`        | Yes      | The source                |
| `config` | `ProcessConfig` | Yes      | The configuration options |

**Returns:** `ProcessResult`
**Errors:** Raises `Error`.

---

#### init()

Initialize the language pack with the given configuration.

Applies any custom cache directory, then downloads all languages and groups
specified in the config. This is the recommended entry point when you want
to pre-warm the cache before use.

**Errors:**

Returns an error if configuration cannot be applied or if downloads fail.

**Signature:**

```ruby
def self.init(config)
```

**Parameters:**

| Name     | Type         | Required | Description               |
| -------- | ------------ | -------- | ------------------------- |
| `config` | `PackConfig` | Yes      | The configuration options |

**Returns:** `nil`
**Errors:** Raises `Error`.

---

#### configure()

Apply download configuration without downloading anything.

Use this to set a custom cache directory before the first call to
`get_language` or any download function. Changing the cache dir
after languages have been registered has no effect on already-loaded
languages.

**Errors:**

Returns an error if the lock cannot be acquired.

**Signature:**

```ruby
def self.configure(config)
```

**Parameters:**

| Name     | Type         | Required | Description               |
| -------- | ------------ | -------- | ------------------------- |
| `config` | `PackConfig` | Yes      | The configuration options |

**Returns:** `nil`
**Errors:** Raises `Error`.

---

#### download()

Download specific languages to the local cache.

Returns the number of requested languages available after the call. Already
compiled or cached languages are included in the count.

**Errors:**

Returns an error if any language is not available in the manifest or if
the download fails.

**Signature:**

```ruby
def self.download(names)
```

**Parameters:**

| Name    | Type            | Required | Description |
| ------- | --------------- | -------- | ----------- |
| `names` | `Array<String>` | Yes      | The names   |

**Returns:** `Integer`
**Errors:** Raises `Error`.

---

#### download_all()

Download all available languages from the remote manifest.

Downloads the platform bundle and extracts every library it contains.
Languages that appear in the manifest but are absent from the bundle
(e.g. grammars that failed to compile at release time) are silently
skipped — they are not treated as an error.

Returns the total number of languages now available (statically compiled
plus downloaded and cached).

**Errors:**

Returns an error if the manifest cannot be fetched or the bundle download fails.

**Signature:**

```ruby
def self.download_all()
```

**Returns:** `Integer`
**Errors:** Raises `Error`.

---

#### download_group()

Download every language in a named group (e.g. `"web"`, `"data"`).

Groups are defined in the remote manifest and let you ensure a curated
set of related grammars in one call instead of listing each name to
`download`. Already-cached languages are skipped.

Returns the total number of languages now available (statically compiled
plus downloaded and cached).

**Errors:**

Returns an error if the manifest cannot be fetched, the group is unknown,
or any constituent language fails to download.

**Signature:**

```ruby
def self.download_group(name)
```

**Parameters:**

| Name   | Type     | Required | Description |
| ------ | -------- | -------- | ----------- |
| `name` | `String` | Yes      | The name    |

**Returns:** `Integer`
**Errors:** Raises `Error`.

---

#### manifest_languages()

Return all language names available in the remote manifest (305).

Fetches (and caches) the remote manifest to discover the full list of
downloadable languages. Use `downloaded_languages` to list what is
already cached locally.

**Errors:**

Returns an error if the manifest cannot be fetched.

**Signature:**

```ruby
def self.manifest_languages()
```

**Returns:** `Array<String>`
**Errors:** Raises `Error`.

---

#### downloaded_languages()

Return languages that are already downloaded and cached locally.

Does not perform any network requests. Returns an empty list if the
cache directory does not exist or cannot be read.

**Signature:**

```ruby
def self.downloaded_languages()
```

**Returns:** `Array<String>`

---

#### clean_cache()

Delete all cached parser shared libraries.

Resets the cache registration so the next call to `get_language` or
a download function will re-register the (now empty) cache directory.

**Errors:**

Returns an error if the cache directory cannot be removed.

**Signature:**

```ruby
def self.clean_cache()
```

**Returns:** `nil`
**Errors:** Raises `Error`.

---

#### cache_dir()

Return the effective cache directory path.

This is either the custom path set via `configure` / `init` or the
default: `~/.cache/tree-sitter-language-pack/v{version}/libs/`.

**Errors:**

Returns an error if the system cache directory cannot be determined.

**Signature:**

```ruby
def self.cache_dir()
```

**Returns:** `String`
**Errors:** Raises `Error`.

---

### Types

#### ByteRange

A byte range — start (inclusive) to end (exclusive).

| Field   | Type      | Default | Description                  |
| ------- | --------- | ------- | ---------------------------- |
| `start` | `Integer` | —       | Inclusive start byte offset. |
| `end`   | `Integer` | —       | Exclusive end byte offset.   |

---

#### ChunkContext

Metadata for a single chunk of source code.

| Field             | Type                   | Default | Description         |
| ----------------- | ---------------------- | ------- | ------------------- |
| `language`        | `String`               | —       | Language            |
| `chunk_index`     | `Integer`              | —       | Chunk index         |
| `total_chunks`    | `Integer`              | —       | Total chunks        |
| `node_types`      | `Array<String>`        | `[]`    | Node types          |
| `context_path`    | `Array<String>`        | `[]`    | Context path        |
| `symbols_defined` | `Array<String>`        | `[]`    | Symbols defined     |
| `comments`        | `Array<CommentInfo>`   | `[]`    | Comments            |
| `docstrings`      | `Array<DocstringInfo>` | `[]`    | Docstrings          |
| `has_error_nodes` | `Boolean`              | —       | Whether error nodes |

---

#### CodeChunk

A chunk of source code with rich metadata.

| Field        | Type           | Default | Description                |
| ------------ | -------------- | ------- | -------------------------- |
| `content`    | `String`       | —       | The extracted text content |
| `start_byte` | `Integer`      | —       | Start byte                 |
| `end_byte`   | `Integer`      | —       | End byte                   |
| `start_line` | `Integer`      | —       | Start line                 |
| `end_line`   | `Integer`      | —       | End line                   |
| `metadata`   | `ChunkContext` | —       | Document metadata          |

---

#### CommentInfo

A comment extracted from source code.

| Field             | Type          | Default | Description         |
| ----------------- | ------------- | ------- | ------------------- |
| `text`            | `String`      | —       | Text                |
| `kind`            | `CommentKind` | `:line` | Kind (comment kind) |
| `span`            | `Span`        | —       | Span (span)         |
| `associated_node` | `String?`     | `nil`   | Associated node     |

---

#### Diagnostic

A diagnostic (syntax error, missing node, etc.) from parsing.

| Field      | Type                 | Default  | Description                    |
| ---------- | -------------------- | -------- | ------------------------------ |
| `message`  | `String`             | —        | Message                        |
| `severity` | `DiagnosticSeverity` | `:error` | Severity (diagnostic severity) |
| `span`     | `Span`               | —        | Span (span)                    |

---

#### DocSection

A section within a docstring (e.g., Args, Returns, Raises).

| Field         | Type      | Default | Description                |
| ------------- | --------- | ------- | -------------------------- |
| `kind`        | `String`  | —       | Kind                       |
| `name`        | `String?` | `nil`   | The name                   |
| `description` | `String`  | —       | Human-readable description |

---

#### DocstringInfo

A docstring extracted from source code.

| Field             | Type                | Default                | Description               |
| ----------------- | ------------------- | ---------------------- | ------------------------- |
| `text`            | `String`            | —                      | Text                      |
| `format`          | `DocstringFormat`   | `:python_triple_quote` | Format (docstring format) |
| `span`            | `Span`              | —                      | Span (span)               |
| `associated_item` | `String?`           | `nil`                  | Associated item           |
| `parsed_sections` | `Array<DocSection>` | `[]`                   | Parsed sections           |

---

#### DownloadManager

Manages downloading and caching of pre-built parser shared libraries.

### Methods

#### new()

Create a new download manager for the given version.

**Signature:**

```ruby
def self.new(version)
```

#### with_cache_dir()

Create a download manager with a custom cache directory.

**Signature:**

```ruby
def self.with_cache_dir(version, cache_dir)
```

#### installed_languages()

List languages that are already downloaded and cached.

**Signature:**

```ruby
def installed_languages()
```

#### download_all_best_effort()

Download the platform bundle and extract every library file it contains.

Unlike `ensure_languages`, this does not check the manifest language list
against archive contents — it simply extracts all `.so`/`.dylib`/`.dll` files
from the bundle. Languages in the manifest that are missing from the archive
are silently ignored rather than returning an error.

Returns the number of library files extracted (including those already cached).

**Signature:**

```ruby
def download_all_best_effort()
```

#### clean_cache()

Remove all cached parser libraries.

**Signature:**

```ruby
def clean_cache()
```

---

#### ExportInfo

An export statement extracted from source code.

| Field  | Type         | Default  | Description        |
| ------ | ------------ | -------- | ------------------ |
| `name` | `String`     | —        | The name           |
| `kind` | `ExportKind` | `:named` | Kind (export kind) |
| `span` | `Span`       | —        | Span (span)        |

---

#### FileMetrics

Aggregate metrics for a source file.

| Field           | Type      | Default | Description      |
| --------------- | --------- | ------- | ---------------- |
| `total_lines`   | `Integer` | —       | Total lines      |
| `code_lines`    | `Integer` | —       | Code lines       |
| `comment_lines` | `Integer` | —       | Comment lines    |
| `blank_lines`   | `Integer` | —       | Blank lines      |
| `total_bytes`   | `Integer` | —       | Total bytes      |
| `node_count`    | `Integer` | —       | Number of nodes  |
| `error_count`   | `Integer` | —       | Number of errors |
| `max_depth`     | `Integer` | —       | Maximum depth    |

---

#### ImportInfo

An import statement extracted from source code.

| Field         | Type            | Default | Description      |
| ------------- | --------------- | ------- | ---------------- |
| `source`      | `String`        | —       | Source           |
| `items`       | `Array<String>` | `[]`    | Items            |
| `alias`       | `String?`       | `nil`   | Alias            |
| `is_wildcard` | `Boolean`       | —       | Whether wildcard |
| `span`        | `Span`          | —       | Span (span)      |

---

#### Language

---

#### LanguageRegistry

Thread-safe registry of tree-sitter language parsers.

Manages both statically compiled and dynamically loaded language grammars.
Use `LanguageRegistry.new()` for the default registry, or access the
global instance via the module-level convenience functions
(`get_language`, `available_languages`, etc.).

### Methods

#### get_language()

Get a tree-sitter `Language` by name.

Resolves aliases (e.g., `"shell"` -> `"bash"`, `"makefile"` -> `"make"`),
then looks up the language in the static table. When the `dynamic-loading`
feature is enabled, falls back to loading a shared library on demand.

**Errors:**

Returns `Error.LanguageNotFound` if the name (after alias resolution)
does not match any known grammar.

**Signature:**

```ruby
def get_language(name)
```

#### available_languages()

List all available language names, sorted and deduplicated.

Includes statically compiled languages, dynamically loadable languages
(if the `dynamic-loading` feature is enabled), and all configured aliases.

**Signature:**

```ruby
def available_languages()
```

#### has_language()

Check whether a language is available by name or alias.

Returns `true` if the language can be loaded, either from the static
table or from a dynamic library on disk.

**Signature:**

```ruby
def has_language(name)
```

#### language_count()

Return the total number of available languages (including aliases).

**Signature:**

```ruby
def language_count()
```

#### process()

Parse source code and extract file intelligence based on config in a single pass.

**Signature:**

```ruby
def process(source, config)
```

#### default()

**Signature:**

```ruby
def self.default()
```

---

#### Node

A single syntax node within a `Tree`.

Nodes hold a strong reference to their parent tree so they remain valid
regardless of how the tree is moved or stored at the FFI boundary.

### Methods

#### clone()

**Signature:**

```ruby
def clone()
```

#### kind()

Return the node's kind name (e.g. `"function_definition"`).

**Signature:**

```ruby
def kind()
```

#### kind_id()

Return the node's numeric kind ID.

Tree-sitter assigns a stable `u16` ID to every node kind in a grammar
(e.g. `"function_definition" → 42`). Comparing `kind_id()` is cheaper
than comparing the string `kind()` in tight AST loops.

**Signature:**

```ruby
def kind_id()
```

#### start_byte()

Return the inclusive start byte offset of this node.

**Signature:**

```ruby
def start_byte()
```

#### end_byte()

Return the exclusive end byte offset of this node.

**Signature:**

```ruby
def end_byte()
```

#### byte_range()

Return the node's byte range as a `ByteRange`.

Callers should slice their own source bytes — this is a zero-copy
text accessor.

**Signature:**

```ruby
def byte_range()
```

#### start_position()

Return the start `Point` (row, column).

**Signature:**

```ruby
def start_position()
```

#### end_position()

Return the end `Point` (row, column).

**Signature:**

```ruby
def end_position()
```

#### is_named()

True when this node is named (not punctuation/whitespace).

**Signature:**

```ruby
def is_named()
```

#### is_error()

True when this is an error node.

**Signature:**

```ruby
def is_error()
```

#### is_missing()

True when this is a missing-token node.

**Signature:**

```ruby
def is_missing()
```

#### is_extra()

True when this is an "extra" node (e.g. a comment).

**Signature:**

```ruby
def is_extra()
```

#### has_error()

True when this node or any descendant is an error.

**Signature:**

```ruby
def has_error()
```

#### parent()

Return this node's parent, if any.

**Signature:**

```ruby
def parent()
```

#### child()

Return the i-th child of this node, if any.

**Signature:**

```ruby
def child(index)
```

#### child_count()

Total number of children (including unnamed).

**Signature:**

```ruby
def child_count()
```

#### named_child()

Return the i-th named child of this node, if any.

**Signature:**

```ruby
def named_child(index)
```

#### named_child_count()

Number of named children of this node.

**Signature:**

```ruby
def named_child_count()
```

#### child_by_field_name()

Look up a child by its grammar-defined field name.

**Signature:**

```ruby
def child_by_field_name(name)
```

#### to_sexp()

Return the S-expression form of this node's subtree.

**Signature:**

```ruby
def to_sexp()
```

#### walk()

Return a `TreeCursor` positioned at this node.

**Signature:**

```ruby
def walk()
```

---

#### PackConfig

Configuration for the tree-sitter language pack.

Controls cache directory and which languages to pre-download.
Can be loaded from a TOML file, constructed programmatically,
or passed as a dict/object from language bindings.

| Field       | Type             | Default | Description                                                                                      |
| ----------- | ---------------- | ------- | ------------------------------------------------------------------------------------------------ |
| `cache_dir` | `String?`        | `nil`   | Override default cache directory. Default: `~/.cache/tree-sitter-language-pack/v{version}/libs/` |
| `languages` | `Array<String>?` | `[]`    | Languages to pre-download on init. Each entry is a language name (e.g. `"python"`, `"rust"`).    |
| `groups`    | `Array<String>?` | `[]`    | Language groups to pre-download (e.g. `"web"`, `"systems"`, `"scripting"`).                      |

---

#### Parser

A tree-sitter parser configured for one language at a time.

### Methods

#### set_language()

Configure the parser to use the language identified by name (e.g. `"python"`).

Resolves the language through the global registry — auto-downloading
if necessary, when the `download` feature is enabled.

**Errors:**

Returns `Error.LanguageNotFound` if the language is not recognized,
or `Error.ParserSetup` if the language ABI is incompatible.

**Signature:**

```ruby
def set_language(name)
```

#### parse()

Parse a UTF-8 source string. Returns `nil` if parsing was cancelled
or no language is set.

**Signature:**

```ruby
def parse(source)
```

#### parse_bytes()

Parse a raw byte slice. Returns `nil` if parsing was cancelled or
no language is set.

**Signature:**

```ruby
def parse_bytes(source)
```

#### reset()

Reset internal state. The next call to `parse` will
not be incremental.

**Signature:**

```ruby
def reset()
```

#### default()

**Signature:**

```ruby
def self.default()
```

---

#### Point

A source position — row + column, zero-indexed.

| Field    | Type      | Default | Description                                       |
| -------- | --------- | ------- | ------------------------------------------------- |
| `row`    | `Integer` | —       | Zero-indexed row number.                          |
| `column` | `Integer` | —       | Zero-indexed column number, in UTF-16 code units. |

---

#### ProcessConfig

Configuration for the `process()` function.

Controls which analysis features are enabled and whether chunking is performed.

| Field            | Type       | Default | Description                                                         |
| ---------------- | ---------- | ------- | ------------------------------------------------------------------- |
| `language`       | `String`   | —       | Language name (required).                                           |
| `structure`      | `Boolean`  | `true`  | Extract structural items (functions, classes, etc.). Default: true. |
| `imports`        | `Boolean`  | `true`  | Extract import statements. Default: true.                           |
| `exports`        | `Boolean`  | `true`  | Extract export statements. Default: true.                           |
| `comments`       | `Boolean`  | `false` | Extract comments. Default: false.                                   |
| `docstrings`     | `Boolean`  | `false` | Extract docstrings. Default: false.                                 |
| `symbols`        | `Boolean`  | `false` | Extract symbol definitions. Default: false.                         |
| `diagnostics`    | `Boolean`  | `false` | Include parse diagnostics. Default: false.                          |
| `chunk_max_size` | `Integer?` | `nil`   | Maximum chunk size in bytes. `nil` disables chunking.               |

### Methods

#### default()

**Signature:**

```ruby
def self.default()
```

#### with_chunking()

Enable chunking with the given maximum chunk size in bytes.

**Signature:**

```ruby
def with_chunking(max_size)
```

#### all()

Enable all analysis features.

**Signature:**

```ruby
def all()
```

#### minimal()

Disable all analysis features (only metrics computed).

**Signature:**

```ruby
def minimal()
```

---

#### ProcessResult

Complete analysis result from processing a source file.

Contains metrics, structural analysis, imports/exports, comments,
docstrings, symbols, diagnostics, and optionally chunked code segments.
Fields are populated based on the `ProcessConfig` flags.

| Field         | Type                   | Default | Description                        |
| ------------- | ---------------------- | ------- | ---------------------------------- |
| `language`    | `String`               | —       | Language                           |
| `metrics`     | `FileMetrics`          | —       | Metrics (file metrics)             |
| `structure`   | `Array<StructureItem>` | `[]`    | Structure                          |
| `imports`     | `Array<ImportInfo>`    | `[]`    | Imports                            |
| `exports`     | `Array<ExportInfo>`    | `[]`    | Exports                            |
| `comments`    | `Array<CommentInfo>`   | `[]`    | Comments                           |
| `docstrings`  | `Array<DocstringInfo>` | `[]`    | Docstrings                         |
| `symbols`     | `Array<SymbolInfo>`    | `[]`    | Symbols                            |
| `diagnostics` | `Array<Diagnostic>`    | `[]`    | Diagnostics                        |
| `chunks`      | `Array<CodeChunk>`     | `[]`    | Text chunks for chunking/embedding |

---

#### Span

Byte and line/column range in source code.

Represents both byte offsets (for slicing) and human-readable line/column
positions (for display and diagnostics).

| Field          | Type      | Default | Description  |
| -------------- | --------- | ------- | ------------ |
| `start_byte`   | `Integer` | —       | Start byte   |
| `end_byte`     | `Integer` | —       | End byte     |
| `start_line`   | `Integer` | —       | Start line   |
| `start_column` | `Integer` | —       | Start column |
| `end_line`     | `Integer` | —       | End line     |
| `end_column`   | `Integer` | —       | End column   |

---

#### StructureItem

A structural item (function, class, struct, etc.) in source code.

| Field         | Type                   | Default     | Description           |
| ------------- | ---------------------- | ----------- | --------------------- |
| `kind`        | `StructureKind`        | `:function` | Kind (structure kind) |
| `name`        | `String?`              | `nil`       | The name              |
| `visibility`  | `String?`              | `nil`       | Visibility            |
| `span`        | `Span`                 | —           | Span (span)           |
| `children`    | `Array<StructureItem>` | `[]`        | Children              |
| `decorators`  | `Array<String>`        | `[]`        | Decorators            |
| `doc_comment` | `String?`              | `nil`       | Doc comment           |
| `signature`   | `String?`              | `nil`       | Signature             |
| `body_span`   | `Span?`                | `nil`       | Body span (span)      |

---

#### SymbolInfo

A symbol (variable, function, type, etc.) extracted from source code.

| Field             | Type         | Default     | Description        |
| ----------------- | ------------ | ----------- | ------------------ |
| `name`            | `String`     | —           | The name           |
| `kind`            | `SymbolKind` | `:variable` | Kind (symbol kind) |
| `span`            | `Span`       | —           | Span (span)        |
| `type_annotation` | `String?`    | `nil`       | Type annotation    |
| `doc`             | `String?`    | `nil`       | Doc                |

---

#### Tree

A parsed syntax tree. Cheap to clone (refcount bump).

### Methods

#### root_node()

Return the root `Node` of this tree.

**Signature:**

```ruby
def root_node()
```

#### walk()

Return a `TreeCursor` positioned at the root.

**Signature:**

```ruby
def walk()
```

---

#### TreeCursor

A cursor for traversing a `Tree`.

### Methods

#### node()

Return the `Node` at the cursor's current position.

**Signature:**

```ruby
def node()
```

#### goto_first_child()

Move the cursor to the first child of the current node.
Returns `true` if a child existed.

**Signature:**

```ruby
def goto_first_child()
```

#### goto_parent()

Move the cursor to the parent of the current node.
Returns `true` if a parent existed.

**Signature:**

```ruby
def goto_parent()
```

#### goto_next_sibling()

Move the cursor to the next sibling of the current node.
Returns `true` if a sibling existed.

**Signature:**

```ruby
def goto_next_sibling()
```

#### field_name()

Return the field name for the current node, if any.

**Signature:**

```ruby
def field_name()
```

---

### Enums

#### StructureKind

The kind of structural item found in source code.

Categorizes top-level and nested declarations such as functions, classes,
structs, enums, traits, and more. Use `Other` for
language-specific constructs that do not fit a standard category.

| Value       | Description                   |
| ----------- | ----------------------------- |
| `function`  | Function                      |
| `method`    | Method                        |
| `class`     | Class                         |
| `struct`    | Struct                        |
| `interface` | Interface                     |
| `enum`      | Enum                          |
| `module`    | Module                        |
| `trait`     | Trait                         |
| `impl`      | Impl                          |
| `namespace` | Namespace                     |
| `other`     | Other — Fields: `0`: `String` |

---

#### CommentKind

The kind of a comment found in source code.

Distinguishes between single-line comments, block (multi-line) comments,
and documentation comments.

| Value   | Description |
| ------- | ----------- |
| `line`  | Line        |
| `block` | Block       |
| `doc`   | Doc         |

---

#### DocstringFormat

The format of a docstring extracted from source code.

Identifies the docstring convention used, which varies by language
(e.g., Python triple-quoted strings, JSDoc, Rustdoc `///` comments).

| Value                 | Description                   |
| --------------------- | ----------------------------- |
| `python_triple_quote` | Python triple quote           |
| `js_doc`              | J s doc                       |
| `rustdoc`             | Rustdoc                       |
| `go_doc`              | Go doc                        |
| `java_doc`            | Java doc                      |
| `other`               | Other — Fields: `0`: `String` |

---

#### ExportKind

The kind of an export statement found in source code.

Covers named exports, default exports, and re-exports from other modules.

| Value       | Description |
| ----------- | ----------- |
| `named`     | Named       |
| `default`   | Default     |
| `re_export` | Re export   |

---

#### SymbolKind

The kind of a symbol definition found in source code.

Categorizes symbol definitions such as variables, constants, functions,
classes, types, interfaces, enums, and modules.

| Value       | Description                   |
| ----------- | ----------------------------- |
| `variable`  | Variable                      |
| `constant`  | Constant                      |
| `function`  | Function                      |
| `class`     | Class                         |
| `type`      | Type                          |
| `interface` | Interface                     |
| `enum`      | Enum                          |
| `module`    | Module                        |
| `other`     | Other — Fields: `0`: `String` |

---

#### DiagnosticSeverity

Severity level of a diagnostic produced during parsing.

Used to classify parse errors, warnings, and informational messages
found in the syntax tree.

| Value     | Description |
| --------- | ----------- |
| `error`   | Error       |
| `warning` | Warning     |
| `info`    | Info        |

---

### Errors

#### Error

Errors that can occur when using the tree-sitter language pack.

Covers language lookup failures, parse errors, query errors, and I/O issues.
Feature-gated variants are included when `config`, `download`, or related
features are enabled.

| Variant                 | Description                                       |
| ----------------------- | ------------------------------------------------- |
| `language_not_found`    | Language '{0}' not found                          |
| `dynamic_load`          | Dynamic library load error: {0}                   |
| `null_language_pointer` | Language function returned null pointer for '{0}' |
| `parser_setup`          | Failed to set parser language: {0}                |
| `lock_poisoned`         | Registry lock poisoned: {0}                       |
| `config`                | Configuration error: {0}                          |
| `parse_failed`          | Parse failed: parsing returned no tree            |
| `query_error`           | Query error: {0}                                  |
| `invalid_range`         | Invalid byte range: {0}                           |
| `io`                    | IO error: {0}                                     |

---
