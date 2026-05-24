---
title: "C# API Reference"
---

## C# API Reference <span class="version-badge">v1.9.0-rc.4</span>

### Functions

#### DetectLanguageFromExtension()

Detect language name from a file extension (without leading dot).

Returns `null` for unrecognized extensions. The match is case-insensitive.

**Signature:**

```csharp
public static string? DetectLanguageFromExtension(string ext)
```

**Parameters:**

| Name  | Type     | Required | Description |
| ----- | -------- | -------- | ----------- |
| `Ext` | `string` | Yes      | The ext     |

**Returns:** `string?`

---

#### DetectLanguageFromPath()

Detect language name from a file path.

Extracts the file extension and looks it up. Returns `null` if the
path has no extension or the extension is not recognized.

**Signature:**

```csharp
public static string? DetectLanguageFromPath(string path)
```

**Parameters:**

| Name   | Type     | Required | Description      |
| ------ | -------- | -------- | ---------------- |
| `Path` | `string` | Yes      | Path to the file |

**Returns:** `string?`

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

Returns `null` when content does not start with `#!`, the shebang is
malformed, or the interpreter is not recognised.

**Signature:**

```csharp
public static string? DetectLanguageFromContent(string content)
```

**Parameters:**

| Name      | Type     | Required | Description            |
| --------- | -------- | -------- | ---------------------- |
| `Content` | `string` | Yes      | The content to process |

**Returns:** `string?`

---

#### GetHighlightsQuery()

Get the highlights query for a language, if bundled.

Returns the contents of `highlights.scm` as a static string, or `null`
if no highlights query is bundled for this language.

**Signature:**

```csharp
public static string? GetHighlightsQuery(string language)
```

**Parameters:**

| Name       | Type     | Required | Description  |
| ---------- | -------- | -------- | ------------ |
| `Language` | `string` | Yes      | The language |

**Returns:** `string?`

---

#### GetInjectionsQuery()

Get the injections query for a language, if bundled.

Returns the contents of `injections.scm` as a static string, or `null`
if no injections query is bundled for this language.

**Signature:**

```csharp
public static string? GetInjectionsQuery(string language)
```

**Parameters:**

| Name       | Type     | Required | Description  |
| ---------- | -------- | -------- | ------------ |
| `Language` | `string` | Yes      | The language |

**Returns:** `string?`

---

#### GetLocalsQuery()

Get the locals query for a language, if bundled.

Returns the contents of `locals.scm` as a static string, or `null`
if no locals query is bundled for this language.

**Signature:**

```csharp
public static string? GetLocalsQuery(string language)
```

**Parameters:**

| Name       | Type     | Required | Description  |
| ---------- | -------- | -------- | ------------ |
| `Language` | `string` | Yes      | The language |

**Returns:** `string?`

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

```csharp
public static Language GetLanguage(string name)
```

**Parameters:**

| Name   | Type     | Required | Description |
| ------ | -------- | -------- | ----------- |
| `Name` | `string` | Yes      | The name    |

**Returns:** `Language`
**Errors:** Throws `Error`.

---

#### GetParser()

Get a `Parser` pre-configured for the given language.

This is a convenience function that calls `get_language` and configures
a new parser in one step.

**Errors:**

Returns `Error.LanguageNotFound` if the language is not recognized, or
`Error.ParserSetup` if the language cannot be applied to the parser.

**Signature:**

```csharp
public static Parser GetParser(string name)
```

**Parameters:**

| Name   | Type     | Required | Description |
| ------ | -------- | -------- | ----------- |
| `Name` | `string` | Yes      | The name    |

**Returns:** `Parser`
**Errors:** Throws `Error`.

---

#### DetectLanguage()

Detect language name from a file path or extension.

This compatibility alias matches the pre-Alef Python binding API.

**Signature:**

```csharp
public static string? DetectLanguage(string path)
```

**Parameters:**

| Name   | Type     | Required | Description      |
| ------ | -------- | -------- | ---------------- |
| `Path` | `string` | Yes      | Path to the file |

**Returns:** `string?`

---

#### AvailableLanguages()

List all available language names (sorted, deduplicated, includes aliases).

Returns names of both statically compiled and dynamically loadable languages,
plus any configured aliases.

**Signature:**

```csharp
public static List<string> AvailableLanguages()
```

**Returns:** `List<string>`

---

#### HasLanguage()

Check if a language is available by name or alias.

