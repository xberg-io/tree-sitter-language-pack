---
title: "C API Reference"
---

## C API Reference <span class="version-badge">v1.8.0-rc.24</span>

### Functions

#### Ts_pack_detect_language_from_extension()

Detect language name from a file extension (without leading dot).

Returns `NULL` for unrecognized extensions. The match is case-insensitive.

**Signature:**

```c
const char** ts_pack_detect_language_from_extension(const char* ext);
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `ext` | `const char*` | Yes | The ext |

**Returns:** `const char**`


---

#### Ts_pack_detect_language_from_path()

Detect language name from a file path.

Extracts the file extension and looks it up. Returns `NULL` if the
path has no extension or the extension is not recognized.

**Signature:**

```c
const char** ts_pack_detect_language_from_path(const char* path);
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `path` | `const char*` | Yes | Path to the file |

**Returns:** `const char**`


---

#### Ts_pack_detect_language_from_content()

Detect language name from file content using the shebang line (`#!`).

Inspects only the first line of `content`. If it begins with `#!`, the
interpreter name is extracted and mapped to a language name.

Handles common patterns:

- `#!/usr/bin/env python3` → `"python"`
- `#!/bin/bash` → `"bash"`
- `#!/usr/bin/env node` → `"javascript"`

The `-S` flag accepted by some `env` implementations is skipped automatically.
Version suffixes (e.g. `python3.11`, `ruby3.2`) are stripped before matching.

Returns `NULL` when content does not start with `#!`, the shebang is
malformed, or the interpreter is not recognised.

**Signature:**

```c
const char** ts_pack_detect_language_from_content(const char* content);
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `content` | `const char*` | Yes | The content to process |

**Returns:** `const char**`


---

#### Ts_pack_root_node_info()

Get a `NodeInfo` snapshot of the root node.

**Signature:**

```c
TsPackNodeInfo* ts_pack_root_node_info(TsPackTree tree);
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `tree` | `TsPackTree` | Yes | The tree |

**Returns:** `TsPackNodeInfo`


---

#### Ts_pack_find_nodes_by_type()

Find all nodes matching the given type name, returning their `NodeInfo`.

Performs a depth-first traversal. Returns an empty vec if no matches.

**Signature:**

```c
TsPackNodeInfo* ts_pack_find_nodes_by_type(TsPackTree tree, const char* node_type);
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `tree` | `TsPackTree` | Yes | The tree |
| `node_type` | `const char*` | Yes | The node type |

**Returns:** `TsPackNodeInfo*`


---

#### Ts_pack_named_children_info()

Get `NodeInfo` for all named children of the root node.

Useful for understanding the top-level structure of a file
(e.g., list of function definitions, class declarations, imports).

**Signature:**

```c
TsPackNodeInfo* ts_pack_named_children_info(TsPackTree tree);
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `tree` | `TsPackTree` | Yes | The tree |

**Returns:** `TsPackNodeInfo*`


---

#### Ts_pack_parse_string()

Parse source code with the named language, returning the syntax tree.

Uses the global registry to look up the language by name.
Caches parsers per-thread so repeated calls for the same language avoid
re-creating the parser.

**Signature:**

```c
TsPackTree* ts_pack_parse_string(const char* language, const uint8_t* source);
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `language` | `const char*` | Yes | The language |
| `source` | `const uint8_t*` | Yes | The source |

**Returns:** `TsPackTree`

**Errors:** Returns `NULL` on error.


---

#### Ts_pack_tree_contains_node_type()

Check whether any node in the tree matches the given type name.

Performs a depth-first traversal using `TreeCursor`.

**Signature:**

```c
bool ts_pack_tree_contains_node_type(TsPackTree tree, const char* node_type);
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `tree` | `TsPackTree` | Yes | The tree |
| `node_type` | `const char*` | Yes | The node type |

**Returns:** `bool`


---

#### Ts_pack_tree_has_error_nodes()

Check whether the tree contains any ERROR or MISSING nodes.

Useful for determining if the parse was clean or had syntax errors.

**Signature:**

```c
bool ts_pack_tree_has_error_nodes(TsPackTree tree);
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `tree` | `TsPackTree` | Yes | The tree |

**Returns:** `bool`


---

#### Ts_pack_tree_to_sexp()

Return the S-expression representation of the entire tree.

This is the standard tree-sitter debug format, useful for logging,
snapshot testing, and debugging grammars.

**Signature:**

```c
const char* ts_pack_tree_to_sexp(TsPackTree tree);
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `tree` | `TsPackTree` | Yes | The tree |

**Returns:** `const char*`


---

#### Ts_pack_tree_error_count()

Count the number of ERROR and MISSING nodes in the tree.

Returns 0 for a clean parse.

