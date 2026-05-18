---
title: "C API Reference"
---

## C API Reference <span class="version-badge">v1.8.1</span>

### Functions

#### ts_pack_detect_language_from_extension()

Detect language name from a file extension (without leading dot).

Returns `NULL` for unrecognized extensions. The match is case-insensitive.

**Signature:**

```c
const char** ts_pack_detect_language_from_extension(const char* ext);
```

**Parameters:**

| Name  | Type          | Required | Description |
| ----- | ------------- | -------- | ----------- |
| `ext` | `const char*` | Yes      | The ext     |

**Returns:** `const char**`

---

#### ts_pack_detect_language_from_path()

Detect language name from a file path.

Extracts the file extension and looks it up. Returns `NULL` if the
path has no extension or the extension is not recognized.

**Signature:**

```c
const char** ts_pack_detect_language_from_path(const char* path);
```

**Parameters:**

| Name   | Type          | Required | Description      |
| ------ | ------------- | -------- | ---------------- |
| `path` | `const char*` | Yes      | Path to the file |

**Returns:** `const char**`

---

#### ts_pack_detect_language_from_content()

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

| Name      | Type          | Required | Description            |
| --------- | ------------- | -------- | ---------------------- |
| `content` | `const char*` | Yes      | The content to process |

**Returns:** `const char**`

---

#### ts_pack_get_highlights_query()

Get the highlights query for a language, if bundled.

Returns the contents of `highlights.scm` as a static string, or `NULL`
if no highlights query is bundled for this language.

**Signature:**

```c
const char** ts_pack_get_highlights_query(const char* language);
```

**Parameters:**

| Name       | Type          | Required | Description  |
| ---------- | ------------- | -------- | ------------ |
| `language` | `const char*` | Yes      | The language |

**Returns:** `const char**`

---

#### ts_pack_get_injections_query()

Get the injections query for a language, if bundled.

Returns the contents of `injections.scm` as a static string, or `NULL`
if no injections query is bundled for this language.

**Signature:**

```c
const char** ts_pack_get_injections_query(const char* language);
```

**Parameters:**

| Name       | Type          | Required | Description  |
| ---------- | ------------- | -------- | ------------ |
| `language` | `const char*` | Yes      | The language |

**Returns:** `const char**`

---

#### ts_pack_get_locals_query()

Get the locals query for a language, if bundled.

Returns the contents of `locals.scm` as a static string, or `NULL`
if no locals query is bundled for this language.

**Signature:**

```c
const char** ts_pack_get_locals_query(const char* language);
```

**Parameters:**

| Name       | Type          | Required | Description  |
| ---------- | ------------- | -------- | ------------ |
| `language` | `const char*` | Yes      | The language |

**Returns:** `const char**`

---

#### ts_pack_get_language()

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

| Name   | Type          | Required | Description |
| ------ | ------------- | -------- | ----------- |
| `name` | `const char*` | Yes      | The name    |

**Returns:** `TsPackLanguage`
**Errors:** Returns `NULL` on error.

---

#### ts_pack_get_parser()

Get a `Parser` pre-configured for the given language.

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

| Name   | Type          | Required | Description |
| ------ | ------------- | -------- | ----------- |
| `name` | `const char*` | Yes      | The name    |

**Returns:** `TsPackParser`
**Errors:** Returns `NULL` on error.

---

#### ts_pack_detect_language()

Detect language name from a file path or extension.

This compatibility alias matches the pre-Alef Python binding API.

**Signature:**

```c
const char** ts_pack_detect_language(const char* path);
```

**Parameters:**

| Name   | Type          | Required | Description      |
| ------ | ------------- | -------- | ---------------- |
| `path` | `const char*` | Yes      | Path to the file |

**Returns:** `const char**`

---

#### ts_pack_available_languages()

List all available language names (sorted, deduplicated, includes aliases).

Returns names of both statically compiled and dynamically loadable languages,
plus any configured aliases.

**Signature:**

```c
const char** ts_pack_available_languages();
```

**Returns:** `const char**`

---

#### ts_pack_has_language()

