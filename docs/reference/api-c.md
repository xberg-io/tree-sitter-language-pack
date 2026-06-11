---
title: "C API Reference"
---

## C API Reference <span class="version-badge">v1.9.0-rc.33</span>

### Functions

#### ts_pack_detect_language_from_extension()

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

#### ts_pack_detect_language_from_path()

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

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `content` | `const char*` | Yes | The content to process |

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

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `language` | `const char*` | Yes | The language |

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

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `language` | `const char*` | Yes | The language |

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

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `language` | `const char*` | Yes | The language |

**Returns:** `const char**`

---

#### ts_pack_get_tags_query()

Get the tags query for a language, if bundled.

Returns the contents of `tags.scm` as a static string, or `NULL`
if no tags query is bundled for this language.

**Signature:**

```c
const char** ts_pack_get_tags_query(const char* language);
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `language` | `const char*` | Yes | The language |

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

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `name` | `const char*` | Yes | The name |

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

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `name` | `const char*` | Yes | The name |

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

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `path` | `const char*` | Yes | Path to the file |

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

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `name` | `const char*` | Yes | The name |

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

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `source` | `const char*` | Yes | The source |
| `config` | `TsPackProcessConfig` | Yes | The configuration options |

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

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `config` | `TsPackPackConfig` | Yes | The configuration options |

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

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `config` | `TsPackPackConfig` | Yes | The configuration options |

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

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `names` | `const char**` | Yes | The names |

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
`download()`. Already-cached languages are skipped.

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

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `name` | `const char*` | Yes | The name |

**Returns:** `uintptr_t`
**Errors:** Returns `NULL` on error.

---

#### ts_pack_manifest_languages()

Return all language names available in the remote manifest (306).

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

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `start` | `uintptr_t` | — | Inclusive start byte offset. |
| `end` | `uintptr_t` | — | Exclusive end byte offset. |

---

#### TsPackChunkContext

Metadata for a single chunk of source code.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `language` | `const char*` | — | Language name used to parse this chunk. |
| `chunk_index` | `uintptr_t` | — | Zero-indexed position of this chunk within the file's chunk list. |
| `total_chunks` | `uintptr_t` | — | Total number of chunks the file was split into. |
| `node_types` | `const char**` | `NULL` | Tree-sitter node kinds that appear at the top level of this chunk. |
| `context_path` | `const char**` | `NULL` | Hierarchical path of enclosing structural items (e.g., `["MyClass", "my_method"]`). |
| `symbols_defined` | `const char**` | `NULL` | Names of symbols defined within this chunk. |
| `comments` | `TsPackCommentInfo*` | `NULL` | Comments contained within this chunk. |
| `docstrings` | `TsPackDocstringInfo*` | `NULL` | Docstrings contained within this chunk. |
| `has_error_nodes` | `bool` | — | Whether this chunk contains any tree-sitter error nodes. |

---

#### TsPackCodeChunk

A chunk of source code with rich metadata.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `content` | `const char*` | — | The raw source text of this chunk. |
| `start_byte` | `uintptr_t` | — | Inclusive start byte offset of this chunk in the original source. |
| `end_byte` | `uintptr_t` | — | Exclusive end byte offset of this chunk in the original source. |
| `start_line` | `uintptr_t` | — | Zero-indexed start line of this chunk. |
| `end_line` | `uintptr_t` | — | Zero-indexed end line of this chunk. |
| `metadata` | `TsPackChunkContext` | — | Contextual metadata about this chunk. |

---

#### TsPackCommentInfo

A comment extracted from source code.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `text` | `const char*` | — | The raw text content of the comment. |
| `kind` | `TsPackCommentKind` | `TS_PACK_TS_PACK_LINE` | The kind of comment (line, block, or doc). |
| `span` | `TsPackSpan` | — | Source span covering the comment. |
| `associated_node` | `const char**` | `NULL` | Name of the syntax node this comment is directly associated with. |

---

#### TsPackDiagnostic

A diagnostic (syntax error, missing node, etc.) from parsing.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `message` | `const char*` | — | Human-readable description of the diagnostic. |
| `severity` | `TsPackDiagnosticSeverity` | `TS_PACK_TS_PACK_ERROR` | Severity of the diagnostic. |
| `span` | `TsPackSpan` | — | Source span where the diagnostic was detected. |

---

#### TsPackDocSection

A section within a docstring (e.g., Args, Returns, Raises).

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `kind` | `const char*` | — | Section kind (e.g., `"args"`, `"returns"`, `"raises"`). |
| `name` | `const char**` | `NULL` | Parameter or return value name, if applicable. |
| `description` | `const char*` | — | Description text for this section. |

---

#### TsPackDocstringInfo

A docstring extracted from source code.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `text` | `const char*` | — | The raw text of the docstring. |
| `format` | `TsPackDocstringFormat` | `TS_PACK_TS_PACK_PYTHON_TRIPLE_QUOTE` | The docstring format (Python, JSDoc, Rustdoc, etc.). |
| `span` | `TsPackSpan` | — | Source span covering the docstring. |
| `associated_item` | `const char**` | `NULL` | Name of the item this docstring documents. |
| `parsed_sections` | `TsPackDocSection*` | `NULL` | Parsed sections of the docstring (Args, Returns, Raises, etc.). |

---

#### TsPackDownloadManager

Manages downloading and caching of pre-built parser shared libraries.

### Methods

#### ts_pack_new()

Create a new download manager for the given version.

**Signature:**

```c
TsPackDownloadManager ts_pack_new(const char* version);
```

#### ts_pack_installed_languages()

List languages that are already downloaded and cached.

**Signature:**

```c
const char** ts_pack_installed_languages();
```

#### ts_pack_download_all_best_effort()

Download the platform bundle and extract every library file it contains.

Unlike `Self.ensure_languages`, this does not check the manifest language list
against archive contents — it simply extracts all `.so`/`.dylib`/`.dll` files
from the bundle. Languages in the manifest that are missing from the archive
are silently ignored rather than returning an error.

Returns the number of library files extracted (including those already cached).

**Signature:**

```c
uintptr_t ts_pack_download_all_best_effort();
```

#### ts_pack_clean_cache()

Remove all cached parser libraries.

Acquires the cross-process lock so `clean_cache` cannot race a concurrent
downloader (avoids Windows sharing-violation errors against an in-flight
bundle write). The `.download.lock` file itself is **not** removed — it is
permanent infrastructure; deleting it could allow a concurrent process that
already opened the file to continue holding a stale lock handle while a new
process opens a fresh inode, breaking the mutual-exclusion guarantee.

**Signature:**

```c
void ts_pack_clean_cache();
```

---

#### TsPackExportInfo

An export statement extracted from source code.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `name` | `const char*` | — | The exported name. |
| `kind` | `TsPackExportKind` | `TS_PACK_TS_PACK_NAMED` | The kind of export (named, default, or re-export). |
| `span` | `TsPackSpan` | — | Source span covering the export statement. |

---

#### TsPackFileMetrics

Aggregate metrics for a source file.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `total_lines` | `uintptr_t` | — | Total number of lines (including blank and comment lines). |
| `code_lines` | `uintptr_t` | — | Number of lines containing non-blank, non-comment source code. |
| `comment_lines` | `uintptr_t` | — | Number of lines that are entirely comments. |
| `blank_lines` | `uintptr_t` | — | Number of blank (whitespace-only) lines. |
| `total_bytes` | `uintptr_t` | — | Total byte length of the source file. |
| `node_count` | `uintptr_t` | — | Total number of nodes in the syntax tree. |
| `error_count` | `uintptr_t` | — | Number of error nodes in the syntax tree (parse errors). |
| `max_depth` | `uintptr_t` | — | Maximum nesting depth reached in the syntax tree. |

---

#### TsPackImportInfo

An import statement extracted from source code.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `source` | `const char*` | — | The module or path being imported from. |
| `items` | `const char**` | `NULL` | Specific names imported from the source module. |
| `alias` | `const char**` | `NULL` | Alias assigned to the import (e.g., `import numpy as np`). |
| `is_wildcard` | `bool` | — | Whether this is a wildcard import (e.g., `import *` or `use foo.*`). |
| `span` | `TsPackSpan` | — | Source span covering the import statement. |

---

#### TsPackLanguage

---

#### TsPackLanguageRegistry

Thread-safe registry of tree-sitter language parsers.

Manages both statically compiled and dynamically loaded language grammars.
Use `LanguageRegistry.new()` for the default registry, or access the
global instance via the module-level convenience functions
(`get_language`, `available_languages`, etc.).

### Methods

#### ts_pack_new()

Create a new registry populated with all statically compiled languages.

When the `dynamic-loading` feature is enabled, the registry also knows
about dynamically loadable grammars and will load them on demand.

**Signature:**

```c
TsPackLanguageRegistry ts_pack_new();
```

#### ts_pack_get_language()

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

#### ts_pack_available_languages()

List all available language names, sorted and deduplicated.

Includes statically compiled languages, dynamically loadable languages
(if the `dynamic-loading` feature is enabled), and all configured aliases.

**Signature:**

```c
const char** ts_pack_available_languages();
```

#### ts_pack_has_parser()

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

```c
bool ts_pack_has_parser(const char* name);
```

#### ts_pack_has_language()

Check whether a language is available by name or alias.

Returns `true` if the language can be loaded, either from the static
table or from a dynamic library on disk.

**Signature:**

```c
bool ts_pack_has_language(const char* name);
```

#### ts_pack_language_count()

Return the total number of available languages (including aliases).

**Signature:**

```c
uintptr_t ts_pack_language_count();
```

#### ts_pack_process()

Parse source code and extract file intelligence based on config in a single pass.

**Signature:**

```c
TsPackProcessResult ts_pack_process(const char* source, TsPackProcessConfig config);
```

#### ts_pack_default()

**Signature:**

```c
TsPackLanguageRegistry ts_pack_default();
```

---

#### TsPackNode

A single syntax node within a `Tree`.

Nodes hold a strong reference to their parent tree so they remain valid
regardless of how the tree is moved or stored at the FFI boundary.

### Methods

#### ts_pack_clone()

**Signature:**

```c
TsPackNode ts_pack_clone();
```

#### ts_pack_kind()

Return the node's kind name (e.g. `"function_definition"`).

**Signature:**

```c
const char* ts_pack_kind();
```

#### ts_pack_kind_id()

Return the node's numeric kind ID.

Tree-sitter assigns a stable `u16` ID to every node kind in a grammar
(e.g. `"function_definition" → 42`). Comparing `kind_id()` is cheaper
than comparing the string `kind()` in tight AST loops.

**Signature:**

```c
uint16_t ts_pack_kind_id();
```

#### ts_pack_start_byte()

Return the inclusive start byte offset of this node.

**Signature:**

```c
uintptr_t ts_pack_start_byte();
```

#### ts_pack_end_byte()

Return the exclusive end byte offset of this node.

**Signature:**

```c
uintptr_t ts_pack_end_byte();
```

#### ts_pack_byte_range()

Return the node's byte range as a `ByteRange`.

Callers should slice their own source bytes — this is a zero-copy
text accessor.

**Signature:**

```c
TsPackByteRange ts_pack_byte_range();
```

#### ts_pack_start_position()

Return the start `Point` (row, column).

**Signature:**

```c
TsPackPoint ts_pack_start_position();
```

#### ts_pack_end_position()

Return the end `Point` (row, column).

**Signature:**

```c
TsPackPoint ts_pack_end_position();
```

#### ts_pack_is_named()

True when this node is named (not punctuation/whitespace).

**Signature:**

```c
bool ts_pack_is_named();
```

#### ts_pack_is_error()

True when this is an error node.

**Signature:**

```c
bool ts_pack_is_error();
```

#### ts_pack_is_missing()

True when this is a missing-token node.

**Signature:**

```c
bool ts_pack_is_missing();
```

#### ts_pack_is_extra()

True when this is an "extra" node (e.g. a comment).

**Signature:**

```c
bool ts_pack_is_extra();
```

#### ts_pack_has_error()

True when this node or any descendant is an error.

**Signature:**

```c
bool ts_pack_has_error();
```

#### ts_pack_parent()

Return this node's parent, if any.

**Signature:**

```c
TsPackNode* ts_pack_parent();
```

#### ts_pack_child()

Return the i-th child of this node, if any.

**Signature:**

```c
TsPackNode* ts_pack_child(uint32_t index);
```

#### ts_pack_child_count()

Total number of children (including unnamed).

**Signature:**

```c
uintptr_t ts_pack_child_count();
```

#### ts_pack_named_child()

Return the i-th named child of this node, if any.

**Signature:**

```c
TsPackNode* ts_pack_named_child(uint32_t index);
```

#### ts_pack_named_child_count()

Number of named children of this node.

**Signature:**

```c
uintptr_t ts_pack_named_child_count();
```

#### ts_pack_child_by_field_name()

Look up a child by its grammar-defined field name.

**Signature:**

```c
TsPackNode* ts_pack_child_by_field_name(const char* name);
```

#### ts_pack_to_sexp()

Return the S-expression form of this node's subtree.

**Signature:**

```c
const char* ts_pack_to_sexp();
```

#### ts_pack_walk()

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

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `cache_dir` | `const char**` | `NULL` | Override default cache directory. Default: `~/.cache/tree-sitter-language-pack/v{version}/libs/` |
| `languages` | `const char***` | `NULL` | Languages to pre-download on init. Each entry is a language name (e.g. `"python"`, `"rust"`). |
| `groups` | `const char***` | `NULL` | Language groups to pre-download (e.g. `"web"`, `"systems"`, `"scripting"`). |

---

#### TsPackParser

A tree-sitter parser configured for one language at a time.

### Methods

#### ts_pack_new()

Construct a new parser with no language set.

Call `Parser.set_language` before parsing.

**Signature:**

```c
TsPackParser ts_pack_new();
```

#### ts_pack_set_language()

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

#### ts_pack_parse()

Parse a UTF-8 source string. Returns `NULL` if parsing was cancelled
or no language is set.

**Signature:**

```c
TsPackTree* ts_pack_parse(const char* source);
```

#### ts_pack_parse_bytes()

Parse a raw byte slice. Returns `NULL` if parsing was cancelled or
no language is set.

**Signature:**

```c
TsPackTree* ts_pack_parse_bytes(const uint8_t* source);
```

#### ts_pack_reset()

Reset internal state. The next call to `parse` will
not be incremental.

**Signature:**

```c
void ts_pack_reset();
```

#### ts_pack_default()

**Signature:**

```c
TsPackParser ts_pack_default();
```

---

#### TsPackPoint

A source position — row + column, zero-indexed.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `row` | `uintptr_t` | — | Zero-indexed row number. |
| `column` | `uintptr_t` | — | Zero-indexed column number, in UTF-16 code units. |

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

### Methods

#### ts_pack_default()

**Signature:**

```c
TsPackProcessConfig ts_pack_default();
```

#### ts_pack_with_chunking()

Enable chunking with the given maximum chunk size in bytes.

**Signature:**

```c
TsPackProcessConfig ts_pack_with_chunking(uintptr_t max_size);
```

#### ts_pack_all()

Enable all analysis features.

**Signature:**

```c
TsPackProcessConfig ts_pack_all();
```

#### ts_pack_minimal()

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

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `language` | `const char*` | — | The language name used to parse the source file. |
| `metrics` | `TsPackFileMetrics` | — | File-level metrics (line counts, byte size, error count). |
| `structure` | `TsPackStructureItem*` | `NULL` | Top-level structural items (functions, classes, etc.). |
| `imports` | `TsPackImportInfo*` | `NULL` | Import statements extracted from the source. |
| `exports` | `TsPackExportInfo*` | `NULL` | Export statements extracted from the source. |
| `comments` | `TsPackCommentInfo*` | `NULL` | Comments extracted from the source. |
| `docstrings` | `TsPackDocstringInfo*` | `NULL` | Docstrings extracted from the source. |
| `symbols` | `TsPackSymbolInfo*` | `NULL` | Symbol definitions (variables, types, functions) extracted from the source. |
| `diagnostics` | `TsPackDiagnostic*` | `NULL` | Parse diagnostics (syntax errors, missing nodes) from tree-sitter. |
| `chunks` | `TsPackCodeChunk*` | `NULL` | Syntax-aware code chunks produced when chunking is enabled. |

---

#### TsPackSpan

Byte and line/column range in source code.

Represents both byte offsets (for slicing) and human-readable line/column
positions (for display and diagnostics).

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `start_byte` | `uintptr_t` | — | Inclusive start byte offset in the source. |
| `end_byte` | `uintptr_t` | — | Exclusive end byte offset in the source. |
| `start_line` | `uintptr_t` | — | Zero-indexed line number of the span's start. |
| `start_column` | `uintptr_t` | — | Zero-indexed column number of the span's start. |
| `end_line` | `uintptr_t` | — | Zero-indexed line number of the span's end. |
| `end_column` | `uintptr_t` | — | Zero-indexed column number of the span's end. |

---

#### TsPackStructureItem

A structural item (function, class, struct, etc.) in source code.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `kind` | `TsPackStructureKind` | `TS_PACK_TS_PACK_FUNCTION` | The kind of structural item. |
| `name` | `const char**` | `NULL` | The declared name of the item, if present. |
| `visibility` | `const char**` | `NULL` | Visibility modifier (e.g., `"pub"`, `"public"`, `"private"`). |
| `span` | `TsPackSpan` | — | Source span covering the entire item declaration. |
| `children` | `TsPackStructureItem*` | `NULL` | Nested structural items (e.g., methods within a class). |
| `decorators` | `const char**` | `NULL` | Decorator or attribute names applied to the item. |
| `doc_comment` | `const char**` | `NULL` | Documentation comment attached to the item, if any. |
| `signature` | `const char**` | `NULL` | Full signature text of the item (e.g., function parameters and return type). |
| `body_span` | `TsPackSpan*` | `NULL` | Source span covering only the body of the item, if distinct from the declaration. |

---

#### TsPackSymbolInfo

A symbol (variable, function, type, etc.) extracted from source code.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `name` | `const char*` | — | The name of the symbol. |
| `kind` | `TsPackSymbolKind` | `TS_PACK_TS_PACK_VARIABLE` | The kind of symbol (variable, function, class, etc.). |
| `span` | `TsPackSpan` | — | Source span covering the symbol definition. |
| `type_annotation` | `const char**` | `NULL` | Explicit type annotation, if present in the source. |
| `doc` | `const char**` | `NULL` | Documentation comment associated with this symbol. |

---

#### TsPackTree

A parsed syntax tree. Cheap to clone (refcount bump).

### Methods

#### ts_pack_root_node()

Return the root `Node` of this tree.

**Signature:**

```c
TsPackNode ts_pack_root_node();
```

#### ts_pack_walk()

Return a `TreeCursor` positioned at the root.

**Signature:**

```c
TsPackTreeCursor ts_pack_walk();
```

---

#### TsPackTreeCursor

A cursor for traversing a `Tree`.

### Methods

#### ts_pack_node()

Return the `Node` at the cursor's current position.

**Signature:**

```c
TsPackNode ts_pack_node();
```

#### ts_pack_goto_first_child()

Move the cursor to the first child of the current node.
Returns `true` if a child existed.

**Signature:**

```c
bool ts_pack_goto_first_child();
```

#### ts_pack_goto_parent()

Move the cursor to the parent of the current node.
Returns `true` if a parent existed.

**Signature:**

```c
bool ts_pack_goto_parent();
```

#### ts_pack_goto_next_sibling()

Move the cursor to the next sibling of the current node.
Returns `true` if a sibling existed.

**Signature:**

```c
bool ts_pack_goto_next_sibling();
```

#### ts_pack_field_name()

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

| Value | Description |
|-------|-------------|
| `TS_PACK_FUNCTION` | A free-standing or associated function. |
| `TS_PACK_METHOD` | A method defined inside a class, struct, trait, or impl block. |
| `TS_PACK_CLASS` | A class definition. |
| `TS_PACK_STRUCT` | A struct definition. |
| `TS_PACK_INTERFACE` | An interface or protocol definition. |
| `TS_PACK_ENUM` | An enum definition. |
| `TS_PACK_MODULE` | A module or package declaration. |
| `TS_PACK_TRAIT` | A trait definition. |
| `TS_PACK_IMPL` | An impl block (Rust) or similar implementation block. |
| `TS_PACK_NAMESPACE` | A namespace declaration. |
| `TS_PACK_OTHER` | A language-specific construct that does not fit any standard category. — Fields: `0`: `const char*` |

---

#### TsPackCommentKind

The kind of a comment found in source code.

Distinguishes between single-line comments, block (multi-line) comments,
and documentation comments.

| Value | Description |
|-------|-------------|
| `TS_PACK_LINE` | A single-line comment (e.g., `// ...` or `# ...`). |
| `TS_PACK_BLOCK` | A block or multi-line comment using slash-star delimiters. |
| `TS_PACK_DOC` | A documentation comment such as `/// ...` or slash-double-star block. |

