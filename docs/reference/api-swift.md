---
title: "Swift API Reference"
---

## Swift API Reference <span class="version-badge">v1.9.0-rc.1</span>

### Functions

#### detectLanguageFromExtension()

Detect language name from a file extension (without leading dot).

Returns `null` for unrecognized extensions. The match is case-insensitive.

**Signature:**

```swift
public static func detectLanguageFromExtension(ext: String) -> String?
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
public static func detectLanguageFromPath(path: String) -> String?
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
public static func detectLanguageFromContent(content: String) -> String?
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
public static func getHighlightsQuery(language: String) -> String?
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
public static func getInjectionsQuery(language: String) -> String?
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
public static func getLocalsQuery(language: String) -> String?
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
public static func getLanguage(name: String) throws -> Language
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
public static func getParser(name: String) throws -> Parser
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
public static func detectLanguage(path: String) -> String?
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
public static func availableLanguages() -> [String]
```

**Returns:** `[String]`

---

#### hasLanguage()

Check if a language is available by name or alias.

Returns `true` if the language can be loaded (statically compiled,
dynamically available, or a known alias for one of these).

**Signature:**

```swift
public static func hasLanguage(name: String) -> Bool
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
public static func languageCount() -> UInt64
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
public static func process(source: String, config: ProcessConfig) throws -> ProcessResult
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
public static func init(config: PackConfig) throws
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
public static func configure(config: PackConfig) throws
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
public static func download(names: [String]) throws -> UInt64
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
public static func downloadAll() throws -> UInt64
```

**Returns:** `UInt64`
**Errors:** Throws `Error`.

---

#### downloadGroup()

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

```swift
public static func downloadGroup(name: String) throws -> UInt64
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `name` | `String` | Yes | The name |

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
public static func manifestLanguages() throws -> [String]
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
public static func downloadedLanguages() -> [String]
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
public static func cleanCache() throws
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
public static func cacheDir() throws -> String
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

### Methods

#### new()

Create a new download manager for the given version.

**Signature:**

```swift
public static func new(version: String) throws -> DownloadManager
```

#### withCacheDir()

Create a download manager with a custom cache directory.

**Signature:**

```swift
public static func withCacheDir(version: String, cacheDir: URL) -> DownloadManager
```

#### installedLanguages()

List languages that are already downloaded and cached.

**Signature:**

```swift
public func installedLanguages() -> [String]
```

#### downloadAllBestEffort()

Download the platform bundle and extract every library file it contains.

Unlike `ensure_languages`, this does not check the manifest language list
against archive contents — it simply extracts all `.so`/`.dylib`/`.dll` files
from the bundle. Languages in the manifest that are missing from the archive
are silently ignored rather than returning an error.

Returns the number of library files extracted (including those already cached).

**Signature:**

```swift
public func downloadAllBestEffort() throws -> UInt64
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

```swift
public func cleanCache() throws
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

#### LanguageRegistry

Thread-safe registry of tree-sitter language parsers.

Manages both statically compiled and dynamically loaded language grammars.
Use `LanguageRegistry.new()` for the default registry, or access the
global instance via the module-level convenience functions
(`get_language`, `available_languages`, etc.).

### Methods

#### getLanguage()

Get a tree-sitter `Language` by name.

Resolves aliases (e.g., `"shell"` -> `"bash"`, `"makefile"` -> `"make"`),
then looks up the language in the static table. When the `dynamic-loading`
feature is enabled, falls back to loading a shared library on demand.

**Errors:**

Returns `Error.LanguageNotFound` if the name (after alias resolution)
does not match any known grammar.

**Signature:**

```swift
public func getLanguage(name: String) throws -> Language
```

#### availableLanguages()

List all available language names, sorted and deduplicated.

Includes statically compiled languages, dynamically loadable languages
(if the `dynamic-loading` feature is enabled), and all configured aliases.

**Signature:**

```swift
public func availableLanguages() -> [String]
```

#### hasLanguage()

Check whether a language is available by name or alias.

Returns `true` if the language can be loaded, either from the static
table or from a dynamic library on disk.

**Signature:**

```swift
public func hasLanguage(name: String) -> Bool
```

#### languageCount()

Return the total number of available languages (including aliases).

**Signature:**

```swift
public func languageCount() -> UInt64
```

#### process()

Parse source code and extract file intelligence based on config in a single pass.

**Signature:**

```swift
public func process(source: String, config: ProcessConfig) throws -> ProcessResult
```

#### default()

**Signature:**

```swift
public static func default() -> LanguageRegistry
```


---

#### Node

A single syntax node within a `Tree`.

Nodes hold a strong reference to their parent tree so they remain valid
regardless of how the tree is moved or stored at the FFI boundary.

### Methods

#### clone()

**Signature:**

```swift
public func clone() -> Node
```

#### kind()

Return the node's kind name (e.g. `"function_definition"`).

**Signature:**

```swift
public func kind() -> String
```

#### kindId()

Return the node's numeric kind ID.

Tree-sitter assigns a stable `u16` ID to every node kind in a grammar
(e.g. `"function_definition" → 42`). Comparing `kind_id()` is cheaper
than comparing the string `kind()` in tight AST loops.

**Signature:**

```swift
public func kindId() -> UInt16
```

#### startByte()

Return the inclusive start byte offset of this node.

**Signature:**

```swift
public func startByte() -> UInt64
```

#### endByte()

Return the exclusive end byte offset of this node.

**Signature:**

```swift
public func endByte() -> UInt64
```

#### byteRange()

Return the node's byte range as a `ByteRange`.

Callers should slice their own source bytes — this is a zero-copy
text accessor.

**Signature:**

```swift
public func byteRange() -> ByteRange
```

#### startPosition()

Return the start `Point` (row, column).

**Signature:**

```swift
public func startPosition() -> Point
```

#### endPosition()

Return the end `Point` (row, column).

**Signature:**

```swift
public func endPosition() -> Point
```

#### isNamed()

True when this node is named (not punctuation/whitespace).

**Signature:**

```swift
public func isNamed() -> Bool
```

#### isError()

True when this is an error node.

**Signature:**

```swift
public func isError() -> Bool
```

#### isMissing()

True when this is a missing-token node.

**Signature:**

```swift
public func isMissing() -> Bool
```

#### isExtra()

True when this is an "extra" node (e.g. a comment).

**Signature:**

```swift
public func isExtra() -> Bool
```

#### hasError()

True when this node or any descendant is an error.

**Signature:**

```swift
public func hasError() -> Bool
```

#### parent()

Return this node's parent, if any.

**Signature:**

```swift
public func parent() -> Node?
```

#### child()

Return the i-th child of this node, if any.

**Signature:**

```swift
public func child(index: UInt32) -> Node?
```

#### childCount()

Total number of children (including unnamed).

**Signature:**

```swift
public func childCount() -> UInt64
```

#### namedChild()

Return the i-th named child of this node, if any.

**Signature:**

```swift
public func namedChild(index: UInt32) -> Node?
```

#### namedChildCount()

Number of named children of this node.

**Signature:**

```swift
public func namedChildCount() -> UInt64
```

#### childByFieldName()

Look up a child by its grammar-defined field name.

**Signature:**

```swift
public func childByFieldName(name: String) -> Node?
```

#### toSexp()

Return the S-expression form of this node's subtree.

**Signature:**

```swift
public func toSexp() -> String
```

#### walk()

Return a `TreeCursor` positioned at this node.

**Signature:**

```swift
public func walk() -> TreeCursor
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