Check if a language is available by name or alias.

Returns `true` if the language can be loaded (statically compiled,
dynamically available, or a known alias for one of these).

**Signature:**

```c
bool ts_pack_has_language(const char* name);
```

**Parameters:**

| Name   | Type          | Required | Description |
| ------ | ------------- | -------- | ----------- |
| `name` | `const char*` | Yes      | The name    |

**Returns:** `bool`

---

#### ts_pack_language_count()

Return the number of available languages.

Includes statically compiled languages, dynamically loadable languages,
and aliases.

**Signature:**

```c
uintptr_t ts_pack_language_count();
```

**Returns:** `uintptr_t`

---

#### ts_pack_process()

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

| Name     | Type                  | Required | Description               |
| -------- | --------------------- | -------- | ------------------------- |
| `source` | `const char*`         | Yes      | The source                |
| `config` | `TsPackProcessConfig` | Yes      | The configuration options |

**Returns:** `TsPackProcessResult`
**Errors:** Returns `NULL` on error.

---

#### ts_pack_init()

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

| Name     | Type               | Required | Description               |
| -------- | ------------------ | -------- | ------------------------- |
| `config` | `TsPackPackConfig` | Yes      | The configuration options |

**Returns:** `void`
**Errors:** Returns `NULL` on error.

---

#### ts_pack_configure()

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

| Name     | Type               | Required | Description               |
| -------- | ------------------ | -------- | ------------------------- |
| `config` | `TsPackPackConfig` | Yes      | The configuration options |

**Returns:** `void`
**Errors:** Returns `NULL` on error.

---

#### ts_pack_download()

Download specific languages to the local cache.

Returns the number of requested languages available after the call. Already
compiled or cached languages are included in the count.

**Errors:**

Returns an error if any language is not available in the manifest or if
the download fails.

**Signature:**

```c
uintptr_t ts_pack_download(const char** names);
```

**Parameters:**

| Name    | Type           | Required | Description |
| ------- | -------------- | -------- | ----------- |
| `names` | `const char**` | Yes      | The names   |

**Returns:** `uintptr_t`
**Errors:** Returns `NULL` on error.

---

#### ts_pack_download_all()

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

```c
uintptr_t ts_pack_download_all();
```

**Returns:** `uintptr_t`
**Errors:** Returns `NULL` on error.

---

#### ts_pack_download_group()

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

```c
uintptr_t ts_pack_download_group(const char* name);
```

**Parameters:**

| Name   | Type          | Required | Description |
| ------ | ------------- | -------- | ----------- |
| `name` | `const char*` | Yes      | The name    |

**Returns:** `uintptr_t`
**Errors:** Returns `NULL` on error.

---

#### ts_pack_manifest_languages()

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

#### ts_pack_downloaded_languages()

Return languages that are already downloaded and cached locally.

Does not perform any network requests. Returns an empty list if the
cache directory does not exist or cannot be read.

**Signature:**

```c
const char** ts_pack_downloaded_languages();
```

**Returns:** `const char**`

---

#### ts_pack_clean_cache()

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

#### ts_pack_cache_dir()

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

#### TsPackByteRange

A byte range — start (inclusive) to end (exclusive).

| Field   | Type        | Default | Description                  |
| ------- | ----------- | ------- | ---------------------------- |
| `start` | `uintptr_t` | —       | Inclusive start byte offset. |
| `end`   | `uintptr_t` | —       | Exclusive end byte offset.   |

---

#### TsPackChunkContext

Metadata for a single chunk of source code.

| Field             | Type                   | Default | Description         |
| ----------------- | ---------------------- | ------- | ------------------- |
| `language`        | `const char*`          | —       | Language            |
| `chunk_index`     | `uintptr_t`            | —       | Chunk index         |
| `total_chunks`    | `uintptr_t`            | —       | Total chunks        |
| `node_types`      | `const char**`         | `NULL`  | Node types          |
| `context_path`    | `const char**`         | `NULL`  | Context path        |
| `symbols_defined` | `const char**`         | `NULL`  | Symbols defined     |
| `comments`        | `TsPackCommentInfo*`   | `NULL`  | Comments            |
| `docstrings`      | `TsPackDocstringInfo*` | `NULL`  | Docstrings          |
| `has_error_nodes` | `bool`                 | —       | Whether error nodes |