---

#### TsPackDocstringFormat

The format of a docstring extracted from source code.

Identifies the docstring convention used, which varies by language
(e.g., Python triple-quoted strings, JSDoc, Rustdoc `///` comments).

| Value | Description |
|-------|-------------|
| `TS_PACK_PYTHON_TRIPLE_QUOTE` | Python triple-quoted string docstring (`"""..."""`). |
| `TS_PACK_JS_DOC` | JavaScript/TypeScript JSDoc comment (`/** ... */`). |
| `TS_PACK_RUSTDOC` | Rust `///` or `//!` doc comment. |
| `TS_PACK_GO_DOC` | Go doc comment (a comment block immediately preceding a declaration). |
| `TS_PACK_JAVA_DOC` | Java Javadoc comment (`/** ... */`). |
| `TS_PACK_OTHER` | A language-specific docstring format not covered by the standard variants. — Fields: `0`: `const char*` |

---

#### TsPackExportKind

The kind of an export statement found in source code.

Covers named exports, default exports, and re-exports from other modules.

| Value | Description |
|-------|-------------|
| `TS_PACK_NAMED` | A named export (e.g., `export { foo }`). |
| `TS_PACK_DEFAULT` | A default export (e.g., `export default foo`). |
| `TS_PACK_RE_EXPORT` | A re-export from another module (e.g., `export { foo } from 'bar'`). |

