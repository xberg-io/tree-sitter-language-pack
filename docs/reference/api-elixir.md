---
title: "Elixir API Reference"
---

## Elixir API Reference <span class="version-badge">v1.8.0-rc.24</span>

### Functions

#### Detect_language_from_extension()

Detect language name from a file extension (without leading dot).

Returns `nil` for unrecognized extensions. The match is case-insensitive.

**Signature:**

```elixir
@spec detect_language_from_extension(ext) :: {:ok, term()} | {:error, term()}
def detect_language_from_extension(ext)
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `ext` | `String.t()` | Yes | The ext |

**Returns:** `String.t() | nil`


---

#### Detect_language_from_path()

Detect language name from a file path.

Extracts the file extension and looks it up. Returns `nil` if the
path has no extension or the extension is not recognized.

**Signature:**

```elixir
@spec detect_language_from_path(path) :: {:ok, term()} | {:error, term()}
def detect_language_from_path(path)
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `path` | `String.t()` | Yes | Path to the file |

**Returns:** `String.t() | nil`


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

Returns `nil` when content does not start with `#!`, the shebang is
malformed, or the interpreter is not recognised.

**Signature:**

```elixir
@spec detect_language_from_content(content) :: {:ok, term()} | {:error, term()}
def detect_language_from_content(content)
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `content` | `String.t()` | Yes | The content to process |

**Returns:** `String.t() | nil`


---

#### Root_node_info()

Get a `NodeInfo` snapshot of the root node.

**Signature:**

```elixir
@spec root_node_info(tree) :: {:ok, term()} | {:error, term()}
def root_node_info(tree)
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

```elixir
@spec find_nodes_by_type(tree, node_type) :: {:ok, term()} | {:error, term()}
def find_nodes_by_type(tree, node_type)
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `tree` | `Tree` | Yes | The tree |
| `node_type` | `String.t()` | Yes | The node type |

**Returns:** `list(NodeInfo)`


---

#### Named_children_info()

Get `NodeInfo` for all named children of the root node.

Useful for understanding the top-level structure of a file
(e.g., list of function definitions, class declarations, imports).

**Signature:**

```elixir
@spec named_children_info(tree) :: {:ok, term()} | {:error, term()}
def named_children_info(tree)
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `tree` | `Tree` | Yes | The tree |

**Returns:** `list(NodeInfo)`


---

#### Parse_string()

Parse source code with the named language, returning the syntax tree.

Uses the global registry to look up the language by name.
Caches parsers per-thread so repeated calls for the same language avoid
re-creating the parser.

**Signature:**

```elixir
@spec parse_string(language, source) :: {:ok, term()} | {:error, term()}
def parse_string(language, source)
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `language` | `String.t()` | Yes | The language |
| `source` | `binary()` | Yes | The source |

**Returns:** `Tree`

**Errors:** Returns `{:error, reason}`


---

#### Tree_contains_node_type()

Check whether any node in the tree matches the given type name.

Performs a depth-first traversal using `TreeCursor`.

**Signature:**

```elixir
@spec tree_contains_node_type(tree, node_type) :: {:ok, term()} | {:error, term()}
def tree_contains_node_type(tree, node_type)
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `tree` | `Tree` | Yes | The tree |
| `node_type` | `String.t()` | Yes | The node type |

**Returns:** `boolean()`


---

#### Tree_has_error_nodes()

Check whether the tree contains any ERROR or MISSING nodes.

Useful for determining if the parse was clean or had syntax errors.

**Signature:**

```elixir
@spec tree_has_error_nodes(tree) :: {:ok, term()} | {:error, term()}
def tree_has_error_nodes(tree)
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `tree` | `Tree` | Yes | The tree |

**Returns:** `boolean()`


---

#### Tree_to_sexp()

Return the S-expression representation of the entire tree.

This is the standard tree-sitter debug format, useful for logging,
snapshot testing, and debugging grammars.

**Signature:**

```elixir
@spec tree_to_sexp(tree) :: {:ok, term()} | {:error, term()}
def tree_to_sexp(tree)
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `tree` | `Tree` | Yes | The tree |

**Returns:** `String.t()`


