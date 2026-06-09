---
title: "Zig API Reference"
---

## Zig API Reference <span class="version-badge">v1.9.0-rc.31</span>

### Functions

#### detectLanguageFromExtension()

Detect language name from a file extension (without leading dot).

Returns `null` for unrecognized extensions. The match is case-insensitive.

**Signature:**

```zig
pub fn detect_language_from_extension(ext: [:0]const u8) ?[:0]const u8
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `ext` | `[:0]const u8` | Yes | The ext |

**Returns:** `?[:0]const u8`

---

#### detectLanguageFromPath()

Detect language name from a file path.

Extracts the file extension and looks it up. Returns `null` if the
path has no extension or the extension is not recognized.

**Signature:**

```zig
pub fn detect_language_from_path(path: [:0]const u8) ?[:0]const u8
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `path` | `[:0]const u8` | Yes | Path to the file |

**Returns:** `?[:0]const u8`

---

#### detectLanguageFromContent()

Detect language name from file content using the shebang line (`#!`).

Inspects only the first line of `content`. If it begins with `#!`, the
interpreter name is extracted and mapped to a language name.

Handles common patterns:

- `#!/usr/bin/env python3` → `"python"`
- `#!/bin/bash` → `"bash"`
- `#!/usr/bin/env node` → `"javascript"`

The `-S` flag accepted by some `env` implementations is skipped automatically.
Version suffixes (e.g. `python3.11`, `ruby3.2`) are stripped before matching.

Returns `null` when content does not start with `#!`, the shebang is
malformed, or the interpreter is not recognised.

**Signature:**

```zig
pub fn detect_language_from_content(content: [:0]const u8) ?[:0]const u8
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `content` | `[:0]const u8` | Yes | The content to process |

**Returns:** `?[:0]const u8`

---

#### getHighlightsQuery()

Get the highlights query for a language, if bundled.

Returns the contents of `highlights.scm` as a static string, or `null`
if no highlights query is bundled for this language.

**Signature:**

```zig
pub fn get_highlights_query(language: [:0]const u8) ?[:0]const u8
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `language` | `[:0]const u8` | Yes | The language |

**Returns:** `?[:0]const u8`

---

#### getInjectionsQuery()

Get the injections query for a language, if bundled.

Returns the contents of `injections.scm` as a static string, or `null`
if no injections query is bundled for this language.

**Signature:**

```zig
pub fn get_injections_query(language: [:0]const u8) ?[:0]const u8
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `language` | `[:0]const u8` | Yes | The language |

**Returns:** `?[:0]const u8`

---

#### getLocalsQuery()

Get the locals query for a language, if bundled.

Returns the contents of `locals.scm` as a static string, or `null`
if no locals query is bundled for this language.

**Signature:**

```zig
pub fn get_locals_query(language: [:0]const u8) ?[:0]const u8
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `language` | `[:0]const u8` | Yes | The language |

**Returns:** `?[:0]const u8`

---

#### getTagsQuery()

Get the tags query for a language, if bundled.

Returns the contents of `tags.scm` as a static string, or `null`
if no tags query is bundled for this language.

**Signature:**

```zig
pub fn get_tags_query(language: [:0]const u8) ?[:0]const u8
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `language` | `[:0]const u8` | Yes | The language |

**Returns:** `?[:0]const u8`

---

#### getLanguage()

Get a tree-sitter `Language` by name using the global registry.

Resolves language aliases (e.g., `"shell"` maps to `"bash"`).
When the `download` feature is enabled (default), automatically downloads
the parser from GitHub releases if not found locally.

**Errors:**

Returns `Error.LanguageNotFound` if the language is not recognized,
or `Error.Download` if auto-download fails.

**Signature:**

```zig
pub fn get_language(name: [:0]const u8) Error!Language
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `name` | `[:0]const u8` | Yes | The name |

**Returns:** `Language`
**Errors:** Throws `Error`.

---

#### getParser()

Get a `Parser` pre-configured for the given language.

This is a convenience function that calls `get_language` and configures
a new parser in one step.

**Errors:**

Returns `Error.LanguageNotFound` if the language is not recognized, or
`Error.ParserSetup` if the language cannot be applied to the parser.

**Signature:**

