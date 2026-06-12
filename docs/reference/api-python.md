---
title: "Python API Reference"
---

## Python API Reference <span class="version-badge">v1.9.0-rc.33</span>

### Functions

#### detect_language_from_extension()

Detect language name from a file extension (without leading dot).

Returns `None` for unrecognized extensions. The match is case-insensitive.

**Signature:**

```python
def detect_language_from_extension(ext: str) -> str | None
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `ext` | `str` | Yes | The ext |

**Returns:** `str | None`

---

#### detect_language_from_path()

Detect language name from a file path.

Extracts the file extension and looks it up. Returns `None` if the
path has no extension or the extension is not recognized.

**Signature:**

```python
def detect_language_from_path(path: str) -> str | None
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `path` | `str` | Yes | Path to the file |

**Returns:** `str | None`

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

Returns `None` when content does not start with `#!`, the shebang is
malformed, or the interpreter is not recognised.

**Signature:**

```python
def detect_language_from_content(content: str) -> str | None
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `content` | `str` | Yes | The content to process |

**Returns:** `str | None`

---

#### get_highlights_query()

Get the highlights query for a language, if bundled.

Returns the contents of `highlights.scm` as a static string, or `None`
if no highlights query is bundled for this language.

**Signature:**

```python
def get_highlights_query(language: str) -> str | None
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `language` | `str` | Yes | The language |

**Returns:** `str | None`

---

#### get_injections_query()

Get the injections query for a language, if bundled.

Returns the contents of `injections.scm` as a static string, or `None`
if no injections query is bundled for this language.

**Signature:**

```python
def get_injections_query(language: str) -> str | None
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `language` | `str` | Yes | The language |

**Returns:** `str | None`

---

#### get_locals_query()

Get the locals query for a language, if bundled.

Returns the contents of `locals.scm` as a static string, or `None`
if no locals query is bundled for this language.

**Signature:**

```python
def get_locals_query(language: str) -> str | None
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `language` | `str` | Yes | The language |

**Returns:** `str | None`

---

#### get_tags_query()

Get the tags query for a language, if bundled.

Returns the contents of `tags.scm` as a static string, or `None`
if no tags query is bundled for this language.

**Signature:**

```python
def get_tags_query(language: str) -> str | None
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `language` | `str` | Yes | The language |

**Returns:** `str | None`

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

```python
def get_language(name: str) -> Language
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `name` | `str` | Yes | The name |

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

```python
def get_parser(name: str) -> Parser
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `name` | `str` | Yes | The name |

**Returns:** `Parser`
**Errors:** Raises `Error`.

---

#### detect_language()

Detect language name from a file path or extension.

This compatibility alias matches the pre-Alef Python binding API.

**Signature:**

```python
def detect_language(path: str) -> str | None
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `path` | `str` | Yes | Path to the file |

**Returns:** `str | None`

---

#### available_languages()

List all available language names (sorted, deduplicated, includes aliases).

Returns names of both statically compiled and dynamically loadable languages,
plus any configured aliases.

**Signature:**

```python
def available_languages() -> list[str]
```

**Returns:** `list[str]`

---

#### has_language()

Check if a language is available by name or alias.

Returns `True` if the language can be loaded (statically compiled,
dynamically available, or a known alias for one of these).

**Signature:**

```python
def has_language(name: str) -> bool
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `name` | `str` | Yes | The name |

**Returns:** `bool`

---

#### language_count()

Return the number of available languages.

Includes statically compiled languages, dynamically loadable languages,
and aliases.

**Signature:**

```python
def language_count() -> int
```

**Returns:** `int`

---

#### process()

Process source code and extract file intelligence using the global registry.

Parses the source with tree-sitter and extracts metrics, structure, imports,
exports, comments, docstrings, symbols, diagnostics, and/or chunks based on
the flags set in `ProcessConfig`.

**Errors:**

Returns an error if the language is not found or parsing fails.

**Signature:**

```python
def process(source: str, config: ProcessConfig) -> ProcessResult
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `source` | `str` | Yes | The source |
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

