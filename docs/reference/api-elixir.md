---
title: "Elixir API Reference"
---

## Elixir API Reference <span class="version-badge">v1.10.4</span>

### Functions

#### detect_language_from_extension()

Detect language name from a file extension (without leading dot).

Returns `nil` for unrecognized extensions. The match is case-insensitive.

**Signature:**

```elixir
@spec detect_language_from_extension(ext) :: {:ok, term()} | {:error, term()}
def detect_language_from_extension(ext)
```

**Example:**

```elixir
{:ok, result} = detect_language_from_extension("value")
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `ext` | `String.t()` | Yes | The ext |

**Returns:** `String.t() | nil`

---

#### detect_language_from_path()

Detect language name from a file path.

Extracts the file extension and looks it up. Returns `nil` if the
path has no extension or the extension is not recognized.

**Signature:**

```elixir
@spec detect_language_from_path(path) :: {:ok, term()} | {:error, term()}
def detect_language_from_path(path)
```

**Example:**

```elixir
{:ok, result} = detect_language_from_path("value")
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `path` | `String.t()` | Yes | Path to the file |

**Returns:** `String.t() | nil`

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

```elixir
@spec detect_language_from_content(content) :: {:ok, term()} | {:error, term()}
def detect_language_from_content(content)
```

**Example:**

```elixir
{:ok, result} = detect_language_from_content("value")
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `content` | `String.t()` | Yes | The content to process |

**Returns:** `String.t() | nil`

---

#### get_highlights_query()

Get the highlights query for a language, if bundled.

Returns the contents of `highlights.scm` as a static string, or `nil`
if no highlights query is bundled for this language.

**Signature:**

```elixir
@spec get_highlights_query(language) :: {:ok, term()} | {:error, term()}
def get_highlights_query(language)
```

**Example:**

```elixir
{:ok, result} = get_highlights_query("value")
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `language` | `String.t()` | Yes | The language |

**Returns:** `String.t() | nil`

---

#### get_injections_query()

Get the injections query for a language, if bundled.

Returns the contents of `injections.scm` as a static string, or `nil`
if no injections query is bundled for this language.

**Signature:**

```elixir
@spec get_injections_query(language) :: {:ok, term()} | {:error, term()}
def get_injections_query(language)
```

**Example:**

```elixir
{:ok, result} = get_injections_query("value")
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `language` | `String.t()` | Yes | The language |

**Returns:** `String.t() | nil`

---

#### get_locals_query()

Get the locals query for a language, if bundled.

Returns the contents of `locals.scm` as a static string, or `nil`
if no locals query is bundled for this language.

**Signature:**

```elixir
@spec get_locals_query(language) :: {:ok, term()} | {:error, term()}
def get_locals_query(language)
```

**Example:**

```elixir
{:ok, result} = get_locals_query("value")
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `language` | `String.t()` | Yes | The language |

**Returns:** `String.t() | nil`

---

#### get_tags_query()

Get the tags query for a language, if bundled.

Returns the contents of `tags.scm` as a static string, or `nil`
if no tags query is bundled for this language.

**Signature:**

```elixir
@spec get_tags_query(language) :: {:ok, term()} | {:error, term()}
def get_tags_query(language)
```

**Example:**

```elixir
{:ok, result} = get_tags_query("value")
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `language` | `String.t()` | Yes | The language |

**Returns:** `String.t() | nil`

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

```elixir
@spec get_language(name) :: {:ok, term()} | {:error, term()}
def get_language(name)
```

**Example:**

```elixir
{:ok, result} = get_language("value")
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `name` | `String.t()` | Yes | The name |

**Returns:** `Language`

**Errors:** Returns `{:error, reason}`

---

#### get_parser()

Get a `Parser` pre-configured for the given language.

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

**Example:**

```elixir
{:ok, result} = get_parser("value")
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `name` | `String.t()` | Yes | The name |

**Returns:** `Parser`

**Errors:** Returns `{:error, reason}`

---

#### detect_language()

Detect language name from a file path or extension.

This compatibility alias matches the pre-Alef Python binding API.

**Signature:**

```elixir
@spec detect_language(path) :: {:ok, term()} | {:error, term()}
def detect_language(path)
```

**Example:**

```elixir
{:ok, result} = detect_language("value")
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `path` | `String.t()` | Yes | Path to the file |

**Returns:** `String.t() | nil`

---

#### available_languages()

List all available language names (sorted, deduplicated, includes aliases).

Returns names of both statically compiled and dynamically loadable languages,
plus any configured aliases.

**Signature:**

```elixir
@spec available_languages() :: {:ok, term()} | {:error, term()}
def available_languages()
```

**Example:**

```elixir
{:ok, result} = available_languages()
```

**Returns:** `list(String.t())`

---

#### has_language()

Check if a language is available by name or alias.