**Signature:**

```c
uintptr_t ts_pack_tree_error_count(TsPackTree tree);
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `tree` | `TsPackTree` | Yes | The tree |

**Returns:** `uintptr_t`


---

#### Ts_pack_get_highlights_query()

Get the highlights query for a language, if bundled.

Returns the contents of `highlights.scm` as a static string, or `NULL`
if no highlights query is bundled for this language.

**Signature:**

```c
const char** ts_pack_get_highlights_query(const char* language);
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `language` | `const char*` | Yes | The language |

**Returns:** `const char**`


---

#### Ts_pack_get_injections_query()

Get the injections query for a language, if bundled.

Returns the contents of `injections.scm` as a static string, or `NULL`
if no injections query is bundled for this language.

**Signature:**

```c
const char** ts_pack_get_injections_query(const char* language);
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `language` | `const char*` | Yes | The language |

**Returns:** `const char**`


---

#### Ts_pack_get_locals_query()

Get the locals query for a language, if bundled.

Returns the contents of `locals.scm` as a static string, or `NULL`
if no locals query is bundled for this language.

**Signature:**

```c
const char** ts_pack_get_locals_query(const char* language);
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `language` | `const char*` | Yes | The language |

**Returns:** `const char**`


---

#### Ts_pack_run_query()

Execute a tree-sitter query pattern against a parsed tree.

The `query_source` is an S-expression pattern like:

```text
(function_definition name: (identifier) @name)
```

Returns all matches with their captured nodes.

**Signature:**

```c
TsPackQueryMatch* ts_pack_run_query(TsPackTree tree, const char* language, const char* query_source, const uint8_t* source);
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `tree` | `TsPackTree` | Yes | The parsed syntax tree to query. |
| `language` | `const char*` | Yes | Language name (used to compile the query pattern). |
| `query_source` | `const char*` | Yes | The tree-sitter query pattern string. |
| `source` | `const uint8_t*` | Yes | The original source code bytes (needed for capture resolution). |

**Returns:** `TsPackQueryMatch*`

**Errors:** Returns `NULL` on error.


---

#### Ts_pack_get_language()

Get a tree-sitter `Language` by name using the global registry.

Resolves language aliases (e.g., `"shell"` maps to `"bash"`).
When the `download` feature is enabled (default), automatically downloads
the parser from GitHub releases if not found locally.

**Errors:**

Returns `Error.LanguageNotFound` if the language is not recognized,
or `Error.Download` if auto-download fails.

**Signature:**

```c
TsPackLanguage* ts_pack_get_language(const char* name);
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `name` | `const char*` | Yes | The name |

**Returns:** `TsPackLanguage`

**Errors:** Returns `NULL` on error.


---

#### Ts_pack_get_parser()

Get a tree-sitter `Parser` pre-configured for the given language.

This is a convenience function that calls `get_language` and configures
a new parser in one step.

**Errors:**

Returns `Error.LanguageNotFound` if the language is not recognized, or
`Error.ParserSetup` if the language cannot be applied to the parser.

**Signature:**

```c
TsPackParser* ts_pack_get_parser(const char* name);
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `name` | `const char*` | Yes | The name |

**Returns:** `TsPackParser`

**Errors:** Returns `NULL` on error.


---

#### Ts_pack_available_languages()

List all available language names (sorted, deduplicated, includes aliases).

Returns names of both statically compiled and dynamically loadable languages,
plus any configured aliases.

**Signature:**

```c
const char** ts_pack_available_languages();
```

**Returns:** `const char**`


---

#### Ts_pack_has_language()

Check if a language is available by name or alias.

Returns `true` if the language can be loaded (statically compiled,
dynamically available, or a known alias for one of these).

**Signature:**

```c
bool ts_pack_has_language(const char* name);
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `name` | `const char*` | Yes | The name |

**Returns:** `bool`


---

#### Ts_pack_language_count()

Return the number of available languages.

Includes statically compiled languages, dynamically loadable languages,
and aliases.

**Signature:**

```c
uintptr_t ts_pack_language_count();
```

**Returns:** `uintptr_t`


---

#### Ts_pack_process()

Process source code and extract file intelligence using the global registry.

Parses the source with tree-sitter and extracts metrics, structure, imports,
exports, comments, docstrings, symbols, diagnostics, and/or chunks based on
the flags set in `ProcessConfig`.

**Errors:**

Returns an error if the language is not found or parsing fails.

**Signature:**

