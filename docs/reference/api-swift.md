---
title: "Swift API Reference"
---
## Swift API Reference <span class="version-badge">v1.8.1</span>

### Functions

#### detectLanguageFromExtension()

Detect language name from a file extension (without leading dot).

Returns `null` for unrecognized extensions. The match is case-insensitive.

**Signature:**

```swift
// Phase 1: swift backend signature generation
```
**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `ext` | `String` | Yes | The ext |

**Returns:** `String?`

---

#### detectLanguageFromPath()

Detect language name from a file path.

Extracts the file extension and looks it up. Returns `null` if the
path has no extension or the extension is not recognized.

**Signature:**

```swift
// Phase 1: swift backend signature generation
```
**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `path` | `String` | Yes | Path to the file |

**Returns:** `String?`

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

```swift
// Phase 1: swift backend signature generation
```
**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `content` | `String` | Yes | The content to process |

**Returns:** `String?`

---

#### getHighlightsQuery()

Get the highlights query for a language, if bundled.

Returns the contents of `highlights.scm` as a static string, or `null`
if no highlights query is bundled for this language.

**Signature:**

```swift
// Phase 1: swift backend signature generation
```
**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `language` | `String` | Yes | The language |

**Returns:** `String?`

---

#### getInjectionsQuery()

Get the injections query for a language, if bundled.

Returns the contents of `injections.scm` as a static string, or `null`
if no injections query is bundled for this language.

**Signature:**

```swift
// Phase 1: swift backend signature generation
```
**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `language` | `String` | Yes | The language |

**Returns:** `String?`

---

#### getLocalsQuery()

Get the locals query for a language, if bundled.

Returns the contents of `locals.scm` as a static string, or `null`
if no locals query is bundled for this language.

**Signature:**

```swift
// Phase 1: swift backend signature generation
```
**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `language` | `String` | Yes | The language |

**Returns:** `String?`

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

```swift
// Phase 1: swift backend signature generation
```
**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `name` | `String` | Yes | The name |

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

```swift
// Phase 1: swift backend signature generation
```
**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `name` | `String` | Yes | The name |

**Returns:** `Parser`
**Errors:** Throws `Error`.

---

#### detectLanguage()

Detect language name from a file path or extension.

This compatibility alias matches the pre-Alef Python binding API.

**Signature:**

```swift
// Phase 1: swift backend signature generation
```
**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `path` | `String` | Yes | Path to the file |

**Returns:** `String?`

---

#### availableLanguages()

List all available language names (sorted, deduplicated, includes aliases).

Returns names of both statically compiled and dynamically loadable languages,
plus any configured aliases.

**Signature:**

```swift
// Phase 1: swift backend signature generation
```
**Returns:** `[String]`

---

#### hasLanguage()

Check if a language is available by name or alias.

Returns `true` if the language can be loaded (statically compiled,
dynamically available, or a known alias for one of these).

**Signature:**

```swift
// Phase 1: swift backend signature generation
```
**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `name` | `String` | Yes | The name |

**Returns:** `Bool`

---

#### languageCount()

Return the number of available languages.

Includes statically compiled languages, dynamically loadable languages,
and aliases.

**Signature:**

```swift
// Phase 1: swift backend signature generation
```
**Returns:** `UInt64`

---

#### process()

Process source code and extract file intelligence using the global registry.

Parses the source with tree-sitter and extracts metrics, structure, imports,
exports, comments, docstrings, symbols, diagnostics, and/or chunks based on
the flags set in `ProcessConfig`.

**Errors:**

Returns an error if the language is not found or parsing fails.

**Signature:**

```swift
// Phase 1: swift backend signature generation
```
**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `source` | `String` | Yes | The source |
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

```swift
// Phase 1: swift backend signature generation
```
**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `config` | `PackConfig` | Yes | The configuration options |

**Returns:** `Void`
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

```swift
// Phase 1: swift backend signature generation
```
**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `config` | `PackConfig` | Yes | The configuration options |

**Returns:** `Void`
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

