---
title: "Ruby API Reference"
---

## Ruby API Reference <span class="version-badge">v1.9.0-rc.27</span>

### Functions

#### detect_language_from_extension()

Detect language name from a file extension (without leading dot).

Returns `nil` for unrecognized extensions. The match is case-insensitive.

**Signature:**

```ruby
def self.detect_language_from_extension(ext)
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `ext` | `String` | Yes | The ext |

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

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `path` | `String` | Yes | Path to the file |

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

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `content` | `String` | Yes | The content to process |

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

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `language` | `String` | Yes | The language |

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

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `language` | `String` | Yes | The language |

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

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `language` | `String` | Yes | The language |

**Returns:** `String?`

---

#### get_tags_query()

Get the tags query for a language, if bundled.

Returns the contents of `tags.scm` as a static string, or `nil`
if no tags query is bundled for this language.

**Signature:**

```ruby
def self.get_tags_query(language)
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `language` | `String` | Yes | The language |

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

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `name` | `String` | Yes | The name |

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

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `name` | `String` | Yes | The name |

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

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `path` | `String` | Yes | Path to the file |

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

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `name` | `String` | Yes | The name |

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

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `source` | `String` | Yes | The source |
| `config` | `ProcessConfig` | Yes | The configuration options |

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

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `config` | `PackConfig` | Yes | The configuration options |

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

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `config` | `PackConfig` | Yes | The configuration options |

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

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `names` | `Array<String>` | Yes | The names |

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
`download()`. Already-cached languages are skipped.

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

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `name` | `String` | Yes | The name |

**Returns:** `Integer`
**Errors:** Raises `Error`.

---

#### manifest_languages()

Return all language names available in the remote manifest (306).

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

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `start` | `Integer` | — | Inclusive start byte offset. |
| `end` | `Integer` | — | Exclusive end byte offset. |

---

#### ChunkContext

Metadata for a single chunk of source code.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `language` | `String` | — | Language name used to parse this chunk. |
| `chunk_index` | `Integer` | — | Zero-indexed position of this chunk within the file's chunk list. |
| `total_chunks` | `Integer` | — | Total number of chunks the file was split into. |
| `node_types` | `Array<String>` | `[]` | Tree-sitter node kinds that appear at the top level of this chunk. |
| `context_path` | `Array<String>` | `[]` | Hierarchical path of enclosing structural items (e.g., `["MyClass", "my_method"]`). |
| `symbols_defined` | `Array<String>` | `[]` | Names of symbols defined within this chunk. |
| `comments` | `Array<CommentInfo>` | `[]` | Comments contained within this chunk. |
| `docstrings` | `Array<DocstringInfo>` | `[]` | Docstrings contained within this chunk. |
| `has_error_nodes` | `Boolean` | — | Whether this chunk contains any tree-sitter error nodes. |

---

#### CodeChunk

A chunk of source code with rich metadata.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `content` | `String` | — | The raw source text of this chunk. |
| `start_byte` | `Integer` | — | Inclusive start byte offset of this chunk in the original source. |
| `end_byte` | `Integer` | — | Exclusive end byte offset of this chunk in the original source. |
| `start_line` | `Integer` | — | Zero-indexed start line of this chunk. |
| `end_line` | `Integer` | — | Zero-indexed end line of this chunk. |
| `metadata` | `ChunkContext` | — | Contextual metadata about this chunk. |

---

#### CommentInfo

A comment extracted from source code.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `text` | `String` | — | The raw text content of the comment. |
| `kind` | `CommentKind` | `:line` | The kind of comment (line, block, or doc). |
| `span` | `Span` | — | Source span covering the comment. |
| `associated_node` | `String?` | `nil` | Name of the syntax node this comment is directly associated with. |

---

#### Diagnostic

A diagnostic (syntax error, missing node, etc.) from parsing.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `message` | `String` | — | Human-readable description of the diagnostic. |
| `severity` | `DiagnosticSeverity` | `:error` | Severity of the diagnostic. |
| `span` | `Span` | — | Source span where the diagnostic was detected. |

---

#### DocSection

A section within a docstring (e.g., Args, Returns, Raises).

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `kind` | `String` | — | Section kind (e.g., `"args"`, `"returns"`, `"raises"`). |
| `name` | `String?` | `nil` | Parameter or return value name, if applicable. |
| `description` | `String` | — | Description text for this section. |