```c
TsPackProcessResult* ts_pack_process(const char* source, TsPackProcessConfig config);
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `source` | `const char*` | Yes | The source |
| `config` | `TsPackProcessConfig` | Yes | The configuration options |

**Returns:** `TsPackProcessResult`

**Errors:** Returns `NULL` on error.


---

#### Ts_pack_extract_patterns()

Run extraction patterns against source code.

Convenience wrapper around `extract.extract`.

**Errors:**

Returns an error if the language is not found, parsing fails, or a query
pattern is invalid.

**Signature:**

```c
TsPackExtractionResult* ts_pack_extract_patterns(const char* source, TsPackExtractionConfig config);
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `source` | `const char*` | Yes | The source |
| `config` | `TsPackExtractionConfig` | Yes | The configuration options |

**Returns:** `TsPackExtractionResult`

**Errors:** Returns `NULL` on error.


---

#### Ts_pack_validate_extraction()

Validate extraction patterns without running them.

Convenience wrapper around `extract.validate_extraction`.

**Errors:**

Returns an error if the language cannot be loaded.

**Signature:**

```c
TsPackValidationResult* ts_pack_validate_extraction(TsPackExtractionConfig config);
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `config` | `TsPackExtractionConfig` | Yes | The configuration options |

**Returns:** `TsPackValidationResult`

**Errors:** Returns `NULL` on error.


---

#### Ts_pack_init()

Initialize the language pack with the given configuration.

Applies any custom cache directory, then downloads all languages and groups
specified in the config. This is the recommended entry point when you want
to pre-warm the cache before use.

**Errors:**

Returns an error if configuration cannot be applied or if downloads fail.

**Signature:**

```c
void ts_pack_init(TsPackPackConfig config);
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `config` | `TsPackPackConfig` | Yes | The configuration options |

**Returns:** `void`

**Errors:** Returns `NULL` on error.


---

#### Ts_pack_configure()

Apply download configuration without downloading anything.

Use this to set a custom cache directory before the first call to
`get_language` or any download function. Changing the cache dir
after languages have been registered has no effect on already-loaded
languages.

**Errors:**

Returns an error if the lock cannot be acquired.

**Signature:**

```c
void ts_pack_configure(TsPackPackConfig config);
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `config` | `TsPackPackConfig` | Yes | The configuration options |

**Returns:** `void`

**Errors:** Returns `NULL` on error.


---

#### Ts_pack_download()

Download specific languages to the local cache.

Returns the number of newly downloaded languages (languages that were
already cached are not counted).

**Errors:**

Returns an error if any language is not available in the manifest or if
the download fails.

**Signature:**

```c
uintptr_t ts_pack_download(const char** names);
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `names` | `const char**` | Yes | The names |

**Returns:** `uintptr_t`

**Errors:** Returns `NULL` on error.


---

#### Ts_pack_download_all()

Download all available languages from the remote manifest.

Returns the number of newly downloaded languages.

**Errors:**

Returns an error if the manifest cannot be fetched or a download fails.

**Signature:**

```c
uintptr_t ts_pack_download_all();
```

**Returns:** `uintptr_t`

**Errors:** Returns `NULL` on error.


---

#### Ts_pack_manifest_languages()

Return all language names available in the remote manifest (305).

Fetches (and caches) the remote manifest to discover the full list of
downloadable languages. Use `downloaded_languages` to list what is
already cached locally.

**Errors:**

Returns an error if the manifest cannot be fetched.

**Signature:**

```c
const char** ts_pack_manifest_languages();
```

**Returns:** `const char**`

**Errors:** Returns `NULL` on error.


---

#### Ts_pack_downloaded_languages()

Return languages that are already downloaded and cached locally.

Does not perform any network requests. Returns an empty list if the
cache directory does not exist or cannot be read.

**Signature:**

```c
const char** ts_pack_downloaded_languages();
```

**Returns:** `const char**`


---

#### Ts_pack_clean_cache()

Delete all cached parser shared libraries.

Resets the cache registration so the next call to `get_language` or
a download function will re-register the (now empty) cache directory.

**Errors:**

Returns an error if the cache directory cannot be removed.

**Signature:**

```c
void ts_pack_clean_cache();
```

**Returns:** `void`

**Errors:** Returns `NULL` on error.


---

#### Ts_pack_cache_dir()

Return the effective cache directory path.

This is either the custom path set via `configure` / `init` or the
default: `~/.cache/tree-sitter-language-pack/v{version}/libs/`.

**Errors:**

Returns an error if the system cache directory cannot be determined.

**Signature:**

```c
const char* ts_pack_cache_dir();
```

**Returns:** `const char*`

**Errors:** Returns `NULL` on error.


---

### Types

#### TsPackCaptureResult