```zig
pub fn get_parser(name: [:0]const u8) Error!Parser
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `name` | `[:0]const u8` | Yes | The name |

**Returns:** `Parser`
**Errors:** Throws `Error`.

---

#### detectLanguage()

Detect language name from a file path or extension.

This compatibility alias matches the pre-Alef Python binding API.

**Signature:**

```zig
pub fn detect_language(path: [:0]const u8) ?[:0]const u8
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `path` | `[:0]const u8` | Yes | Path to the file |

**Returns:** `?[:0]const u8`

---

#### availableLanguages()

List all available language names (sorted, deduplicated, includes aliases).

Returns names of both statically compiled and dynamically loadable languages,
plus any configured aliases.

**Signature:**

```zig
pub fn available_languages() []const [:0]const u8
```

**Returns:** `[]const [:0]const u8`

---

#### hasLanguage()

Check if a language is available by name or alias.

Returns `true` if the language can be loaded (statically compiled,
dynamically available, or a known alias for one of these).

**Signature:**

```zig
pub fn has_language(name: [:0]const u8) bool
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `name` | `[:0]const u8` | Yes | The name |

**Returns:** `bool`

---

#### languageCount()

Return the number of available languages.

Includes statically compiled languages, dynamically loadable languages,
and aliases.

**Signature:**

```zig
pub fn language_count() u64
```

**Returns:** `u64`

---

#### process()

Process source code and extract file intelligence using the global registry.

Parses the source with tree-sitter and extracts metrics, structure, imports,
exports, comments, docstrings, symbols, diagnostics, and/or chunks based on
the flags set in `ProcessConfig`.

**Errors:**

Returns an error if the language is not found or parsing fails.

**Signature:**

```zig
pub fn process(source: [:0]const u8, config: ProcessConfig) Error!ProcessResult
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `source` | `[:0]const u8` | Yes | The source |
| `config` | `ProcessConfig` | Yes | The configuration options |

**Returns:** `ProcessResult`
**Errors:** Throws `Error`.

---

#### init()

Initialize the language pack with the given configuration.

Applies any custom cache directory, then downloads all languages and groups
specified in the config. This is the recommended entry point when you want
to pre-warm the cache before use.

**Errors:**

Returns an error if configuration cannot be applied or if downloads fail.

**Signature:**

```zig
pub fn init(config: PackConfig) Error!void
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `config` | `PackConfig` | Yes | The configuration options |

**Returns:** `void`
**Errors:** Throws `Error`.

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

```zig
pub fn configure(config: PackConfig) Error!void
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `config` | `PackConfig` | Yes | The configuration options |

**Returns:** `void`
**Errors:** Throws `Error`.

---

#### download()

Download specific languages to the local cache.

Returns the number of requested languages available after the call. Already
compiled or cached languages are included in the count.

**Errors:**

Returns an error if any language is not available in the manifest or if
the download fails.

**Signature:**

```zig
pub fn download(names: []const [:0]const u8) Error!u64
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `names` | `[]const [:0]const u8` | Yes | The names |

**Returns:** `u64`
**Errors:** Throws `Error`.

---

#### downloadAll()

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

```zig
pub fn download_all() Error!u64
```

**Returns:** `u64`
**Errors:** Throws `Error`.

---

#### downloadGroup()

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

```zig
pub fn download_group(name: [:0]const u8) Error!u64
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `name` | `[:0]const u8` | Yes | The name |

**Returns:** `u64`
**Errors:** Throws `Error`.

---

#### manifestLanguages()

Return all language names available in the remote manifest (306).

Fetches (and caches) the remote manifest to discover the full list of
downloadable languages. Use `downloaded_languages` to list what is
already cached locally.

**Errors:**

Returns an error if the manifest cannot be fetched.

**Signature:**

```zig
pub fn manifest_languages() Error![]const [:0]const u8
```

**Returns:** `[]const [:0]const u8`
**Errors:** Throws `Error`.

---

#### downloadedLanguages()

Return languages that are already downloaded and cached locally.

Does not perform any network requests. Returns an empty list if the
cache directory does not exist or cannot be read.

**Signature:**

```zig
pub fn downloaded_languages() []const [:0]const u8
```

**Returns:** `[]const [:0]const u8`