Returns `true` if the language can be loaded (statically compiled,
dynamically available, or a known alias for one of these).

**Signature:**

```elixir
@spec has_language(name) :: {:ok, term()} | {:error, term()}
def has_language(name)
```

**Example:**

```elixir
{:ok, result} = has_language("value")
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `name` | `String.t()` | Yes | The name |

**Returns:** `boolean()`

---

#### language_count()

Return the number of available languages.

Includes statically compiled languages, dynamically loadable languages,
and aliases.

**Signature:**

```elixir
@spec language_count() :: {:ok, term()} | {:error, term()}
def language_count()
```

**Example:**

```elixir
{:ok, result} = language_count()
```

**Returns:** `integer()`

---

#### process()

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

**Example:**

```elixir
{:ok, result} = process("value", %{{}})
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `source` | `String.t()` | Yes | The source |
| `config` | `ProcessConfig` | Yes | The configuration options |

**Returns:** `ProcessResult`

**Errors:** Returns `{:error, reason}`

---

#### init()

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

**Example:**

```elixir
:ok = init(%{{}})
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `config` | `PackConfig` | Yes | The configuration options |

**Returns:** No return value.

**Errors:** Returns `{:error, reason}`

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

```elixir
@spec configure(config) :: {:ok, term()} | {:error, term()}
def configure(config)
```

**Example:**

```elixir
:ok = configure(%{{}})
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `config` | `PackConfig` | Yes | The configuration options |

**Returns:** No return value.

**Errors:** Returns `{:error, reason}`

---

#### download()

Download specific languages to the local cache.

Returns the number of requested languages available after the call. Already
compiled or cached languages are included in the count.

**Errors:**

Returns an error if any language is not available in the manifest or if
the download fails.

**Signature:**

```elixir
@spec download(names) :: {:ok, term()} | {:error, term()}
def download(names)
```

**Example:**

```elixir
{:ok, result} = download([])
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `names` | `list(String.t())` | Yes | The names |

**Returns:** `integer()`

**Errors:** Returns `{:error, reason}`

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

```elixir
@spec download_all() :: {:ok, term()} | {:error, term()}
def download_all()
```

**Example:**

```elixir
{:ok, result} = download_all()
```

**Returns:** `integer()`

**Errors:** Returns `{:error, reason}`

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

```elixir
@spec download_group(name) :: {:ok, term()} | {:error, term()}
def download_group(name)
```

**Example:**

```elixir
{:ok, result} = download_group("value")
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `name` | `String.t()` | Yes | The name |

**Returns:** `integer()`

**Errors:** Returns `{:error, reason}`

---

#### manifest_languages()

Return all language names available in the remote manifest (306).

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

**Example:**

```elixir
{:ok, result} = manifest_languages()
```

**Returns:** `list(String.t())`

**Errors:** Returns `{:error, reason}`

---

#### downloaded_languages()

Return languages that are already downloaded and cached locally.

Does not perform any network requests. Returns an empty list if the
cache directory does not exist or cannot be read.

**Signature:**

```elixir
@spec downloaded_languages() :: {:ok, term()} | {:error, term()}
def downloaded_languages()
```

**Example:**

```elixir
{:ok, result} = downloaded_languages()
```

**Returns:** `list(String.t())`

---

#### clean_cache()

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

**Example:**

```elixir
:ok = clean_cache()
```

**Returns:** No return value.

**Errors:** Returns `{:error, reason}`

---

#### cache_dir()

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

**Example:**

```elixir
{:ok, result} = cache_dir()
```

**Returns:** `String.t()`

**Errors:** Returns `{:error, reason}`

---

### Types

#### ByteRange

A byte range — start (inclusive) to end (exclusive).

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `start` | `integer()` | — | Inclusive start byte offset. |
| `end` | `integer()` | — | Exclusive end byte offset. |

---

#### ChunkContext

Metadata for a single chunk of source code.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `language` | `String.t()` | — | Language name used to parse this chunk. |
| `chunk_index` | `integer()` | — | Zero-indexed position of this chunk within the file's chunk list. |
| `total_chunks` | `integer()` | — | Total number of chunks the file was split into. |
| `node_types` | `list(String.t())` | `\[\]` | Tree-sitter node kinds that appear at the top level of this chunk. |
| `context_path` | `list(String.t())` | `\[\]` | Hierarchical path of enclosing structural items (e.g., `\["MyClass", "my_method"\]`). |
| `symbols_defined` | `list(String.t())` | `\[\]` | Names of symbols defined within this chunk. |
| `comments` | `list(CommentInfo)` | `\[\]` | Comments contained within this chunk. |
| `docstrings` | `list(DocstringInfo)` | `\[\]` | Docstrings contained within this chunk. |
| `has_error_nodes` | `boolean()` | — | Whether this chunk contains any tree-sitter error nodes. |