---

#### DocstringInfo

A docstring extracted from source code.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `text` | `String` | — | The raw text of the docstring. |
| `format` | `DocstringFormat` | `:python_triple_quote` | The docstring format (Python, JSDoc, Rustdoc, etc.). |
| `span` | `Span` | — | Source span covering the docstring. |
| `associated_item` | `String?` | `nil` | Name of the item this docstring documents. |
| `parsed_sections` | `Array<DocSection>` | `[]` | Parsed sections of the docstring (Args, Returns, Raises, etc.). |

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

#### installed_languages()

List languages that are already downloaded and cached.

**Signature:**

```ruby
def installed_languages()
```

#### download_all_best_effort()

Download the platform bundle and extract every library file it contains.

Unlike `Self.ensure_languages`, this does not check the manifest language list
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

Acquires the cross-process lock so `clean_cache` cannot race a concurrent
downloader (avoids Windows sharing-violation errors against an in-flight
bundle write). The `.download.lock` file itself is **not** removed — it is
permanent infrastructure; deleting it could allow a concurrent process that
already opened the file to continue holding a stale lock handle while a new
process opens a fresh inode, breaking the mutual-exclusion guarantee.

**Signature:**

```ruby
def clean_cache()
```

---

#### ExportInfo

An export statement extracted from source code.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `name` | `String` | — | The exported name. |
| `kind` | `ExportKind` | `:named` | The kind of export (named, default, or re-export). |
| `span` | `Span` | — | Source span covering the export statement. |

---

#### FileMetrics

Aggregate metrics for a source file.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `total_lines` | `Integer` | — | Total number of lines (including blank and comment lines). |
| `code_lines` | `Integer` | — | Number of lines containing non-blank, non-comment source code. |
| `comment_lines` | `Integer` | — | Number of lines that are entirely comments. |
| `blank_lines` | `Integer` | — | Number of blank (whitespace-only) lines. |
| `total_bytes` | `Integer` | — | Total byte length of the source file. |
| `node_count` | `Integer` | — | Total number of nodes in the syntax tree. |
| `error_count` | `Integer` | — | Number of error nodes in the syntax tree (parse errors). |
| `max_depth` | `Integer` | — | Maximum nesting depth reached in the syntax tree. |

---

#### ImportInfo

An import statement extracted from source code.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `source` | `String` | — | The module or path being imported from. |
| `items` | `Array<String>` | `[]` | Specific names imported from the source module. |
| `alias` | `String?` | `nil` | Alias assigned to the import (e.g., `import numpy as np`). |
| `is_wildcard` | `Boolean` | — | Whether this is a wildcard import (e.g., `import *` or `use foo.*`). |
| `span` | `Span` | — | Source span covering the import statement. |

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

#### new()

Create a new registry populated with all statically compiled languages.

When the `dynamic-loading` feature is enabled, the registry also knows
about dynamically loadable grammars and will load them on demand.

**Signature:**

```ruby
def self.new()
```

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

#### has_parser()

Check whether a parser is statically compiled into this build.

Returns `true` only when the grammar was compiled in at build time
(i.e. it appears in the `STATIC_LANGUAGES` table). This is independent
of the extension-to-language mapping: `detect_language_from_extension`
consults the static ext table for all 306 grammars regardless of which
parsers are compiled in.

Use this when you need to distinguish "we know the language name" from
"we can actually parse files in that language right now".

```no_run
use tree_sitter_language_pack::{detect_language_from_extension, LanguageRegistry};

let registry = LanguageRegistry::new();
// Extension detection uses the static table — independent of compiled parsers.
let lang = detect_language_from_extension("feature"); // always returns Some("gherkin")
// Parser availability depends on which grammars were compiled in.
let can_parse = lang.map(|name| registry.has_parser(name)).unwrap_or(false);
```

**Signature:**

