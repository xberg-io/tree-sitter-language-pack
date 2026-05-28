---
title: "Go API Reference"
---

## Go API Reference <span class="version-badge">v1.9.0-rc.12</span>

### Functions

#### DetectLanguageFromExtension()

Detect language name from a file extension (without leading dot).

Returns `nil` for unrecognized extensions. The match is case-insensitive.

**Signature:**

```go
func DetectLanguageFromExtension(ext string) *string
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `Ext` | `string` | Yes | The ext |

**Returns:** `*string`

---

#### DetectLanguageFromPath()

Detect language name from a file path.

Extracts the file extension and looks it up. Returns `nil` if the
path has no extension or the extension is not recognized.

**Signature:**

```go
func DetectLanguageFromPath(path string) *string
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `Path` | `string` | Yes | Path to the file |

**Returns:** `*string`

---

#### DetectLanguageFromContent()

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

```go
func DetectLanguageFromContent(content string) *string
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `Content` | `string` | Yes | The content to process |

**Returns:** `*string`

---

#### GetHighlightsQuery()

Get the highlights query for a language, if bundled.

Returns the contents of `highlights.scm` as a static string, or `nil`
if no highlights query is bundled for this language.

**Signature:**

```go
func GetHighlightsQuery(language string) *string
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `Language` | `string` | Yes | The language |

**Returns:** `*string`

---

#### GetInjectionsQuery()

Get the injections query for a language, if bundled.

Returns the contents of `injections.scm` as a static string, or `nil`
if no injections query is bundled for this language.

**Signature:**

```go
func GetInjectionsQuery(language string) *string
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `Language` | `string` | Yes | The language |

**Returns:** `*string`

---

#### GetLocalsQuery()

Get the locals query for a language, if bundled.

Returns the contents of `locals.scm` as a static string, or `nil`
if no locals query is bundled for this language.

**Signature:**

```go
func GetLocalsQuery(language string) *string
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `Language` | `string` | Yes | The language |

**Returns:** `*string`

---

#### GetLanguage()

Get a tree-sitter `Language` by name using the global registry.

Resolves language aliases (e.g., `"shell"` maps to `"bash"`).
When the `download` feature is enabled (default), automatically downloads
the parser from GitHub releases if not found locally.

**Errors:**

Returns `Error.LanguageNotFound` if the language is not recognized,
or `Error.Download` if auto-download fails.

**Signature:**

```go
func GetLanguage(name string) (Language, error)
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `Name` | `string` | Yes | The name |

**Returns:** `Language`
**Errors:** Returns `error`.

---

#### GetParser()

Get a `Parser` pre-configured for the given language.

This is a convenience function that calls `get_language` and configures
a new parser in one step.

**Errors:**

Returns `Error.LanguageNotFound` if the language is not recognized, or
`Error.ParserSetup` if the language cannot be applied to the parser.

**Signature:**

```go
func GetParser(name string) (Parser, error)
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `Name` | `string` | Yes | The name |

**Returns:** `Parser`
**Errors:** Returns `error`.

---

#### DetectLanguage()

Detect language name from a file path or extension.

This compatibility alias matches the pre-Alef Python binding API.

**Signature:**

```go
func DetectLanguage(path string) *string
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `Path` | `string` | Yes | Path to the file |

**Returns:** `*string`

---

#### AvailableLanguages()

List all available language names (sorted, deduplicated, includes aliases).

Returns names of both statically compiled and dynamically loadable languages,
plus any configured aliases.

**Signature:**

```go
func AvailableLanguages() []string
```

**Returns:** `[]string`

---

#### HasLanguage()

Check if a language is available by name or alias.

Returns `true` if the language can be loaded (statically compiled,
dynamically available, or a known alias for one of these).

**Signature:**

```go
func HasLanguage(name string) bool
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `Name` | `string` | Yes | The name |

**Returns:** `bool`

---

#### LanguageCount()

Return the number of available languages.

Includes statically compiled languages, dynamically loadable languages,
and aliases.

**Signature:**