---

#### CodeChunk

A chunk of source code with rich metadata.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `content` | `String.t()` | — | The raw source text of this chunk. |
| `start_byte` | `integer()` | — | Inclusive start byte offset of this chunk in the original source. |
| `end_byte` | `integer()` | — | Exclusive end byte offset of this chunk in the original source. |
| `start_line` | `integer()` | — | Zero-indexed start line of this chunk. |
| `end_line` | `integer()` | — | Zero-indexed end line of this chunk. |
| `metadata` | `ChunkContext` | — | Contextual metadata about this chunk. |

---

#### CommentInfo

A comment extracted from source code.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `text` | `String.t()` | — | The raw text content of the comment. |
| `kind` | `CommentKind` | `:line` | The kind of comment (line, block, or doc). |
| `span` | `Span` | — | Source span covering the comment. |
| `associated_node` | `String.t() \| nil` | `nil` | Name of the syntax node this comment is directly associated with. |

---

#### DataAttribute

An XML-style attribute attached to an `Element` node.

Populated only for `DataNodeKind.Element`; always empty for `KeyValue` and
`Sequence` nodes.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `name` | `String.t()` | — | Attribute name (e.g. `"class"`, `"href"`). |
| `value` | `String.t()` | — | Attribute value as a raw string (quotes stripped). |
| `span` | `Span` | — | Source span covering the entire `name="value"` attribute token. |

---

#### DataNode

A node in the hierarchical data tree produced by data-format extraction.

When `ProcessConfig.data_extraction` is
`true`, `ProcessResult.data` is populated with a root `DataNode` whose
`children` mirror the structure of the parsed file.

The `kind` field determines which other fields are meaningful:

| `kind`     | `key`                    | `value`       | `attributes` | `children` |
|------------|--------------------------|---------------|--------------|------------|
| `KeyValue` | key / mapping key / index | leaf value   | empty        | nested map |
| `Element`  | XML tag name             | text content  | XML attrs    | child elements |
| `Sequence` | positional index (`"0"`) | leaf value   | empty        | sub-items  |

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `kind` | `DataNodeKind` | `:key_value` | Whether this node is a key/value pair, XML element, or sequence item. |
| `key` | `String.t() \| nil` | `nil` | Key, attribute name, tag name, or positional index (`"0"`, `"1"`, …). `nil` at the document root. |
| `value` | `String.t() \| nil` | `nil` | Leaf scalar value, if any. `nil` for containers (objects, arrays, XML elements with child elements). |
| `attributes` | `list(DataAttribute)` | `\[\]` | Attributes on element-shape nodes (XML `STag` attributes). Empty for all other kinds. |
| `children` | `list(DataNode)` | `\[\]` | Children for nested containers and XML element bodies. |
| `span` | `Span` | — | Source span covering this node in the original source file. |

---

#### Diagnostic

A diagnostic (syntax error, missing node, etc.) from parsing.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `message` | `String.t()` | — | Human-readable description of the diagnostic. |
| `severity` | `DiagnosticSeverity` | `:error` | Severity of the diagnostic. |
| `span` | `Span` | — | Source span where the diagnostic was detected. |

---

#### DocSection

A section within a docstring (e.g., Args, Returns, Raises).

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `kind` | `String.t()` | — | Section kind (e.g., `"args"`, `"returns"`, `"raises"`). |
| `name` | `String.t() \| nil` | `nil` | Parameter or return value name, if applicable. |
| `description` | `String.t()` | — | Description text for this section. |

---

#### DocstringInfo

A docstring extracted from source code.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `text` | `String.t()` | — | The raw text of the docstring. |
| `format` | `DocstringFormat` | `:python_triple_quote` | The docstring format (Python, JSDoc, Rustdoc, etc.). |
| `span` | `Span` | — | Source span covering the docstring. |
| `associated_item` | `String.t() \| nil` | `nil` | Name of the item this docstring documents. |
| `parsed_sections` | `list(DocSection)` | `\[\]` | Parsed sections of the docstring (Args, Returns, Raises, etc.). |

---

#### DownloadManager

Manages downloading and caching of pre-built parser shared libraries.

##### Functions

###### new()

Create a new download manager for the given version.

**Signature:**

```elixir
def new(version)
```

**Example:**