---

#### Tree_error_count()

Count the number of ERROR and MISSING nodes in the tree.

Returns 0 for a clean parse.

**Signature:**

```elixir
@spec tree_error_count(tree) :: {:ok, term()} | {:error, term()}
def tree_error_count(tree)
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `tree` | `Tree` | Yes | The tree |

**Returns:** `integer()`


---

#### Get_highlights_query()

Get the highlights query for a language, if bundled.

Returns the contents of `highlights.scm` as a static string, or `nil`
if no highlights query is bundled for this language.

**Signature:**

```elixir
@spec get_highlights_query(language) :: {:ok, term()} | {:error, term()}
def get_highlights_query(language)
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `language` | `String.t()` | Yes | The language |

**Returns:** `String.t() | nil`


---

#### Get_injections_query()

Get the injections query for a language, if bundled.

Returns the contents of `injections.scm` as a static string, or `nil`
if no injections query is bundled for this language.

**Signature:**

```elixir
@spec get_injections_query(language) :: {:ok, term()} | {:error, term()}
def get_injections_query(language)
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `language` | `String.t()` | Yes | The language |

**Returns:** `String.t() | nil`


---

#### Get_locals_query()

Get the locals query for a language, if bundled.

Returns the contents of `locals.scm` as a static string, or `nil`
if no locals query is bundled for this language.

**Signature:**

```elixir
@spec get_locals_query(language) :: {:ok, term()} | {:error, term()}
def get_locals_query(language)
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `language` | `String.t()` | Yes | The language |

**Returns:** `String.t() | nil`


---

#### Run_query()

Execute a tree-sitter query pattern against a parsed tree.

The `query_source` is an S-expression pattern like:

```text
(function_definition name: (identifier) @name)
```

Returns all matches with their captured nodes.

**Signature:**

```elixir
@spec run_query(tree, language, query_source, source) :: {:ok, term()} | {:error, term()}
def run_query(tree, language, query_source, source)
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `tree` | `Tree` | Yes | The parsed syntax tree to query. |
| `language` | `String.t()` | Yes | Language name (used to compile the query pattern). |
| `query_source` | `String.t()` | Yes | The tree-sitter query pattern string. |
| `source` | `binary()` | Yes | The original source code bytes (needed for capture resolution). |

**Returns:** `list(QueryMatch)`

**Errors:** Returns `{:error, reason}`


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

```elixir
@spec get_language(name) :: {:ok, term()} | {:error, term()}
def get_language(name)
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `name` | `String.t()` | Yes | The name |

**Returns:** `Language`

**Errors:** Returns `{:error, reason}`


---

#### Get_parser()

Get a tree-sitter `Parser` pre-configured for the given language.

This is a convenience function that calls `get_language` and configures
a new parser in one step.

**Errors:**

Returns `Error.LanguageNotFound` if the language is not recognized, or
`Error.ParserSetup` if the language cannot be applied to the parser.

**Signature:**

```elixir
@spec get_parser(name) :: {:ok, term()} | {:error, term()}
def get_parser(name)
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `name` | `String.t()` | Yes | The name |

**Returns:** `Parser`

**Errors:** Returns `{:error, reason}`


---

#### Available_languages()

List all available language names (sorted, deduplicated, includes aliases).

Returns names of both statically compiled and dynamically loadable languages,
plus any configured aliases.

**Signature:**

```elixir
@spec available_languages() :: {:ok, term()} | {:error, term()}
def available_languages()
```

**Returns:** `list(String.t())`


---

#### Has_language()

Check if a language is available by name or alias.

Returns `true` if the language can be loaded (statically compiled,
dynamically available, or a known alias for one of these).

**Signature:**

```elixir
@spec has_language(name) :: {:ok, term()} | {:error, term()}
def has_language(name)
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `name` | `String.t()` | Yes | The name |

**Returns:** `boolean()`


---

#### Language_count()

Return the number of available languages.

Includes statically compiled languages, dynamically loadable languages,
and aliases.

**Signature:**

```elixir
@spec language_count() :: {:ok, term()} | {:error, term()}
def language_count()
```

**Returns:** `integer()`


---