```go
func LanguageCount() int
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

```go
func Process(source string, config ProcessConfig) (ProcessResult, error)
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `Source` | `string` | Yes | The source |
| `Config` | `ProcessConfig` | Yes | The configuration options |

**Returns:** `ProcessResult`
**Errors:** Returns `error`.

---

#### Init()

Initialize the language pack with the given configuration.

Applies any custom cache directory, then downloads all languages and groups
specified in the config. This is the recommended entry point when you want
to pre-warm the cache before use.

**Errors:**

Returns an error if configuration cannot be applied or if downloads fail.

**Signature:**

```go
func Init(config PackConfig) error
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `Config` | `PackConfig` | Yes | The configuration options |

**Returns:** ``
**Errors:** Returns `error`.

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

```go
func Configure(config PackConfig) error
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `Config` | `PackConfig` | Yes | The configuration options |

**Returns:** ``
**Errors:** Returns `error`.

---

#### Download()

Download specific languages to the local cache.

Returns the number of requested languages available after the call. Already
compiled or cached languages are included in the count.

**Errors:**

Returns an error if any language is not available in the manifest or if
the download fails.

**Signature:**

```go
func Download(names []string) (int, error)
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `Names` | `[]string` | Yes | The names |

**Returns:** `int`
**Errors:** Returns `error`.

---

#### DownloadAll()

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

```go
func DownloadAll() (int, error)
```

**Returns:** `int`
**Errors:** Returns `error`.

---

#### DownloadGroup()

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

```go
func DownloadGroup(name string) (int, error)
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `Name` | `string` | Yes | The name |

**Returns:** `int`
**Errors:** Returns `error`.

---

#### ManifestLanguages()

Return all language names available in the remote manifest (306).

Fetches (and caches) the remote manifest to discover the full list of
downloadable languages. Use `downloaded_languages` to list what is
already cached locally.

**Errors:**

Returns an error if the manifest cannot be fetched.

**Signature:**

```go
func ManifestLanguages() ([]string, error)
```

**Returns:** `[]string`
**Errors:** Returns `error`.

---

#### DownloadedLanguages()

Return languages that are already downloaded and cached locally.

Does not perform any network requests. Returns an empty list if the
cache directory does not exist or cannot be read.

**Signature:**

```go
func DownloadedLanguages() []string
```

**Returns:** `[]string`

---

#### CleanCache()

Delete all cached parser shared libraries.

Resets the cache registration so the next call to `get_language` or
a download function will re-register the (now empty) cache directory.

**Errors:**

Returns an error if the cache directory cannot be removed.

**Signature:**

```go
func CleanCache() error
```

**Returns:** ``
**Errors:** Returns `error`.

---

#### CacheDir()

Return the effective cache directory path.

This is either the custom path set via `configure` / `init` or the
default: `~/.cache/tree-sitter-language-pack/v{version}/libs/`.

**Errors:**

Returns an error if the system cache directory cannot be determined.

**Signature:**

```go
func CacheDir() (string, error)
```

**Returns:** `string`
**Errors:** Returns `error`.

---

### Types

#### ByteRange

A byte range — start (inclusive) to end (exclusive).

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Start` | `int` | — | Inclusive start byte offset. |
| `End` | `int` | — | Exclusive end byte offset. |

---

#### ChunkContext

Metadata for a single chunk of source code.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Language` | `string` | — | Language |
| `ChunkIndex` | `int` | — | Chunk index |
| `TotalChunks` | `int` | — | Total chunks |
| `NodeTypes` | `[]string` | `nil` | Node types |
| `ContextPath` | `[]string` | `nil` | Context path |
| `SymbolsDefined` | `[]string` | `nil` | Symbols defined |
| `Comments` | `[]CommentInfo` | `nil` | Comments |
| `Docstrings` | `[]DocstringInfo` | `nil` | Docstrings |
| `HasErrorNodes` | `bool` | — | Whether error nodes |

---

#### CodeChunk