---

#### cleanCache()

Delete all cached parser shared libraries.

Resets the cache registration so the next call to `get_language` or
a download function will re-register the (now empty) cache directory.

**Errors:**

Returns an error if the cache directory cannot be removed.

**Signature:**

```zig
pub fn clean_cache() Error!void
```

**Returns:** `void`
**Errors:** Throws `Error`.

---

#### cacheDir()

Return the effective cache directory path.

This is either the custom path set via `configure` / `init` or the
default: `~/.cache/tree-sitter-language-pack/v{version}/libs/`.

**Errors:**

Returns an error if the system cache directory cannot be determined.

**Signature:**

```zig
pub fn cache_dir() Error![:0]const u8
```

**Returns:** `[:0]const u8`
**Errors:** Throws `Error`.

---

### Types

#### ByteRange

A byte range — start (inclusive) to end (exclusive).

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `start` | `u64` | — | Inclusive start byte offset. |
| `end` | `u64` | — | Exclusive end byte offset. |

---

#### ChunkContext

Metadata for a single chunk of source code.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `language` | `[:0]const u8` | — | Language name used to parse this chunk. |
| `chunkIndex` | `u64` | — | Zero-indexed position of this chunk within the file's chunk list. |
| `totalChunks` | `u64` | — | Total number of chunks the file was split into. |
| `nodeTypes` | `[]const [:0]const u8` | `[]` | Tree-sitter node kinds that appear at the top level of this chunk. |
| `contextPath` | `[]const [:0]const u8` | `[]` | Hierarchical path of enclosing structural items (e.g., `["MyClass", "my_method"]`). |
| `symbolsDefined` | `[]const [:0]const u8` | `[]` | Names of symbols defined within this chunk. |
| `comments` | `[]const CommentInfo` | `[]` | Comments contained within this chunk. |
| `docstrings` | `[]const DocstringInfo` | `[]` | Docstrings contained within this chunk. |
| `hasErrorNodes` | `bool` | — | Whether this chunk contains any tree-sitter error nodes. |

---

#### CodeChunk

A chunk of source code with rich metadata.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `content` | `[:0]const u8` | — | The raw source text of this chunk. |
| `startByte` | `u64` | — | Inclusive start byte offset of this chunk in the original source. |
| `endByte` | `u64` | — | Exclusive end byte offset of this chunk in the original source. |
| `startLine` | `u64` | — | Zero-indexed start line of this chunk. |
| `endLine` | `u64` | — | Zero-indexed end line of this chunk. |
| `metadata` | `ChunkContext` | — | Contextual metadata about this chunk. |

---

#### CommentInfo

A comment extracted from source code.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `text` | `[:0]const u8` | — | The raw text content of the comment. |
| `kind` | `CommentKind` | `CommentKind.Line` | The kind of comment (line, block, or doc). |
| `span` | `Span` | — | Source span covering the comment. |
| `associatedNode` | `[:0]const u8?` | `null` | Name of the syntax node this comment is directly associated with. |

---

#### Diagnostic

A diagnostic (syntax error, missing node, etc.) from parsing.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `message` | `[:0]const u8` | — | Human-readable description of the diagnostic. |
| `severity` | `DiagnosticSeverity` | `DiagnosticSeverity.Error` | Severity of the diagnostic. |
| `span` | `Span` | — | Source span where the diagnostic was detected. |

---

#### DocSection

A section within a docstring (e.g., Args, Returns, Raises).

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `kind` | `[:0]const u8` | — | Section kind (e.g., `"args"`, `"returns"`, `"raises"`). |
| `name` | `[:0]const u8?` | `null` | Parameter or return value name, if applicable. |
| `description` | `[:0]const u8` | — | Description text for this section. |

---

#### DocstringInfo

A docstring extracted from source code.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `text` | `[:0]const u8` | — | The raw text of the docstring. |
| `format` | `DocstringFormat` | `DocstringFormat.PythonTripleQuote` | The docstring format (Python, JSDoc, Rustdoc, etc.). |
| `span` | `Span` | — | Source span covering the docstring. |
| `associatedItem` | `[:0]const u8?` | `null` | Name of the item this docstring documents. |
| `parsedSections` | `[]const DocSection` | `[]` | Parsed sections of the docstring (Args, Returns, Raises, etc.). |