Returns `true` if the language can be loaded (statically compiled,
dynamically available, or a known alias for one of these).

**Signature:**

```csharp
public static bool HasLanguage(string name)
```

**Parameters:**

| Name   | Type     | Required | Description |
| ------ | -------- | -------- | ----------- |
| `Name` | `string` | Yes      | The name    |

**Returns:** `bool`

---

#### LanguageCount()

Return the number of available languages.

Includes statically compiled languages, dynamically loadable languages,
and aliases.

**Signature:**

```csharp
public static nuint LanguageCount()
```

**Returns:** `nuint`

---

#### Process()

Process source code and extract file intelligence using the global registry.

Parses the source with tree-sitter and extracts metrics, structure, imports,
exports, comments, docstrings, symbols, diagnostics, and/or chunks based on
the flags set in `ProcessConfig`.

**Errors:**

Returns an error if the language is not found or parsing fails.

**Signature:**

```csharp
public static ProcessResult Process(string source, ProcessConfig config)
```

**Parameters:**

| Name     | Type            | Required | Description               |
| -------- | --------------- | -------- | ------------------------- |
| `Source` | `string`        | Yes      | The source                |
| `Config` | `ProcessConfig` | Yes      | The configuration options |

**Returns:** `ProcessResult`
**Errors:** Throws `Error`.

---

#### Init()

Initialize the language pack with the given configuration.

Applies any custom cache directory, then downloads all languages and groups
specified in the config. This is the recommended entry point when you want
to pre-warm the cache before use.

**Errors:**

Returns an error if configuration cannot be applied or if downloads fail.

**Signature:**

```csharp
public static void Init(PackConfig config)
```

**Parameters:**

| Name     | Type         | Required | Description               |
| -------- | ------------ | -------- | ------------------------- |
| `Config` | `PackConfig` | Yes      | The configuration options |

**Returns:** `void`
**Errors:** Throws `Error`.

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

```csharp
public static void Configure(PackConfig config)
```

**Parameters:**

| Name     | Type         | Required | Description               |
| -------- | ------------ | -------- | ------------------------- |
| `Config` | `PackConfig` | Yes      | The configuration options |

**Returns:** `void`
**Errors:** Throws `Error`.

---

#### Download()

Download specific languages to the local cache.

Returns the number of requested languages available after the call. Already
compiled or cached languages are included in the count.

**Errors:**

Returns an error if any language is not available in the manifest or if
the download fails.

**Signature:**

```csharp
public static nuint Download(List<string> names)
```

**Parameters:**

| Name    | Type           | Required | Description |
| ------- | -------------- | -------- | ----------- |
| `Names` | `List<string>` | Yes      | The names   |

**Returns:** `nuint`
**Errors:** Throws `Error`.

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

```csharp
public static nuint DownloadAll()
```

**Returns:** `nuint`
**Errors:** Throws `Error`.

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

```csharp
public static nuint DownloadGroup(string name)
```

**Parameters:**

| Name   | Type     | Required | Description |
| ------ | -------- | -------- | ----------- |
| `Name` | `string` | Yes      | The name    |

**Returns:** `nuint`
**Errors:** Throws `Error`.

---

#### ManifestLanguages()

Return all language names available in the remote manifest (305).

Fetches (and caches) the remote manifest to discover the full list of
downloadable languages. Use `downloaded_languages` to list what is
already cached locally.

**Errors:**

Returns an error if the manifest cannot be fetched.

**Signature:**

```csharp
public static List<string> ManifestLanguages()
```

**Returns:** `List<string>`
**Errors:** Throws `Error`.

---

#### DownloadedLanguages()

Return languages that are already downloaded and cached locally.

Does not perform any network requests. Returns an empty list if the
cache directory does not exist or cannot be read.

**Signature:**

```csharp
public static List<string> DownloadedLanguages()
```

**Returns:** `List<string>`

---

#### CleanCache()

Delete all cached parser shared libraries.

Resets the cache registration so the next call to `get_language` or
a download function will re-register the (now empty) cache directory.

**Errors:**

Returns an error if the cache directory cannot be removed.

**Signature:**

```csharp
public static void CleanCache()
```

**Returns:** `void`
**Errors:** Throws `Error`.

---

#### CacheDir()

Return the effective cache directory path.

This is either the custom path set via `configure` / `init` or the
default: `~/.cache/tree-sitter-language-pack/v{version}/libs/`.

**Errors:**