A chunk of source code with rich metadata.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Content` | `string` | — | The extracted text content |
| `StartByte` | `int` | — | Start byte |
| `EndByte` | `int` | — | End byte |
| `StartLine` | `int` | — | Start line |
| `EndLine` | `int` | — | End line |
| `Metadata` | `ChunkContext` | — | Document metadata |

---

#### CommentInfo

A comment extracted from source code.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Text` | `string` | — | Text |
| `Kind` | `CommentKind` | `CommentKind.Line` | Kind (comment kind) |
| `Span` | `Span` | — | Span (span) |
| `AssociatedNode` | `*string` | `nil` | Associated node |

---

#### Diagnostic

A diagnostic (syntax error, missing node, etc.) from parsing.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Message` | `string` | — | Message |
| `Severity` | `DiagnosticSeverity` | `DiagnosticSeverity.Error` | Severity (diagnostic severity) |
| `Span` | `Span` | — | Span (span) |

---

#### DocSection

A section within a docstring (e.g., Args, Returns, Raises).

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Kind` | `string` | — | Kind |
| `Name` | `*string` | `nil` | The name |
| `Description` | `string` | — | Human-readable description |

---

#### DocstringInfo

A docstring extracted from source code.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Text` | `string` | — | Text |
| `Format` | `DocstringFormat` | `DocstringFormat.PythonTripleQuote` | Format (docstring format) |
| `Span` | `Span` | — | Span (span) |
| `AssociatedItem` | `*string` | `nil` | Associated item |
| `ParsedSections` | `[]DocSection` | `nil` | Parsed sections |

---

#### DownloadManager

Manages downloading and caching of pre-built parser shared libraries.

### Methods

#### New()

Create a new download manager for the given version.

**Signature:**

```go
func (o *DownloadManager) New(version string) (DownloadManager, error)
```

#### WithCacheDir()

Create a download manager with a custom cache directory.

**Signature:**

```go
func (o *DownloadManager) WithCacheDir(version string, cacheDir string) DownloadManager
```

#### InstalledLanguages()

List languages that are already downloaded and cached.

**Signature:**

```go
func (o *DownloadManager) InstalledLanguages() []string
```

#### DownloadAllBestEffort()

Download the platform bundle and extract every library file it contains.

Unlike `ensure_languages`, this does not check the manifest language list
against archive contents — it simply extracts all `.so`/`.dylib`/`.dll` files
from the bundle. Languages in the manifest that are missing from the archive
are silently ignored rather than returning an error.

Returns the number of library files extracted (including those already cached).

**Signature:**

```go
func (o *DownloadManager) DownloadAllBestEffort() (int, error)
```

#### CleanCache()

Remove all cached parser libraries.

Acquires the cross-process lock so `clean_cache` cannot race a concurrent
downloader (avoids Windows sharing-violation errors against an in-flight
bundle write). The `.download.lock` file itself is **not** removed — it is
permanent infrastructure; deleting it could allow a concurrent process that
already opened the file to continue holding a stale lock handle while a new
process opens a fresh inode, breaking the mutual-exclusion guarantee.

**Signature:**

```go
func (o *DownloadManager) CleanCache() error
```

---

#### ExportInfo

An export statement extracted from source code.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Name` | `string` | — | The name |
| `Kind` | `ExportKind` | `ExportKind.Named` | Kind (export kind) |
| `Span` | `Span` | — | Span (span) |

---

#### FileMetrics

Aggregate metrics for a source file.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `TotalLines` | `int` | — | Total lines |
| `CodeLines` | `int` | — | Code lines |
| `CommentLines` | `int` | — | Comment lines |
| `BlankLines` | `int` | — | Blank lines |
| `TotalBytes` | `int` | — | Total bytes |
| `NodeCount` | `int` | — | Number of nodes |
| `ErrorCount` | `int` | — | Number of errors |
| `MaxDepth` | `int` | — | Maximum depth |

---

#### ImportInfo