#### Process()

Process source code and extract file intelligence using the global registry.

Parses the source with tree-sitter and extracts metrics, structure, imports,
exports, comments, docstrings, symbols, diagnostics, and/or chunks based on
the flags set in `ProcessConfig`.

**Errors:**

Returns an error if the language is not found or parsing fails.

**Signature:**

```elixir
@spec process(source, config) :: {:ok, term()} | {:error, term()}
def process(source, config)
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `source` | `String.t()` | Yes | The source |
| `config` | `ProcessConfig` | Yes | The configuration options |

**Returns:** `ProcessResult`

**Errors:** Returns `{:error, reason}`


---

#### Extract_patterns()

Run extraction patterns against source code.

Convenience wrapper around `extract.extract`.

**Errors:**

Returns an error if the language is not found, parsing fails, or a query
pattern is invalid.

**Signature:**

```elixir
@spec extract_patterns(source, config) :: {:ok, term()} | {:error, term()}
def extract_patterns(source, config)
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `source` | `String.t()` | Yes | The source |
| `config` | `ExtractionConfig` | Yes | The configuration options |

**Returns:** `ExtractionResult`

**Errors:** Returns `{:error, reason}`


---

#### Validate_extraction()

Validate extraction patterns without running them.

Convenience wrapper around `extract.validate_extraction`.

**Errors:**

Returns an error if the language cannot be loaded.

**Signature:**

```elixir
@spec validate_extraction(config) :: {:ok, term()} | {:error, term()}
def validate_extraction(config)
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `config` | `ExtractionConfig` | Yes | The configuration options |

**Returns:** `ValidationResult`

**Errors:** Returns `{:error, reason}`


---

#### Init()

Initialize the language pack with the given configuration.

Applies any custom cache directory, then downloads all languages and groups
specified in the config. This is the recommended entry point when you want
to pre-warm the cache before use.

**Errors:**

Returns an error if configuration cannot be applied or if downloads fail.

**Signature:**

```elixir
@spec init(config) :: {:ok, term()} | {:error, term()}
def init(config)
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `config` | `PackConfig` | Yes | The configuration options |

**Returns:** `:ok`

**Errors:** Returns `{:error, reason}`


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

```elixir
@spec configure(config) :: {:ok, term()} | {:error, term()}
def configure(config)
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `config` | `PackConfig` | Yes | The configuration options |

**Returns:** `:ok`

**Errors:** Returns `{:error, reason}`


---

#### Download()

Download specific languages to the local cache.

Returns the number of newly downloaded languages (languages that were
already cached are not counted).

**Errors:**

Returns an error if any language is not available in the manifest or if
the download fails.

**Signature:**

```elixir
@spec download(names) :: {:ok, term()} | {:error, term()}
def download(names)
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `names` | `list(String.t())` | Yes | The names |

**Returns:** `integer()`

**Errors:** Returns `{:error, reason}`


---

#### Download_all()

Download all available languages from the remote manifest.

Returns the number of newly downloaded languages.

**Errors:**

Returns an error if the manifest cannot be fetched or a download fails.

**Signature:**

```elixir
@spec download_all() :: {:ok, term()} | {:error, term()}
def download_all()
```

**Returns:** `integer()`

**Errors:** Returns `{:error, reason}`


---

#### Manifest_languages()

Return all language names available in the remote manifest (305).

Fetches (and caches) the remote manifest to discover the full list of
downloadable languages. Use `downloaded_languages` to list what is
already cached locally.

**Errors:**

Returns an error if the manifest cannot be fetched.

**Signature:**

```elixir
@spec manifest_languages() :: {:ok, term()} | {:error, term()}
def manifest_languages()
```

**Returns:** `list(String.t())`

**Errors:** Returns `{:error, reason}`


---

#### Downloaded_languages()

Return languages that are already downloaded and cached locally.

Does not perform any network requests. Returns an empty list if the
cache directory does not exist or cannot be read.

**Signature:**

```elixir
@spec downloaded_languages() :: {:ok, term()} | {:error, term()}
def downloaded_languages()
```

**Returns:** `list(String.t())`


---

#### Clean_cache()