---

#### DownloadManager

Manages downloading and caching of pre-built parser shared libraries.

### Methods

#### new()

Create a new download manager for the given version.

**Signature:**

```zig
pub fn new(version: [:0]const u8) Error!DownloadManager
```

#### installedLanguages()

List languages that are already downloaded and cached.

**Signature:**

```zig
pub fn installedLanguages(self: *const DownloadManager) []const [:0]const u8
```

#### downloadAllBestEffort()

Download the platform bundle and extract every library file it contains.

Unlike `Self.ensure_languages`, this does not check the manifest language list
against archive contents — it simply extracts all `.so`/`.dylib`/`.dll` files
from the bundle. Languages in the manifest that are missing from the archive
are silently ignored rather than returning an error.

Returns the number of library files extracted (including those already cached).

**Signature:**

```zig
pub fn downloadAllBestEffort(self: *const DownloadManager) Error!u64
```

#### cleanCache()

Remove all cached parser libraries.

Acquires the cross-process lock so `clean_cache` cannot race a concurrent
downloader (avoids Windows sharing-violation errors against an in-flight
bundle write). The `.download.lock` file itself is **not** removed — it is
permanent infrastructure; deleting it could allow a concurrent process that
already opened the file to continue holding a stale lock handle while a new
process opens a fresh inode, breaking the mutual-exclusion guarantee.

**Signature:**

```zig
pub fn cleanCache(self: *const DownloadManager) Error!void
```

---

#### ExportInfo

An export statement extracted from source code.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `name` | `[:0]const u8` | — | The exported name. |
| `kind` | `ExportKind` | `ExportKind.Named` | The kind of export (named, default, or re-export). |
| `span` | `Span` | — | Source span covering the export statement. |

---

#### FileMetrics

Aggregate metrics for a source file.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `totalLines` | `u64` | — | Total number of lines (including blank and comment lines). |
| `codeLines` | `u64` | — | Number of lines containing non-blank, non-comment source code. |
| `commentLines` | `u64` | — | Number of lines that are entirely comments. |
| `blankLines` | `u64` | — | Number of blank (whitespace-only) lines. |
| `totalBytes` | `u64` | — | Total byte length of the source file. |
| `nodeCount` | `u64` | — | Total number of nodes in the syntax tree. |
| `errorCount` | `u64` | — | Number of error nodes in the syntax tree (parse errors). |
| `maxDepth` | `u64` | — | Maximum nesting depth reached in the syntax tree. |

---

#### ImportInfo

An import statement extracted from source code.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `source` | `[:0]const u8` | — | The module or path being imported from. |
| `items` | `[]const [:0]const u8` | `[]` | Specific names imported from the source module. |
| `alias` | `[:0]const u8?` | `null` | Alias assigned to the import (e.g., `import numpy as np`). |
| `isWildcard` | `bool` | — | Whether this is a wildcard import (e.g., `import *` or `use foo.*`). |
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

```zig
pub fn new() LanguageRegistry
```

#### getLanguage()

Get a tree-sitter `Language` by name.

Resolves aliases (e.g., `"shell"` -> `"bash"`, `"makefile"` -> `"make"`),
then looks up the language in the static table. When the `dynamic-loading`
feature is enabled, falls back to loading a shared library on demand.

**Errors:**

Returns `Error.LanguageNotFound` if the name (after alias resolution)
does not match any known grammar.

**Signature:**

```zig
pub fn getLanguage(self: *const LanguageRegistry, name: [:0]const u8) Error!Language
```

#### availableLanguages()

List all available language names, sorted and deduplicated.

Includes statically compiled languages, dynamically loadable languages
(if the `dynamic-loading` feature is enabled), and all configured aliases.

**Signature:**

```zig
pub fn availableLanguages(self: *const LanguageRegistry) []const [:0]const u8
```

#### hasParser()

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

```zig
pub fn hasParser(self: *const LanguageRegistry, name: [:0]const u8) bool
```

#### hasLanguage()

Check whether a language is available by name or alias.

Returns `true` if the language can be loaded, either from the static
table or from a dynamic library on disk.

**Signature:**