A single captured node within a match.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `name` | `const char*` | — | The capture name from the query (e.g., `"fn_name"`). |
| `node` | `TsPackNodeInfo*` | `NULL` | The `NodeInfo` snapshot, present when `CaptureOutput` is `Node` or `Full`. |
| `text` | `const char**` | `NULL` | The matched source text, present when `CaptureOutput` is `Text` or `Full`. |
| `child_fields` | `const char*` | — | Values of requested child fields, keyed by field name. |
| `start_byte` | `uintptr_t` | — | Byte offset where this capture starts in the source. |


---

#### TsPackChunkContext

Metadata for a single chunk of source code.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `language` | `const char*` | — | Language |
| `chunk_index` | `uintptr_t` | — | Chunk index |
| `total_chunks` | `uintptr_t` | — | Total chunks |
| `node_types` | `const char**` | `NULL` | Node types |
| `context_path` | `const char**` | `NULL` | Context path |
| `symbols_defined` | `const char**` | `NULL` | Symbols defined |
| `comments` | `TsPackCommentInfo*` | `NULL` | Comments |
| `docstrings` | `TsPackDocstringInfo*` | `NULL` | Docstrings |
| `has_error_nodes` | `bool` | — | Whether error nodes |


---

#### TsPackCodeChunk

A chunk of source code with rich metadata.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `content` | `const char*` | — | The extracted text content |
| `start_byte` | `uintptr_t` | — | Start byte |
| `end_byte` | `uintptr_t` | — | End byte |
| `start_line` | `uintptr_t` | — | Start line |
| `end_line` | `uintptr_t` | — | End line |
| `metadata` | `TsPackChunkContext` | — | Document metadata |


---

#### TsPackCommentInfo

A comment extracted from source code.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `text` | `const char*` | — | Text |
| `kind` | `TsPackCommentKind` | `TS_PACK_TS_PACK_LINE` | Kind (comment kind) |
| `span` | `TsPackSpan` | — | Span (span) |
| `associated_node` | `const char**` | `NULL` | Associated node |


---

#### TsPackDiagnostic

A diagnostic (syntax error, missing node, etc.) from parsing.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `message` | `const char*` | — | Message |
| `severity` | `TsPackDiagnosticSeverity` | `TS_PACK_TS_PACK_ERROR` | Severity (diagnostic severity) |
| `span` | `TsPackSpan` | — | Span (span) |


---

#### TsPackDocSection

A section within a docstring (e.g., Args, Returns, Raises).

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `kind` | `const char*` | — | Kind |
| `name` | `const char**` | `NULL` | The name |
| `description` | `const char*` | — | Human-readable description |


---

#### TsPackDocstringInfo

A docstring extracted from source code.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `text` | `const char*` | — | Text |
| `format` | `TsPackDocstringFormat` | `TS_PACK_TS_PACK_PYTHON_TRIPLE_QUOTE` | Format (docstring format) |
| `span` | `TsPackSpan` | — | Span (span) |
| `associated_item` | `const char**` | `NULL` | Associated item |
| `parsed_sections` | `TsPackDocSection*` | `NULL` | Parsed sections |


---

#### TsPackDownloadManager

Manages downloading and caching of pre-built parser shared libraries.

##### Methods

###### Ts_pack_new()

Create a new download manager for the given version.

**Signature:**

```c
TsPackDownloadManager ts_pack_new(const char* version);
```

###### Ts_pack_with_cache_dir()

Create a download manager with a custom cache directory.

**Signature:**

```c
TsPackDownloadManager ts_pack_with_cache_dir(const char* version, const char* cache_dir);
```

###### Ts_pack_default_cache_dir()

Default cache directory: `~/.cache/tree-sitter-language-pack/v{version}/libs/`

**Signature:**

```c
const char* ts_pack_default_cache_dir(const char* version);
```

###### Ts_pack_cache_dir()

Return the path to the libs cache directory.

**Signature:**

```c
const char* ts_pack_cache_dir();
```

###### Ts_pack_installed_languages()

List languages that are already downloaded and cached.

**Signature:**

```c
const char** ts_pack_installed_languages();
```

###### Ts_pack_ensure_languages()

Ensure the specified languages are available in the cache.
Downloads the platform bundle if any requested languages are missing.

**Signature:**

```c
void ts_pack_ensure_languages(const char** names);
```

###### Ts_pack_ensure_group()

Ensure all languages in a named group are available.

**Signature:**

```c
void ts_pack_ensure_group(const char* group);
```

###### Ts_pack_lib_path()

Get the expected path for a language's shared library in the cache.

**Signature:**

```c
const char* ts_pack_lib_path(const char* name);
```

###### Ts_pack_fetch_manifest()

Fetch the parser manifest from GitHub Releases.

**Signature:**

```c
TsPackParserManifest ts_pack_fetch_manifest();
```

###### Ts_pack_clean_cache()

Remove all cached parser libraries.

**Signature:**

```c
void ts_pack_clean_cache();
```


---