```elixir
{:ok, result} = DownloadManager.new("value")
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `version` | `String.t()` | Yes | The version |

**Returns:** `DownloadManager`

**Errors:** Returns `{:error, reason}`

###### installed_languages()

List languages that are already downloaded and cached.

**Signature:**

```elixir
def installed_languages()
```

**Example:**

```elixir
{:ok, result} = instance.installed_languages()
```

**Returns:** `list(String.t())`

###### download_all_best_effort()

Download the platform bundle and extract every library file it contains.

Unlike `Self.ensure_languages`, this does not check the manifest language list
against archive contents — it simply extracts all `.so`/`.dylib`/`.dll` files
from the bundle. Languages in the manifest that are missing from the archive
are silently ignored rather than returning an error.

Returns the number of library files extracted (including those already cached).

**Signature:**

```elixir
def download_all_best_effort()
```

**Example:**

```elixir
{:ok, result} = instance.download_all_best_effort()
```

**Returns:** `integer()`

**Errors:** Returns `{:error, reason}`

###### clean_cache()

Remove all cached parser libraries.

Acquires the cross-process lock so `clean_cache` cannot race a concurrent
downloader (avoids Windows sharing-violation errors against an in-flight
bundle write). The `.download.lock` file itself is **not** removed — it is
permanent infrastructure; deleting it could allow a concurrent process that
already opened the file to continue holding a stale lock handle while a new
process opens a fresh inode, breaking the mutual-exclusion guarantee.

**Signature:**

```elixir
def clean_cache()
```

**Example:**

```elixir
:ok = instance.clean_cache()
```

**Returns:** No return value.

**Errors:** Returns `{:error, reason}`

---

#### ExportInfo

An export statement extracted from source code.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `name` | `String.t()` | — | The exported name. |
| `kind` | `ExportKind` | `:named` | The kind of export (named, default, or re-export). |
| `span` | `Span` | — | Source span covering the export statement. |

---

#### FileMetrics

Aggregate metrics for a source file.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `total_lines` | `integer()` | — | Total number of lines (including blank and comment lines). |
| `code_lines` | `integer()` | — | Number of lines containing non-blank, non-comment source code. |
| `comment_lines` | `integer()` | — | Number of lines that are entirely comments. |
| `blank_lines` | `integer()` | — | Number of blank (whitespace-only) lines. |
| `total_bytes` | `integer()` | — | Total byte length of the source file. |
| `node_count` | `integer()` | — | Total number of nodes in the syntax tree. |
| `error_count` | `integer()` | — | Number of error nodes in the syntax tree (parse errors). |
| `max_depth` | `integer()` | — | Maximum nesting depth reached in the syntax tree. |

---

#### ImportInfo

An import statement extracted from source code.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `source` | `String.t()` | — | The module or path being imported from. |
| `items` | `list(String.t())` | `\[\]` | Specific names imported from the source module. |
| `alias` | `String.t() \| nil` | `nil` | Alias assigned to the import (e.g., `import numpy as np`). |
| `is_wildcard` | `boolean()` | — | Whether this is a wildcard import (e.g., `import *` or `use foo.*`). |
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

##### Functions

###### new()

Create a new registry populated with all statically compiled languages.

When the `dynamic-loading` feature is enabled, the registry also knows
about dynamically loadable grammars and will load them on demand.

**Signature:**

```elixir
def new()
```

**Example:**

```elixir
{:ok, result} = LanguageRegistry.new()
```

**Returns:** `LanguageRegistry`

###### get_language()

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

**Example:**

```elixir
{:ok, result} = instance.get_language("value")
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `name` | `String.t()` | Yes | The name |

**Returns:** `Language`

**Errors:** Returns `{:error, reason}`

###### available_languages()

List all available language names, sorted and deduplicated.

Includes statically compiled languages, dynamically loadable languages
(if the `dynamic-loading` feature is enabled), and all configured aliases.

**Signature:**

```elixir
def available_languages()
```

**Example:**

```elixir
{:ok, result} = instance.available_languages()
```

**Returns:** `list(String.t())`

###### has_parser()

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

```elixir
def has_parser(name)
```

**Example:**

```elixir
{:ok, result} = instance.has_parser("value")
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `name` | `String.t()` | Yes | The name |

**Returns:** `boolean()`

###### has_language()

Check whether a language is available by name or alias.

Returns `true` if the language can be loaded, either from the static
table or from a dynamic library on disk.

**Signature:**

```elixir
def has_language(name)
```

**Example:**

```elixir
{:ok, result} = instance.has_language("value")
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `name` | `String.t()` | Yes | The name |

**Returns:** `boolean()`

###### language_count()

Return the total number of available languages (including aliases).

**Signature:**

```elixir
def language_count()
```

**Example:**

```elixir
{:ok, result} = instance.language_count()
```

**Returns:** `integer()`

###### process()

Parse source code and extract file intelligence based on config in a single pass.

**Signature:**

```elixir
def process(source, config)
```

**Example:**