```zig
pub fn hasLanguage(self: *const LanguageRegistry, name: [:0]const u8) bool
```

#### languageCount()

Return the total number of available languages (including aliases).

**Signature:**

```zig
pub fn languageCount(self: *const LanguageRegistry) u64
```

#### process()

Parse source code and extract file intelligence based on config in a single pass.

**Signature:**

```zig
pub fn process(self: *const LanguageRegistry, source: [:0]const u8, config: ProcessConfig) Error!ProcessResult
```

#### default()

**Signature:**

```zig
pub fn default() LanguageRegistry
```

---

#### Node

A single syntax node within a `Tree`.

Nodes hold a strong reference to their parent tree so they remain valid
regardless of how the tree is moved or stored at the FFI boundary.

### Methods

#### clone()

**Signature:**

```zig
pub fn clone(self: *const Node) Node
```

#### kind()

Return the node's kind name (e.g. `"function_definition"`).

**Signature:**

```zig
pub fn kind(self: *const Node) [:0]const u8
```

#### kindId()

Return the node's numeric kind ID.

Tree-sitter assigns a stable `u16` ID to every node kind in a grammar
(e.g. `"function_definition" → 42`). Comparing `kind_id()` is cheaper
than comparing the string `kind()` in tight AST loops.

**Signature:**

```zig
pub fn kindId(self: *const Node) u16
```

#### startByte()

Return the inclusive start byte offset of this node.

**Signature:**

```zig
pub fn startByte(self: *const Node) u64
```

#### endByte()

Return the exclusive end byte offset of this node.

**Signature:**

```zig
pub fn endByte(self: *const Node) u64
```

#### byteRange()

Return the node's byte range as a `ByteRange`.

Callers should slice their own source bytes — this is a zero-copy
text accessor.

**Signature:**

```zig
pub fn byteRange(self: *const Node) ByteRange
```

#### startPosition()

Return the start `Point` (row, column).

**Signature:**

```zig
pub fn startPosition(self: *const Node) Point
```

#### endPosition()

Return the end `Point` (row, column).

**Signature:**

```zig
pub fn endPosition(self: *const Node) Point
```

#### isNamed()

True when this node is named (not punctuation/whitespace).

**Signature:**

```zig
pub fn isNamed(self: *const Node) bool
```

#### isError()

True when this is an error node.

**Signature:**

```zig
pub fn isError(self: *const Node) bool
```

#### isMissing()

True when this is a missing-token node.

**Signature:**

```zig
pub fn isMissing(self: *const Node) bool
```

#### isExtra()

True when this is an "extra" node (e.g. a comment).

**Signature:**

```zig
pub fn isExtra(self: *const Node) bool
```

#### hasError()

True when this node or any descendant is an error.

**Signature:**

```zig
pub fn hasError(self: *const Node) bool
```

#### parent()

Return this node's parent, if any.

**Signature:**

```zig
pub fn parent(self: *const Node) ?Node
```

#### child()

Return the i-th child of this node, if any.

**Signature:**

```zig
pub fn child(self: *const Node, index: u32) ?Node
```

#### childCount()

Total number of children (including unnamed).

**Signature:**

```zig
pub fn childCount(self: *const Node) u64
```

#### namedChild()

Return the i-th named child of this node, if any.

**Signature:**

```zig
pub fn namedChild(self: *const Node, index: u32) ?Node
```

#### namedChildCount()

Number of named children of this node.

**Signature:**

```zig
pub fn namedChildCount(self: *const Node) u64
```

#### childByFieldName()

Look up a child by its grammar-defined field name.

**Signature:**

```zig
pub fn childByFieldName(self: *const Node, name: [:0]const u8) ?Node
```

#### toSexp()

Return the S-expression form of this node's subtree.

**Signature:**

```zig
pub fn toSexp(self: *const Node) [:0]const u8
```

#### walk()

Return a `TreeCursor` positioned at this node.

**Signature:**

```zig
pub fn walk(self: *const Node) TreeCursor
```

---

#### PackConfig

Configuration for the tree-sitter language pack.