```ruby
def has_parser(name)
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

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `cache_dir` | `String?` | `nil` | Override default cache directory. Default: `~/.cache/tree-sitter-language-pack/v{version}/libs/` |
| `languages` | `Array<String>?` | `[]` | Languages to pre-download on init. Each entry is a language name (e.g. `"python"`, `"rust"`). |
| `groups` | `Array<String>?` | `[]` | Language groups to pre-download (e.g. `"web"`, `"systems"`, `"scripting"`). |

---

#### Parser

A tree-sitter parser configured for one language at a time.

### Methods

#### new()

Construct a new parser with no language set.

Call `Parser.set_language` before parsing.

**Signature:**

```ruby
def self.new()
```

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

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `row` | `Integer` | — | Zero-indexed row number. |
| `column` | `Integer` | — | Zero-indexed column number, in UTF-16 code units. |

---

#### ProcessConfig

Configuration for the `process()` function.

Controls which analysis features are enabled and whether chunking is performed.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `language` | `String` | — | Language name (required). |
| `structure` | `Boolean` | `true` | Extract structural items (functions, classes, etc.). Default: true. |
| `imports` | `Boolean` | `true` | Extract import statements. Default: true. |
| `exports` | `Boolean` | `true` | Extract export statements. Default: true. |
| `comments` | `Boolean` | `false` | Extract comments. Default: false. |
| `docstrings` | `Boolean` | `false` | Extract docstrings. Default: false. |
| `symbols` | `Boolean` | `false` | Extract symbol definitions. Default: false. |
| `diagnostics` | `Boolean` | `false` | Include parse diagnostics. Default: false. |
| `chunk_max_size` | `Integer?` | `nil` | Maximum chunk size in bytes. `nil` disables chunking. |

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

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `language` | `String` | — | The language name used to parse the source file. |
| `metrics` | `FileMetrics` | — | File-level metrics (line counts, byte size, error count). |
| `structure` | `Array<StructureItem>` | `[]` | Top-level structural items (functions, classes, etc.). |
| `imports` | `Array<ImportInfo>` | `[]` | Import statements extracted from the source. |
| `exports` | `Array<ExportInfo>` | `[]` | Export statements extracted from the source. |
| `comments` | `Array<CommentInfo>` | `[]` | Comments extracted from the source. |
| `docstrings` | `Array<DocstringInfo>` | `[]` | Docstrings extracted from the source. |
| `symbols` | `Array<SymbolInfo>` | `[]` | Symbol definitions (variables, types, functions) extracted from the source. |
| `diagnostics` | `Array<Diagnostic>` | `[]` | Parse diagnostics (syntax errors, missing nodes) from tree-sitter. |
| `chunks` | `Array<CodeChunk>` | `[]` | Syntax-aware code chunks produced when chunking is enabled. |

---

#### Span

Byte and line/column range in source code.

Represents both byte offsets (for slicing) and human-readable line/column
positions (for display and diagnostics).

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `start_byte` | `Integer` | — | Inclusive start byte offset in the source. |
| `end_byte` | `Integer` | — | Exclusive end byte offset in the source. |
| `start_line` | `Integer` | — | Zero-indexed line number of the span's start. |
| `start_column` | `Integer` | — | Zero-indexed column number of the span's start. |
| `end_line` | `Integer` | — | Zero-indexed line number of the span's end. |
| `end_column` | `Integer` | — | Zero-indexed column number of the span's end. |

---

#### StructureItem

A structural item (function, class, struct, etc.) in source code.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `kind` | `StructureKind` | `:function` | The kind of structural item. |
| `name` | `String?` | `nil` | The declared name of the item, if present. |
| `visibility` | `String?` | `nil` | Visibility modifier (e.g., `"pub"`, `"public"`, `"private"`). |
| `span` | `Span` | — | Source span covering the entire item declaration. |
| `children` | `Array<StructureItem>` | `[]` | Nested structural items (e.g., methods within a class). |
| `decorators` | `Array<String>` | `[]` | Decorator or attribute names applied to the item. |
| `doc_comment` | `String?` | `nil` | Documentation comment attached to the item, if any. |
| `signature` | `String?` | `nil` | Full signature text of the item (e.g., function parameters and return type). |
| `body_span` | `Span?` | `nil` | Source span covering only the body of the item, if distinct from the declaration. |

---

#### SymbolInfo

A symbol (variable, function, type, etc.) extracted from source code.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `name` | `String` | — | The name of the symbol. |
| `kind` | `SymbolKind` | `:variable` | The kind of symbol (variable, function, class, etc.). |
| `span` | `Span` | — | Source span covering the symbol definition. |
| `type_annotation` | `String?` | `nil` | Explicit type annotation, if present in the source. |
| `doc` | `String?` | `nil` | Documentation comment associated with this symbol. |

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

| Value | Description |
|-------|-------------|
| `function` | A free-standing or associated function. |
| `method` | A method defined inside a class, struct, trait, or impl block. |
| `class` | A class definition. |
| `struct` | A struct definition. |
| `interface` | An interface or protocol definition. |
| `enum` | An enum definition. |
| `module` | A module or package declaration. |
| `trait` | A trait definition. |
| `impl` | An impl block (Rust) or similar implementation block. |
| `namespace` | A namespace declaration. |
| `other` | A language-specific construct that does not fit any standard category. — Fields: `0`: `String` |

---

#### CommentKind

The kind of a comment found in source code.

Distinguishes between single-line comments, block (multi-line) comments,
and documentation comments.

| Value | Description |
|-------|-------------|
| `line` | A single-line comment (e.g., `// ...` or `# ...`). |
| `block` | A block or multi-line comment (e.g., `/* ... */`). |
| `doc` | A documentation comment (e.g., `/// ...` or `/** ... */`). |