An import statement extracted from source code.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Source` | `string` | — | Source |
| `Items` | `[]string` | `nil` | Items |
| `Alias` | `*string` | `nil` | Alias |
| `IsWildcard` | `bool` | — | Whether wildcard |
| `Span` | `Span` | — | Span (span) |

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

#### GetLanguage()

Get a tree-sitter `Language` by name.

Resolves aliases (e.g., `"shell"` -> `"bash"`, `"makefile"` -> `"make"`),
then looks up the language in the static table. When the `dynamic-loading`
feature is enabled, falls back to loading a shared library on demand.

**Errors:**

Returns `Error.LanguageNotFound` if the name (after alias resolution)
does not match any known grammar.

**Signature:**

```go
func (o *LanguageRegistry) GetLanguage(name string) (Language, error)
```

#### AvailableLanguages()

List all available language names, sorted and deduplicated.

Includes statically compiled languages, dynamically loadable languages
(if the `dynamic-loading` feature is enabled), and all configured aliases.

**Signature:**

```go
func (o *LanguageRegistry) AvailableLanguages() []string
```

#### HasLanguage()

Check whether a language is available by name or alias.

Returns `true` if the language can be loaded, either from the static
table or from a dynamic library on disk.

**Signature:**

```go
func (o *LanguageRegistry) HasLanguage(name string) bool
```

#### LanguageCount()

Return the total number of available languages (including aliases).

**Signature:**

```go
func (o *LanguageRegistry) LanguageCount() int
```

#### Process()

Parse source code and extract file intelligence based on config in a single pass.

**Signature:**

```go
func (o *LanguageRegistry) Process(source string, config ProcessConfig) (ProcessResult, error)
```

#### Default()

**Signature:**

```go
func (o *LanguageRegistry) Default() LanguageRegistry
```

---

#### Node

A single syntax node within a `Tree`.

Nodes hold a strong reference to their parent tree so they remain valid
regardless of how the tree is moved or stored at the FFI boundary.

### Methods

#### Clone()

**Signature:**

```go
func (o *Node) Clone() Node
```

#### Kind()

Return the node's kind name (e.g. `"function_definition"`).

**Signature:**

```go
func (o *Node) Kind() string
```

#### KindId()

Return the node's numeric kind ID.

Tree-sitter assigns a stable `u16` ID to every node kind in a grammar
(e.g. `"function_definition" → 42`). Comparing `kind_id()` is cheaper
than comparing the string `kind()` in tight AST loops.

**Signature:**

```go
func (o *Node) KindId() uint16
```

#### StartByte()

Return the inclusive start byte offset of this node.

**Signature:**

```go
func (o *Node) StartByte() int
```

#### EndByte()

Return the exclusive end byte offset of this node.

**Signature:**

```go
func (o *Node) EndByte() int
```

#### ByteRange()

Return the node's byte range as a `ByteRange`.

Callers should slice their own source bytes — this is a zero-copy
text accessor.

**Signature:**

```go
func (o *Node) ByteRange() ByteRange
```

#### StartPosition()

Return the start `Point` (row, column).

**Signature:**

```go
func (o *Node) StartPosition() Point
```

#### EndPosition()

Return the end `Point` (row, column).

**Signature:**

```go
func (o *Node) EndPosition() Point
```

#### IsNamed()

True when this node is named (not punctuation/whitespace).

**Signature:**

```go
func (o *Node) IsNamed() bool
```

#### IsError()

True when this is an error node.

**Signature:**

```go
func (o *Node) IsError() bool
```

#### IsMissing()

True when this is a missing-token node.

**Signature:**

```go
func (o *Node) IsMissing() bool
```

#### IsExtra()

True when this is an "extra" node (e.g. a comment).

**Signature:**

```go
func (o *Node) IsExtra() bool
```

#### HasError()

True when this node or any descendant is an error.

**Signature:**

```go
func (o *Node) HasError() bool
```

#### Parent()

Return this node's parent, if any.

**Signature:**

```go
func (o *Node) Parent() *Node
```

#### Child()

Return the i-th child of this node, if any.

**Signature:**

```go
func (o *Node) Child(index uint32) *Node
```

#### ChildCount()

Total number of children (including unnamed).

**Signature:**

```go
func (o *Node) ChildCount() int
```

#### NamedChild()

Return the i-th named child of this node, if any.

**Signature:**

```go
func (o *Node) NamedChild(index uint32) *Node
```

#### NamedChildCount()

Number of named children of this node.

**Signature:**

```go
func (o *Node) NamedChildCount() int
```

#### ChildByFieldName()

Look up a child by its grammar-defined field name.

**Signature:**

```go
func (o *Node) ChildByFieldName(name string) *Node
```

#### ToSexp()

Return the S-expression form of this node's subtree.

**Signature:**

```go
func (o *Node) ToSexp() string
```

#### Walk()

Return a `TreeCursor` positioned at this node.

**Signature:**

```go
func (o *Node) Walk() TreeCursor
```

---

#### PackConfig

Configuration for the tree-sitter language pack.

Controls cache directory and which languages to pre-download.
Can be loaded from a TOML file, constructed programmatically,
or passed as a dict/object from language bindings.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `CacheDir` | `*string` | `nil` | Override default cache directory. Default: `~/.cache/tree-sitter-language-pack/v{version}/libs/` |
| `Languages` | `*[]string` | `nil` | Languages to pre-download on init. Each entry is a language name (e.g. `"python"`, `"rust"`). |
| `Groups` | `*[]string` | `nil` | Language groups to pre-download (e.g. `"web"`, `"systems"`, `"scripting"`). |

---

#### Parser

A tree-sitter parser configured for one language at a time.

### Methods

#### SetLanguage()

Configure the parser to use the language identified by name (e.g. `"python"`).

Resolves the language through the global registry — auto-downloading
if necessary, when the `download` feature is enabled.

**Errors:**

Returns `Error.LanguageNotFound` if the language is not recognized,
or `Error.ParserSetup` if the language ABI is incompatible.

**Signature:**

```go
func (o *Parser) SetLanguage(name string) error
```

#### Parse()

Parse a UTF-8 source string. Returns `nil` if parsing was cancelled
or no language is set.

**Signature:**

```go
func (o *Parser) Parse(source string) *Tree
```

#### ParseBytes()

Parse a raw byte slice. Returns `nil` if parsing was cancelled or
no language is set.

**Signature:**

```go
func (o *Parser) ParseBytes(source []byte) *Tree
```

#### Reset()

Reset internal state. The next call to `parse` will
not be incremental.

**Signature:**

```go
func (o *Parser) Reset()
```

#### Default()

**Signature:**

```go
func (o *Parser) Default() Parser
```

---

#### Point

A source position — row + column, zero-indexed.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Row` | `int` | — | Zero-indexed row number. |
| `Column` | `int` | — | Zero-indexed column number, in UTF-16 code units. |