Controls cache directory and which languages to pre-download.
Can be loaded from a TOML file, constructed programmatically,
or passed as a dict/object from language bindings.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `cacheDir` | `[:0]const u8?` | `null` | Override default cache directory. Default: `~/.cache/tree-sitter-language-pack/v{version}/libs/` |
| `languages` | `[]const [:0]const u8?` | `[]` | Languages to pre-download on init. Each entry is a language name (e.g. `"python"`, `"rust"`). |
| `groups` | `[]const [:0]const u8?` | `[]` | Language groups to pre-download (e.g. `"web"`, `"systems"`, `"scripting"`). |

---

#### Parser

A tree-sitter parser configured for one language at a time.

### Methods

#### new()

Construct a new parser with no language set.

Call `Parser.set_language` before parsing.

**Signature:**

```zig
pub fn new() Parser
```

#### setLanguage()

Configure the parser to use the language identified by name (e.g. `"python"`).

Resolves the language through the global registry — auto-downloading
if necessary, when the `download` feature is enabled.

**Errors:**

Returns `Error.LanguageNotFound` if the language is not recognized,
or `Error.ParserSetup` if the language ABI is incompatible.

**Signature:**

```zig
pub fn setLanguage(self: *const Parser, name: [:0]const u8) Error!void
```

#### parse()

Parse a UTF-8 source string. Returns `null` if parsing was cancelled
or no language is set.

**Signature:**

```zig
pub fn parse(self: *const Parser, source: [:0]const u8) ?Tree
```

#### parseBytes()

Parse a raw byte slice. Returns `null` if parsing was cancelled or
no language is set.

**Signature:**

```zig
pub fn parseBytes(self: *const Parser, source: []const u8) ?Tree
```

#### reset()

Reset internal state. The next call to `parse` will
not be incremental.

**Signature:**

```zig
pub fn reset(self: *const Parser) void
```

#### default()

**Signature:**

```zig
pub fn default() Parser
```

---

#### Point

A source position — row + column, zero-indexed.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `row` | `u64` | — | Zero-indexed row number. |
| `column` | `u64` | — | Zero-indexed column number, in UTF-16 code units. |

---

#### ProcessConfig

Configuration for the `process()` function.

Controls which analysis features are enabled and whether chunking is performed.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `language` | `[:0]const u8` | — | Language name (required). |
| `structure` | `bool` | `true` | Extract structural items (functions, classes, etc.). Default: true. |
| `imports` | `bool` | `true` | Extract import statements. Default: true. |
| `exports` | `bool` | `true` | Extract export statements. Default: true. |
| `comments` | `bool` | `false` | Extract comments. Default: false. |
| `docstrings` | `bool` | `false` | Extract docstrings. Default: false. |
| `symbols` | `bool` | `false` | Extract symbol definitions. Default: false. |
| `diagnostics` | `bool` | `false` | Include parse diagnostics. Default: false. |
| `chunkMaxSize` | `u64?` | `null` | Maximum chunk size in bytes. `null` disables chunking. |

### Methods

#### default()

**Signature:**

```zig
pub fn default() ProcessConfig
```

#### withChunking()

Enable chunking with the given maximum chunk size in bytes.

**Signature:**

```zig
pub fn withChunking(self: *const ProcessConfig, max_size: u64) ProcessConfig
```

#### all()

Enable all analysis features.

**Signature:**

```zig
pub fn all(self: *const ProcessConfig) ProcessConfig
```

#### minimal()

Disable all analysis features (only metrics computed).

**Signature:**

```zig
pub fn minimal(self: *const ProcessConfig) ProcessConfig
```

---

#### ProcessResult

Complete analysis result from processing a source file.

Contains metrics, structural analysis, imports/exports, comments,
docstrings, symbols, diagnostics, and optionally chunked code segments.
Fields are populated based on the `ProcessConfig` flags.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `language` | `[:0]const u8` | — | The language name used to parse the source file. |
| `metrics` | `FileMetrics` | — | File-level metrics (line counts, byte size, error count). |
| `structure` | `[]const StructureItem` | `[]` | Top-level structural items (functions, classes, etc.). |
| `imports` | `[]const ImportInfo` | `[]` | Import statements extracted from the source. |
| `exports` | `[]const ExportInfo` | `[]` | Export statements extracted from the source. |
| `comments` | `[]const CommentInfo` | `[]` | Comments extracted from the source. |
| `docstrings` | `[]const DocstringInfo` | `[]` | Docstrings extracted from the source. |
| `symbols` | `[]const SymbolInfo` | `[]` | Symbol definitions (variables, types, functions) extracted from the source. |
| `diagnostics` | `[]const Diagnostic` | `[]` | Parse diagnostics (syntax errors, missing nodes) from tree-sitter. |
| `chunks` | `[]const CodeChunk` | `[]` | Syntax-aware code chunks produced when chunking is enabled. |