---

#### TsPackSymbolKind

The kind of a symbol definition found in source code.

Categorizes symbol definitions such as variables, constants, functions,
classes, types, interfaces, enums, and modules.

| Value | Description |
|-------|-------------|
| `TS_PACK_VARIABLE` | A variable binding. |
| `TS_PACK_CONSTANT` | A constant (immutable binding). |
| `TS_PACK_FUNCTION` | A function definition. |
| `TS_PACK_CLASS` | A class definition. |
| `TS_PACK_TYPE` | A type alias or typedef. |
| `TS_PACK_INTERFACE` | An interface definition. |
| `TS_PACK_ENUM` | An enum definition. |
| `TS_PACK_MODULE` | A module declaration. |
| `TS_PACK_OTHER` | A symbol kind not covered by the standard variants. — Fields: `0`: `const char*` |

---

#### TsPackDiagnosticSeverity

Severity level of a diagnostic produced during parsing.

Used to classify parse errors, warnings, and informational messages
found in the syntax tree.

| Value | Description |
|-------|-------------|
| `TS_PACK_ERROR` | A parse error (e.g., an `ERROR` or `MISSING` node in the tree). |
| `TS_PACK_WARNING` | A warning-level diagnostic. |
| `TS_PACK_INFO` | An informational diagnostic. |