---

#### ProcessConfig

Configuration for the `process()` function.

Controls which analysis features are enabled and whether chunking is performed.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Language` | `string` | — | Language name (required). |
| `Structure` | `bool` | `true` | Extract structural items (functions, classes, etc.). Default: true. |
| `Imports` | `bool` | `true` | Extract import statements. Default: true. |
| `Exports` | `bool` | `true` | Extract export statements. Default: true. |
| `Comments` | `bool` | `false` | Extract comments. Default: false. |
| `Docstrings` | `bool` | `false` | Extract docstrings. Default: false. |
| `Symbols` | `bool` | `false` | Extract symbol definitions. Default: false. |
| `Diagnostics` | `bool` | `false` | Include parse diagnostics. Default: false. |
| `ChunkMaxSize` | `*int` | `nil` | Maximum chunk size in bytes. `nil` disables chunking. |

### Methods

#### Default()

**Signature:**

```go
func (o *ProcessConfig) Default() ProcessConfig
```

#### WithChunking()

Enable chunking with the given maximum chunk size in bytes.

**Signature:**

```go
func (o *ProcessConfig) WithChunking(maxSize int) ProcessConfig
```

#### All()

Enable all analysis features.

**Signature:**

```go
func (o *ProcessConfig) All() ProcessConfig
```

#### Minimal()

Disable all analysis features (only metrics computed).

**Signature:**

```go
func (o *ProcessConfig) Minimal() ProcessConfig
```

---

#### ProcessResult

Complete analysis result from processing a source file.

Contains metrics, structural analysis, imports/exports, comments,
docstrings, symbols, diagnostics, and optionally chunked code segments.
Fields are populated based on the `ProcessConfig` flags.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Language` | `string` | — | Language |
| `Metrics` | `FileMetrics` | — | Metrics (file metrics) |
| `Structure` | `[]StructureItem` | `nil` | Structure |
| `Imports` | `[]ImportInfo` | `nil` | Imports |
| `Exports` | `[]ExportInfo` | `nil` | Exports |
| `Comments` | `[]CommentInfo` | `nil` | Comments |
| `Docstrings` | `[]DocstringInfo` | `nil` | Docstrings |
| `Symbols` | `[]SymbolInfo` | `nil` | Symbols |
| `Diagnostics` | `[]Diagnostic` | `nil` | Diagnostics |
| `Chunks` | `[]CodeChunk` | `nil` | Text chunks for chunking/embedding |