```python
def init(config: PackConfig) -> None
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `config` | `PackConfig` | Yes | The configuration options |

**Returns:** `None`
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

```python
def configure(config: PackConfig) -> None
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `config` | `PackConfig` | Yes | The configuration options |

**Returns:** `None`
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

```python
def download(names: list[str]) -> int
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `names` | `list[str]` | Yes | The names |

**Returns:** `int`
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

```python
def download_all() -> int
```

**Returns:** `int`
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

```python
def download_group(name: str) -> int
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `name` | `str` | Yes | The name |

**Returns:** `int`
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

```python
def manifest_languages() -> list[str]
```

**Returns:** `list[str]`
**Errors:** Raises `Error`.

---

#### downloaded_languages()

Return languages that are already downloaded and cached locally.

Does not perform any network requests. Returns an empty list if the
cache directory does not exist or cannot be read.

**Signature:**

```python
def downloaded_languages() -> list[str]
```

**Returns:** `list[str]`

---

#### clean_cache()

Delete all cached parser shared libraries.

Resets the cache registration so the next call to `get_language` or
a download function will re-register the (now empty) cache directory.

**Errors:**

Returns an error if the cache directory cannot be removed.

**Signature:**

```python
def clean_cache() -> None
```

**Returns:** `None`
**Errors:** Raises `Error`.

---

#### cache_dir()

Return the effective cache directory path.

This is either the custom path set via `configure` / `init` or the
default: `~/.cache/tree-sitter-language-pack/v{version}/libs/`.

**Errors:**

Returns an error if the system cache directory cannot be determined.

**Signature:**

```python
def cache_dir() -> str
```

**Returns:** `str`
**Errors:** Raises `Error`.

---

### Types

#### ByteRange

A byte range — start (inclusive) to end (exclusive).

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `start` | `int` | — | Inclusive start byte offset. |
| `end` | `int` | — | Exclusive end byte offset. |

---

#### ChunkContext

Metadata for a single chunk of source code.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `language` | `str` | — | Language name used to parse this chunk. |
| `chunk_index` | `int` | — | Zero-indexed position of this chunk within the file's chunk list. |
| `total_chunks` | `int` | — | Total number of chunks the file was split into. |
| `node_types` | `list[str]` | `[]` | Tree-sitter node kinds that appear at the top level of this chunk. |
| `context_path` | `list[str]` | `[]` | Hierarchical path of enclosing structural items (e.g., `["MyClass", "my_method"]`). |
| `symbols_defined` | `list[str]` | `[]` | Names of symbols defined within this chunk. |
| `comments` | `list[CommentInfo]` | `[]` | Comments contained within this chunk. |
| `docstrings` | `list[DocstringInfo]` | `[]` | Docstrings contained within this chunk. |
| `has_error_nodes` | `bool` | — | Whether this chunk contains any tree-sitter error nodes. |

---

#### CodeChunk

A chunk of source code with rich metadata.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `content` | `str` | — | The raw source text of this chunk. |
| `start_byte` | `int` | — | Inclusive start byte offset of this chunk in the original source. |
| `end_byte` | `int` | — | Exclusive end byte offset of this chunk in the original source. |
| `start_line` | `int` | — | Zero-indexed start line of this chunk. |
| `end_line` | `int` | — | Zero-indexed end line of this chunk. |
| `metadata` | `ChunkContext` | — | Contextual metadata about this chunk. |

---

#### CommentInfo

A comment extracted from source code.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `text` | `str` | — | The raw text content of the comment. |
| `kind` | `CommentKind` | `CommentKind.LINE` | The kind of comment (line, block, or doc). |
| `span` | `Span` | — | Source span covering the comment. |
| `associated_node` | `str \| None` | `None` | Name of the syntax node this comment is directly associated with. |

---

#### Diagnostic

A diagnostic (syntax error, missing node, etc.) from parsing.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `message` | `str` | — | Human-readable description of the diagnostic. |
| `severity` | `DiagnosticSeverity` | `DiagnosticSeverity.ERROR` | Severity of the diagnostic. |
| `span` | `Span` | — | Source span where the diagnostic was detected. |