---

### Errors

#### TsPackError

Errors that can occur when using the tree-sitter language pack.

Covers language lookup failures, parse errors, query errors, and I/O issues.
Feature-gated variants are included when `config`, `download`, or related
features are enabled.

| Variant | Description |
|---------|-------------|
| `TS_PACK_LANGUAGE_NOT_FOUND` | The requested language name (or alias) was not found in the registry. |
| `TS_PACK_DYNAMIC_LOAD` | A dynamic shared library could not be loaded at runtime. |
| `TS_PACK_NULL_LANGUAGE_POINTER` | The tree-sitter language function returned a null pointer for the given language name. |
| `TS_PACK_PARSER_SETUP` | The language could not be applied to the parser (e.g., ABI version mismatch). |
| `TS_PACK_LOCK_POISONED` | An internal `RwLock` or `Mutex` was poisoned by a previous panic. |
| `TS_PACK_CONFIG` | A configuration file or value was invalid or could not be applied. |
| `TS_PACK_PARSE_FAILED` | The tree-sitter parser returned no tree for the given source input. |
| `TS_PACK_QUERY_ERROR` | A tree-sitter query could not be compiled or executed. |
| `TS_PACK_INVALID_RANGE` | A byte range was invalid (e.g., end before start, or out of bounds). |
| `TS_PACK_DOWNLOAD` | A parser download from GitHub releases failed. |
| `TS_PACK_CHECKSUM_MISMATCH` | The downloaded file's SHA-256 digest did not match the manifest's expected value. |
| `TS_PACK_CACHE_LOCK` | The cross-process download cache lock file could not be acquired or created. |

---