---

#### TsPackCodeChunk

A chunk of source code with rich metadata.

| Field        | Type                 | Default | Description                |
| ------------ | -------------------- | ------- | -------------------------- |
| `content`    | `const char*`        | —       | The extracted text content |
| `start_byte` | `uintptr_t`          | —       | Start byte                 |
| `end_byte`   | `uintptr_t`          | —       | End byte                   |
| `start_line` | `uintptr_t`          | —       | Start line                 |
| `end_line`   | `uintptr_t`          | —       | End line                   |
| `metadata`   | `TsPackChunkContext` | —       | Document metadata          |

---

#### TsPackCommentInfo

A comment extracted from source code.

| Field             | Type                | Default                | Description         |
| ----------------- | ------------------- | ---------------------- | ------------------- |
| `text`            | `const char*`       | —                      | Text                |
| `kind`            | `TsPackCommentKind` | `TS_PACK_TS_PACK_LINE` | Kind (comment kind) |
| `span`            | `TsPackSpan`        | —                      | Span (span)         |
| `associated_node` | `const char**`      | `NULL`                 | Associated node     |

---

#### TsPackDiagnostic

A diagnostic (syntax error, missing node, etc.) from parsing.

| Field      | Type                       | Default                 | Description                    |
| ---------- | -------------------------- | ----------------------- | ------------------------------ |
| `message`  | `const char*`              | —                       | Message                        |
| `severity` | `TsPackDiagnosticSeverity` | `TS_PACK_TS_PACK_ERROR` | Severity (diagnostic severity) |
| `span`     | `TsPackSpan`               | —                       | Span (span)                    |

---

#### TsPackDocSection

A section within a docstring (e.g., Args, Returns, Raises).

| Field         | Type           | Default | Description                |
| ------------- | -------------- | ------- | -------------------------- |
| `kind`        | `const char*`  | —       | Kind                       |
| `name`        | `const char**` | `NULL`  | The name                   |
| `description` | `const char*`  | —       | Human-readable description |

---

#### TsPackDocstringInfo

A docstring extracted from source code.

| Field             | Type                    | Default                               | Description               |
| ----------------- | ----------------------- | ------------------------------------- | ------------------------- |
| `text`            | `const char*`           | —                                     | Text                      |
| `format`          | `TsPackDocstringFormat` | `TS_PACK_TS_PACK_PYTHON_TRIPLE_QUOTE` | Format (docstring format) |
| `span`            | `TsPackSpan`            | —                                     | Span (span)               |
| `associated_item` | `const char**`          | `NULL`                                | Associated item           |
| `parsed_sections` | `TsPackDocSection*`     | `NULL`                                | Parsed sections           |

---

#### TsPackDownloadManager

Manages downloading and caching of pre-built parser shared libraries.

##### Methods

###### ts_pack_new()

Create a new download manager for the given version.

**Signature:**

```c
TsPackDownloadManager ts_pack_new(const char* version);
```

###### ts_pack_with_cache_dir()

Create a download manager with a custom cache directory.

**Signature:**

```c
TsPackDownloadManager ts_pack_with_cache_dir(const char* version, const char* cache_dir);
```

###### ts_pack_installed_languages()

List languages that are already downloaded and cached.

**Signature:**

```c
const char** ts_pack_installed_languages();
```

###### ts_pack_download_all_best_effort()

Download the platform bundle and extract every library file it contains.

Unlike `ensure_languages`, this does not check the manifest language list
against archive contents — it simply extracts all `.so`/`.dylib`/`.dll` files
from the bundle. Languages in the manifest that are missing from the archive
are silently ignored rather than returning an error.

Returns the number of library files extracted (including those already cached).

**Signature:**

```c
uintptr_t ts_pack_download_all_best_effort();
```

###### ts_pack_clean_cache()