```swift
// Phase 1: swift backend signature generation
```
**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `names` | `[String]` | Yes | The names |

**Returns:** `UInt64`
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

```swift
// Phase 1: swift backend signature generation
```
**Returns:** `UInt64`
**Errors:** Throws `Error`.

---

#### manifestLanguages()

Return all language names available in the remote manifest (305).

Fetches (and caches) the remote manifest to discover the full list of
downloadable languages. Use `downloaded_languages` to list what is
already cached locally.

**Errors:**

Returns an error if the manifest cannot be fetched.

**Signature:**

```swift
// Phase 1: swift backend signature generation
```
**Returns:** `[String]`
**Errors:** Throws `Error`.

---

#### downloadedLanguages()

Return languages that are already downloaded and cached locally.

Does not perform any network requests. Returns an empty list if the
cache directory does not exist or cannot be read.

**Signature:**

```swift
// Phase 1: swift backend signature generation
```
**Returns:** `[String]`

---

#### cleanCache()

Delete all cached parser shared libraries.

Resets the cache registration so the next call to `get_language` or
a download function will re-register the (now empty) cache directory.

**Errors:**

Returns an error if the cache directory cannot be removed.

**Signature:**

```swift
// Phase 1: swift backend signature generation
```
**Returns:** `Void`
**Errors:** Throws `Error`.

---

#### cacheDir()

Return the effective cache directory path.

This is either the custom path set via `configure` / `init` or the
default: `~/.cache/tree-sitter-language-pack/v{version}/libs/`.

**Errors:**

Returns an error if the system cache directory cannot be determined.

**Signature:**

```swift
// Phase 1: swift backend signature generation
```
**Returns:** `String`
**Errors:** Throws `Error`.

---

### Types

#### ByteRange

A byte range — start (inclusive) to end (exclusive).

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `start` | `UInt64` | — | Inclusive start byte offset. |
| `end` | `UInt64` | — | Exclusive end byte offset. |


---

#### ChunkContext

Metadata for a single chunk of source code.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `language` | `String` | — | Language |
| `chunkIndex` | `UInt64` | — | Chunk index |
| `totalChunks` | `UInt64` | — | Total chunks |
| `nodeTypes` | `[String]` | `[]` | Node types |
| `contextPath` | `[String]` | `[]` | Context path |
| `symbolsDefined` | `[String]` | `[]` | Symbols defined |
| `comments` | `[CommentInfo]` | `[]` | Comments |
| `docstrings` | `[DocstringInfo]` | `[]` | Docstrings |
| `hasErrorNodes` | `Bool` | — | Whether error nodes |


---

#### CodeChunk

A chunk of source code with rich metadata.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `content` | `String` | — | The extracted text content |
| `startByte` | `UInt64` | — | Start byte |
| `endByte` | `UInt64` | — | End byte |
| `startLine` | `UInt64` | — | Start line |
| `endLine` | `UInt64` | — | End line |
| `metadata` | `ChunkContext` | — | Document metadata |


---

#### CommentInfo

A comment extracted from source code.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `text` | `String` | — | Text |
| `kind` | `CommentKind` | `CommentKind.Line` | Kind (comment kind) |
| `span` | `Span` | — | Span (span) |
| `associatedNode` | `String?` | `null` | Associated node |


---

#### Diagnostic

A diagnostic (syntax error, missing node, etc.) from parsing.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `message` | `String` | — | Message |
| `severity` | `DiagnosticSeverity` | `DiagnosticSeverity.Error` | Severity (diagnostic severity) |
| `span` | `Span` | — | Span (span) |


---

#### DocSection

A section within a docstring (e.g., Args, Returns, Raises).

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `kind` | `String` | — | Kind |
| `name` | `String?` | `null` | The name |
| `description` | `String` | — | Human-readable description |


---

#### DocstringInfo

A docstring extracted from source code.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `text` | `String` | — | Text |
| `format` | `DocstringFormat` | `DocstringFormat.PythonTripleQuote` | Format (docstring format) |
| `span` | `Span` | — | Span (span) |
| `associatedItem` | `String?` | `null` | Associated item |
| `parsedSections` | `[DocSection]` | `[]` | Parsed sections |