---

#### Parser

A tree-sitter parser configured for one language at a time.

### Methods

#### setLanguage()

Configure the parser to use the language identified by name (e.g. `"python"`).

Resolves the language through the global registry — auto-downloading
if necessary, when the `download` feature is enabled.

**Errors:**

Returns `Error.LanguageNotFound` if the language is not recognized,
or `Error.ParserSetup` if the language ABI is incompatible.

**Signature:**

```swift
public func setLanguage(name: String) throws
```

#### parse()

Parse a UTF-8 source string. Returns `null` if parsing was cancelled
or no language is set.

**Signature:**

```swift
public func parse(source: String) -> Tree?
```

#### parseBytes()

Parse a raw byte slice. Returns `null` if parsing was cancelled or
no language is set.

**Signature:**

```swift
public func parseBytes(source: Data) -> Tree?
```

#### reset()

Reset internal state. The next call to `parse` will
not be incremental.

**Signature:**

```swift
public func reset()
```

#### default()

**Signature:**

```swift
public static func default() -> Parser
```


---

#### Point

A source position — row + column, zero-indexed.


| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `row` | `UInt64` | — | Zero-indexed row number. |
| `column` | `UInt64` | — | Zero-indexed column number, in UTF-16 code units. |


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

### Methods

#### default()

**Signature:**

```swift
public static func default() -> ProcessConfig
```

#### withChunking()

Enable chunking with the given maximum chunk size in bytes.

**Signature:**

```swift
public func withChunking(maxSize: UInt64) -> ProcessConfig
```

#### all()

Enable all analysis features.

**Signature:**

```swift
public func all() -> ProcessConfig
```

#### minimal()

Disable all analysis features (only metrics computed).

**Signature:**

```swift
public func minimal() -> ProcessConfig
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

### Methods

#### rootNode()

Return the root `Node` of this tree.

**Signature:**

```swift
public func rootNode() -> Node
```

#### walk()

Return a `TreeCursor` positioned at the root.

**Signature:**

```swift
public func walk() -> TreeCursor
```


---

#### TreeCursor

A cursor for traversing a `Tree`.

### Methods

#### node()

Return the `Node` at the cursor's current position.

**Signature:**

```swift
public func node() -> Node
```

#### gotoFirstChild()

Move the cursor to the first child of the current node.
Returns `true` if a child existed.

**Signature:**

```swift
public func gotoFirstChild() -> Bool
```

#### gotoParent()

Move the cursor to the parent of the current node.
Returns `true` if a parent existed.

**Signature:**

```swift
public func gotoParent() -> Bool
```

#### gotoNextSibling()

Move the cursor to the next sibling of the current node.
Returns `true` if a sibling existed.

**Signature:**

```swift
public func gotoNextSibling() -> Bool
```

#### fieldName()

Return the field name for the current node, if any.

**Signature:**

```swift
public func fieldName() -> String?
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