#### TsPackExportInfo

An export statement extracted from source code.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `name` | `const char*` | — | The name |
| `kind` | `TsPackExportKind` | `TS_PACK_TS_PACK_NAMED` | Kind (export kind) |
| `span` | `TsPackSpan` | — | Span (span) |


---

#### TsPackExtractionConfig

Configuration for an extraction run against a single language.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `language` | `const char*` | — | The language name (e.g., `"python"`). |
| `patterns` | `const char*` | — | Named patterns to run. Keys become the keys in `ExtractionResult.results`. |


---

#### TsPackExtractionPattern

Defines a single extraction pattern and its configuration.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `query` | `const char*` | — | The tree-sitter query string (S-expression). |
| `capture_output` | `TsPackCaptureOutput` | `TS_PACK_TS_PACK_FULL` | What to include in each capture result. |
| `child_fields` | `const char**` | `NULL` | Field names to extract from child nodes of each capture. Maps a label to a tree-sitter field name used with `child_by_field_name`. |
| `max_results` | `uintptr_t*` | `NULL` | Maximum number of matches to return. `NULL` means unlimited. |
| `byte_range` | `uintptr_t**` | `NULL` | Restrict matches to a byte range `(start, end)`. |


---

#### TsPackExtractionResult

Complete extraction results for all patterns.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `language` | `const char*` | — | The language that was used. |
| `results` | `const char*` | — | Results keyed by pattern name. |


---

#### TsPackFileMetrics

Aggregate metrics for a source file.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `total_lines` | `uintptr_t` | — | Total lines |
| `code_lines` | `uintptr_t` | — | Code lines |
| `comment_lines` | `uintptr_t` | — | Comment lines |
| `blank_lines` | `uintptr_t` | — | Blank lines |
| `total_bytes` | `uintptr_t` | — | Total bytes |
| `node_count` | `uintptr_t` | — | Number of nodes |
| `error_count` | `uintptr_t` | — | Number of errors |
| `max_depth` | `uintptr_t` | — | Maximum depth |


---

#### TsPackImportInfo

An import statement extracted from source code.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `source` | `const char*` | — | Source |
| `items` | `const char**` | `NULL` | Items |
| `alias` | `const char**` | `NULL` | Alias |
| `is_wildcard` | `bool` | — | Whether wildcard |
| `span` | `TsPackSpan` | — | Span (span) |


---

#### TsPackLanguage


---

#### TsPackLanguageInfo

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `group` | `const char*` | — | Group |
| `size` | `uint64_t` | — | Size in bytes |


---

#### TsPackLanguageRegistry

Thread-safe registry of tree-sitter language parsers.

Manages both statically compiled and dynamically loaded language grammars.
Use `LanguageRegistry.new()` for the default registry, or access the
global instance via the module-level convenience functions
(`crate.get_language`, `crate.available_languages`, etc.).

##### Methods

###### Ts_pack_with_libs_dir()

Create a registry with a custom directory for dynamic libraries.

Overrides the default build-time library directory. Useful when
dynamic grammar shared libraries are stored in a non-standard location.

**Signature:**

```c
TsPackLanguageRegistry ts_pack_with_libs_dir(const char* libs_dir);
```

###### Ts_pack_add_extra_libs_dir()

Add an additional directory to search for dynamic libraries.

When `get_language` cannot find a grammar in the
primary library directory, it searches these extra directories in order.
Typically used by the download system to register its cache directory.

Takes `&self` (not `&mut self`) because `extra_lib_dirs` uses interior
mutability via an `Arc<RwLock<...>>`, so the outer registry can remain
immutable while the directory list is updated.

**Signature:**

```c
void ts_pack_add_extra_libs_dir(const char* dir);
```

###### Ts_pack_get_language()

Get a tree-sitter `Language` by name.

Resolves aliases (e.g., `"shell"` -> `"bash"`, `"makefile"` -> `"make"`),
then looks up the language in the static table. When the `dynamic-loading`
feature is enabled, falls back to loading a shared library on demand.

**Errors:**

Returns `Error.LanguageNotFound` if the name (after alias resolution)
does not match any known grammar.

**Signature:**

```c
TsPackLanguage ts_pack_get_language(const char* name);
```

###### Ts_pack_available_languages()

List all available language names, sorted and deduplicated.

Includes statically compiled languages, dynamically loadable languages
(if the `dynamic-loading` feature is enabled), and all configured aliases.

**Signature:**

```c
const char** ts_pack_available_languages();
```

###### Ts_pack_has_language()

Check whether a language is available by name or alias.

Returns `true` if the language can be loaded, either from the static
table or from a dynamic library on disk.

**Signature:**

```c
bool ts_pack_has_language(const char* name);
```

###### Ts_pack_language_count()