---

#### DownloadManager

Manages downloading and caching of pre-built parser shared libraries.

##### Methods

###### new()

Create a new download manager for the given version.

**Signature:**

```swift
// Phase 1: swift backend method signature generation
```
###### withCacheDir()

Create a download manager with a custom cache directory.

**Signature:**

```swift
// Phase 1: swift backend method signature generation
```
###### defaultCacheDir()

Default cache directory: `~/.cache/tree-sitter-language-pack/v{version}/libs/`

**Signature:**

```swift
// Phase 1: swift backend method signature generation
```
###### cacheDir()

Return the path to the libs cache directory.

**Signature:**

```swift
// Phase 1: swift backend method signature generation
```
###### installedLanguages()

List languages that are already downloaded and cached.

**Signature:**

```swift
// Phase 1: swift backend method signature generation
```
###### ensureLanguages()

Ensure the specified languages are available in the cache.
Downloads the platform bundle if any requested languages are missing.

**Signature:**

```swift
// Phase 1: swift backend method signature generation
```
###### ensureGroup()

Ensure all languages in a named group are available.

**Signature:**

```swift
// Phase 1: swift backend method signature generation
```
###### libPath()

Get the expected path for a language's shared library in the cache.

**Signature:**

```swift
// Phase 1: swift backend method signature generation
```
###### fetchManifest()

Fetch the parser manifest from GitHub Releases.

**Signature:**

```swift
// Phase 1: swift backend method signature generation
```
###### downloadAllBestEffort()

Download the platform bundle and extract every library file it contains.

Unlike `ensure_languages`, this does not check the manifest language list
against archive contents — it simply extracts all `.so`/`.dylib`/`.dll` files
from the bundle. Languages in the manifest that are missing from the archive
are silently ignored rather than returning an error.

Returns the number of library files extracted (including those already cached).

**Signature:**

```swift
// Phase 1: swift backend method signature generation
```
###### cleanCache()

Remove all cached parser libraries.

**Signature:**

```swift
// Phase 1: swift backend method signature generation
```

---

#### ExportInfo

An export statement extracted from source code.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `name` | `String` | — | The name |
| `kind` | `ExportKind` | `ExportKind.Named` | Kind (export kind) |
| `span` | `Span` | — | Span (span) |


---

#### FileMetrics

Aggregate metrics for a source file.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `totalLines` | `UInt64` | — | Total lines |
| `codeLines` | `UInt64` | — | Code lines |
| `commentLines` | `UInt64` | — | Comment lines |
| `blankLines` | `UInt64` | — | Blank lines |
| `totalBytes` | `UInt64` | — | Total bytes |
| `nodeCount` | `UInt64` | — | Number of nodes |
| `errorCount` | `UInt64` | — | Number of errors |
| `maxDepth` | `UInt64` | — | Maximum depth |


---

#### ImportInfo

An import statement extracted from source code.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `source` | `String` | — | Source |
| `items` | `[String]` | `[]` | Items |
| `alias` | `String?` | `null` | Alias |
| `isWildcard` | `Bool` | — | Whether wildcard |
| `span` | `Span` | — | Span (span) |


---

#### Language


---

#### LanguageInfo

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `group` | `String` | — | Group |
| `size` | `UInt64` | — | Size in bytes |


---

#### LanguageRegistry

Thread-safe registry of tree-sitter language parsers.

Manages both statically compiled and dynamically loaded language grammars.
Use `LanguageRegistry.new()` for the default registry, or access the
global instance via the module-level convenience functions
(`get_language`, `available_languages`, etc.).

##### Methods

###### withLibsDir()

Create a registry with a custom directory for dynamic libraries.

Overrides the default build-time library directory. Useful when
dynamic grammar shared libraries are stored in a non-standard location.

**Signature:**

```swift
// Phase 1: swift backend method signature generation
```
###### addExtraLibsDir()