---

#### DocSection

A section within a docstring (e.g., Args, Returns, Raises).

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `kind` | `str` | — | Section kind (e.g., `"args"`, `"returns"`, `"raises"`). |
| `name` | `str \| None` | `None` | Parameter or return value name, if applicable. |
| `description` | `str` | — | Description text for this section. |

---

#### DocstringInfo

A docstring extracted from source code.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `text` | `str` | — | The raw text of the docstring. |
| `format` | `DocstringFormat` | `DocstringFormat.PYTHON_TRIPLE_QUOTE` | The docstring format (Python, JSDoc, Rustdoc, etc.). |
| `span` | `Span` | — | Source span covering the docstring. |
| `associated_item` | `str \| None` | `None` | Name of the item this docstring documents. |
| `parsed_sections` | `list[DocSection]` | `[]` | Parsed sections of the docstring (Args, Returns, Raises, etc.). |

---

#### DownloadManager

Manages downloading and caching of pre-built parser shared libraries.

### Methods

#### new()

Create a new download manager for the given version.

**Signature:**

```python
@staticmethod
def new(version: str) -> DownloadManager
```

#### installed_languages()

List languages that are already downloaded and cached.

**Signature:**

```python
def installed_languages(self) -> list[str]
```

#### download_all_best_effort()

Download the platform bundle and extract every library file it contains.

Unlike `Self.ensure_languages`, this does not check the manifest language list
against archive contents — it simply extracts all `.so`/`.dylib`/`.dll` files
from the bundle. Languages in the manifest that are missing from the archive
are silently ignored rather than returning an error.

Returns the number of library files extracted (including those already cached).

**Signature:**

```python
def download_all_best_effort(self) -> int
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

```python
def clean_cache(self) -> None
```

---

#### ExportInfo

An export statement extracted from source code.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `name` | `str` | — | The exported name. |
| `kind` | `ExportKind` | `ExportKind.NAMED` | The kind of export (named, default, or re-export). |
| `span` | `Span` | — | Source span covering the export statement. |

---

#### FileMetrics

Aggregate metrics for a source file.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `total_lines` | `int` | — | Total number of lines (including blank and comment lines). |
| `code_lines` | `int` | — | Number of lines containing non-blank, non-comment source code. |
| `comment_lines` | `int` | — | Number of lines that are entirely comments. |
| `blank_lines` | `int` | — | Number of blank (whitespace-only) lines. |
| `total_bytes` | `int` | — | Total byte length of the source file. |
| `node_count` | `int` | — | Total number of nodes in the syntax tree. |
| `error_count` | `int` | — | Number of error nodes in the syntax tree (parse errors). |
| `max_depth` | `int` | — | Maximum nesting depth reached in the syntax tree. |

---

#### ImportInfo

An import statement extracted from source code.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `source` | `str` | — | The module or path being imported from. |
| `items` | `list[str]` | `[]` | Specific names imported from the source module. |
| `alias` | `str \| None` | `None` | Alias assigned to the import (e.g., `import numpy as np`). |
| `is_wildcard` | `bool` | — | Whether this is a wildcard import (e.g., `import *` or `use foo.*`). |
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

```python
@staticmethod
def new() -> LanguageRegistry
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

```python
def get_language(self, name: str) -> Language
```

#### available_languages()

List all available language names, sorted and deduplicated.

Includes statically compiled languages, dynamically loadable languages
(if the `dynamic-loading` feature is enabled), and all configured aliases.

**Signature:**

```python
def available_languages(self) -> list[str]
```

#### has_parser()

Check whether a parser is statically compiled into this build.

Returns `True` only when the grammar was compiled in at build time
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

```python
def has_parser(self, name: str) -> bool
```

#### has_language()

Check whether a language is available by name or alias.

Returns `True` if the language can be loaded, either from the static
table or from a dynamic library on disk.

**Signature:**

```python
def has_language(self, name: str) -> bool
```