---

#### DocstringFormat

The format of a docstring extracted from source code.

Identifies the docstring convention used, which varies by language
(e.g., Python triple-quoted strings, JSDoc, Rustdoc `///` comments).

| Value | Description |
|-------|-------------|
| `python_triple_quote` | Python triple-quoted string docstring (`"""..."""`). |
| `js_doc` | JavaScript/TypeScript JSDoc comment (`/** ... */`). |
| `rustdoc` | Rust `///` or `//!` doc comment. |
| `go_doc` | Go doc comment (a comment block immediately preceding a declaration). |
| `java_doc` | Java Javadoc comment (`/** ... */`). |
| `other` | A language-specific docstring format not covered by the standard variants. — Fields: `0`: `String` |

---

#### ExportKind

The kind of an export statement found in source code.

Covers named exports, default exports, and re-exports from other modules.

| Value | Description |
|-------|-------------|
| `named` | A named export (e.g., `export { foo }`). |
| `default` | A default export (e.g., `export default foo`). |
| `re_export` | A re-export from another module (e.g., `export { foo } from 'bar'`). |

---

#### SymbolKind

The kind of a symbol definition found in source code.

Categorizes symbol definitions such as variables, constants, functions,
classes, types, interfaces, enums, and modules.

| Value | Description |
|-------|-------------|
| `variable` | A variable binding. |
| `constant` | A constant (immutable binding). |
| `function` | A function definition. |
| `class` | A class definition. |
| `type` | A type alias or typedef. |
| `interface` | An interface definition. |
| `enum` | An enum definition. |
| `module` | A module declaration. |
| `other` | A symbol kind not covered by the standard variants. — Fields: `0`: `String` |

---

#### DiagnosticSeverity

Severity level of a diagnostic produced during parsing.

Used to classify parse errors, warnings, and informational messages
found in the syntax tree.

| Value | Description |
|-------|-------------|
| `error` | A parse error (e.g., an `ERROR` or `MISSING` node in the tree). |
| `warning` | A warning-level diagnostic. |
| `info` | An informational diagnostic. |

---

### Errors

#### Error

Errors that can occur when using the tree-sitter language pack.

Covers language lookup failures, parse errors, query errors, and I/O issues.
Feature-gated variants are included when `config`, `download`, or related
features are enabled.

| Variant | Description |
|---------|-------------|
| `language_not_found` | The requested language name (or alias) was not found in the registry. |
| `dynamic_load` | A dynamic shared library could not be loaded at runtime. |
| `null_language_pointer` | The tree-sitter language function returned a null pointer for the given language name. |
| `parser_setup` | The language could not be applied to the parser (e.g., ABI version mismatch). |
| `lock_poisoned` | An internal `RwLock` or `Mutex` was poisoned by a previous panic. |
| `config` | A configuration file or value was invalid or could not be applied. |
| `parse_failed` | The tree-sitter parser returned no tree for the given source input. |
| `query_error` | A tree-sitter query could not be compiled or executed. |
| `invalid_range` | A byte range was invalid (e.g., end before start, or out of bounds). |
| `download` | A parser download from GitHub releases failed. |
| `checksum_mismatch` | The downloaded file's SHA-256 digest did not match the manifest's expected value. |
| `cache_lock` | The cross-process download cache lock file could not be acquired or created. |

---