Add an additional directory to search for dynamic libraries.

When `get_language` cannot find a grammar in the
primary library directory, it searches these extra directories in order.
Typically used by the download system to register its cache directory.

Takes `&self` (not `&mut self`) because `extra_lib_dirs` uses interior
mutability via an `Arc<RwLock<...>>`, so the outer registry can remain
immutable while the directory list is updated.

**Signature:**

```swift
// Phase 1: swift backend method signature generation
```
###### getLanguage()

Get a tree-sitter `Language` by name.

Resolves aliases (e.g., `"shell"` -> `"bash"`, `"makefile"` -> `"make"`),
then looks up the language in the static table. When the `dynamic-loading`
feature is enabled, falls back to loading a shared library on demand.

**Errors:**

Returns `Error.LanguageNotFound` if the name (after alias resolution)
does not match any known grammar.

**Signature:**

```swift
// Phase 1: swift backend method signature generation
```
###### availableLanguages()

List all available language names, sorted and deduplicated.

Includes statically compiled languages, dynamically loadable languages
(if the `dynamic-loading` feature is enabled), and all configured aliases.

**Signature:**

```swift
// Phase 1: swift backend method signature generation
```
###### hasLanguage()

Check whether a language is available by name or alias.

Returns `true` if the language can be loaded, either from the static
table or from a dynamic library on disk.

**Signature:**

```swift
// Phase 1: swift backend method signature generation
```
###### languageCount()

Return the total number of available languages (including aliases).

**Signature:**

```swift
// Phase 1: swift backend method signature generation
```
###### process()

Parse source code and extract file intelligence based on config in a single pass.

**Signature:**

```swift
// Phase 1: swift backend method signature generation
```
###### default()

**Signature:**

```swift
// Phase 1: swift backend method signature generation
```

---

#### Node

A single syntax node within a `Tree`.

Nodes hold a strong reference to their parent tree so they remain valid
regardless of how the tree is moved or stored at the FFI boundary.

##### Methods

###### clone()

**Signature:**

```swift
// Phase 1: swift backend method signature generation
```
###### kind()

Return the node's kind name (e.g. `"function_definition"`).

**Signature:**

```swift
// Phase 1: swift backend method signature generation
```
###### kindId()

Return the node's numeric kind ID.

**Signature:**

```swift
// Phase 1: swift backend method signature generation
```
###### startByte()

Return the inclusive start byte offset of this node.

**Signature:**

```swift
// Phase 1: swift backend method signature generation
```
###### endByte()

Return the exclusive end byte offset of this node.

**Signature:**

```swift
// Phase 1: swift backend method signature generation
```
###### byteRange()

Return the node's byte range as a `ByteRange`.

Callers should slice their own source bytes — this is a zero-copy
text accessor.

**Signature:**

```swift
// Phase 1: swift backend method signature generation
```
###### startPosition()

Return the start `Point` (row, column).

**Signature:**

```swift
// Phase 1: swift backend method signature generation
```
###### endPosition()

Return the end `Point` (row, column).

**Signature:**

```swift
// Phase 1: swift backend method signature generation
```
###### isNamed()

True when this node is named (not punctuation/whitespace).

**Signature:**

```swift
// Phase 1: swift backend method signature generation
```
###### isError()

True when this is an error node.

**Signature:**

```swift
// Phase 1: swift backend method signature generation
```
###### isMissing()

True when this is a missing-token node.

**Signature:**

```swift
// Phase 1: swift backend method signature generation
```
###### isExtra()

True when this is an "extra" node (e.g. a comment).

**Signature:**

```swift
// Phase 1: swift backend method signature generation
```
###### hasError()

True when this node or any descendant is an error.

**Signature:**

```swift
// Phase 1: swift backend method signature generation
```
###### parent()

Return this node's parent, if any.

**Signature:**

```swift
// Phase 1: swift backend method signature generation
```
###### child()

Return the i-th child of this node, if any.

**Signature:**