#### language_count()

Return the total number of available languages (including aliases).

**Signature:**

```python
def language_count(self) -> int
```

#### process()

Parse source code and extract file intelligence based on config in a single pass.

**Signature:**

```python
def process(self, source: str, config: ProcessConfig) -> ProcessResult
```

#### default()

**Signature:**

```python
@staticmethod
def default() -> LanguageRegistry
```

---

#### Node

A single syntax node within a `Tree`.

Nodes hold a strong reference to their parent tree so they remain valid
regardless of how the tree is moved or stored at the FFI boundary.

### Methods

#### clone()

**Signature:**

```python
def clone(self) -> Node
```

#### kind()

Return the node's kind name (e.g. `"function_definition"`).

**Signature:**

```python
def kind(self) -> str
```

#### kind_id()

Return the node's numeric kind ID.

Tree-sitter assigns a stable `u16` ID to every node kind in a grammar
(e.g. `"function_definition" → 42`). Comparing `kind_id()` is cheaper
than comparing the string `kind()` in tight AST loops.

**Signature:**

```python
def kind_id(self) -> int
```

#### start_byte()

Return the inclusive start byte offset of this node.

**Signature:**

```python
def start_byte(self) -> int
```

#### end_byte()

Return the exclusive end byte offset of this node.

**Signature:**

```python
def end_byte(self) -> int
```

#### byte_range()

Return the node's byte range as a `ByteRange`.

Callers should slice their own source bytes — this is a zero-copy
text accessor.

**Signature:**

```python
def byte_range(self) -> ByteRange
```

#### start_position()

Return the start `Point` (row, column).

**Signature:**

```python
def start_position(self) -> Point
```

#### end_position()

Return the end `Point` (row, column).

**Signature:**

```python
def end_position(self) -> Point
```

#### is_named()

True when this node is named (not punctuation/whitespace).

**Signature:**

```python
def is_named(self) -> bool
```

#### is_error()

True when this is an error node.

**Signature:**

```python
def is_error(self) -> bool
```

#### is_missing()

True when this is a missing-token node.

**Signature:**

```python
def is_missing(self) -> bool
```

#### is_extra()

True when this is an "extra" node (e.g. a comment).

**Signature:**

```python
def is_extra(self) -> bool
```

#### has_error()

True when this node or any descendant is an error.

**Signature:**

```python
def has_error(self) -> bool
```

#### parent()

Return this node's parent, if any.

**Signature:**

```python
def parent(self) -> Node | None
```

#### child()

Return the i-th child of this node, if any.

**Signature:**

```python
def child(self, index: int) -> Node | None
```

#### child_count()

Total number of children (including unnamed).

**Signature:**

```python
def child_count(self) -> int
```

#### named_child()

Return the i-th named child of this node, if any.

**Signature:**

```python
def named_child(self, index: int) -> Node | None
```

#### named_child_count()

Number of named children of this node.

**Signature:**

```python
def named_child_count(self) -> int
```

#### child_by_field_name()

Look up a child by its grammar-defined field name.

**Signature:**

```python
def child_by_field_name(self, name: str) -> Node | None
```

#### to_sexp()

Return the S-expression form of this node's subtree.

**Signature:**

```python
def to_sexp(self) -> str
```

#### walk()

Return a `TreeCursor` positioned at this node.

**Signature:**

```python
def walk(self) -> TreeCursor
```

---

#### PackConfig

Configuration for the tree-sitter language pack.

Controls cache directory and which languages to pre-download.
Can be loaded from a TOML file, constructed programmatically,
or passed as a dict/object from language bindings.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `cache_dir` | `str \| None` | `None` | Override default cache directory. Default: `~/.cache/tree-sitter-language-pack/v{version}/libs/` |
| `languages` | `list[str] \| None` | `[]` | Languages to pre-download on init. Each entry is a language name (e.g. `"python"`, `"rust"`). |
| `groups` | `list[str] \| None` | `[]` | Language groups to pre-download (e.g. `"web"`, `"systems"`, `"scripting"`). |