---

#### Span

Byte and line/column range in source code.

Represents both byte offsets (for slicing) and human-readable line/column
positions (for display and diagnostics).

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `startByte` | `u64` | — | Inclusive start byte offset in the source. |
| `endByte` | `u64` | — | Exclusive end byte offset in the source. |
| `startLine` | `u64` | — | Zero-indexed line number of the span's start. |
| `startColumn` | `u64` | — | Zero-indexed column number of the span's start. |
| `endLine` | `u64` | — | Zero-indexed line number of the span's end. |
| `endColumn` | `u64` | — | Zero-indexed column number of the span's end. |

---

#### StructureItem

A structural item (function, class, struct, etc.) in source code.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `kind` | `StructureKind` | `StructureKind.Function` | The kind of structural item. |
| `name` | `[:0]const u8?` | `null` | The declared name of the item, if present. |
| `visibility` | `[:0]const u8?` | `null` | Visibility modifier (e.g., `"pub"`, `"public"`, `"private"`). |
| `span` | `Span` | — | Source span covering the entire item declaration. |
| `children` | `[]const StructureItem` | `[]` | Nested structural items (e.g., methods within a class). |
| `decorators` | `[]const [:0]const u8` | `[]` | Decorator or attribute names applied to the item. |
| `docComment` | `[:0]const u8?` | `null` | Documentation comment attached to the item, if any. |
| `signature` | `[:0]const u8?` | `null` | Full signature text of the item (e.g., function parameters and return type). |
| `bodySpan` | `Span?` | `null` | Source span covering only the body of the item, if distinct from the declaration. |

---

#### SymbolInfo

A symbol (variable, function, type, etc.) extracted from source code.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `name` | `[:0]const u8` | — | The name of the symbol. |
| `kind` | `SymbolKind` | `SymbolKind.Variable` | The kind of symbol (variable, function, class, etc.). |
| `span` | `Span` | — | Source span covering the symbol definition. |
| `typeAnnotation` | `[:0]const u8?` | `null` | Explicit type annotation, if present in the source. |
| `doc` | `[:0]const u8?` | `null` | Documentation comment associated with this symbol. |

---

#### Tree

A parsed syntax tree. Cheap to clone (refcount bump).

### Methods

#### rootNode()

Return the root `Node` of this tree.

**Signature:**

```zig
pub fn rootNode(self: *const Tree) Node
```

#### walk()

Return a `TreeCursor` positioned at the root.

**Signature:**

```zig
pub fn walk(self: *const Tree) TreeCursor
```

---

#### TreeCursor

A cursor for traversing a `Tree`.

### Methods

#### node()

Return the `Node` at the cursor's current position.

**Signature:**

```zig
pub fn node(self: *const TreeCursor) Node
```

#### gotoFirstChild()

Move the cursor to the first child of the current node.
Returns `true` if a child existed.

**Signature:**

```zig
pub fn gotoFirstChild(self: *const TreeCursor) bool
```

#### gotoParent()

Move the cursor to the parent of the current node.
Returns `true` if a parent existed.

**Signature:**

```zig
pub fn gotoParent(self: *const TreeCursor) bool
```

#### gotoNextSibling()

Move the cursor to the next sibling of the current node.
Returns `true` if a sibling existed.

**Signature:**

```zig
pub fn gotoNextSibling(self: *const TreeCursor) bool
```

#### fieldName()

Return the field name for the current node, if any.

**Signature:**