Delete all cached parser shared libraries.

Resets the cache registration so the next call to `get_language` or
a download function will re-register the (now empty) cache directory.

**Errors:**

Returns an error if the cache directory cannot be removed.

**Signature:**

```elixir
@spec clean_cache() :: {:ok, term()} | {:error, term()}
def clean_cache()
```

**Returns:** `:ok`

**Errors:** Returns `{:error, reason}`


---

#### Cache_dir()

Return the effective cache directory path.

This is either the custom path set via `configure` / `init` or the
default: `~/.cache/tree-sitter-language-pack/v{version}/libs/`.

**Errors:**

Returns an error if the system cache directory cannot be determined.

**Signature:**

```elixir
@spec cache_dir() :: {:ok, term()} | {:error, term()}
def cache_dir()
```

**Returns:** `String.t()`

**Errors:** Returns `{:error, reason}`


---

### Types

#### CaptureResult

A single captured node within a match.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `name` | `String.t()` | — | The capture name from the query (e.g., `"fn_name"`). |
| `node` | `NodeInfo | nil` | `nil` | The `NodeInfo` snapshot, present when `CaptureOutput` is `Node` or `Full`. |
| `text` | `String.t() | nil` | `nil` | The matched source text, present when `CaptureOutput` is `Text` or `Full`. |
| `child_fields` | `String.t()` | — | Values of requested child fields, keyed by field name. |
| `start_byte` | `integer()` | — | Byte offset where this capture starts in the source. |


---

#### ChunkContext

Metadata for a single chunk of source code.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `language` | `String.t()` | — | Language |
| `chunk_index` | `integer()` | — | Chunk index |
| `total_chunks` | `integer()` | — | Total chunks |
| `node_types` | `list(String.t())` | `[]` | Node types |
| `context_path` | `list(String.t())` | `[]` | Context path |
| `symbols_defined` | `list(String.t())` | `[]` | Symbols defined |
| `comments` | `list(CommentInfo)` | `[]` | Comments |
| `docstrings` | `list(DocstringInfo)` | `[]` | Docstrings |
| `has_error_nodes` | `boolean()` | — | Whether error nodes |


---

#### CodeChunk

A chunk of source code with rich metadata.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `content` | `String.t()` | — | The extracted text content |
| `start_byte` | `integer()` | — | Start byte |
| `end_byte` | `integer()` | — | End byte |
| `start_line` | `integer()` | — | Start line |
| `end_line` | `integer()` | — | End line |
| `metadata` | `ChunkContext` | — | Document metadata |


---

#### CommentInfo

A comment extracted from source code.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `text` | `String.t()` | — | Text |
| `kind` | `CommentKind` | `:line` | Kind (comment kind) |
| `span` | `Span` | — | Span (span) |
| `associated_node` | `String.t() | nil` | `nil` | Associated node |


---

#### Diagnostic

A diagnostic (syntax error, missing node, etc.) from parsing.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `message` | `String.t()` | — | Message |
| `severity` | `DiagnosticSeverity` | `:error` | Severity (diagnostic severity) |
| `span` | `Span` | — | Span (span) |


---

#### DocSection

A section within a docstring (e.g., Args, Returns, Raises).

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `kind` | `String.t()` | — | Kind |
| `name` | `String.t() | nil` | `nil` | The name |
| `description` | `String.t()` | — | Human-readable description |


---

#### DocstringInfo

A docstring extracted from source code.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `text` | `String.t()` | — | Text |
| `format` | `DocstringFormat` | `:python_triple_quote` | Format (docstring format) |
| `span` | `Span` | — | Span (span) |
| `associated_item` | `String.t() | nil` | `nil` | Associated item |
| `parsed_sections` | `list(DocSection)` | `[]` | Parsed sections |


---

#### DownloadManager

Manages downloading and caching of pre-built parser shared libraries.

##### Functions

###### New()

Create a new download manager for the given version.

**Signature:**

```elixir
def new(version)
```

###### With_cache_dir()

Create a download manager with a custom cache directory.

**Signature:**

```elixir
def with_cache_dir(version, cache_dir)
```

###### Default_cache_dir()