```elixir
{:ok, result} = instance.process("value", %{{}})
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `source` | `String.t()` | Yes | The source |
| `config` | `ProcessConfig` | Yes | The configuration options |

**Returns:** `ProcessResult`

**Errors:** Returns `{:error, reason}`

###### default()

**Signature:**

```elixir
def default()
```

**Example:**

```elixir
{:ok, result} = LanguageRegistry.default()
```

**Returns:** `LanguageRegistry`

---

#### Node

A single syntax node within a `Tree`.

Nodes hold a strong reference to their parent tree so they remain valid
regardless of how the tree is moved or stored at the FFI boundary.

##### Functions

###### clone()

**Signature:**

```elixir
def clone()
```

**Example:**

```elixir
{:ok, result} = instance.clone()
```

**Returns:** `Node`

###### kind()

Return the node's kind name (e.g. `"function_definition"`).

**Signature:**

```elixir
def kind()
```

**Example:**

```elixir
{:ok, result} = instance.kind()
```

**Returns:** `String.t()`

###### kind_id()

Return the node's numeric kind ID.

Tree-sitter assigns a stable `u16` ID to every node kind in a grammar
(e.g. `"function_definition" → 42`). Comparing `kind_id()` is cheaper
than comparing the string `kind()` in tight AST loops.

**Signature:**

```elixir
def kind_id()
```

**Example:**

```elixir
{:ok, result} = instance.kind_id()
```

**Returns:** `integer()`

###### start_byte()

Return the inclusive start byte offset of this node.

**Signature:**

```elixir
def start_byte()
```

**Example:**

```elixir
{:ok, result} = instance.start_byte()
```

**Returns:** `integer()`

###### end_byte()

Return the exclusive end byte offset of this node.

**Signature:**

```elixir
def end_byte()
```

**Example:**

```elixir
{:ok, result} = instance.end_byte()
```

**Returns:** `integer()`

###### byte_range()

Return the node's byte range as a `ByteRange`.

Callers should slice their own source bytes — this is a zero-copy
text accessor.

**Signature:**

```elixir
def byte_range()
```

**Example:**

```elixir
{:ok, result} = instance.byte_range()
```

**Returns:** `ByteRange`

###### start_position()

Return the start `Point` (row, column).

**Signature:**

```elixir
def start_position()
```

**Example:**

```elixir
{:ok, result} = instance.start_position()
```

**Returns:** `Point`

###### end_position()

Return the end `Point` (row, column).

**Signature:**

```elixir
def end_position()
```

**Example:**

```elixir
{:ok, result} = instance.end_position()
```

**Returns:** `Point`

###### is_named()

True when this node is named (not punctuation/whitespace).

**Signature:**

```elixir
def is_named()
```

**Example:**

```elixir
{:ok, result} = instance.is_named()
```

**Returns:** `boolean()`

###### is_error()

True when this is an error node.

**Signature:**

```elixir
def is_error()
```

**Example:**

```elixir
{:ok, result} = instance.is_error()
```

**Returns:** `boolean()`

###### is_missing()

True when this is a missing-token node.

**Signature:**

```elixir
def is_missing()
```

**Example:**

```elixir
{:ok, result} = instance.is_missing()
```

**Returns:** `boolean()`

###### is_extra()

True when this is an "extra" node (e.g. a comment).

**Signature:**

```elixir
def is_extra()
```

**Example:**

```elixir
{:ok, result} = instance.is_extra()
```

**Returns:** `boolean()`

###### has_error()

True when this node or any descendant is an error.

**Signature:**

```elixir
def has_error()
```

**Example:**

```elixir
{:ok, result} = instance.has_error()
```

**Returns:** `boolean()`

###### parent()

Return this node's parent, if any.

**Signature:**

```elixir
def parent()
```

**Example:**

```elixir
{:ok, result} = instance.parent()
```

**Returns:** `Node | nil`

###### child()

Return the i-th child of this node, if any.

**Signature:**

```elixir
def child(index)
```

**Example:**

```elixir
{:ok, result} = instance.child(42)
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `index` | `integer()` | Yes | The index |

**Returns:** `Node | nil`

###### child_count()

Total number of children (including unnamed).

**Signature:**

```elixir
def child_count()
```

**Example:**

```elixir
{:ok, result} = instance.child_count()
```

**Returns:** `integer()`

###### named_child()

Return the i-th named child of this node, if any.

**Signature:**

```elixir
def named_child(index)
```

**Example:**

```elixir
{:ok, result} = instance.named_child(42)
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `index` | `integer()` | Yes | The index |

**Returns:** `Node | nil`

###### named_child_count()

Number of named children of this node.

**Signature:**

```elixir
def named_child_count()
```

**Example:**

```elixir
{:ok, result} = instance.named_child_count()
```

**Returns:** `integer()`

###### child_by_field_name()

Look up a child by its grammar-defined field name.

**Signature:**

```elixir
def child_by_field_name(name)
```

**Example:**

```elixir
{:ok, result} = instance.child_by_field_name("value")
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `name` | `String.t()` | Yes | The name |

**Returns:** `Node | nil`