Returns an error if the system cache directory cannot be determined.

**Signature:**

```csharp
public static string CacheDir()
```

**Returns:** `string`
**Errors:** Throws `Error`.

---

### Types

#### ByteRange

A byte range — start (inclusive) to end (exclusive).

| Field   | Type    | Default | Description                  |
| ------- | ------- | ------- | ---------------------------- |
| `Start` | `nuint` | —       | Inclusive start byte offset. |
| `End`   | `nuint` | —       | Exclusive end byte offset.   |

---

#### ChunkContext

Metadata for a single chunk of source code.

| Field            | Type                  | Default                     | Description         |
| ---------------- | --------------------- | --------------------------- | ------------------- |
| `Language`       | `string`              | —                           | Language            |
| `ChunkIndex`     | `nuint`               | —                           | Chunk index         |
| `TotalChunks`    | `nuint`               | —                           | Total chunks        |
| `NodeTypes`      | `List<string>`        | `new List<string>()`        | Node types          |
| `ContextPath`    | `List<string>`        | `new List<string>()`        | Context path        |
| `SymbolsDefined` | `List<string>`        | `new List<string>()`        | Symbols defined     |
| `Comments`       | `List<CommentInfo>`   | `new List<CommentInfo>()`   | Comments            |
| `Docstrings`     | `List<DocstringInfo>` | `new List<DocstringInfo>()` | Docstrings          |
| `HasErrorNodes`  | `bool`                | —                           | Whether error nodes |

---

#### CodeChunk

A chunk of source code with rich metadata.

| Field       | Type           | Default | Description                |
| ----------- | -------------- | ------- | -------------------------- |
| `Content`   | `string`       | —       | The extracted text content |
| `StartByte` | `nuint`        | —       | Start byte                 |
| `EndByte`   | `nuint`        | —       | End byte                   |
| `StartLine` | `nuint`        | —       | Start line                 |
| `EndLine`   | `nuint`        | —       | End line                   |
| `Metadata`  | `ChunkContext` | —       | Document metadata          |

---

#### CommentInfo

A comment extracted from source code.

| Field            | Type          | Default            | Description         |
| ---------------- | ------------- | ------------------ | ------------------- |
| `Text`           | `string`      | —                  | Text                |
| `Kind`           | `CommentKind` | `CommentKind.Line` | Kind (comment kind) |
| `Span`           | `Span`        | —                  | Span (span)         |
| `AssociatedNode` | `string?`     | `null`             | Associated node     |

---

#### Diagnostic

A diagnostic (syntax error, missing node, etc.) from parsing.

| Field      | Type                 | Default                    | Description                    |
| ---------- | -------------------- | -------------------------- | ------------------------------ |
| `Message`  | `string`             | —                          | Message                        |
| `Severity` | `DiagnosticSeverity` | `DiagnosticSeverity.Error` | Severity (diagnostic severity) |
| `Span`     | `Span`               | —                          | Span (span)                    |

---

#### DocSection

A section within a docstring (e.g., Args, Returns, Raises).

| Field         | Type      | Default | Description                |
| ------------- | --------- | ------- | -------------------------- |
| `Kind`        | `string`  | —       | Kind                       |
| `Name`        | `string?` | `null`  | The name                   |
| `Description` | `string`  | —       | Human-readable description |

---

#### DocstringInfo

A docstring extracted from source code.

| Field            | Type               | Default                             | Description               |
| ---------------- | ------------------ | ----------------------------------- | ------------------------- |
| `Text`           | `string`           | —                                   | Text                      |
| `Format`         | `DocstringFormat`  | `DocstringFormat.PythonTripleQuote` | Format (docstring format) |
| `Span`           | `Span`             | —                                   | Span (span)               |
| `AssociatedItem` | `string?`          | `null`                              | Associated item           |
| `ParsedSections` | `List<DocSection>` | `new List<DocSection>()`            | Parsed sections           |

---

#### DownloadManager

Manages downloading and caching of pre-built parser shared libraries.

### Methods

#### New()

Create a new download manager for the given version.

**Signature:**

```csharp
public DownloadManager New(string version)
```

#### WithCacheDir()

Create a download manager with a custom cache directory.

**Signature:**

```csharp
public DownloadManager WithCacheDir(string version, string cacheDir)
```

#### InstalledLanguages()

List languages that are already downloaded and cached.

**Signature:**

```csharp
public List<string> InstalledLanguages()
```