Default cache directory: `~/.cache/tree-sitter-language-pack/v{version}/libs/`

**Signature:**

```elixir
def default_cache_dir(version)
```

###### Cache_dir()

Return the path to the libs cache directory.

**Signature:**

```elixir
def cache_dir()
```

###### Installed_languages()

List languages that are already downloaded and cached.

**Signature:**

```elixir
def installed_languages()
```

###### Ensure_languages()

Ensure the specified languages are available in the cache.
Downloads the platform bundle if any requested languages are missing.

**Signature:**

```elixir
def ensure_languages(names)
```

###### Ensure_group()

Ensure all languages in a named group are available.

**Signature:**

```elixir
def ensure_group(group)
```

###### Lib_path()

Get the expected path for a language's shared library in the cache.

**Signature:**

```elixir
def lib_path(name)
```

###### Fetch_manifest()

Fetch the parser manifest from GitHub Releases.

**Signature:**

```elixir
def fetch_manifest()
```

###### Clean_cache()

Remove all cached parser libraries.

**Signature:**

```elixir
def clean_cache()
```


---

#### ExportInfo

An export statement extracted from source code.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `name` | `String.t()` | — | The name |
| `kind` | `ExportKind` | `:named` | Kind (export kind) |
| `span` | `Span` | — | Span (span) |


---

#### ExtractionConfig

Configuration for an extraction run against a single language.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `language` | `String.t()` | — | The language name (e.g., `"python"`). |
| `patterns` | `String.t()` | — | Named patterns to run. Keys become the keys in `ExtractionResult.results`. |


---

#### ExtractionPattern

Defines a single extraction pattern and its configuration.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `query` | `String.t()` | — | The tree-sitter query string (S-expression). |
| `capture_output` | `CaptureOutput` | `:full` | What to include in each capture result. |
| `child_fields` | `list(String.t())` | `[]` | Field names to extract from child nodes of each capture. Maps a label to a tree-sitter field name used with `child_by_field_name`. |
| `max_results` | `integer() | nil` | `nil` | Maximum number of matches to return. `nil` means unlimited. |
| `byte_range` | `list(integer()) | nil` | `[]` | Restrict matches to a byte range `(start, end)`. |


---

#### ExtractionResult

Complete extraction results for all patterns.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `language` | `String.t()` | — | The language that was used. |
| `results` | `String.t()` | — | Results keyed by pattern name. |


---

#### FileMetrics

Aggregate metrics for a source file.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `total_lines` | `integer()` | — | Total lines |
| `code_lines` | `integer()` | — | Code lines |
| `comment_lines` | `integer()` | — | Comment lines |
| `blank_lines` | `integer()` | — | Blank lines |
| `total_bytes` | `integer()` | — | Total bytes |
| `node_count` | `integer()` | — | Number of nodes |
| `error_count` | `integer()` | — | Number of errors |
| `max_depth` | `integer()` | — | Maximum depth |


---

#### ImportInfo

An import statement extracted from source code.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `source` | `String.t()` | — | Source |
| `items` | `list(String.t())` | `[]` | Items |
| `alias` | `String.t() | nil` | `nil` | Alias |
| `is_wildcard` | `boolean()` | — | Whether wildcard |
| `span` | `Span` | — | Span (span) |


---

#### Language


---

#### LanguageInfo

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `group` | `String.t()` | — | Group |
| `size` | `integer()` | — | Size in bytes |


---

#### LanguageRegistry

Thread-safe registry of tree-sitter language parsers.

Manages both statically compiled and dynamically loaded language grammars.
Use `LanguageRegistry.new()` for the default registry, or access the
global instance via the module-level convenience functions
(`crate.get_language`, `crate.available_languages`, etc.).

##### Functions

###### With_libs_dir()

Create a registry with a custom directory for dynamic libraries.

Overrides the default build-time library directory. Useful when
dynamic grammar shared libraries are stored in a non-standard location.

**Signature:**

```elixir
def with_libs_dir(libs_dir)
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

```elixir
def add_extra_libs_dir(dir)
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

```elixir
def get_language(name)
```

###### Available_languages()

List all available language names, sorted and deduplicated.