Remove all cached parser libraries.

**Signature:**

```c
void ts_pack_clean_cache();
```

---

#### TsPackExportInfo

An export statement extracted from source code.

| Field  | Type               | Default                 | Description        |
| ------ | ------------------ | ----------------------- | ------------------ |
| `name` | `const char*`      | —                       | The name           |
| `kind` | `TsPackExportKind` | `TS_PACK_TS_PACK_NAMED` | Kind (export kind) |
| `span` | `TsPackSpan`       | —                       | Span (span)        |

---

#### TsPackFileMetrics

Aggregate metrics for a source file.

| Field           | Type        | Default | Description      |
| --------------- | ----------- | ------- | ---------------- |
| `total_lines`   | `uintptr_t` | —       | Total lines      |
| `code_lines`    | `uintptr_t` | —       | Code lines       |
| `comment_lines` | `uintptr_t` | —       | Comment lines    |
| `blank_lines`   | `uintptr_t` | —       | Blank lines      |
| `total_bytes`   | `uintptr_t` | —       | Total bytes      |
| `node_count`    | `uintptr_t` | —       | Number of nodes  |
| `error_count`   | `uintptr_t` | —       | Number of errors |
| `max_depth`     | `uintptr_t` | —       | Maximum depth    |

---

#### TsPackImportInfo

An import statement extracted from source code.

| Field         | Type           | Default | Description      |
| ------------- | -------------- | ------- | ---------------- |
| `source`      | `const char*`  | —       | Source           |
| `items`       | `const char**` | `NULL`  | Items            |
| `alias`       | `const char**` | `NULL`  | Alias            |
| `is_wildcard` | `bool`         | —       | Whether wildcard |
| `span`        | `TsPackSpan`   | —       | Span (span)      |

---

#### TsPackLanguage

---

#### TsPackLanguageRegistry

Thread-safe registry of tree-sitter language parsers.

Manages both statically compiled and dynamically loaded language grammars.
Use `LanguageRegistry.new()` for the default registry, or access the
global instance via the module-level convenience functions
(`get_language`, `available_languages`, etc.).

##### Methods

###### ts_pack_get_language()

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

###### ts_pack_available_languages()

List all available language names, sorted and deduplicated.

Includes statically compiled languages, dynamically loadable languages
(if the `dynamic-loading` feature is enabled), and all configured aliases.

**Signature:**

```c
const char** ts_pack_available_languages();
```

###### ts_pack_has_language()

Check whether a language is available by name or alias.

Returns `true` if the language can be loaded, either from the static
table or from a dynamic library on disk.

**Signature:**

```c
bool ts_pack_has_language(const char* name);
```

###### ts_pack_language_count()

Return the total number of available languages (including aliases).

**Signature:**

```c
uintptr_t ts_pack_language_count();
```

###### ts_pack_process()

Parse source code and extract file intelligence based on config in a single pass.

**Signature:**

```c
TsPackProcessResult ts_pack_process(const char* source, TsPackProcessConfig config);
```

###### ts_pack_default()

**Signature:**

```c
TsPackLanguageRegistry ts_pack_default();
```

---

#### TsPackNode

A single syntax node within a `Tree`.

Nodes hold a strong reference to their parent tree so they remain valid
regardless of how the tree is moved or stored at the FFI boundary.

##### Methods

###### ts_pack_clone()

**Signature:**

```c
TsPackNode ts_pack_clone();
```

###### ts_pack_kind()

Return the node's kind name (e.g. `"function_definition"`).

**Signature:**

```c
const char* ts_pack_kind();
```

###### ts_pack_kind_id()

Return the node's numeric kind ID.

Tree-sitter assigns a stable `u16` ID to every node kind in a grammar
(e.g. `"function_definition" → 42`). Comparing `kind_id()` is cheaper
than comparing the string `kind()` in tight AST loops.

**Signature:**

```c
uint16_t ts_pack_kind_id();
```

###### ts_pack_start_byte()

Return the inclusive start byte offset of this node.

**Signature:**

```c
uintptr_t ts_pack_start_byte();
```