#### DownloadAllBestEffort()

Download the platform bundle and extract every library file it contains.

Unlike `ensure_languages`, this does not check the manifest language list
against archive contents — it simply extracts all `.so`/`.dylib`/`.dll` files
from the bundle. Languages in the manifest that are missing from the archive
are silently ignored rather than returning an error.

Returns the number of library files extracted (including those already cached).

**Signature:**

```csharp
public nuint DownloadAllBestEffort()
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

```csharp
public void CleanCache()
```

---

#### ExportInfo

An export statement extracted from source code.

| Field  | Type         | Default            | Description        |
| ------ | ------------ | ------------------ | ------------------ |
| `Name` | `string`     | —                  | The name           |
| `Kind` | `ExportKind` | `ExportKind.Named` | Kind (export kind) |
| `Span` | `Span`       | —                  | Span (span)        |

---

#### FileMetrics

Aggregate metrics for a source file.

| Field          | Type    | Default | Description      |
| -------------- | ------- | ------- | ---------------- |
| `TotalLines`   | `nuint` | —       | Total lines      |
| `CodeLines`    | `nuint` | —       | Code lines       |
| `CommentLines` | `nuint` | —       | Comment lines    |
| `BlankLines`   | `nuint` | —       | Blank lines      |
| `TotalBytes`   | `nuint` | —       | Total bytes      |
| `NodeCount`    | `nuint` | —       | Number of nodes  |
| `ErrorCount`   | `nuint` | —       | Number of errors |
| `MaxDepth`     | `nuint` | —       | Maximum depth    |

---

#### ImportInfo

An import statement extracted from source code.

| Field        | Type           | Default              | Description      |
| ------------ | -------------- | -------------------- | ---------------- |
| `Source`     | `string`       | —                    | Source           |
| `Items`      | `List<string>` | `new List<string>()` | Items            |
| `Alias`      | `string?`      | `null`               | Alias            |
| `IsWildcard` | `bool`         | —                    | Whether wildcard |
| `Span`       | `Span`         | —                    | Span (span)      |

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

```csharp
public Language GetLanguage(string name)
```

#### AvailableLanguages()

List all available language names, sorted and deduplicated.

Includes statically compiled languages, dynamically loadable languages
(if the `dynamic-loading` feature is enabled), and all configured aliases.

**Signature:**

```csharp
public List<string> AvailableLanguages()
```

#### HasLanguage()

Check whether a language is available by name or alias.

Returns `true` if the language can be loaded, either from the static
table or from a dynamic library on disk.

**Signature:**

```csharp
public bool HasLanguage(string name)
```

#### LanguageCount()

Return the total number of available languages (including aliases).

**Signature:**

```csharp
public nuint LanguageCount()
```

#### Process()

Parse source code and extract file intelligence based on config in a single pass.

**Signature:**

```csharp
public ProcessResult Process(string source, ProcessConfig config)
```

#### CreateDefault()

**Signature:**

```csharp
public LanguageRegistry CreateDefault()
```

---

#### Node

A single syntax node within a `Tree`.

Nodes hold a strong reference to their parent tree so they remain valid
regardless of how the tree is moved or stored at the FFI boundary.

### Methods

#### Clone()

**Signature:**

```csharp
public Node Clone()
```

#### Kind()

Return the node's kind name (e.g. `"function_definition"`).

**Signature:**

```csharp
public string Kind()
```

#### KindId()

Return the node's numeric kind ID.

Tree-sitter assigns a stable `u16` ID to every node kind in a grammar
(e.g. `"function_definition" → 42`). Comparing `kind_id()` is cheaper
than comparing the string `kind()` in tight AST loops.

**Signature:**

```csharp
public ushort KindId()
```

#### StartByte()

Return the inclusive start byte offset of this node.

**Signature:**

```csharp
public nuint StartByte()
```

#### EndByte()

Return the exclusive end byte offset of this node.

**Signature:**

```csharp
public nuint EndByte()
```

#### ByteRange()

Return the node's byte range as a `ByteRange`.

Callers should slice their own source bytes — this is a zero-copy
text accessor.

**Signature:**

```csharp
public ByteRange ByteRange()
```

#### StartPosition()

Return the start `Point` (row, column).

**Signature:**

```csharp
public Point StartPosition()
```

#### EndPosition()

Return the end `Point` (row, column).

**Signature:**

```csharp
public Point EndPosition()
```

#### IsNamed()

True when this node is named (not punctuation/whitespace).

**Signature:**

```csharp
public bool IsNamed()
```

#### IsError()

True when this is an error node.

**Signature:**

```csharp
public bool IsError()
```

#### IsMissing()

True when this is a missing-token node.

**Signature:**

```csharp
public bool IsMissing()
```

#### IsExtra()

True when this is an "extra" node (e.g. a comment).

**Signature:**

```csharp
public bool IsExtra()
```

#### HasError()

True when this node or any descendant is an error.

**Signature:**

```csharp
public bool HasError()
```

#### Parent()

Return this node's parent, if any.

**Signature:**

```csharp
public Node? Parent()
```

#### Child()

Return the i-th child of this node, if any.

**Signature:**

```csharp
public Node? Child(uint index)
```

#### ChildCount()

Total number of children (including unnamed).

**Signature:**

```csharp
public nuint ChildCount()
```

#### NamedChild()

Return the i-th named child of this node, if any.

**Signature:**

```csharp
public Node? NamedChild(uint index)
```

#### NamedChildCount()

Number of named children of this node.

**Signature:**

```csharp
public nuint NamedChildCount()
```

#### ChildByFieldName()

Look up a child by its grammar-defined field name.

**Signature:**

```csharp
public Node? ChildByFieldName(string name)
```

#### ToSexp()

Return the S-expression form of this node's subtree.

**Signature:**

```csharp
public string ToSexp()
```

#### Walk()

Return a `TreeCursor` positioned at this node.

**Signature:**

```csharp
public TreeCursor Walk()
```

---

#### PackConfig

Configuration for the tree-sitter language pack.

Controls cache directory and which languages to pre-download.
Can be loaded from a TOML file, constructed programmatically,
or passed as a dict/object from language bindings.

| Field       | Type            | Default              | Description                                                                                      |
| ----------- | --------------- | -------------------- | ------------------------------------------------------------------------------------------------ |
| `CacheDir`  | `string?`       | `null`               | Override default cache directory. Default: `~/.cache/tree-sitter-language-pack/v{version}/libs/` |
| `Languages` | `List<string>?` | `new List<string>()` | Languages to pre-download on init. Each entry is a language name (e.g. `"python"`, `"rust"`).    |
| `Groups`    | `List<string>?` | `new List<string>()` | Language groups to pre-download (e.g. `"web"`, `"systems"`, `"scripting"`).                      |

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

```csharp
public void SetLanguage(string name)
```

#### Parse()

Parse a UTF-8 source string. Returns `null` if parsing was cancelled
or no language is set.

**Signature:**

```csharp
public Tree? Parse(string source)
```

#### ParseBytes()

Parse a raw byte slice. Returns `null` if parsing was cancelled or
no language is set.

**Signature:**

```csharp
public Tree? ParseBytes(byte[] source)
```

#### Reset()

Reset internal state. The next call to `parse` will
not be incremental.

**Signature:**

```csharp
public void Reset()
```

#### CreateDefault()

**Signature:**

```csharp
public Parser CreateDefault()
```

---

#### Point

A source position — row + column, zero-indexed.

| Field    | Type    | Default | Description                                       |
| -------- | ------- | ------- | ------------------------------------------------- |
| `Row`    | `nuint` | —       | Zero-indexed row number.                          |
| `Column` | `nuint` | —       | Zero-indexed column number, in UTF-16 code units. |

---

#### ProcessConfig

Configuration for the `process()` function.

Controls which analysis features are enabled and whether chunking is performed.

| Field          | Type     | Default | Description                                                         |
| -------------- | -------- | ------- | ------------------------------------------------------------------- |
| `Language`     | `string` | —       | Language name (required).                                           |
| `Structure`    | `bool`   | `true`  | Extract structural items (functions, classes, etc.). Default: true. |
| `Imports`      | `bool`   | `true`  | Extract import statements. Default: true.                           |
| `Exports`      | `bool`   | `true`  | Extract export statements. Default: true.                           |
| `Comments`     | `bool`   | `false` | Extract comments. Default: false.                                   |
| `Docstrings`   | `bool`   | `false` | Extract docstrings. Default: false.                                 |
| `Symbols`      | `bool`   | `false` | Extract symbol definitions. Default: false.                         |
| `Diagnostics`  | `bool`   | `false` | Include parse diagnostics. Default: false.                          |
| `ChunkMaxSize` | `nuint?` | `null`  | Maximum chunk size in bytes. `null` disables chunking.              |

### Methods

#### CreateDefault()

**Signature:**

```csharp
public ProcessConfig CreateDefault()
```

#### WithChunking()

Enable chunking with the given maximum chunk size in bytes.

**Signature:**

```csharp
public ProcessConfig WithChunking(nuint maxSize)
```

#### All()

Enable all analysis features.

**Signature:**

```csharp
public ProcessConfig All()
```

#### Minimal()

Disable all analysis features (only metrics computed).

**Signature:**

```csharp
public ProcessConfig Minimal()
```

---

#### ProcessResult

Complete analysis result from processing a source file.

Contains metrics, structural analysis, imports/exports, comments,
docstrings, symbols, diagnostics, and optionally chunked code segments.
Fields are populated based on the `ProcessConfig` flags.

| Field         | Type                  | Default                     | Description                        |
| ------------- | --------------------- | --------------------------- | ---------------------------------- |
| `Language`    | `string`              | —                           | Language                           |
| `Metrics`     | `FileMetrics`         | —                           | Metrics (file metrics)             |
| `Structure`   | `List<StructureItem>` | `new List<StructureItem>()` | Structure                          |
| `Imports`     | `List<ImportInfo>`    | `new List<ImportInfo>()`    | Imports                            |
| `Exports`     | `List<ExportInfo>`    | `new List<ExportInfo>()`    | Exports                            |
| `Comments`    | `List<CommentInfo>`   | `new List<CommentInfo>()`   | Comments                           |
| `Docstrings`  | `List<DocstringInfo>` | `new List<DocstringInfo>()` | Docstrings                         |
| `Symbols`     | `List<SymbolInfo>`    | `new List<SymbolInfo>()`    | Symbols                            |
| `Diagnostics` | `List<Diagnostic>`    | `new List<Diagnostic>()`    | Diagnostics                        |
| `Chunks`      | `List<CodeChunk>`     | `new List<CodeChunk>()`     | Text chunks for chunking/embedding |

---

#### Span

Byte and line/column range in source code.

Represents both byte offsets (for slicing) and human-readable line/column
positions (for display and diagnostics).

| Field         | Type    | Default | Description  |
| ------------- | ------- | ------- | ------------ |
| `StartByte`   | `nuint` | —       | Start byte   |
| `EndByte`     | `nuint` | —       | End byte     |
| `StartLine`   | `nuint` | —       | Start line   |
| `StartColumn` | `nuint` | —       | Start column |
| `EndLine`     | `nuint` | —       | End line     |
| `EndColumn`   | `nuint` | —       | End column   |

---

#### StructureItem

A structural item (function, class, struct, etc.) in source code.

| Field        | Type                  | Default                     | Description           |
| ------------ | --------------------- | --------------------------- | --------------------- |
| `Kind`       | `StructureKind`       | `StructureKind.Function`    | Kind (structure kind) |
| `Name`       | `string?`             | `null`                      | The name              |
| `Visibility` | `string?`             | `null`                      | Visibility            |
| `Span`       | `Span`                | —                           | Span (span)           |
| `Children`   | `List<StructureItem>` | `new List<StructureItem>()` | Children              |
| `Decorators` | `List<string>`        | `new List<string>()`        | Decorators            |
| `DocComment` | `string?`             | `null`                      | Doc comment           |
| `Signature`  | `string?`             | `null`                      | Signature             |
| `BodySpan`   | `Span?`               | `null`                      | Body span (span)      |

---

#### SymbolInfo

A symbol (variable, function, type, etc.) extracted from source code.

| Field            | Type         | Default               | Description        |
| ---------------- | ------------ | --------------------- | ------------------ |
| `Name`           | `string`     | —                     | The name           |
| `Kind`           | `SymbolKind` | `SymbolKind.Variable` | Kind (symbol kind) |
| `Span`           | `Span`       | —                     | Span (span)        |
| `TypeAnnotation` | `string?`    | `null`                | Type annotation    |
| `Doc`            | `string?`    | `null`                | Doc                |

---

#### Tree

A parsed syntax tree. Cheap to clone (refcount bump).

### Methods

#### RootNode()

Return the root `Node` of this tree.

**Signature:**

```csharp
public Node RootNode()
```

#### Walk()

Return a `TreeCursor` positioned at the root.

**Signature:**

```csharp
public TreeCursor Walk()
```

---

#### TreeCursor

A cursor for traversing a `Tree`.

### Methods

#### Node()

Return the `Node` at the cursor's current position.

**Signature:**

```csharp
public Node Node()
```

#### GotoFirstChild()

Move the cursor to the first child of the current node.
Returns `true` if a child existed.

**Signature:**

```csharp
public bool GotoFirstChild()
```

#### GotoParent()

Move the cursor to the parent of the current node.
Returns `true` if a parent existed.

**Signature:**

```csharp
public bool GotoParent()
```

#### GotoNextSibling()

Move the cursor to the next sibling of the current node.
Returns `true` if a sibling existed.

**Signature:**

```csharp
public bool GotoNextSibling()
```

#### FieldName()

Return the field name for the current node, if any.

**Signature:**

```csharp
public string? FieldName()
```

---

### Enums

#### StructureKind

The kind of structural item found in source code.

Categorizes top-level and nested declarations such as functions, classes,
structs, enums, traits, and more. Use `Other` for
language-specific constructs that do not fit a standard category.

| Value       | Description                   |
| ----------- | ----------------------------- |
| `Function`  | Function                      |
| `Method`    | Method                        |
| `Class`     | Class                         |
| `Struct`    | Struct                        |
| `Interface` | Interface                     |
| `Enum`      | Enum                          |
| `Module`    | Module                        |
| `Trait`     | Trait                         |
| `Impl`      | Impl                          |
| `Namespace` | Namespace                     |
| `Other`     | Other — Fields: `0`: `string` |

---

#### CommentKind

The kind of a comment found in source code.

Distinguishes between single-line comments, block (multi-line) comments,
and documentation comments.

| Value   | Description |
| ------- | ----------- |
| `Line`  | Line        |
| `Block` | Block       |
| `Doc`   | Doc         |

---

#### DocstringFormat

The format of a docstring extracted from source code.

Identifies the docstring convention used, which varies by language
(e.g., Python triple-quoted strings, JSDoc, Rustdoc `///` comments).