```swift
// Phase 1: swift backend method signature generation
```
###### childCount()

Total number of children (including unnamed).

**Signature:**

```swift
// Phase 1: swift backend method signature generation
```
###### namedChild()

Return the i-th named child of this node, if any.

**Signature:**

```swift
// Phase 1: swift backend method signature generation
```
###### namedChildCount()

Number of named children of this node.

**Signature:**

```swift
// Phase 1: swift backend method signature generation
```
###### childByFieldName()

Look up a child by its grammar-defined field name.

**Signature:**

```swift
// Phase 1: swift backend method signature generation
```
###### toSexp()

Return the S-expression form of this node's subtree.

**Signature:**

```swift
// Phase 1: swift backend method signature generation
```
###### walk()

Return a `TreeCursor` positioned at this node.

**Signature:**

```swift
// Phase 1: swift backend method signature generation
```

---

#### PackConfig

Configuration for the tree-sitter language pack.

Controls cache directory and which languages to pre-download.
Can be loaded from a TOML file, constructed programmatically,
or passed as a dict/object from language bindings.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `cacheDir` | `URL?` | `null` | Override default cache directory. Default: `~/.cache/tree-sitter-language-pack/v{version}/libs/` |
| `languages` | `[String]?` | `[]` | Languages to pre-download on init. Each entry is a language name (e.g. `"python"`, `"rust"`). |
| `groups` | `[String]?` | `[]` | Language groups to pre-download (e.g. `"web"`, `"systems"`, `"scripting"`). |

##### Methods

###### fromTomlFile()

Load configuration from a TOML file.

**Errors:**

Returns an error if the file cannot be read or the TOML is invalid.

**Signature:**

```swift
// Phase 1: swift backend method signature generation
```
###### discover()

Discover configuration by searching for `language-pack.toml` in:

1. Current directory and up to 10 parent directories
2. `$XDG_CONFIG_HOME/tree-sitter-language-pack/config.toml`
3. `~/.config/tree-sitter-language-pack/config.toml`

Returns `null` if no configuration file is found.

**Signature:**

```swift
// Phase 1: swift backend method signature generation
```

---

#### Parser

A tree-sitter parser configured for one language at a time.

##### Methods

###### setLanguage()

Configure the parser to use the language identified by name (e.g. `"python"`).

Resolves the language through the global registry — auto-downloading
if necessary, when the `download` feature is enabled.

**Errors:**

Returns `Error.LanguageNotFound` if the language is not recognized,
or `Error.ParserSetup` if the language ABI is incompatible.

**Signature:**

```swift
// Phase 1: swift backend method signature generation
```
###### parse()

Parse a UTF-8 source string. Returns `null` if parsing was cancelled
or no language is set.

**Signature:**

```swift
// Phase 1: swift backend method signature generation
```
###### parseBytes()

Parse a raw byte slice. Returns `null` if parsing was cancelled or
no language is set.

**Signature:**

```swift
// Phase 1: swift backend method signature generation
```
###### reset()

Reset internal state. The next call to `parse` will
not be incremental.

**Signature:**

```swift
// Phase 1: swift backend method signature generation
```
###### default()

**Signature:**

```swift
// Phase 1: swift backend method signature generation
```

---

#### ParserManifest

Manifest describing available parser downloads for a specific version.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `version` | `String` | — | Version string |
| `platforms` | `[String: PlatformBundle]` | — | Platforms |
| `languages` | `[String: LanguageInfo]` | — | Languages |
| `groups` | `[String: [String]]` | — | Groups |


---

#### PlatformBundle

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `url` | `String` | — | Url |
| `sha256` | `String` | — | Sha256 |
| `size` | `UInt64` | — | Size in bytes |


---

#### Point

A source position — row + column, zero-indexed.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `row` | `UInt64` | — | Zero-indexed row number. |
| `column` | `UInt64` | — | Zero-indexed column number, in UTF-16 code units. |

##### Methods

###### from()

**Signature:**

```swift
// Phase 1: swift backend method signature generation
```

---

#### ProcessConfig