Return the total number of available languages (including aliases).

**Signature:**

```c
uintptr_t ts_pack_language_count();
```

###### Ts_pack_process()

Parse source code and extract file intelligence based on config in a single pass.

**Signature:**

```c
TsPackProcessResult ts_pack_process(const char* source, TsPackProcessConfig config);
```

###### Ts_pack_default()

**Signature:**

```c
TsPackLanguageRegistry ts_pack_default();
```


---

#### TsPackMatchResult

A single query match containing one or more captures.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `pattern_index` | `uintptr_t` | — | The pattern index within the query that produced this match. |
| `captures` | `TsPackCaptureResult*` | `NULL` | The captures for this match. |


---

#### TsPackNodeInfo

Lightweight snapshot of a tree-sitter node's properties.

Contains only primitive types for easy cross-language serialization.
This is an owned type that can be passed across FFI boundaries, unlike
`tree_sitter.Node` which borrows from the tree.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `kind` | `const char*` | — | The grammar type name (e.g., "function_definition", "identifier"). |
| `is_named` | `bool` | — | Whether this is a named node (vs anonymous like punctuation). |
| `start_byte` | `uintptr_t` | — | Start byte offset in source. |
| `end_byte` | `uintptr_t` | — | End byte offset in source. |
| `start_row` | `uintptr_t` | — | Start row (zero-indexed). |
| `start_col` | `uintptr_t` | — | Start column (zero-indexed). |
| `end_row` | `uintptr_t` | — | End row (zero-indexed). |
| `end_col` | `uintptr_t` | — | End column (zero-indexed). |
| `named_child_count` | `uintptr_t` | — | Number of named children. |
| `is_error` | `bool` | — | Whether this node is an ERROR node. |
| `is_missing` | `bool` | — | Whether this node is a MISSING node. |


---

#### TsPackPackConfig

Configuration for the tree-sitter language pack.

Controls cache directory and which languages to pre-download.
Can be loaded from a TOML file, constructed programmatically,
or passed as a dict/object from language bindings.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `cache_dir` | `const char**` | `NULL` | Override default cache directory. Default: `~/.cache/tree-sitter-language-pack/v{version}/libs/` |
| `languages` | `const char***` | `NULL` | Languages to pre-download on init. Each entry is a language name (e.g. `"python"`, `"rust"`). |
| `groups` | `const char***` | `NULL` | Language groups to pre-download (e.g. `"web"`, `"systems"`, `"scripting"`). |

##### Methods

###### Ts_pack_from_toml_file()

Load configuration from a TOML file.

**Errors:**

Returns an error if the file cannot be read or the TOML is invalid.

**Signature:**

```c
TsPackPackConfig ts_pack_from_toml_file(const char* path);
```

###### Ts_pack_discover()

Discover configuration by searching for `language-pack.toml` in:

1. Current directory and up to 10 parent directories
2. `$XDG_CONFIG_HOME/tree-sitter-language-pack/config.toml`
3. `~/.config/tree-sitter-language-pack/config.toml`

Returns `NULL` if no configuration file is found.

**Signature:**

```c
TsPackPackConfig* ts_pack_discover();
```


---

#### TsPackParser


---

#### TsPackParserManifest

Manifest describing available parser downloads for a specific version.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `version` | `const char*` | — | Version string |
| `platforms` | `void*` | — | Platforms |
| `languages` | `void*` | — | Languages |
| `groups` | `void*` | — | Groups |


---

#### TsPackPatternResult

Results for a single named pattern.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `matches` | `TsPackMatchResult*` | `NULL` | The individual matches. |
| `total_count` | `uintptr_t` | — | Total number of matches before `max_results` truncation. |


---

#### TsPackPatternValidation

Validation information for a single pattern.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `valid` | `bool` | — | Whether the pattern compiled successfully. |
| `capture_names` | `const char**` | `NULL` | Names of captures defined in the query. |
| `pattern_count` | `uintptr_t` | — | Number of patterns in the query. |
| `warnings` | `const char**` | `NULL` | Non-fatal warnings (e.g., unused captures). |
| `errors` | `const char**` | `NULL` | Fatal errors (e.g., query syntax errors). |


---

#### TsPackPlatformBundle

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `url` | `const char*` | — | Url |
| `sha256` | `const char*` | — | Sha256 |
| `size` | `uint64_t` | — | Size in bytes |


---

#### TsPackProcessConfig

Configuration for the `process()` function.