---

#### Span

Byte and line/column range in source code.

Represents both byte offsets (for slicing) and human-readable line/column
positions (for display and diagnostics).

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `StartByte` | `int` | — | Start byte |
| `EndByte` | `int` | — | End byte |
| `StartLine` | `int` | — | Start line |
| `StartColumn` | `int` | — | Start column |
| `EndLine` | `int` | — | End line |
| `EndColumn` | `int` | — | End column |

---

#### StructureItem

A structural item (function, class, struct, etc.) in source code.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Kind` | `StructureKind` | `StructureKind.Function` | Kind (structure kind) |
| `Name` | `*string` | `nil` | The name |
| `Visibility` | `*string` | `nil` | Visibility |
| `Span` | `Span` | — | Span (span) |
| `Children` | `[]StructureItem` | `nil` | Children |
| `Decorators` | `[]string` | `nil` | Decorators |
| `DocComment` | `*string` | `nil` | Doc comment |
| `Signature` | `*string` | `nil` | Signature |
| `BodySpan` | `*Span` | `nil` | Body span (span) |

---

#### SymbolInfo

A symbol (variable, function, type, etc.) extracted from source code.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Name` | `string` | — | The name |
| `Kind` | `SymbolKind` | `SymbolKind.Variable` | Kind (symbol kind) |
| `Span` | `Span` | — | Span (span) |
| `TypeAnnotation` | `*string` | `nil` | Type annotation |
| `Doc` | `*string` | `nil` | Doc |

---

#### Tree

A parsed syntax tree. Cheap to clone (refcount bump).

### Methods

#### RootNode()

Return the root `Node` of this tree.

**Signature:**

```go
func (o *Tree) RootNode() Node
```

#### Walk()

Return a `TreeCursor` positioned at the root.

**Signature:**

```go
func (o *Tree) Walk() TreeCursor
```

---

#### TreeCursor

A cursor for traversing a `Tree`.

### Methods

#### Node()

Return the `Node` at the cursor's current position.

**Signature:**

```go
func (o *TreeCursor) Node() Node
```

#### GotoFirstChild()

Move the cursor to the first child of the current node.
Returns `true` if a child existed.

**Signature:**

```go
func (o *TreeCursor) GotoFirstChild() bool
```

#### GotoParent()

Move the cursor to the parent of the current node.
Returns `true` if a parent existed.

**Signature:**

```go
func (o *TreeCursor) GotoParent() bool
```

#### GotoNextSibling()

Move the cursor to the next sibling of the current node.
Returns `true` if a sibling existed.

**Signature:**

```go
func (o *TreeCursor) GotoNextSibling() bool
```

#### FieldName()

Return the field name for the current node, if any.

**Signature:**

```go
func (o *TreeCursor) FieldName() *string
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
| `Other` | Other — Fields: `0`: `string` |

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
| `Other` | Other — Fields: `0`: `string` |

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
| `Other` | Other — Fields: `0`: `string` |

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
| `Download` | Download error: {0} |
| `ChecksumMismatch` | Checksum mismatch for '{file}': expected {expected}, got {actual} |
| `CacheLock` | Download cache lock error: {0} |

---