Configuration for the `process()` function.

Controls which analysis features are enabled and whether chunking is performed.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `language` | `String` | — | Language name (required). |
| `structure` | `Bool` | `true` | Extract structural items (functions, classes, etc.). Default: true. |
| `imports` | `Bool` | `true` | Extract import statements. Default: true. |
| `exports` | `Bool` | `true` | Extract export statements. Default: true. |
| `comments` | `Bool` | `false` | Extract comments. Default: false. |
| `docstrings` | `Bool` | `false` | Extract docstrings. Default: false. |
| `symbols` | `Bool` | `false` | Extract symbol definitions. Default: false. |
| `diagnostics` | `Bool` | `false` | Include parse diagnostics. Default: false. |
| `chunkMaxSize` | `UInt64?` | `null` | Maximum chunk size in bytes. `null` disables chunking. |

##### Methods

###### default()

**Signature:**

```swift
// Phase 1: swift backend method signature generation
```
###### withChunking()

Enable chunking with the given maximum chunk size in bytes.

**Signature:**

```swift
// Phase 1: swift backend method signature generation
```
###### all()

Enable all analysis features.

**Signature:**

```swift
// Phase 1: swift backend method signature generation
```
###### minimal()

Disable all analysis features (only metrics computed).

**Signature:**

```swift
// Phase 1: swift backend method signature generation
```

---

#### ProcessResult

Complete analysis result from processing a source file.

Contains metrics, structural analysis, imports/exports, comments,
docstrings, symbols, diagnostics, and optionally chunked code segments.
Fields are populated based on the `ProcessConfig` flags.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `language` | `String` | — | Language |
| `metrics` | `FileMetrics` | — | Metrics (file metrics) |
| `structure` | `[StructureItem]` | `[]` | Structure |
| `imports` | `[ImportInfo]` | `[]` | Imports |
| `exports` | `[ExportInfo]` | `[]` | Exports |
| `comments` | `[CommentInfo]` | `[]` | Comments |
| `docstrings` | `[DocstringInfo]` | `[]` | Docstrings |
| `symbols` | `[SymbolInfo]` | `[]` | Symbols |
| `diagnostics` | `[Diagnostic]` | `[]` | Diagnostics |
| `chunks` | `[CodeChunk]` | `[]` | Text chunks for chunking/embedding |


---

#### Span

Byte and line/column range in source code.

Represents both byte offsets (for slicing) and human-readable line/column
positions (for display and diagnostics).

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `startByte` | `UInt64` | — | Start byte |
| `endByte` | `UInt64` | — | End byte |
| `startLine` | `UInt64` | — | Start line |
| `startColumn` | `UInt64` | — | Start column |
| `endLine` | `UInt64` | — | End line |
| `endColumn` | `UInt64` | — | End column |


---

#### StructureItem

A structural item (function, class, struct, etc.) in source code.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `kind` | `StructureKind` | `StructureKind.Function` | Kind (structure kind) |
| `name` | `String?` | `null` | The name |
| `visibility` | `String?` | `null` | Visibility |
| `span` | `Span` | — | Span (span) |
| `children` | `[StructureItem]` | `[]` | Children |
| `decorators` | `[String]` | `[]` | Decorators |
| `docComment` | `String?` | `null` | Doc comment |
| `signature` | `String?` | `null` | Signature |
| `bodySpan` | `Span?` | `null` | Body span (span) |


---

#### SymbolInfo

A symbol (variable, function, type, etc.) extracted from source code.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `name` | `String` | — | The name |
| `kind` | `SymbolKind` | `SymbolKind.Variable` | Kind (symbol kind) |
| `span` | `Span` | — | Span (span) |
| `typeAnnotation` | `String?` | `null` | Type annotation |
| `doc` | `String?` | `null` | Doc |


---

#### Tree

A parsed syntax tree. Cheap to clone (refcount bump).

##### Methods

###### rootNode()

Return the root `Node` of this tree.

**Signature:**

```swift
// Phase 1: swift backend method signature generation
```
###### walk()