Controls which analysis features are enabled and whether chunking is performed.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `language` | `const char*` | — | Language name (required). |
| `structure` | `bool` | `true` | Extract structural items (functions, classes, etc.). Default: true. |
| `imports` | `bool` | `true` | Extract import statements. Default: true. |
| `exports` | `bool` | `true` | Extract export statements. Default: true. |
| `comments` | `bool` | `false` | Extract comments. Default: false. |
| `docstrings` | `bool` | `false` | Extract docstrings. Default: false. |
| `symbols` | `bool` | `false` | Extract symbol definitions. Default: false. |
| `diagnostics` | `bool` | `false` | Include parse diagnostics. Default: false. |
| `chunk_max_size` | `uintptr_t*` | `NULL` | Maximum chunk size in bytes. `NULL` disables chunking. |
| `extractions` | `const char**` | `NULL` | Custom extraction patterns to run against the parsed tree. Keys become the keys in `ProcessResult.extractions`. |

##### Methods

###### Ts_pack_default()

**Signature:**

```c
TsPackProcessConfig ts_pack_default();
```

###### Ts_pack_with_chunking()

Enable chunking with the given maximum chunk size in bytes.

**Signature:**

```c
TsPackProcessConfig ts_pack_with_chunking(uintptr_t max_size);
```

###### Ts_pack_all()

Enable all analysis features.

**Signature:**

```c
TsPackProcessConfig ts_pack_all();
```

###### Ts_pack_minimal()

Disable all analysis features (only metrics computed).

**Signature:**

```c
TsPackProcessConfig ts_pack_minimal();
```


---

#### TsPackProcessResult

Complete analysis result from processing a source file.

Contains metrics, structural analysis, imports/exports, comments,
docstrings, symbols, diagnostics, and optionally chunked code segments.
Fields are populated based on the `crate.ProcessConfig` flags.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `language` | `const char*` | — | Language |
| `metrics` | `TsPackFileMetrics` | — | Metrics (file metrics) |
| `structure` | `TsPackStructureItem*` | `NULL` | Structure |
| `imports` | `TsPackImportInfo*` | `NULL` | Imports |
| `exports` | `TsPackExportInfo*` | `NULL` | Exports |
| `comments` | `TsPackCommentInfo*` | `NULL` | Comments |
| `docstrings` | `TsPackDocstringInfo*` | `NULL` | Docstrings |
| `symbols` | `TsPackSymbolInfo*` | `NULL` | Symbols |
| `diagnostics` | `TsPackDiagnostic*` | `NULL` | Diagnostics |
| `chunks` | `TsPackCodeChunk*` | `NULL` | Text chunks for chunking/embedding |
| `extractions` | `const char*` | — | Results of custom extraction patterns (when `config.extractions` is set). |


---

#### TsPackQueryMatch

A single match from a tree-sitter query, with captured nodes.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `pattern_index` | `uintptr_t` | — | The pattern index that matched (position in the query string). |
| `captures` | `const char**` | `NULL` | Captures: list of (capture_name, node_info) pairs. |


---

#### TsPackSpan

Byte and line/column range in source code.

Represents both byte offsets (for slicing) and human-readable line/column
positions (for display and diagnostics).

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `start_byte` | `uintptr_t` | — | Start byte |
| `end_byte` | `uintptr_t` | — | End byte |
| `start_line` | `uintptr_t` | — | Start line |
| `start_column` | `uintptr_t` | — | Start column |
| `end_line` | `uintptr_t` | — | End line |
| `end_column` | `uintptr_t` | — | End column |


---

#### TsPackStructureItem

A structural item (function, class, struct, etc.) in source code.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `kind` | `TsPackStructureKind` | `TS_PACK_TS_PACK_FUNCTION` | Kind (structure kind) |
| `name` | `const char**` | `NULL` | The name |
| `visibility` | `const char**` | `NULL` | Visibility |
| `span` | `TsPackSpan` | — | Span (span) |
| `children` | `TsPackStructureItem*` | `NULL` | Children |
| `decorators` | `const char**` | `NULL` | Decorators |
| `doc_comment` | `const char**` | `NULL` | Doc comment |
| `signature` | `const char**` | `NULL` | Signature |
| `body_span` | `TsPackSpan*` | `NULL` | Body span (span) |


---

#### TsPackSymbolInfo

A symbol (variable, function, type, etc.) extracted from source code.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `name` | `const char*` | — | The name |
| `kind` | `TsPackSymbolKind` | `TS_PACK_TS_PACK_VARIABLE` | Kind (symbol kind) |
| `span` | `TsPackSpan` | — | Span (span) |
| `type_annotation` | `const char**` | `NULL` | Type annotation |
| `doc` | `const char**` | `NULL` | Doc |


---

#### TsPackTree


---

#### TsPackValidationResult

Validation results for an entire extraction config.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `valid` | `bool` | — | Whether all patterns are valid. |
| `patterns` | `const char*` | — | Per-pattern validation details. |


---

### Enums