---

#### Parser

A tree-sitter parser configured for one language at a time.

### Methods

#### new()

Construct a new parser with no language set.

Call `Parser.set_language` before parsing.

**Signature:**

```python
@staticmethod
def new() -> Parser
```

#### set_language()

Configure the parser to use the language identified by name (e.g. `"python"`).

Resolves the language through the global registry — auto-downloading
if necessary, when the `download` feature is enabled.

**Errors:**

Returns `Error.LanguageNotFound` if the language is not recognized,
or `Error.ParserSetup` if the language ABI is incompatible.

**Signature:**

```python
def set_language(self, name: str) -> None
```

#### parse()

Parse a UTF-8 source string. Returns `None` if parsing was cancelled
or no language is set.

**Signature:**

```python
def parse(self, source: str) -> Tree | None
```

#### parse_bytes()

Parse a raw byte slice. Returns `None` if parsing was cancelled or
no language is set.

**Signature:**

```python
def parse_bytes(self, source: bytes) -> Tree | None
```

#### reset()

Reset internal state. The next call to `parse` will
not be incremental.

**Signature:**

```python
def reset(self) -> None
```

#### default()

**Signature:**

```python
@staticmethod
def default() -> Parser
```

---

#### Point

A source position — row + column, zero-indexed.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `row` | `int` | — | Zero-indexed row number. |
| `column` | `int` | — | Zero-indexed column number, in UTF-16 code units. |

---

#### ProcessConfig

Configuration for the `process()` function.

Controls which analysis features are enabled and whether chunking is performed.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `language` | `str` | — | Language name (required). |
| `structure` | `bool` | `True` | Extract structural items (functions, classes, etc.). Default: true. |
| `imports` | `bool` | `True` | Extract import statements. Default: true. |
| `exports` | `bool` | `True` | Extract export statements. Default: true. |
| `comments` | `bool` | `False` | Extract comments. Default: false. |
| `docstrings` | `bool` | `False` | Extract docstrings. Default: false. |
| `symbols` | `bool` | `False` | Extract symbol definitions. Default: false. |
| `diagnostics` | `bool` | `False` | Include parse diagnostics. Default: false. |
| `chunk_max_size` | `int \| None` | `None` | Maximum chunk size in bytes. `None` disables chunking. |

### Methods

#### default()

**Signature:**

```python
@staticmethod
def default() -> ProcessConfig
```

#### with_chunking()

Enable chunking with the given maximum chunk size in bytes.

**Signature:**

```python
def with_chunking(self, max_size: int) -> ProcessConfig
```

#### all()

Enable all analysis features.

**Signature:**

```python
def all(self) -> ProcessConfig
```

#### minimal()

Disable all analysis features (only metrics computed).

**Signature:**

```python
def minimal(self) -> ProcessConfig
```

---

#### ProcessResult

Complete analysis result from processing a source file.

Contains metrics, structural analysis, imports/exports, comments,
docstrings, symbols, diagnostics, and optionally chunked code segments.
Fields are populated based on the `ProcessConfig` flags.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `language` | `str` | — | The language name used to parse the source file. |
| `metrics` | `FileMetrics` | — | File-level metrics (line counts, byte size, error count). |
| `structure` | `list[StructureItem]` | `[]` | Top-level structural items (functions, classes, etc.). |
| `imports` | `list[ImportInfo]` | `[]` | Import statements extracted from the source. |
| `exports` | `list[ExportInfo]` | `[]` | Export statements extracted from the source. |
| `comments` | `list[CommentInfo]` | `[]` | Comments extracted from the source. |
| `docstrings` | `list[DocstringInfo]` | `[]` | Docstrings extracted from the source. |
| `symbols` | `list[SymbolInfo]` | `[]` | Symbol definitions (variables, types, functions) extracted from the source. |
| `diagnostics` | `list[Diagnostic]` | `[]` | Parse diagnostics (syntax errors, missing nodes) from tree-sitter. |
| `chunks` | `list[CodeChunk]` | `[]` | Syntax-aware code chunks produced when chunking is enabled. |