###### to_sexp()

Return the S-expression form of this node's subtree.

**Signature:**

```elixir
def to_sexp()
```

**Example:**

```elixir
{:ok, result} = instance.to_sexp()
```

**Returns:** `String.t()`

###### walk()

Return a `TreeCursor` positioned at this node.

**Signature:**

```elixir
def walk()
```

**Example:**

```elixir
{:ok, result} = instance.walk()
```

**Returns:** `TreeCursor`

---

#### PackConfig

Configuration for the tree-sitter language pack.

Controls cache directory and which languages to pre-download.
Can be loaded from a TOML file, constructed programmatically,
or passed as a dict/object from language bindings.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `cache_dir` | `String.t() \| nil` | `nil` | Override default cache directory. Default: `~/.cache/tree-sitter-language-pack/v{version}/libs/` |
| `languages` | `list(String.t()) \| nil` | `\[\]` | Languages to pre-download on init. Each entry is a language name (e.g. `"python"`, `"rust"`). |
| `groups` | `list(String.t()) \| nil` | `\[\]` | Language groups to pre-download (e.g. `"web"`, `"systems"`, `"scripting"`). |

---

#### Parser

A tree-sitter parser configured for one language at a time.

##### Functions

###### new()

Construct a new parser with no language set.

Call `Parser.set_language` before parsing.

**Signature:**

```elixir
def new()
```

**Example:**

```elixir
{:ok, result} = Parser.new()
```

**Returns:** `Parser`

###### set_language()

Configure the parser to use the language identified by name (e.g. `"python"`).

Resolves the language through the global registry — auto-downloading
if necessary, when the `download` feature is enabled.

**Errors:**

Returns `Error.LanguageNotFound` if the language is not recognized,
or `Error.ParserSetup` if the language ABI is incompatible.

**Signature:**

```elixir
def set_language(name)
```

**Example:**

```elixir
:ok = instance.set_language("value")
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `name` | `String.t()` | Yes | The name |

**Returns:** No return value.

**Errors:** Returns `{:error, reason}`

###### parse()

Parse a UTF-8 source string. Returns `nil` if parsing was cancelled
or no language is set.

**Signature:**

```elixir
def parse(source)
```

**Example:**

```elixir
{:ok, result} = instance.parse("value")
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `source` | `String.t()` | Yes | The source |

**Returns:** `Tree | nil`

###### parse_bytes()

Parse a raw byte slice. Returns `nil` if parsing was cancelled or
no language is set.

**Signature:**

```elixir
def parse_bytes(source)
```

**Example:**

```elixir
{:ok, result} = instance.parse_bytes(<<100, 97, 116, 97>>)
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `source` | `binary()` | Yes | The source |

**Returns:** `Tree | nil`

###### reset()

Reset internal state. The next call to `parse` will
not be incremental.

**Signature:**

```elixir
def reset()
```

**Example:**

```elixir
:ok = instance.reset()
```

**Returns:** No return value.

###### default()

**Signature:**

```elixir
def default()
```

**Example:**

```elixir
{:ok, result} = Parser.default()
```

**Returns:** `Parser`

---

#### Point

A source position — row + column, zero-indexed.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `row` | `integer()` | — | Zero-indexed row number. |
| `column` | `integer()` | — | Zero-indexed column number, in UTF-16 code units. |

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
| `chunk_max_size` | `integer() \| nil` | `nil` | Maximum chunk size in bytes. `nil` disables chunking. |
| `data_extraction` | `boolean()` | `false` | Extract hierarchical key/value data tree from data-format files. Default: false. When `true`, `ProcessResult.data` is populated with a `DataNode` tree for supported languages: JSON, YAML, TOML, `.properties`, HCL/HOCON, INI, editorconfig, KDL, CUE, CSV, PSV, PO, nginx config, Caddy config, XML, and DTD. For languages outside this set the field is left as `nil`. |

##### Functions

###### default()

**Signature:**

```elixir
def default()
```

**Example:**

```elixir
{:ok, result} = ProcessConfig.default()
```

**Returns:** `ProcessConfig`

###### with_chunking()

Enable chunking with the given maximum chunk size in bytes.

**Signature:**

```elixir
def with_chunking(max_size)
```

**Example:**

```elixir
{:ok, result} = instance.with_chunking(42)
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `max_size` | `integer()` | Yes | The max size |

**Returns:** `ProcessConfig`

###### all()

Enable all analysis features.

**Signature:**

```elixir
def all()
```

**Example:**

```elixir
{:ok, result} = instance.all()
```

**Returns:** `ProcessConfig`

###### minimal()

Disable all analysis features (only metrics computed).

**Signature:**

```elixir
def minimal()
```

**Example:**

```elixir
{:ok, result} = instance.minimal()
```

**Returns:** `ProcessConfig`