#### TsPackCaptureOutput

Controls what data is captured for each query match.

| Value | Description |
|-------|-------------|
| `TS_PACK_TEXT` | Capture only the matched text. |
| `TS_PACK_NODE` | Capture only the `NodeInfo`. |
| `TS_PACK_FULL` | Capture both text and `NodeInfo` (default). |


---

#### TsPackStructureKind

The kind of structural item found in source code.

Categorizes top-level and nested declarations such as functions, classes,
structs, enums, traits, and more. Use `Other` for
language-specific constructs that do not fit a standard category.

| Value | Description |
|-------|-------------|
| `TS_PACK_FUNCTION` | Function |
| `TS_PACK_METHOD` | Method |
| `TS_PACK_CLASS` | Class |
| `TS_PACK_STRUCT` | Struct |
| `TS_PACK_INTERFACE` | Interface |
| `TS_PACK_ENUM` | Enum |
| `TS_PACK_MODULE` | Module |
| `TS_PACK_TRAIT` | Trait |
| `TS_PACK_IMPL` | Impl |
| `TS_PACK_NAMESPACE` | Namespace |
| `TS_PACK_OTHER` | Other — Fields: `0`: `const char*` |


---

#### TsPackCommentKind

The kind of a comment found in source code.

Distinguishes between single-line comments, block (multi-line) comments,
and documentation comments.

| Value | Description |
|-------|-------------|
| `TS_PACK_LINE` | Line |
| `TS_PACK_BLOCK` | Block |
| `TS_PACK_DOC` | Doc |


---

#### TsPackDocstringFormat

The format of a docstring extracted from source code.

Identifies the docstring convention used, which varies by language
(e.g., Python triple-quoted strings, JSDoc, Rustdoc `///` comments).

| Value | Description |
|-------|-------------|
| `TS_PACK_PYTHON_TRIPLE_QUOTE` | Python triple quote |
| `TS_PACK_JS_DOC` | J s doc |
| `TS_PACK_RUSTDOC` | Rustdoc |
| `TS_PACK_GO_DOC` | Go doc |
| `TS_PACK_JAVA_DOC` | Java doc |
| `TS_PACK_OTHER` | Other — Fields: `0`: `const char*` |


---

#### TsPackExportKind

The kind of an export statement found in source code.

Covers named exports, default exports, and re-exports from other modules.

| Value | Description |
|-------|-------------|
| `TS_PACK_NAMED` | Named |
| `TS_PACK_DEFAULT` | Default |
| `TS_PACK_RE_EXPORT` | Re export |


---

#### TsPackSymbolKind

The kind of a symbol definition found in source code.

Categorizes symbol definitions such as variables, constants, functions,
classes, types, interfaces, enums, and modules.

| Value | Description |
|-------|-------------|
| `TS_PACK_VARIABLE` | Variable |
| `TS_PACK_CONSTANT` | Constant |
| `TS_PACK_FUNCTION` | Function |
| `TS_PACK_CLASS` | Class |
| `TS_PACK_TYPE` | Type |
| `TS_PACK_INTERFACE` | Interface |
| `TS_PACK_ENUM` | Enum |
| `TS_PACK_MODULE` | Module |
| `TS_PACK_OTHER` | Other — Fields: `0`: `const char*` |


---

#### TsPackDiagnosticSeverity

Severity level of a diagnostic produced during parsing.

Used to classify parse errors, warnings, and informational messages
found in the syntax tree.

| Value | Description |
|-------|-------------|
| `TS_PACK_ERROR` | Error |
| `TS_PACK_WARNING` | Warning |
| `TS_PACK_INFO` | Info |


---

### Errors

#### TsPackError

Errors that can occur when using the tree-sitter language pack.

Covers language lookup failures, parse errors, query errors, and I/O issues.
Feature-gated variants are included when `config`, `download`, or related
features are enabled.

| Variant | Description |
|---------|-------------|
| `TS_PACK_LANGUAGE_NOT_FOUND` | Language '{0}' not found |
| `TS_PACK_DYNAMIC_LOAD` | Dynamic library load error: {0} |
| `TS_PACK_NULL_LANGUAGE_POINTER` | Language function returned null pointer for '{0}' |
| `TS_PACK_PARSER_SETUP` | Failed to set parser language: {0} |
| `TS_PACK_LOCK_POISONED` | Registry lock poisoned: {0} |
| `TS_PACK_CONFIG` | Configuration error: {0} |
| `TS_PACK_PARSE_FAILED` | Parse failed: parsing returned no tree |
| `TS_PACK_QUERY_ERROR` | Query error: {0} |
| `TS_PACK_INVALID_RANGE` | Invalid byte range: {0} |
| `TS_PACK_IO` | IO error: {0} |


---
