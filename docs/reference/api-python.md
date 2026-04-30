---
title: "Python API Reference"
---

## Python API Reference <span class="version-badge">v1.8.0-rc.24</span>

### Functions

#### Detect_language_from_extension()

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

#### Detect_language_from_path()

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

#### Detect_language_from_content()

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

#### Root_node_info()

Get a `NodeInfo` snapshot of the root node.

**Signature:**

```python
def root_node_info(tree: Tree) -> NodeInfo
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `tree` | `Tree` | Yes | The tree |

**Returns:** `NodeInfo`


---

#### Find_nodes_by_type()

Find all nodes matching the given type name, returning their `NodeInfo`.

Performs a depth-first traversal. Returns an empty vec if no matches.

**Signature:**

```python
def find_nodes_by_type(tree: Tree, node_type: str) -> list[NodeInfo]
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `tree` | `Tree` | Yes | The tree |
| `node_type` | `str` | Yes | The node type |

**Returns:** `list[NodeInfo]`


---

#### Named_children_info()

Get `NodeInfo` for all named children of the root node.

Useful for understanding the top-level structure of a file
(e.g., list of function definitions, class declarations, imports).

**Signature:**

```python
def named_children_info(tree: Tree) -> list[NodeInfo]
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `tree` | `Tree` | Yes | The tree |

**Returns:** `list[NodeInfo]`


---

#### Parse_string()

Parse source code with the named language, returning the syntax tree.

Uses the global registry to look up the language by name.
Caches parsers per-thread so repeated calls for the same language avoid
re-creating the parser.

**Signature:**

```python
def parse_string(language: str, source: bytes) -> Tree
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `language` | `str` | Yes | The language |
| `source` | `bytes` | Yes | The source |

**Returns:** `Tree`

**Errors:** Raises `Error`.


---

#### Tree_contains_node_type()

Check whether any node in the tree matches the given type name.

Performs a depth-first traversal using `TreeCursor`.

**Signature:**

```python
def tree_contains_node_type(tree: Tree, node_type: str) -> bool
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `tree` | `Tree` | Yes | The tree |
| `node_type` | `str` | Yes | The node type |

**Returns:** `bool`


---

#### Tree_has_error_nodes()

Check whether the tree contains any ERROR or MISSING nodes.

Useful for determining if the parse was clean or had syntax errors.

**Signature:**

```python
def tree_has_error_nodes(tree: Tree) -> bool
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `tree` | `Tree` | Yes | The tree |

**Returns:** `bool`


---

#### Tree_to_sexp()

Return the S-expression representation of the entire tree.

This is the standard tree-sitter debug format, useful for logging,
snapshot testing, and debugging grammars.

**Signature:**

```python
def tree_to_sexp(tree: Tree) -> str
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `tree` | `Tree` | Yes | The tree |

**Returns:** `str`


---

#### Tree_error_count()

Count the number of ERROR and MISSING nodes in the tree.

Returns 0 for a clean parse.

**Signature:**

```python
def tree_error_count(tree: Tree) -> int
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `tree` | `Tree` | Yes | The tree |

**Returns:** `int`


---

#### Get_highlights_query()

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

#### Get_injections_query()

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

#### Get_locals_query()

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

#### Run_query()

Execute a tree-sitter query pattern against a parsed tree.

The `query_source` is an S-expression pattern like:

```text
(function_definition name: (identifier) @name)
```

Returns all matches with their captured nodes.

**Signature:**

```python
def run_query(tree: Tree, language: str, query_source: str, source: bytes) -> list[QueryMatch]
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `tree` | `Tree` | Yes | The parsed syntax tree to query. |
| `language` | `str` | Yes | Language name (used to compile the query pattern). |
| `query_source` | `str` | Yes | The tree-sitter query pattern string. |
| `source` | `bytes` | Yes | The original source code bytes (needed for capture resolution). |

**Returns:** `list[QueryMatch]`

**Errors:** Raises `Error`.


---

#### Get_language()

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

#### Get_parser()

Get a tree-sitter `Parser` pre-configured for the given language.

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

#### Available_languages()

List all available language names (sorted, deduplicated, includes aliases).

Returns names of both statically compiled and dynamically loadable languages,
plus any configured aliases.

**Signature:**

```python
def available_languages() -> list[str]
```

**Returns:** `list[str]`


---

#### Has_language()

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

#### Language_count()

Return the number of available languages.

Includes statically compiled languages, dynamically loadable languages,
and aliases.

**Signature:**

```python
def language_count() -> int
```

**Returns:** `int`


---

#### Process()

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

#### Extract_patterns()

Run extraction patterns against source code.

Convenience wrapper around `extract.extract`.

**Errors:**

Returns an error if the language is not found, parsing fails, or a query
pattern is invalid.

**Signature:**

```python
def extract_patterns(source: str, config: ExtractionConfig) -> ExtractionResult
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `source` | `str` | Yes | The source |
| `config` | `ExtractionConfig` | Yes | The configuration options |