###### ts_pack_end_byte()

Return the exclusive end byte offset of this node.

**Signature:**

```c
uintptr_t ts_pack_end_byte();
```

###### ts_pack_byte_range()

Return the node's byte range as a `ByteRange`.

Callers should slice their own source bytes — this is a zero-copy
text accessor.

**Signature:**

```c
TsPackByteRange ts_pack_byte_range();
```

###### ts_pack_start_position()

Return the start `Point` (row, column).

**Signature:**

```c
TsPackPoint ts_pack_start_position();
```

###### ts_pack_end_position()

Return the end `Point` (row, column).

**Signature:**

```c
TsPackPoint ts_pack_end_position();
```

###### ts_pack_is_named()

True when this node is named (not punctuation/whitespace).

**Signature:**

```c
bool ts_pack_is_named();
```

###### ts_pack_is_error()

True when this is an error node.

**Signature:**

```c
bool ts_pack_is_error();
```

###### ts_pack_is_missing()

True when this is a missing-token node.

**Signature:**

```c
bool ts_pack_is_missing();
```

###### ts_pack_is_extra()

True when this is an "extra" node (e.g. a comment).

**Signature:**

```c
bool ts_pack_is_extra();
```

###### ts_pack_has_error()

True when this node or any descendant is an error.

**Signature:**

```c
bool ts_pack_has_error();
```

###### ts_pack_parent()

Return this node's parent, if any.

**Signature:**

```c
TsPackNode* ts_pack_parent();
```

###### ts_pack_child()

Return the i-th child of this node, if any.

**Signature:**

```c
TsPackNode* ts_pack_child(uint32_t index);
```

###### ts_pack_child_count()

Total number of children (including unnamed).

**Signature:**

```c
uintptr_t ts_pack_child_count();
```

###### ts_pack_named_child()

Return the i-th named child of this node, if any.

**Signature:**

```c
TsPackNode* ts_pack_named_child(uint32_t index);
```

###### ts_pack_named_child_count()

Number of named children of this node.

**Signature:**

```c
uintptr_t ts_pack_named_child_count();
```

###### ts_pack_child_by_field_name()

Look up a child by its grammar-defined field name.

**Signature:**

```c
TsPackNode* ts_pack_child_by_field_name(const char* name);
```

###### ts_pack_to_sexp()

Return the S-expression form of this node's subtree.

**Signature:**

```c
const char* ts_pack_to_sexp();
```

###### ts_pack_walk()

Return a `TreeCursor` positioned at this node.

**Signature:**

```c
TsPackTreeCursor ts_pack_walk();
```

---

#### TsPackPackConfig

Configuration for the tree-sitter language pack.

Controls cache directory and which languages to pre-download.
Can be loaded from a TOML file, constructed programmatically,
or passed as a dict/object from language bindings.

| Field       | Type            | Default | Description                                                                                      |
| ----------- | --------------- | ------- | ------------------------------------------------------------------------------------------------ |
| `cache_dir` | `const char**`  | `NULL`  | Override default cache directory. Default: `~/.cache/tree-sitter-language-pack/v{version}/libs/` |
| `languages` | `const char***` | `NULL`  | Languages to pre-download on init. Each entry is a language name (e.g. `"python"`, `"rust"`).    |
| `groups`    | `const char***` | `NULL`  | Language groups to pre-download (e.g. `"web"`, `"systems"`, `"scripting"`).                      |

---

#### TsPackParser

A tree-sitter parser configured for one language at a time.

##### Methods

###### ts_pack_set_language()

Configure the parser to use the language identified by name (e.g. `"python"`).

Resolves the language through the global registry — auto-downloading
if necessary, when the `download` feature is enabled.

**Errors:**

Returns `Error.LanguageNotFound` if the language is not recognized,
or `Error.ParserSetup` if the language ABI is incompatible.

**Signature:**

```c
void ts_pack_set_language(const char* name);
```

###### ts_pack_parse()

Parse a UTF-8 source string. Returns `NULL` if parsing was cancelled
or no language is set.

**Signature:**