Return a `TreeCursor` positioned at the root.

**Signature:**

```swift
// Phase 1: swift backend method signature generation
```

---

#### TreeCursor

A cursor for traversing a `Tree`.

##### Methods

###### node()

Return the `Node` at the cursor's current position.

**Signature:**

```swift
// Phase 1: swift backend method signature generation
```
###### gotoFirstChild()

Move the cursor to the first child of the current node.
Returns `true` if a child existed.

**Signature:**

```swift
// Phase 1: swift backend method signature generation
```
###### gotoParent()

Move the cursor to the parent of the current node.
Returns `true` if a parent existed.

**Signature:**

```swift
// Phase 1: swift backend method signature generation
```
###### gotoNextSibling()

Move the cursor to the next sibling of the current node.
Returns `true` if a sibling existed.

**Signature:**

```swift
// Phase 1: swift backend method signature generation
```
###### fieldName()

Return the field name for the current node, if any.

**Signature:**

```swift
// Phase 1: swift backend method signature generation
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
| `Function` | Function |
| `Method` | Method |
| `Class` | Class |
| `Struct` | Struct |
| `Interface` | Interface |
| `Enum` | Enum |
| `Module` | Module |
| `Trait` | Trait |
| `Impl` | Impl |
| `Namespace` | Namespace |
| `Other` | Other — Fields: `0`: `String` |


---

#### CommentKind

The kind of a comment found in source code.

Distinguishes between single-line comments, block (multi-line) comments,
and documentation comments.

| Value | Description |
|-------|-------------|
| `Line` | Line |
| `Block` | Block |
| `Doc` | Doc |


---

#### DocstringFormat

The format of a docstring extracted from source code.

Identifies the docstring convention used, which varies by language
(e.g., Python triple-quoted strings, JSDoc, Rustdoc `///` comments).

| Value | Description |
|-------|-------------|
| `PythonTripleQuote` | Python triple quote |
| `JsDoc` | J s doc |
| `Rustdoc` | Rustdoc |
| `GoDoc` | Go doc |
| `JavaDoc` | Java doc |
| `Other` | Other — Fields: `0`: `String` |


---

#### ExportKind

The kind of an export statement found in source code.

Covers named exports, default exports, and re-exports from other modules.

| Value | Description |
|-------|-------------|
| `Named` | Named |
| `Default` | Default |
| `ReExport` | Re export |


---

#### SymbolKind

The kind of a symbol definition found in source code.

Categorizes symbol definitions such as variables, constants, functions,
classes, types, interfaces, enums, and modules.

| Value | Description |
|-------|-------------|
| `Variable` | Variable |
| `Constant` | Constant |
| `Function` | Function |
| `Class` | Class |
| `Type` | Type |
| `Interface` | Interface |
| `Enum` | Enum |
| `Module` | Module |
| `Other` | Other — Fields: `0`: `String` |


---

#### DiagnosticSeverity

Severity level of a diagnostic produced during parsing.

Used to classify parse errors, warnings, and informational messages
found in the syntax tree.

| Value | Description |
|-------|-------------|
| `Error` | Error |
| `Warning` | Warning |
| `Info` | Info |


---

### Errors

#### Error

Errors that can occur when using the tree-sitter language pack.

Covers language lookup failures, parse errors, query errors, and I/O issues.
Feature-gated variants are included when `config`, `download`, or related
features are enabled.

| Variant | Description |
|---------|-------------|
| `LanguageNotFound` | Language '{0}' not found |
| `DynamicLoad` | Dynamic library load error: {0} |
| `NullLanguagePointer` | Language function returned null pointer for '{0}' |
| `ParserSetup` | Failed to set parser language: {0} |
| `LockPoisoned` | Registry lock poisoned: {0} |
| `Config` | Configuration error: {0} |
| `ParseFailed` | Parse failed: parsing returned no tree |
| `QueryError` | Query error: {0} |
| `InvalidRange` | Invalid byte range: {0} |
| `Io` | IO error: {0} |


---