Includes statically compiled languages, dynamically loadable languages
(if the `dynamic-loading` feature is enabled), and all configured aliases.

**Signature:**

```elixir
def available_languages()
```

###### Has_language()

Check whether a language is available by name or alias.

Returns `true` if the language can be loaded, either from the static
table or from a dynamic library on disk.

**Signature:**

```elixir
def has_language(name)
```

###### Language_count()

Return the total number of available languages (including aliases).

**Signature:**

```elixir
def language_count()
```

###### Process()

Parse source code and extract file intelligence based on config in a single pass.

**Signature:**

```elixir
def process(source, config)
```

###### Default()

**Signature:**

```elixir
def default()
```


---

#### MatchResult

A single query match containing one or more captures.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `pattern_index` | `integer()` | — | The pattern index within the query that produced this match. |
| `captures` | `list(CaptureResult)` | `[]` | The captures for this match. |


---

#### NodeInfo

Lightweight snapshot of a tree-sitter node's properties.

Contains only primitive types for easy cross-language serialization.
This is an owned type that can be passed across FFI boundaries, unlike
`tree_sitter.Node` which borrows from the tree.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `kind` | `String.t()` | — | The grammar type name (e.g., "function_definition", "identifier"). |
| `is_named` | `boolean()` | — | Whether this is a named node (vs anonymous like punctuation). |
| `start_byte` | `integer()` | — | Start byte offset in source. |
| `end_byte` | `integer()` | — | End byte offset in source. |
| `start_row` | `integer()` | — | Start row (zero-indexed). |
| `start_col` | `integer()` | — | Start column (zero-indexed). |
| `end_row` | `integer()` | — | End row (zero-indexed). |
| `end_col` | `integer()` | — | End column (zero-indexed). |
| `named_child_count` | `integer()` | — | Number of named children. |
| `is_error` | `boolean()` | — | Whether this node is an ERROR node. |
| `is_missing` | `boolean()` | — | Whether this node is a MISSING node. |


---

#### PackConfig

Configuration for the tree-sitter language pack.

Controls cache directory and which languages to pre-download.
Can be loaded from a TOML file, constructed programmatically,
or passed as a dict/object from language bindings.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `cache_dir` | `String.t() | nil` | `nil` | Override default cache directory. Default: `~/.cache/tree-sitter-language-pack/v{version}/libs/` |
| `languages` | `list(String.t()) | nil` | `[]` | Languages to pre-download on init. Each entry is a language name (e.g. `"python"`, `"rust"`). |
| `groups` | `list(String.t()) | nil` | `[]` | Language groups to pre-download (e.g. `"web"`, `"systems"`, `"scripting"`). |

##### Functions

###### From_toml_file()

Load configuration from a TOML file.

**Errors:**

Returns an error if the file cannot be read or the TOML is invalid.

**Signature:**

```elixir
def from_toml_file(path)
```

###### Discover()

Discover configuration by searching for `language-pack.toml` in:

1. Current directory and up to 10 parent directories
2. `$XDG_CONFIG_HOME/tree-sitter-language-pack/config.toml`
3. `~/.config/tree-sitter-language-pack/config.toml`

Returns `nil` if no configuration file is found.

**Signature:**

```elixir
def discover()
```


---

#### Parser


---

#### ParserManifest

Manifest describing available parser downloads for a specific version.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `version` | `String.t()` | — | Version string |
| `platforms` | `map()` | — | Platforms |
| `languages` | `map()` | — | Languages |
| `groups` | `map()` | — | Groups |


---

#### PatternResult

Results for a single named pattern.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `matches` | `list(MatchResult)` | `[]` | The individual matches. |
| `total_count` | `integer()` | — | Total number of matches before `max_results` truncation. |


---

#### PatternValidation

Validation information for a single pattern.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `valid` | `boolean()` | — | Whether the pattern compiled successfully. |
| `capture_names` | `list(String.t())` | `[]` | Names of captures defined in the query. |
| `pattern_count` | `integer()` | — | Number of patterns in the query. |
| `warnings` | `list(String.t())` | `[]` | Non-fatal warnings (e.g., unused captures). |
| `errors` | `list(String.t())` | `[]` | Fatal errors (e.g., query syntax errors). |