###### with_data_extraction()

Enable or disable hierarchical data extraction for data-format files.

When `true`, `ProcessResult.data` is
populated with a key/value tree for supported data-format languages.

**Signature:**

```elixir
def with_data_extraction(enabled)
```

**Example:**

```elixir
{:ok, result} = instance.with_data_extraction(true)
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `enabled` | `boolean()` | Yes | The enabled |

**Returns:** `ProcessConfig`

---

#### ProcessResult

Complete analysis result from processing a source file.

Contains metrics, structural analysis, imports/exports, comments,
docstrings, symbols, diagnostics, and optionally chunked code segments.
Fields are populated based on the `ProcessConfig` flags.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `language` | `String.t()` | — | The language name used to parse the source file. |
| `metrics` | `FileMetrics` | — | File-level metrics (line counts, byte size, error count). |
| `structure` | `list(StructureItem)` | `\[\]` | Top-level structural items (functions, classes, etc.). |
| `imports` | `list(ImportInfo)` | `\[\]` | Import statements extracted from the source. |
| `exports` | `list(ExportInfo)` | `\[\]` | Export statements extracted from the source. |
| `comments` | `list(CommentInfo)` | `\[\]` | Comments extracted from the source. |
| `docstrings` | `list(DocstringInfo)` | `\[\]` | Docstrings extracted from the source. |
| `symbols` | `list(SymbolInfo)` | `\[\]` | Symbol definitions (variables, types, functions) extracted from the source. |
| `diagnostics` | `list(Diagnostic)` | `\[\]` | Parse diagnostics (syntax errors, missing nodes) from tree-sitter. |
| `chunks` | `list(CodeChunk)` | `\[\]` | Syntax-aware code chunks produced when chunking is enabled. |
| `data` | `DataNode \| nil` | `nil` | Hierarchical data tree extracted when `config.data_extraction` is `true`. Populated for supported data-format languages (JSON, YAML, TOML, properties, HCL, INI, XML, CSV, and more). `nil` when `data_extraction` is `false` (the default) or when the language is not a recognised data format. See `DataNode` for the shape of the returned tree. |

---

#### Span

Byte and line/column range in source code.

Represents both byte offsets (for slicing) and human-readable line/column
positions (for display and diagnostics).

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `start_byte` | `integer()` | — | Inclusive start byte offset in the source. |
| `end_byte` | `integer()` | — | Exclusive end byte offset in the source. |
| `start_line` | `integer()` | — | Zero-indexed line number of the span's start. |
| `start_column` | `integer()` | — | Zero-indexed column number of the span's start. |
| `end_line` | `integer()` | — | Zero-indexed line number of the span's end. |
| `end_column` | `integer()` | — | Zero-indexed column number of the span's end. |

---

#### StructureItem

A structural item (function, class, struct, etc.) in source code.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `kind` | `StructureKind` | `:function` | The kind of structural item. |
| `name` | `String.t() \| nil` | `nil` | The declared name of the item, if present. |
| `visibility` | `String.t() \| nil` | `nil` | Visibility modifier (e.g., `"pub"`, `"public"`, `"private"`). |
| `span` | `Span` | — | Source span covering the entire item declaration. |
| `children` | `list(StructureItem)` | `\[\]` | Nested structural items (e.g., methods within a class). |
| `decorators` | `list(String.t())` | `\[\]` | Decorator or attribute names applied to the item. |
| `doc_comment` | `String.t() \| nil` | `nil` | Documentation comment attached to the item, if any. |
| `signature` | `String.t() \| nil` | `nil` | Full signature text of the item (e.g., function parameters and return type). |
| `body_span` | `Span \| nil` | `nil` | Source span covering only the body of the item, if distinct from the declaration. |

---

#### SymbolInfo

A symbol (variable, function, type, etc.) extracted from source code.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `name` | `String.t()` | — | The name of the symbol. |
| `kind` | `SymbolKind` | `:variable` | The kind of symbol (variable, function, class, etc.). |
| `span` | `Span` | — | Source span covering the symbol definition. |
| `type_annotation` | `String.t() \| nil` | `nil` | Explicit type annotation, if present in the source. |
| `doc` | `String.t() \| nil` | `nil` | Documentation comment associated with this symbol. |

---

#### Tree

A parsed syntax tree. Cheap to clone (refcount bump).

##### Functions

###### root_node()

Return the root `Node` of this tree.

**Signature:**

```elixir
def root_node()
```

**Example:**

```elixir
{:ok, result} = instance.root_node()
```

**Returns:** `Node`

###### walk()

Return a `TreeCursor` positioned at the root.

**Signature:**

```elixir
def walk()
```

**Example:**

```elixir
{:ok, result} = instance.walk()
```

**Returns:** `TreeCursor`

---

#### TreeCursor

A cursor for traversing a `Tree`.

##### Functions

###### node()

Return the `Node` at the cursor's current position.

**Signature:**

```elixir
def node()
```

**Example:**

```elixir
{:ok, result} = instance.node()
```

**Returns:** `Node`

###### goto_first_child()

Move the cursor to the first child of the current node.
Returns `true` if a child existed.

**Signature:**

```elixir
def goto_first_child()
```

**Example:**

```elixir
{:ok, result} = instance.goto_first_child()
```

**Returns:** `boolean()`

###### goto_parent()

Move the cursor to the parent of the current node.
Returns `true` if a parent existed.

**Signature:**

```elixir
def goto_parent()
```

**Example:**

```elixir
{:ok, result} = instance.goto_parent()
```

**Returns:** `boolean()`

###### goto_next_sibling()

Move the cursor to the next sibling of the current node.
Returns `true` if a sibling existed.

**Signature:**

```elixir
def goto_next_sibling()
```

**Example:**

```elixir
{:ok, result} = instance.goto_next_sibling()
```

**Returns:** `boolean()`

###### field_name()

Return the field name for the current node, if any.

**Signature:**

```elixir
def field_name()
```

**Example:**

```elixir
{:ok, result} = instance.field_name()
```

**Returns:** `String.t() | nil`

---

### Enums

#### DataNodeKind

The kind of a data node extracted from a data-format file.

Classifies each node in the hierarchical `DataNode` tree returned when
`data_extraction` is enabled on `ProcessConfig`.

### Wire format (public JSON contract)

Unit variants serialize as a bare string (`"KeyValue"`). DO NOT add
`#[serde(tag = "...")]` or rename variants — every language binding has a
hand-written deserializer matching this exact shape, and any change breaks
all bindings' `process()` tests simultaneously.
Covered by `tests/wire_format.rs`.