| Value               | Description                   |
| ------------------- | ----------------------------- |
| `PythonTripleQuote` | Python triple quote           |
| `JsDoc`             | J s doc                       |
| `Rustdoc`           | Rustdoc                       |
| `GoDoc`             | Go doc                        |
| `JavaDoc`           | Java doc                      |
| `Other`             | Other — Fields: `0`: `string` |

---

#### ExportKind

The kind of an export statement found in source code.

Covers named exports, default exports, and re-exports from other modules.

| Value      | Description |
| ---------- | ----------- |
| `Named`    | Named       |
| `Default`  | Default     |
| `ReExport` | Re export   |

---

#### SymbolKind

The kind of a symbol definition found in source code.

Categorizes symbol definitions such as variables, constants, functions,
classes, types, interfaces, enums, and modules.

| Value       | Description                   |
| ----------- | ----------------------------- |
| `Variable`  | Variable                      |
| `Constant`  | Constant                      |
| `Function`  | Function                      |
| `Class`     | Class                         |
| `Type`      | Type                          |
| `Interface` | Interface                     |
| `Enum`      | Enum                          |
| `Module`    | Module                        |
| `Other`     | Other — Fields: `0`: `string` |

---

#### DiagnosticSeverity

Severity level of a diagnostic produced during parsing.

Used to classify parse errors, warnings, and informational messages
found in the syntax tree.

| Value     | Description |
| --------- | ----------- |
| `Error`   | Error       |
| `Warning` | Warning     |
| `Info`    | Info        |

---

### Errors

#### Error

Errors that can occur when using the tree-sitter language pack.

Covers language lookup failures, parse errors, query errors, and I/O issues.
Feature-gated variants are included when `config`, `download`, or related
features are enabled.

| Variant               | Description                                       |
| --------------------- | ------------------------------------------------- |
| `LanguageNotFound`    | Language '{0}' not found                          |
| `DynamicLoad`         | Dynamic library load error: {0}                   |
| `NullLanguagePointer` | Language function returned null pointer for '{0}' |
| `ParserSetup`         | Failed to set parser language: {0}                |
| `LockPoisoned`        | Registry lock poisoned: {0}                       |
| `Config`              | Configuration error: {0}                          |
| `ParseFailed`         | Parse failed: parsing returned no tree            |
| `QueryError`          | Query error: {0}                                  |
| `InvalidRange`        | Invalid byte range: {0}                           |
| `Io`                  | IO error: {0}                                     |

---