```c
TsPackTree* ts_pack_parse(const char* source);
```

###### ts_pack_parse_bytes()

Parse a raw byte slice. Returns `NULL` if parsing was cancelled or
no language is set.

**Signature:**

```c
TsPackTree* ts_pack_parse_bytes(const uint8_t* source);
```

###### ts_pack_reset()

Reset internal state. The next call to `parse` will
not be incremental.

**Signature:**

```c
void ts_pack_reset();
```

###### ts_pack_default()

**Signature:**

```c
TsPackParser ts_pack_default();
```

---

#### TsPackPoint

A source position — row + column, zero-indexed.

| Field    | Type        | Default | Description                                       |
| -------- | ----------- | ------- | ------------------------------------------------- |
| `row`    | `uintptr_t` | —       | Zero-indexed row number.                          |
| `column` | `uintptr_t` | —       | Zero-indexed column number, in UTF-16 code units. |

##### Methods

###### ts_pack_from()

**Signature:**

```c
TsPackPoint ts_pack_from(TsPackPoint p);
```

---

#### TsPackProcessConfig

Configuration for the `process()` function.

Controls which analysis features are enabled and whether chunking is performed.

| Field            | Type          | Default | Description                                                         |
| ---------------- | ------------- | ------- | ------------------------------------------------------------------- |
| `language`       | `const char*` | —       | Language name (required).                                           |
| `structure`      | `bool`        | `true`  | Extract structural items (functions, classes, etc.). Default: true. |
| `imports`        | `bool`        | `true`  | Extract import statements. Default: true.                           |
| `exports`        | `bool`        | `true`  | Extract export statements. Default: true.                           |
| `comments`       | `bool`        | `false` | Extract comments. Default: false.                                   |
| `docstrings`     | `bool`        | `false` | Extract docstrings. Default: false.                                 |
| `symbols`        | `bool`        | `false` | Extract symbol definitions. Default: false.                         |
| `diagnostics`    | `bool`        | `false` | Include parse diagnostics. Default: false.                          |
| `chunk_max_size` | `uintptr_t*`  | `NULL`  | Maximum chunk size in bytes. `NULL` disables chunking.              |

##### Methods

###### ts_pack_default()

**Signature:**

```c
TsPackProcessConfig ts_pack_default();
```

###### ts_pack_with_chunking()

Enable chunking with the given maximum chunk size in bytes.

**Signature:**

```c
TsPackProcessConfig ts_pack_with_chunking(uintptr_t max_size);
```

###### ts_pack_all()

Enable all analysis features.

**Signature:**

```c
TsPackProcessConfig ts_pack_all();
```

###### ts_pack_minimal()

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
Fields are populated based on the `ProcessConfig` flags.

| Field         | Type                   | Default | Description                        |
| ------------- | ---------------------- | ------- | ---------------------------------- |
| `language`    | `const char*`          | —       | Language                           |
| `metrics`     | `TsPackFileMetrics`    | —       | Metrics (file metrics)             |
| `structure`   | `TsPackStructureItem*` | `NULL`  | Structure                          |
| `imports`     | `TsPackImportInfo*`    | `NULL`  | Imports                            |
| `exports`     | `TsPackExportInfo*`    | `NULL`  | Exports                            |
| `comments`    | `TsPackCommentInfo*`   | `NULL`  | Comments                           |
| `docstrings`  | `TsPackDocstringInfo*` | `NULL`  | Docstrings                         |
| `symbols`     | `TsPackSymbolInfo*`    | `NULL`  | Symbols                            |
| `diagnostics` | `TsPackDiagnostic*`    | `NULL`  | Diagnostics                        |
| `chunks`      | `TsPackCodeChunk*`     | `NULL`  | Text chunks for chunking/embedding |

---

#### TsPackSpan

Byte and line/column range in source code.

Represents both byte offsets (for slicing) and human-readable line/column
positions (for display and diagnostics).