---

#### PlatformBundle

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `url` | `String.t()` | — | Url |
| `sha256` | `String.t()` | — | Sha256 |
| `size` | `integer()` | — | Size in bytes |


---

#### ProcessConfig

Configuration for the `process()` function.

Controls which analysis features are enabled and whether chunking is performed.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `language` | `String.t()` | — | Language name (required). |
| `structure` | `boolean()` | `true` | Extract structural items (functions, classes, etc.). Default: true. |
| `imports` | `boolean()` | `true` | Extract import statements. Default: true. |
| `exports` | `boolean()` | `true` | Extract export statements. Default: true. |
| `comments` | `boolean()` | `false` | Extract comments. Default: false. |
| `docstrings` | `boolean()` | `false` | Extract docstrings. Default: false. |
| `symbols` | `boolean()` | `false` | Extract symbol definitions. Default: false. |
| `diagnostics` | `boolean()` | `false` | Include parse diagnostics. Default: false. |
| `chunk_max_size` | `integer() | nil` | `nil` | Maximum chunk size in bytes. `nil` disables chunking. |
| `extractions` | `String.t() | nil` | `nil` | Custom extraction patterns to run against the parsed tree. Keys become the keys in `ProcessResult.extractions`. |

##### Functions

###### Default()

**Signature:**

```elixir
def default()
```

###### With_chunking()

Enable chunking with the given maximum chunk size in bytes.

**Signature:**

```elixir
def with_chunking(max_size)
```

###### All()

Enable all analysis features.

**Signature:**

```elixir
def all()
```

###### Minimal()

Disable all analysis features (only metrics computed).

**Signature:**

```elixir
def minimal()
```


---

#### ProcessResult

Complete analysis result from processing a source file.

Contains metrics, structural analysis, imports/exports, comments,
docstrings, symbols, diagnostics, and optionally chunked code segments.
Fields are populated based on the `crate.ProcessConfig` flags.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `language` | `String.t()` | — | Language |
| `metrics` | `FileMetrics` | — | Metrics (file metrics) |
| `structure` | `list(StructureItem)` | `[]` | Structure |
| `imports` | `list(ImportInfo)` | `[]` | Imports |
| `exports` | `list(ExportInfo)` | `[]` | Exports |
| `comments` | `list(CommentInfo)` | `[]` | Comments |
| `docstrings` | `list(DocstringInfo)` | `[]` | Docstrings |
| `symbols` | `list(SymbolInfo)` | `[]` | Symbols |
| `diagnostics` | `list(Diagnostic)` | `[]` | Diagnostics |
| `chunks` | `list(CodeChunk)` | `[]` | Text chunks for chunking/embedding |
| `extractions` | `String.t()` | — | Results of custom extraction patterns (when `config.extractions` is set). |


---

#### QueryMatch

A single match from a tree-sitter query, with captured nodes.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `pattern_index` | `integer()` | — | The pattern index that matched (position in the query string). |
| `captures` | `list(String.t())` | `[]` | Captures: list of (capture_name, node_info) pairs. |


---

#### Span

Byte and line/column range in source code.

Represents both byte offsets (for slicing) and human-readable line/column
positions (for display and diagnostics).

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `start_byte` | `integer()` | — | Start byte |
| `end_byte` | `integer()` | — | End byte |
| `start_line` | `integer()` | — | Start line |
| `start_column` | `integer()` | — | Start column |
| `end_line` | `integer()` | — | End line |
| `end_column` | `integer()` | — | End column |


---

#### StructureItem

A structural item (function, class, struct, etc.) in source code.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `kind` | `StructureKind` | `:function` | Kind (structure kind) |
| `name` | `String.t() | nil` | `nil` | The name |
| `visibility` | `String.t() | nil` | `nil` | Visibility |
| `span` | `Span` | — | Span (span) |
| `children` | `list(StructureItem)` | `[]` | Children |
| `decorators` | `list(String.t())` | `[]` | Decorators |
| `doc_comment` | `String.t() | nil` | `nil` | Doc comment |
| `signature` | `String.t() | nil` | `nil` | Signature |
| `body_span` | `Span | nil` | `nil` | Body span (span) |