---

#### Span

Byte and line/column range in source code.

Represents both byte offsets (for slicing) and human-readable line/column
positions (for display and diagnostics).

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `start_byte` | `int` | — | Inclusive start byte offset in the source. |
| `end_byte` | `int` | — | Exclusive end byte offset in the source. |
| `start_line` | `int` | — | Zero-indexed line number of the span's start. |
| `start_column` | `int` | — | Zero-indexed column number of the span's start. |
| `end_line` | `int` | — | Zero-indexed line number of the span's end. |
| `end_column` | `int` | — | Zero-indexed column number of the span's end. |

---

#### StructureItem

A structural item (function, class, struct, etc.) in source code.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `kind` | `StructureKind` | `StructureKind.FUNCTION` | The kind of structural item. |
| `name` | `str \| None` | `None` | The declared name of the item, if present. |
| `visibility` | `str \| None` | `None` | Visibility modifier (e.g., `"pub"`, `"public"`, `"private"`). |
| `span` | `Span` | — | Source span covering the entire item declaration. |
| `children` | `list[StructureItem]` | `[]` | Nested structural items (e.g., methods within a class). |
| `decorators` | `list[str]` | `[]` | Decorator or attribute names applied to the item. |
| `doc_comment` | `str \| None` | `None` | Documentation comment attached to the item, if any. |
| `signature` | `str \| None` | `None` | Full signature text of the item (e.g., function parameters and return type). |
| `body_span` | `Span \| None` | `None` | Source span covering only the body of the item, if distinct from the declaration. |

---

#### SymbolInfo

A symbol (variable, function, type, etc.) extracted from source code.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `name` | `str` | — | The name of the symbol. |
| `kind` | `SymbolKind` | `SymbolKind.VARIABLE` | The kind of symbol (variable, function, class, etc.). |
| `span` | `Span` | — | Source span covering the symbol definition. |
| `type_annotation` | `str \| None` | `None` | Explicit type annotation, if present in the source. |
| `doc` | `str \| None` | `None` | Documentation comment associated with this symbol. |

---

#### Tree

A parsed syntax tree. Cheap to clone (refcount bump).

### Methods

#### root_node()

Return the root `Node` of this tree.

**Signature:**

```python
def root_node(self) -> Node
```

#### walk()

Return a `TreeCursor` positioned at the root.

**Signature:**

```python
def walk(self) -> TreeCursor
```

---

#### TreeCursor

A cursor for traversing a `Tree`.

### Methods

#### node()

Return the `Node` at the cursor's current position.

**Signature:**

```python
def node(self) -> Node
```

#### goto_first_child()

Move the cursor to the first child of the current node.
Returns `True` if a child existed.

**Signature:**

```python
def goto_first_child(self) -> bool
```

#### goto_parent()

Move the cursor to the parent of the current node.
Returns `True` if a parent existed.

**Signature:**

```python
def goto_parent(self) -> bool
```

#### goto_next_sibling()

Move the cursor to the next sibling of the current node.
Returns `True` if a sibling existed.

**Signature:**

```python
def goto_next_sibling(self) -> bool
```

#### field_name()

Return the field name for the current node, if any.

**Signature:**

```python
def field_name(self) -> str | None
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
| `FUNCTION` | A free-standing or associated function. |
| `METHOD` | A method defined inside a class, struct, trait, or impl block. |
| `CLASS` | A class definition. |
| `STRUCT` | A struct definition. |
| `INTERFACE` | An interface or protocol definition. |
| `ENUM` | An enum definition. |
| `MODULE` | A module or package declaration. |
| `TRAIT` | A trait definition. |
| `IMPL` | An impl block (Rust) or similar implementation block. |
| `NAMESPACE` | A namespace declaration. |
| `OTHER` | A language-specific construct that does not fit any standard category. The `value` field carries the language-specific kind label. — Fields: `value`: `str` |

---

#### CommentKind

The kind of a comment found in source code.

Distinguishes between single-line comments, block (multi-line) comments,
and documentation comments.

| Value | Description |
|-------|-------------|
| `LINE` | A single-line comment (e.g., `// ...` or `# ...`). |
| `BLOCK` | A block or multi-line comment using slash-star delimiters. |
| `DOC` | A documentation comment such as `/// ...` or slash-double-star block. |