| Field          | Type        | Default | Description  |
| -------------- | ----------- | ------- | ------------ |
| `start_byte`   | `uintptr_t` | —       | Start byte   |
| `end_byte`     | `uintptr_t` | —       | End byte     |
| `start_line`   | `uintptr_t` | —       | Start line   |
| `start_column` | `uintptr_t` | —       | Start column |
| `end_line`     | `uintptr_t` | —       | End line     |
| `end_column`   | `uintptr_t` | —       | End column   |

---

#### TsPackStructureItem

A structural item (function, class, struct, etc.) in source code.

| Field         | Type                   | Default                    | Description           |
| ------------- | ---------------------- | -------------------------- | --------------------- |
| `kind`        | `TsPackStructureKind`  | `TS_PACK_TS_PACK_FUNCTION` | Kind (structure kind) |
| `name`        | `const char**`         | `NULL`                     | The name              |
| `visibility`  | `const char**`         | `NULL`                     | Visibility            |
| `span`        | `TsPackSpan`           | —                          | Span (span)           |
| `children`    | `TsPackStructureItem*` | `NULL`                     | Children              |
| `decorators`  | `const char**`         | `NULL`                     | Decorators            |
| `doc_comment` | `const char**`         | `NULL`                     | Doc comment           |
| `signature`   | `const char**`         | `NULL`                     | Signature             |
| `body_span`   | `TsPackSpan*`          | `NULL`                     | Body span (span)      |

---

#### TsPackSymbolInfo

A symbol (variable, function, type, etc.) extracted from source code.

| Field             | Type               | Default                    | Description        |
| ----------------- | ------------------ | -------------------------- | ------------------ |
| `name`            | `const char*`      | —                          | The name           |
| `kind`            | `TsPackSymbolKind` | `TS_PACK_TS_PACK_VARIABLE` | Kind (symbol kind) |
| `span`            | `TsPackSpan`       | —                          | Span (span)        |
| `type_annotation` | `const char**`     | `NULL`                     | Type annotation    |
| `doc`             | `const char**`     | `NULL`                     | Doc                |

---

#### TsPackTree

A parsed syntax tree. Cheap to clone (refcount bump).

##### Methods

###### ts_pack_root_node()

Return the root `Node` of this tree.

**Signature:**

```c
TsPackNode ts_pack_root_node();
```

###### ts_pack_walk()

Return a `TreeCursor` positioned at the root.

**Signature:**

```c
TsPackTreeCursor ts_pack_walk();
```

---

#### TsPackTreeCursor

A cursor for traversing a `Tree`.

##### Methods

###### ts_pack_node()

Return the `Node` at the cursor's current position.

**Signature:**

```c
TsPackNode ts_pack_node();
```

###### ts_pack_goto_first_child()

Move the cursor to the first child of the current node.
Returns `true` if a child existed.

**Signature:**

```c
bool ts_pack_goto_first_child();
```

###### ts_pack_goto_parent()

Move the cursor to the parent of the current node.
Returns `true` if a parent existed.

**Signature:**

```c
bool ts_pack_goto_parent();
```

###### ts_pack_goto_next_sibling()

Move the cursor to the next sibling of the current node.
Returns `true` if a sibling existed.

**Signature:**

```c
bool ts_pack_goto_next_sibling();
```

###### ts_pack_field_name()

Return the field name for the current node, if any.

**Signature:**

```c
const char** ts_pack_field_name();
```

---

### Enums

#### TsPackStructureKind

The kind of structural item found in source code.

Categorizes top-level and nested declarations such as functions, classes,
structs, enums, traits, and more. Use `Other` for
language-specific constructs that do not fit a standard category.

| Value               | Description                        |
| ------------------- | ---------------------------------- |
| `TS_PACK_FUNCTION`  | Function                           |
| `TS_PACK_METHOD`    | Method                             |
| `TS_PACK_CLASS`     | Class                              |
| `TS_PACK_STRUCT`    | Struct                             |
| `TS_PACK_INTERFACE` | Interface                          |
| `TS_PACK_ENUM`      | Enum                               |
| `TS_PACK_MODULE`    | Module                             |
| `TS_PACK_TRAIT`     | Trait                              |
| `TS_PACK_IMPL`      | Impl                               |
| `TS_PACK_NAMESPACE` | Namespace                          |
| `TS_PACK_OTHER`     | Other — Fields: `0`: `const char*` |