| Value | Description |
|-------|-------------|
| `key_value` | A key/value pair or mapping (json/toml/properties/yaml/hcl/cue/kdl pair, or a wrapper "object"/"mapping" container). |
| `element` | An XML element with a tag name in `key` and attributes in `attributes`. |
| `sequence` | A positional sequence item (JSON array element, YAML block sequence item, CSV/PSV row or cell). |

---

#### StructureKind

The kind of structural item found in source code.

Categorizes top-level and nested declarations such as functions, classes,
structs, enums, traits, and more. Use `Other` for
language-specific constructs that do not fit a standard category.

### Wire format (public JSON contract)

Unit variants serialize as a bare string (`"Function"`); the `Other`
variant serializes as a single-keyed object (`{"Other": "macro"}`). DO
NOT add `#[serde(tag = "...")]` or rename variants — every language
binding has a hand-written deserializer matching this exact shape, and
any change breaks all bindings' `process()` tests simultaneously.
Covered by `tests/wire_format.rs`.

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
| `other` | A language-specific construct that does not fit any standard category. — Fields: `0`: `String.t()` |

---

#### CommentKind

The kind of a comment found in source code.

Distinguishes between single-line comments, block (multi-line) comments,
and documentation comments.

| Value | Description |
|-------|-------------|
| `line` | A single-line comment (e.g., `// ...` or `# ...`). |
| `block` | A block or multi-line comment using slash-star delimiters. |
| `doc` | A documentation comment such as `/// ...` or slash-double-star block. |

---

#### DocstringFormat

The format of a docstring extracted from source code.

Identifies the docstring convention used, which varies by language
(e.g., Python triple-quoted strings, JSDoc, Rustdoc `///` comments).

### Wire format (public JSON contract)

Unit variants serialize as a bare string (`"JSDoc"`); the `Other`
variant serializes as a single-keyed object (`{"Other": "rst"}`). DO
NOT add `#[serde(tag = "...")]`. Covered by `tests/wire_format.rs`.

| Value | Description |
|-------|-------------|
| `python_triple_quote` | Python triple-quoted string docstring (`"""..."""`). |
| `js_doc` | JavaScript/TypeScript JSDoc block comment (opens with two stars, closes with star-slash). |
| `rustdoc` | Rust `///` or `//!` doc comment. |
| `go_doc` | Go doc comment (a comment block immediately preceding a declaration). |
| `java_doc` | Java Javadoc block comment (opens with two stars, closes with star-slash). |
| `other` | A language-specific docstring format not covered by the standard variants. — Fields: `0`: `String.t()` |

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

### Wire format (public JSON contract)

Unit variants serialize as a bare string (`"Function"`); the `Other`
variant serializes as a single-keyed object (`{"Other": "macro"}`). DO
NOT add `#[serde(tag = "...")]`. Covered by `tests/wire_format.rs`.

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
| `other` | A symbol kind not covered by the standard variants. — Fields: `0`: `String.t()` |

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