**Returns:** `ExtractionResult`

**Errors:** Raises `Error`.


---

#### Validate_extraction()

Validate extraction patterns without running them.

Convenience wrapper around `extract.validate_extraction`.

**Errors:**

Returns an error if the language cannot be loaded.

**Signature:**

```python
def validate_extraction(config: ExtractionConfig) -> ValidationResult
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `config` | `ExtractionConfig` | Yes | The configuration options |

**Returns:** `ValidationResult`

**Errors:** Raises `Error`.


---

#### Init()

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

#### Configure()

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

#### Download()

Download specific languages to the local cache.

Returns the number of newly downloaded languages (languages that were
already cached are not counted).

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

#### Download_all()

Download all available languages from the remote manifest.

Returns the number of newly downloaded languages.

**Errors:**

Returns an error if the manifest cannot be fetched or a download fails.

**Signature:**

```python
def download_all() -> int
```

**Returns:** `int`

**Errors:** Raises `Error`.


---

#### Manifest_languages()

Return all language names available in the remote manifest (305).

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

#### Downloaded_languages()

Return languages that are already downloaded and cached locally.

Does not perform any network requests. Returns an empty list if the
cache directory does not exist or cannot be read.

**Signature:**

```python
def downloaded_languages() -> list[str]
```

**Returns:** `list[str]`


---

#### Clean_cache()

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

#### Cache_dir()

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

#### CaptureResult

A single captured node within a match.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `name` | `str` | — | The capture name from the query (e.g., `"fn_name"`). |
| `node` | `NodeInfo | None` | `None` | The `NodeInfo` snapshot, present when `CaptureOutput` is `Node` or `Full`. |
| `text` | `str | None` | `None` | The matched source text, present when `CaptureOutput` is `Text` or `Full`. |
| `child_fields` | `str` | — | Values of requested child fields, keyed by field name. |
| `start_byte` | `int` | — | Byte offset where this capture starts in the source. |


---

#### ChunkContext

Metadata for a single chunk of source code.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `language` | `str` | — | Language |
| `chunk_index` | `int` | — | Chunk index |
| `total_chunks` | `int` | — | Total chunks |
| `node_types` | `list[str]` | `[]` | Node types |
| `context_path` | `list[str]` | `[]` | Context path |
| `symbols_defined` | `list[str]` | `[]` | Symbols defined |
| `comments` | `list[CommentInfo]` | `[]` | Comments |
| `docstrings` | `list[DocstringInfo]` | `[]` | Docstrings |
| `has_error_nodes` | `bool` | — | Whether error nodes |


---

#### CodeChunk

A chunk of source code with rich metadata.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `content` | `str` | — | The extracted text content |
| `start_byte` | `int` | — | Start byte |
| `end_byte` | `int` | — | End byte |
| `start_line` | `int` | — | Start line |
| `end_line` | `int` | — | End line |
| `metadata` | `ChunkContext` | — | Document metadata |


---

#### CommentInfo

A comment extracted from source code.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `text` | `str` | — | Text |
| `kind` | `CommentKind` | `CommentKind.LINE` | Kind (comment kind) |
| `span` | `Span` | — | Span (span) |
| `associated_node` | `str | None` | `None` | Associated node |


---

#### Diagnostic

A diagnostic (syntax error, missing node, etc.) from parsing.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `message` | `str` | — | Message |
| `severity` | `DiagnosticSeverity` | `DiagnosticSeverity.ERROR` | Severity (diagnostic severity) |
| `span` | `Span` | — | Span (span) |


---

#### DocSection

A section within a docstring (e.g., Args, Returns, Raises).

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `kind` | `str` | — | Kind |
| `name` | `str | None` | `None` | The name |
| `description` | `str` | — | Human-readable description |


---

#### DocstringInfo

A docstring extracted from source code.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `text` | `str` | — | Text |
| `format` | `DocstringFormat` | `DocstringFormat.PYTHON_TRIPLE_QUOTE` | Format (docstring format) |
| `span` | `Span` | — | Span (span) |
| `associated_item` | `str | None` | `None` | Associated item |
| `parsed_sections` | `list[DocSection]` | `[]` | Parsed sections |


---

#### DownloadManager

Manages downloading and caching of pre-built parser shared libraries.

##### Methods

###### New()

Create a new download manager for the given version.

**Signature:**

```python
@staticmethod
def new(version: str) -> DownloadManager
```

###### With_cache_dir()

Create a download manager with a custom cache directory.

**Signature:**

```python
@staticmethod
def with_cache_dir(version: str, cache_dir: str) -> DownloadManager
```

###### Default_cache_dir()

Default cache directory: `~/.cache/tree-sitter-language-pack/v{version}/libs/`

**Signature:**

```python
@staticmethod
def default_cache_dir(version: str) -> str
```

###### Cache_dir()

Return the path to the libs cache directory.

**Signature:**

```python
def cache_dir(self) -> str
```

###### Installed_languages()

List languages that are already downloaded and cached.

**Signature:**

```python
def installed_languages(self) -> list[str]
```

###### Ensure_languages()

Ensure the specified languages are available in the cache.
Downloads the platform bundle if any requested languages are missing.

**Signature:**

```python
def ensure_languages(self, names: list[str]) -> None
```

###### Ensure_group()

Ensure all languages in a named group are available.

**Signature:**

```python
def ensure_group(self, group: str) -> None
```

###### Lib_path()

Get the expected path for a language's shared library in the cache.

**Signature:**

```python
def lib_path(self, name: str) -> str
```

###### Fetch_manifest()

Fetch the parser manifest from GitHub Releases.

**Signature:**

```python
def fetch_manifest(self) -> ParserManifest
```

###### Clean_cache()

Remove all cached parser libraries.

**Signature:**

```python
def clean_cache(self) -> None
```


---

#### ExportInfo

An export statement extracted from source code.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `name` | `str` | — | The name |
| `kind` | `ExportKind` | `ExportKind.NAMED` | Kind (export kind) |
| `span` | `Span` | — | Span (span) |


---

#### ExtractionConfig

Configuration for an extraction run against a single language.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `language` | `str` | — | The language name (e.g., `"python"`). |
| `patterns` | `str` | — | Named patterns to run. Keys become the keys in `ExtractionResult.results`. |


---

#### ExtractionPattern

Defines a single extraction pattern and its configuration.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `query` | `str` | — | The tree-sitter query string (S-expression). |
| `capture_output` | `CaptureOutput` | `CaptureOutput.FULL` | What to include in each capture result. |
| `child_fields` | `list[str]` | `[]` | Field names to extract from child nodes of each capture. Maps a label to a tree-sitter field name used with `child_by_field_name`. |
| `max_results` | `int | None` | `None` | Maximum number of matches to return. `None` means unlimited. |
| `byte_range` | `list[int] | None` | `[]` | Restrict matches to a byte range `(start, end)`. |


---

#### ExtractionResult

Complete extraction results for all patterns.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `language` | `str` | — | The language that was used. |
| `results` | `str` | — | Results keyed by pattern name. |


---

#### FileMetrics

Aggregate metrics for a source file.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `total_lines` | `int` | — | Total lines |
| `code_lines` | `int` | — | Code lines |
| `comment_lines` | `int` | — | Comment lines |
| `blank_lines` | `int` | — | Blank lines |
| `total_bytes` | `int` | — | Total bytes |
| `node_count` | `int` | — | Number of nodes |
| `error_count` | `int` | — | Number of errors |
| `max_depth` | `int` | — | Maximum depth |


---

#### ImportInfo

An import statement extracted from source code.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `source` | `str` | — | Source |
| `items` | `list[str]` | `[]` | Items |
| `alias` | `str | None` | `None` | Alias |
| `is_wildcard` | `bool` | — | Whether wildcard |
| `span` | `Span` | — | Span (span) |


---

#### Language


---

#### LanguageInfo

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `group` | `str` | — | Group |
| `size` | `int` | — | Size in bytes |


---

#### LanguageRegistry

Thread-safe registry of tree-sitter language parsers.

Manages both statically compiled and dynamically loaded language grammars.
Use `LanguageRegistry.new()` for the default registry, or access the
global instance via the module-level convenience functions
(`crate.get_language`, `crate.available_languages`, etc.).

##### Methods

###### With_libs_dir()

Create a registry with a custom directory for dynamic libraries.

Overrides the default build-time library directory. Useful when
dynamic grammar shared libraries are stored in a non-standard location.

**Signature:**

```python
@staticmethod
def with_libs_dir(libs_dir: str) -> LanguageRegistry
```

###### Add_extra_libs_dir()

Add an additional directory to search for dynamic libraries.

When `get_language` cannot find a grammar in the
primary library directory, it searches these extra directories in order.
Typically used by the download system to register its cache directory.

Takes `&self` (not `&mut self`) because `extra_lib_dirs` uses interior
mutability via an `Arc<RwLock<...>>`, so the outer registry can remain
immutable while the directory list is updated.

**Signature:**

```python
def add_extra_libs_dir(self, dir: str) -> None
```

###### Get_language()

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

###### Available_languages()

List all available language names, sorted and deduplicated.

Includes statically compiled languages, dynamically loadable languages
(if the `dynamic-loading` feature is enabled), and all configured aliases.

**Signature:**

```python
def available_languages(self) -> list[str]
```

###### Has_language()

Check whether a language is available by name or alias.

Returns `True` if the language can be loaded, either from the static
table or from a dynamic library on disk.

**Signature:**

```python
def has_language(self, name: str) -> bool
```

###### Language_count()

Return the total number of available languages (including aliases).

**Signature:**

```python
def language_count(self) -> int
```

###### Process()

Parse source code and extract file intelligence based on config in a single pass.

**Signature:**

```python
def process(self, source: str, config: ProcessConfig) -> ProcessResult
```

###### Default()

**Signature:**

```python
@staticmethod
def default() -> LanguageRegistry
```


---

#### MatchResult

A single query match containing one or more captures.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `pattern_index` | `int` | — | The pattern index within the query that produced this match. |
| `captures` | `list[CaptureResult]` | `[]` | The captures for this match. |


---

#### NodeInfo

Lightweight snapshot of a tree-sitter node's properties.

Contains only primitive types for easy cross-language serialization.
This is an owned type that can be passed across FFI boundaries, unlike
`tree_sitter.Node` which borrows from the tree.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `kind` | `str` | — | The grammar type name (e.g., "function_definition", "identifier"). |
| `is_named` | `bool` | — | Whether this is a named node (vs anonymous like punctuation). |
| `start_byte` | `int` | — | Start byte offset in source. |
| `end_byte` | `int` | — | End byte offset in source. |
| `start_row` | `int` | — | Start row (zero-indexed). |
| `start_col` | `int` | — | Start column (zero-indexed). |
| `end_row` | `int` | — | End row (zero-indexed). |
| `end_col` | `int` | — | End column (zero-indexed). |
| `named_child_count` | `int` | — | Number of named children. |
| `is_error` | `bool` | — | Whether this node is an ERROR node. |
| `is_missing` | `bool` | — | Whether this node is a MISSING node. |


---

#### PackConfig

Configuration for the tree-sitter language pack.

Controls cache directory and which languages to pre-download.
Can be loaded from a TOML file, constructed programmatically,
or passed as a dict/object from language bindings.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `cache_dir` | `str | None` | `None` | Override default cache directory. Default: `~/.cache/tree-sitter-language-pack/v{version}/libs/` |
| `languages` | `list[str] | None` | `[]` | Languages to pre-download on init. Each entry is a language name (e.g. `"python"`, `"rust"`). |
| `groups` | `list[str] | None` | `[]` | Language groups to pre-download (e.g. `"web"`, `"systems"`, `"scripting"`). |

##### Methods

###### From_toml_file()

Load configuration from a TOML file.

**Errors:**

Returns an error if the file cannot be read or the TOML is invalid.

**Signature:**

```python
@staticmethod
def from_toml_file(path: str) -> PackConfig
```

###### Discover()

Discover configuration by searching for `language-pack.toml` in:

1. Current directory and up to 10 parent directories
2. `$XDG_CONFIG_HOME/tree-sitter-language-pack/config.toml`
3. `~/.config/tree-sitter-language-pack/config.toml`

Returns `None` if no configuration file is found.

**Signature:**

```python
@staticmethod
def discover() -> PackConfig | None
```


---

#### Parser


---

#### ParserManifest

Manifest describing available parser downloads for a specific version.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `version` | `str` | — | Version string |
| `platforms` | `dict[str, PlatformBundle]` | — | Platforms |
| `languages` | `dict[str, LanguageInfo]` | — | Languages |
| `groups` | `dict[str, list[str]]` | — | Groups |


---

#### PatternResult

Results for a single named pattern.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `matches` | `list[MatchResult]` | `[]` | The individual matches. |
| `total_count` | `int` | — | Total number of matches before `max_results` truncation. |


---

#### PatternValidation

Validation information for a single pattern.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `valid` | `bool` | — | Whether the pattern compiled successfully. |
| `capture_names` | `list[str]` | `[]` | Names of captures defined in the query. |
| `pattern_count` | `int` | — | Number of patterns in the query. |
| `warnings` | `list[str]` | `[]` | Non-fatal warnings (e.g., unused captures). |
| `errors` | `list[str]` | `[]` | Fatal errors (e.g., query syntax errors). |


---

#### PlatformBundle

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `url` | `str` | — | Url |
| `sha256` | `str` | — | Sha256 |
| `size` | `int` | — | Size in bytes |


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
| `chunk_max_size` | `int | None` | `None` | Maximum chunk size in bytes. `None` disables chunking. |
| `extractions` | `str | None` | `None` | Custom extraction patterns to run against the parsed tree. Keys become the keys in `ProcessResult.extractions`. |

##### Methods

###### Default()

**Signature:**

```python
@staticmethod
def default() -> ProcessConfig
```

###### With_chunking()

Enable chunking with the given maximum chunk size in bytes.

**Signature:**

```python
def with_chunking(self, max_size: int) -> ProcessConfig
```

###### All()

Enable all analysis features.

**Signature:**

```python
def all(self) -> ProcessConfig
```

###### Minimal()

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
Fields are populated based on the `crate.ProcessConfig` flags.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `language` | `str` | — | Language |
| `metrics` | `FileMetrics` | — | Metrics (file metrics) |
| `structure` | `list[StructureItem]` | `[]` | Structure |
| `imports` | `list[ImportInfo]` | `[]` | Imports |
| `exports` | `list[ExportInfo]` | `[]` | Exports |
| `comments` | `list[CommentInfo]` | `[]` | Comments |
| `docstrings` | `list[DocstringInfo]` | `[]` | Docstrings |
| `symbols` | `list[SymbolInfo]` | `[]` | Symbols |
| `diagnostics` | `list[Diagnostic]` | `[]` | Diagnostics |
| `chunks` | `list[CodeChunk]` | `[]` | Text chunks for chunking/embedding |
| `extractions` | `str` | — | Results of custom extraction patterns (when `config.extractions` is set). |


---

#### QueryMatch

A single match from a tree-sitter query, with captured nodes.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `pattern_index` | `int` | — | The pattern index that matched (position in the query string). |
| `captures` | `list[str]` | `[]` | Captures: list of (capture_name, node_info) pairs. |


---

#### Span

Byte and line/column range in source code.

Represents both byte offsets (for slicing) and human-readable line/column
positions (for display and diagnostics).

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `start_byte` | `int` | — | Start byte |
| `end_byte` | `int` | — | End byte |
| `start_line` | `int` | — | Start line |
| `start_column` | `int` | — | Start column |
| `end_line` | `int` | — | End line |
| `end_column` | `int` | — | End column |


---

#### StructureItem

A structural item (function, class, struct, etc.) in source code.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `kind` | `StructureKind` | `StructureKind.FUNCTION` | Kind (structure kind) |
| `name` | `str | None` | `None` | The name |
| `visibility` | `str | None` | `None` | Visibility |
| `span` | `Span` | — | Span (span) |
| `children` | `list[StructureItem]` | `[]` | Children |
| `decorators` | `list[str]` | `[]` | Decorators |
| `doc_comment` | `str | None` | `None` | Doc comment |
| `signature` | `str | None` | `None` | Signature |
| `body_span` | `Span | None` | `None` | Body span (span) |


---

#### SymbolInfo

A symbol (variable, function, type, etc.) extracted from source code.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `name` | `str` | — | The name |
| `kind` | `SymbolKind` | `SymbolKind.VARIABLE` | Kind (symbol kind) |
| `span` | `Span` | — | Span (span) |
| `type_annotation` | `str | None` | `None` | Type annotation |
| `doc` | `str | None` | `None` | Doc |


---

#### Tree


---

#### ValidationResult

Validation results for an entire extraction config.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `valid` | `bool` | — | Whether all patterns are valid. |
| `patterns` | `str` | — | Per-pattern validation details. |


---

### Enums

#### CaptureOutput

Controls what data is captured for each query match.

| Value | Description |
|-------|-------------|
| `TEXT` | Capture only the matched text. |
| `NODE` | Capture only the `NodeInfo`. |
| `FULL` | Capture both text and `NodeInfo` (default). |


---

#### StructureKind

The kind of structural item found in source code.

Categorizes top-level and nested declarations such as functions, classes,
structs, enums, traits, and more. Use `Other` for
language-specific constructs that do not fit a standard category.

| Value | Description |
|-------|-------------|
| `FUNCTION` | Function |
| `METHOD` | Method |
| `CLASS` | Class |
| `STRUCT` | Struct |
| `INTERFACE` | Interface |
| `ENUM` | Enum |
| `MODULE` | Module |
| `TRAIT` | Trait |
| `IMPL` | Impl |
| `NAMESPACE` | Namespace |
| `OTHER` | Other — Fields: `0`: `str` |


---

#### CommentKind

The kind of a comment found in source code.

Distinguishes between single-line comments, block (multi-line) comments,
and documentation comments.

| Value | Description |
|-------|-------------|
| `LINE` | Line |
| `BLOCK` | Block |
| `DOC` | Doc |


---

#### DocstringFormat

The format of a docstring extracted from source code.

Identifies the docstring convention used, which varies by language
(e.g., Python triple-quoted strings, JSDoc, Rustdoc `///` comments).