---

#### DocstringFormat

The format of a docstring extracted from source code.

Identifies the docstring convention used, which varies by language
(e.g., Python triple-quoted strings, JSDoc, Rustdoc `///` comments).

| Value | Description |
|-------|-------------|
| `PYTHON_TRIPLE_QUOTE` | Python triple-quoted string docstring (`"""..."""`). |
| `JS_DOC` | JavaScript/TypeScript JSDoc comment (`/** ... */`). |
| `RUSTDOC` | Rust `///` or `//!` doc comment. |
| `GO_DOC` | Go doc comment (a comment block immediately preceding a declaration). |
| `JAVA_DOC` | Java Javadoc comment (`/** ... */`). |
| `OTHER` | A language-specific docstring format not covered by the standard variants. — Fields: `value`: `str` |

---

#### ExportKind

The kind of an export statement found in source code.

Covers named exports, default exports, and re-exports from other modules.

| Value | Description |
|-------|-------------|
| `NAMED` | A named export (e.g., `export { foo }`). |
| `DEFAULT` | A default export (e.g., `export default foo`). |
| `RE_EXPORT` | A re-export from another module (e.g., `export { foo } from 'bar'`). |

---

#### SymbolKind

The kind of a symbol definition found in source code.

Categorizes symbol definitions such as variables, constants, functions,
classes, types, interfaces, enums, and modules.

| Value | Description |
|-------|-------------|
| `VARIABLE` | A variable binding. |
| `CONSTANT` | A constant (immutable binding). |
| `FUNCTION` | A function definition. |
| `CLASS` | A class definition. |
| `TYPE` | A type alias or typedef. |
| `INTERFACE` | An interface definition. |
| `ENUM` | An enum definition. |
| `MODULE` | A module declaration. |
| `OTHER` | A symbol kind not covered by the standard variants. — Fields: `value`: `str` |

---

#### DiagnosticSeverity

Severity level of a diagnostic produced during parsing.

Used to classify parse errors, warnings, and informational messages
found in the syntax tree.

| Value | Description |
|-------|-------------|
| `ERROR` | A parse error (e.g., an `ERROR` or `MISSING` node in the tree). |
| `WARNING` | A warning-level diagnostic. |
| `INFO` | An informational diagnostic. |

---

### Errors

#### Error

Errors that can occur when using the tree-sitter language pack.

Covers language lookup failures, parse errors, query errors, and I/O issues.
Feature-gated variants are included when `config`, `download`, or related
features are enabled.

**Base class:** `Error(Exception)`

| Exception | Description |
|-----------|-------------|
| `LanguageNotFound(Error)` | The requested language name (or alias) was not found in the registry. |
| `DynamicLoad(Error)` | A dynamic shared library could not be loaded at runtime. |
| `NullLanguagePointer(Error)` | The tree-sitter language function returned a null pointer for the given language name. |
| `ParserSetup(Error)` | The language could not be applied to the parser (e.g., ABI version mismatch). |
| `LockPoisoned(Error)` | An internal `RwLock` or `Mutex` was poisoned by a previous panic. |
| `Config(Error)` | A configuration file or value was invalid or could not be applied. |
| `ParseFailed(Error)` | The tree-sitter parser returned no tree for the given source input. |
| `QueryError(Error)` | A tree-sitter query could not be compiled or executed. |
| `InvalidRange(Error)` | A byte range was invalid (e.g., end before start, or out of bounds). |
| `Download(Error)` | A parser download from GitHub releases failed. |
| `ChecksumMismatch(Error)` | The downloaded file's SHA-256 digest did not match the manifest's expected value. |
| `CacheLock(Error)` | The cross-process download cache lock file could not be acquired or created. |

---