---

#### SymbolInfo

A symbol (variable, function, type, etc.) extracted from source code.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `name` | `String.t()` | — | The name |
| `kind` | `SymbolKind` | `:variable` | Kind (symbol kind) |
| `span` | `Span` | — | Span (span) |
| `type_annotation` | `String.t() | nil` | `nil` | Type annotation |
| `doc` | `String.t() | nil` | `nil` | Doc |


---

#### Tree


---

#### ValidationResult

Validation results for an entire extraction config.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `valid` | `boolean()` | — | Whether all patterns are valid. |
| `patterns` | `String.t()` | — | Per-pattern validation details. |


---

### Enums

#### CaptureOutput

Controls what data is captured for each query match.

| Value | Description |
|-------|-------------|
| `text` | Capture only the matched text. |
| `node` | Capture only the `NodeInfo`. |
| `full` | Capture both text and `NodeInfo` (default). |


---

#### StructureKind

The kind of structural item found in source code.

Categorizes top-level and nested declarations such as functions, classes,
structs, enums, traits, and more. Use `Other` for
language-specific constructs that do not fit a standard category.

| Value | Description |
|-------|-------------|
| `function` | Function |
| `method` | Method |
| `class` | Class |
| `struct` | Struct |
| `interface` | Interface |
| `enum` | Enum |
| `module` | Module |
| `trait` | Trait |
| `impl` | Impl |
| `namespace` | Namespace |
| `other` | Other — Fields: `0`: `String.t()` |


---

#### CommentKind

The kind of a comment found in source code.

Distinguishes between single-line comments, block (multi-line) comments,
and documentation comments.

| Value | Description |
|-------|-------------|
| `line` | Line |
| `block` | Block |
| `doc` | Doc |


---

#### DocstringFormat

The format of a docstring extracted from source code.

Identifies the docstring convention used, which varies by language
(e.g., Python triple-quoted strings, JSDoc, Rustdoc `///` comments).

| Value | Description |
|-------|-------------|
| `python_triple_quote` | Python triple quote |
| `js_doc` | J s doc |
| `rustdoc` | Rustdoc |
| `go_doc` | Go doc |
| `java_doc` | Java doc |
| `other` | Other — Fields: `0`: `String.t()` |


---

#### ExportKind

The kind of an export statement found in source code.

Covers named exports, default exports, and re-exports from other modules.

| Value | Description |
|-------|-------------|
| `named` | Named |
| `default` | Default |
| `re_export` | Re export |


---

#### SymbolKind

The kind of a symbol definition found in source code.

Categorizes symbol definitions such as variables, constants, functions,
classes, types, interfaces, enums, and modules.

| Value | Description |
|-------|-------------|
| `variable` | Variable |
| `constant` | Constant |
| `function` | Function |
| `class` | Class |
| `type` | Type |
| `interface` | Interface |
| `enum` | Enum |
| `module` | Module |
| `other` | Other — Fields: `0`: `String.t()` |


---

#### DiagnosticSeverity

Severity level of a diagnostic produced during parsing.

Used to classify parse errors, warnings, and informational messages
found in the syntax tree.

| Value | Description |
|-------|-------------|
| `error` | Error |
| `warning` | Warning |
| `info` | Info |


---

### Errors

#### Error

Errors that can occur when using the tree-sitter language pack.

Covers language lookup failures, parse errors, query errors, and I/O issues.
Feature-gated variants are included when `config`, `download`, or related
features are enabled.

| Variant | Description |
|---------|-------------|
| `language_not_found` | Language '{0}' not found |
| `dynamic_load` | Dynamic library load error: {0} |
| `null_language_pointer` | Language function returned null pointer for '{0}' |
| `parser_setup` | Failed to set parser language: {0} |
| `lock_poisoned` | Registry lock poisoned: {0} |
| `config` | Configuration error: {0} |
| `parse_failed` | Parse failed: parsing returned no tree |
| `query_error` | Query error: {0} |
| `invalid_range` | Invalid byte range: {0} |
| `io` | IO error: {0} |


---