| Value | Description |
|-------|-------------|
| `PYTHON_TRIPLE_QUOTE` | Python triple quote |
| `JS_DOC` | J s doc |
| `RUSTDOC` | Rustdoc |
| `GO_DOC` | Go doc |
| `JAVA_DOC` | Java doc |
| `OTHER` | Other — Fields: `0`: `str` |


---

#### ExportKind

The kind of an export statement found in source code.

Covers named exports, default exports, and re-exports from other modules.

| Value | Description |
|-------|-------------|
| `NAMED` | Named |
| `DEFAULT` | Default |
| `RE_EXPORT` | Re export |


---

#### SymbolKind

The kind of a symbol definition found in source code.

Categorizes symbol definitions such as variables, constants, functions,
classes, types, interfaces, enums, and modules.

| Value | Description |
|-------|-------------|
| `VARIABLE` | Variable |
| `CONSTANT` | Constant |
| `FUNCTION` | Function |
| `CLASS` | Class |
| `TYPE` | Type |
| `INTERFACE` | Interface |
| `ENUM` | Enum |
| `MODULE` | Module |
| `OTHER` | Other — Fields: `0`: `str` |


---

#### DiagnosticSeverity

Severity level of a diagnostic produced during parsing.

Used to classify parse errors, warnings, and informational messages
found in the syntax tree.

| Value | Description |
|-------|-------------|
| `ERROR` | Error |
| `WARNING` | Warning |
| `INFO` | Info |


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
| `LanguageNotFound(Error)` | Language '{0}' not found |
| `DynamicLoad(Error)` | Dynamic library load error: {0} |
| `NullLanguagePointer(Error)` | Language function returned null pointer for '{0}' |
| `ParserSetup(Error)` | Failed to set parser language: {0} |
| `LockPoisoned(Error)` | Registry lock poisoned: {0} |
| `Config(Error)` | Configuration error: {0} |
| `ParseFailed(Error)` | Parse failed: parsing returned no tree |
| `QueryError(Error)` | Query error: {0} |
| `InvalidRange(Error)` | Invalid byte range: {0} |
| `Io(Error)` | IO error: {0} |


---