---

#### TsPackCommentKind

The kind of a comment found in source code.

Distinguishes between single-line comments, block (multi-line) comments,
and documentation comments.

| Value           | Description |
| --------------- | ----------- |
| `TS_PACK_LINE`  | Line        |
| `TS_PACK_BLOCK` | Block       |
| `TS_PACK_DOC`   | Doc         |

---

#### TsPackDocstringFormat

The format of a docstring extracted from source code.

Identifies the docstring convention used, which varies by language
(e.g., Python triple-quoted strings, JSDoc, Rustdoc `///` comments).

| Value                         | Description                        |
| ----------------------------- | ---------------------------------- |
| `TS_PACK_PYTHON_TRIPLE_QUOTE` | Python triple quote                |
| `TS_PACK_JS_DOC`              | J s doc                            |
| `TS_PACK_RUSTDOC`             | Rustdoc                            |
| `TS_PACK_GO_DOC`              | Go doc                             |
| `TS_PACK_JAVA_DOC`            | Java doc                           |
| `TS_PACK_OTHER`               | Other — Fields: `0`: `const char*` |

---

#### TsPackExportKind

The kind of an export statement found in source code.

Covers named exports, default exports, and re-exports from other modules.

| Value               | Description |
| ------------------- | ----------- |
| `TS_PACK_NAMED`     | Named       |
| `TS_PACK_DEFAULT`   | Default     |
| `TS_PACK_RE_EXPORT` | Re export   |

---

#### TsPackSymbolKind

The kind of a symbol definition found in source code.

Categorizes symbol definitions such as variables, constants, functions,
classes, types, interfaces, enums, and modules.

| Value               | Description                        |
| ------------------- | ---------------------------------- |
| `TS_PACK_VARIABLE`  | Variable                           |
| `TS_PACK_CONSTANT`  | Constant                           |
| `TS_PACK_FUNCTION`  | Function                           |
| `TS_PACK_CLASS`     | Class                              |
| `TS_PACK_TYPE`      | Type                               |
| `TS_PACK_INTERFACE` | Interface                          |
| `TS_PACK_ENUM`      | Enum                               |
| `TS_PACK_MODULE`    | Module                             |
| `TS_PACK_OTHER`     | Other — Fields: `0`: `const char*` |

---

#### TsPackDiagnosticSeverity

Severity level of a diagnostic produced during parsing.

Used to classify parse errors, warnings, and informational messages
found in the syntax tree.

| Value             | Description |
| ----------------- | ----------- |
| `TS_PACK_ERROR`   | Error       |
| `TS_PACK_WARNING` | Warning     |
| `TS_PACK_INFO`    | Info        |

---

### Errors

#### TsPackError

Errors that can occur when using the tree-sitter language pack.

Covers language lookup failures, parse errors, query errors, and I/O issues.
Feature-gated variants are included when `config`, `download`, or related
features are enabled.

| Variant                         | Description                                       |
| ------------------------------- | ------------------------------------------------- |
| `TS_PACK_LANGUAGE_NOT_FOUND`    | Language '{0}' not found                          |
| `TS_PACK_DYNAMIC_LOAD`          | Dynamic library load error: {0}                   |
| `TS_PACK_NULL_LANGUAGE_POINTER` | Language function returned null pointer for '{0}' |
| `TS_PACK_PARSER_SETUP`          | Failed to set parser language: {0}                |
| `TS_PACK_LOCK_POISONED`         | Registry lock poisoned: {0}                       |
| `TS_PACK_CONFIG`                | Configuration error: {0}                          |
| `TS_PACK_PARSE_FAILED`          | Parse failed: parsing returned no tree            |
| `TS_PACK_QUERY_ERROR`           | Query error: {0}                                  |
| `TS_PACK_INVALID_RANGE`         | Invalid byte range: {0}                           |
| `TS_PACK_IO`                    | IO error: {0}                                     |

---