```zig
pub fn fieldName(self: *const TreeCursor) ?[:0]const u8
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
| `Function` | A free-standing or associated function. |
| `Method` | A method defined inside a class, struct, trait, or impl block. |
| `Class` | A class definition. |
| `Struct` | A struct definition. |
| `Interface` | An interface or protocol definition. |
| `Enum` | An enum definition. |
| `Module` | A module or package declaration. |
| `Trait` | A trait definition. |
| `Impl` | An impl block (Rust) or similar implementation block. |
| `Namespace` | A namespace declaration. |
| `Other` | A language-specific construct that does not fit any standard category. — Fields: `0`: `[:0]const u8` |

---

#### CommentKind

The kind of a comment found in source code.

Distinguishes between single-line comments, block (multi-line) comments,
and documentation comments.

| Value | Description |
|-------|-------------|
| `Line` | A single-line comment (e.g., `// ...` or `# ...`). |
| `Block` | A block or multi-line comment using slash-star delimiters. |
| `Doc` | A documentation comment such as `/// ...` or slash-double-star block. |

---

#### DocstringFormat

The format of a docstring extracted from source code.

Identifies the docstring convention used, which varies by language
(e.g., Python triple-quoted strings, JSDoc, Rustdoc `///` comments).

| Value | Description |
|-------|-------------|
| `PythonTripleQuote` | Python triple-quoted string docstring (`"""..."""`). |
| `JsDoc` | JavaScript/TypeScript JSDoc comment (`/** ... */`). |
| `Rustdoc` | Rust `///` or `//!` doc comment. |
| `GoDoc` | Go doc comment (a comment block immediately preceding a declaration). |
| `JavaDoc` | Java Javadoc comment (`/** ... */`). |
| `Other` | A language-specific docstring format not covered by the standard variants. — Fields: `0`: `[:0]const u8` |

---

#### ExportKind

The kind of an export statement found in source code.

Covers named exports, default exports, and re-exports from other modules.

| Value | Description |
|-------|-------------|
| `Named` | A named export (e.g., `export { foo }`). |
| `Default` | A default export (e.g., `export default foo`). |
| `ReExport` | A re-export from another module (e.g., `export { foo } from 'bar'`). |

---

#### SymbolKind

The kind of a symbol definition found in source code.

Categorizes symbol definitions such as variables, constants, functions,
classes, types, interfaces, enums, and modules.

| Value | Description |
|-------|-------------|
| `Variable` | A variable binding. |
| `Constant` | A constant (immutable binding). |
| `Function` | A function definition. |
| `Class` | A class definition. |
| `Type` | A type alias or typedef. |
| `Interface` | An interface definition. |
| `Enum` | An enum definition. |
| `Module` | A module declaration. |
| `Other` | A symbol kind not covered by the standard variants. — Fields: `0`: `[:0]const u8` |

---

#### DiagnosticSeverity

Severity level of a diagnostic produced during parsing.

Used to classify parse errors, warnings, and informational messages
found in the syntax tree.

| Value | Description |
|-------|-------------|
| `Error` | A parse error (e.g., an `ERROR` or `MISSING` node in the tree). |
| `Warning` | A warning-level diagnostic. |
| `Info` | An informational diagnostic. |

---

### Errors

#### Error

Errors that can occur when using the tree-sitter language pack.

Covers language lookup failures, parse errors, query errors, and I/O issues.
Feature-gated variants are included when `config`, `download`, or related
features are enabled.

| Variant | Description |
|---------|-------------|
| `LanguageNotFound` | The requested language name (or alias) was not found in the registry. |
| `DynamicLoad` | A dynamic shared library could not be loaded at runtime. |
| `NullLanguagePointer` | The tree-sitter language function returned a null pointer for the given language name. |
| `ParserSetup` | The language could not be applied to the parser (e.g., ABI version mismatch). |
| `LockPoisoned` | An internal `RwLock` or `Mutex` was poisoned by a previous panic. |
| `Config` | A configuration file or value was invalid or could not be applied. |
| `ParseFailed` | The tree-sitter parser returned no tree for the given source input. |
| `QueryError` | A tree-sitter query could not be compiled or executed. |
| `InvalidRange` | A byte range was invalid (e.g., end before start, or out of bounds). |
| `Download` | A parser download from GitHub releases failed. |
| `ChecksumMismatch` | The downloaded file's SHA-256 digest did not match the manifest's expected value. |
| `CacheLock` | The cross-process download cache lock file could not be acquired or created. |

---
