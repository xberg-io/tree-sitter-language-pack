---
title: "C# API Reference"
---

## C# API Reference <span class="version-badge">v1.9.0-rc.49</span>

### Functions

#### DetectLanguageFromExtension()

Detect language name from a file extension (without leading dot).

Returns `null` for unrecognized extensions. The match is case-insensitive.

**Signature:**

```csharp
public static string? DetectLanguageFromExtension(string ext)
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `Ext` | `string` | Yes | The ext |

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

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `Path` | `string` | Yes | Path to the file |

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

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `Content` | `string` | Yes | The content to process |

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

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `Language` | `string` | Yes | The language |

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

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `Language` | `string` | Yes | The language |

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

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `Language` | `string` | Yes | The language |

**Returns:** `string?`

---

#### GetTagsQuery()

Get the tags query for a language, if bundled.

Returns the contents of `tags.scm` as a static string, or `null`
if no tags query is bundled for this language.

**Signature:**

```csharp
public static string? GetTagsQuery(string language)
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `Language` | `string` | Yes | The language |

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

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `Name` | `string` | Yes | The name |

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

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `Name` | `string` | Yes | The name |

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

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `Path` | `string` | Yes | Path to the file |

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

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `Source` | `string` | Yes | The source |
| `Config` | `ProcessConfig` | Yes | The configuration options |

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

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `Config` | `PackConfig` | Yes | The configuration options |

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

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `Config` | `PackConfig` | Yes | The configuration options |

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

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `Names` | `List<string>` | Yes | The names |

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
`download()`. Already-cached languages are skipped.

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

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `Name` | `string` | Yes | The name |

**Returns:** `nuint`
**Errors:** Throws `Error`.

---

#### ManifestLanguages()

Return all language names available in the remote manifest (306).

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

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Start` | `nuint` | — | Inclusive start byte offset. |
| `End` | `nuint` | — | Exclusive end byte offset. |

---

#### ChunkContext

Metadata for a single chunk of source code.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Language` | `string` | — | Language name used to parse this chunk. |
| `ChunkIndex` | `nuint` | — | Zero-indexed position of this chunk within the file's chunk list. |
| `TotalChunks` | `nuint` | — | Total number of chunks the file was split into. |
| `NodeTypes` | `List<string>` | `new List<string>()` | Tree-sitter node kinds that appear at the top level of this chunk. |
| `ContextPath` | `List<string>` | `new List<string>()` | Hierarchical path of enclosing structural items (e.g., `["MyClass", "my_method"]`). |
| `SymbolsDefined` | `List<string>` | `new List<string>()` | Names of symbols defined within this chunk. |
| `Comments` | `List<CommentInfo>` | `new List<CommentInfo>()` | Comments contained within this chunk. |
| `Docstrings` | `List<DocstringInfo>` | `new List<DocstringInfo>()` | Docstrings contained within this chunk. |
| `HasErrorNodes` | `bool` | — | Whether this chunk contains any tree-sitter error nodes. |

---

#### CodeChunk

A chunk of source code with rich metadata.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Content` | `string` | — | The raw source text of this chunk. |
| `StartByte` | `nuint` | — | Inclusive start byte offset of this chunk in the original source. |
| `EndByte` | `nuint` | — | Exclusive end byte offset of this chunk in the original source. |
| `StartLine` | `nuint` | — | Zero-indexed start line of this chunk. |
| `EndLine` | `nuint` | — | Zero-indexed end line of this chunk. |
| `Metadata` | `ChunkContext` | — | Contextual metadata about this chunk. |

---

#### CommentInfo

A comment extracted from source code.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Text` | `string` | — | The raw text content of the comment. |
| `Kind` | `CommentKind` | `CommentKind.Line` | The kind of comment (line, block, or doc). |
| `Span` | `Span` | — | Source span covering the comment. |
| `AssociatedNode` | `string?` | `null` | Name of the syntax node this comment is directly associated with. |

---

#### DataAttribute

An XML-style attribute attached to an `Element` node.

Populated only for `DataNodeKind.Element`; always empty for `KeyValue` and
`Sequence` nodes.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Name` | `string` | — | Attribute name (e.g. `"class"`, `"href"`). |
| `Value` | `string` | — | Attribute value as a raw string (quotes stripped). |
| `Span` | `Span` | — | Source span covering the entire `name="value"` attribute token. |

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
| `Kind` | `DataNodeKind` | `DataNodeKind.KeyValue` | Whether this node is a key/value pair, XML element, or sequence item. |
| `Key` | `string?` | `null` | Key, attribute name, tag name, or positional index (`"0"`, `"1"`, …). `null` at the document root. |
| `Value` | `string?` | `null` | Leaf scalar value, if any. `null` for containers (objects, arrays, XML elements with child elements). |
| `Attributes` | `List<DataAttribute>` | `new List<DataAttribute>()` | Attributes on element-shape nodes (XML `STag` attributes). Empty for all other kinds. |
| `Children` | `List<DataNode>` | `new List<DataNode>()` | Children for nested containers and XML element bodies. |
| `Span` | `Span` | — | Source span covering this node in the original source file. |

---

#### Diagnostic

A diagnostic (syntax error, missing node, etc.) from parsing.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Message` | `string` | — | Human-readable description of the diagnostic. |
| `Severity` | `DiagnosticSeverity` | `DiagnosticSeverity.Error` | Severity of the diagnostic. |
| `Span` | `Span` | — | Source span where the diagnostic was detected. |

---

#### DocSection

A section within a docstring (e.g., Args, Returns, Raises).

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Kind` | `string` | — | Section kind (e.g., `"args"`, `"returns"`, `"raises"`). |
| `Name` | `string?` | `null` | Parameter or return value name, if applicable. |
| `Description` | `string` | — | Description text for this section. |

---

#### DocstringInfo

A docstring extracted from source code.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Text` | `string` | — | The raw text of the docstring. |
| `Format` | `DocstringFormat` | `DocstringFormat.PythonTripleQuote` | The docstring format (Python, JSDoc, Rustdoc, etc.). |
| `Span` | `Span` | — | Source span covering the docstring. |
| `AssociatedItem` | `string?` | `null` | Name of the item this docstring documents. |
| `ParsedSections` | `List<DocSection>` | `new List<DocSection>()` | Parsed sections of the docstring (Args, Returns, Raises, etc.). |

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

#### InstalledLanguages()

List languages that are already downloaded and cached.

**Signature:**

```csharp
public List<string> InstalledLanguages()
```

#### DownloadAllBestEffort()

Download the platform bundle and extract every library file it contains.

Unlike `Self.ensure_languages`, this does not check the manifest language list
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

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Name` | `string` | — | The exported name. |
| `Kind` | `ExportKind` | `ExportKind.Named` | The kind of export (named, default, or re-export). |
| `Span` | `Span` | — | Source span covering the export statement. |

---

#### FileMetrics

Aggregate metrics for a source file.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `TotalLines` | `nuint` | — | Total number of lines (including blank and comment lines). |
| `CodeLines` | `nuint` | — | Number of lines containing non-blank, non-comment source code. |
| `CommentLines` | `nuint` | — | Number of lines that are entirely comments. |
| `BlankLines` | `nuint` | — | Number of blank (whitespace-only) lines. |
| `TotalBytes` | `nuint` | — | Total byte length of the source file. |
| `NodeCount` | `nuint` | — | Total number of nodes in the syntax tree. |
| `ErrorCount` | `nuint` | — | Number of error nodes in the syntax tree (parse errors). |
| `MaxDepth` | `nuint` | — | Maximum nesting depth reached in the syntax tree. |

---

#### ImportInfo

An import statement extracted from source code.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Source` | `string` | — | The module or path being imported from. |
| `Items` | `List<string>` | `new List<string>()` | Specific names imported from the source module. |
| `Alias` | `string?` | `null` | Alias assigned to the import (e.g., `import numpy as np`). |
| `IsWildcard` | `bool` | — | Whether this is a wildcard import (e.g., `import *` or `use foo.*`). |
| `Span` | `Span` | — | Source span covering the import statement. |

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

#### New()

Create a new registry populated with all statically compiled languages.

When the `dynamic-loading` feature is enabled, the registry also knows
about dynamically loadable grammars and will load them on demand.

**Signature:**

```csharp
public LanguageRegistry New()
```

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

#### HasParser()

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

```csharp
public bool HasParser(string name)
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

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `CacheDir` | `string?` | `null` | Override default cache directory. Default: `~/.cache/tree-sitter-language-pack/v{version}/libs/` |
| `Languages` | `List<string>?` | `new List<string>()` | Languages to pre-download on init. Each entry is a language name (e.g. `"python"`, `"rust"`). |
| `Groups` | `List<string>?` | `new List<string>()` | Language groups to pre-download (e.g. `"web"`, `"systems"`, `"scripting"`). |

---

#### Parser

A tree-sitter parser configured for one language at a time.

### Methods

#### New()

Construct a new parser with no language set.

Call `Parser.set_language` before parsing.

**Signature:**

```csharp
public Parser New()
```

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

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Row` | `nuint` | — | Zero-indexed row number. |
| `Column` | `nuint` | — | Zero-indexed column number, in UTF-16 code units. |

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
| `ChunkMaxSize` | `nuint?` | `null` | Maximum chunk size in bytes. `null` disables chunking. |
| `DataExtraction` | `bool` | `false` | Extract hierarchical key/value data tree from data-format files. Default: false. When `true`, `ProcessResult.data` is populated with a `DataNode` tree for supported languages: JSON, YAML, TOML, `.properties`, HCL/HOCON, INI, editorconfig, KDL, CUE, CSV, PSV, PO, nginx config, Caddy config, XML, and DTD. For languages outside this set the field is left as `null`. |

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

#### WithDataExtraction()

Enable or disable hierarchical data extraction for data-format files.

When `true`, `ProcessResult.data` is
populated with a key/value tree for supported data-format languages.

**Signature:**

```csharp
public ProcessConfig WithDataExtraction(bool enabled)
```

---

#### ProcessResult

Complete analysis result from processing a source file.

Contains metrics, structural analysis, imports/exports, comments,
docstrings, symbols, diagnostics, and optionally chunked code segments.
Fields are populated based on the `ProcessConfig` flags.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Language` | `string` | — | The language name used to parse the source file. |
| `Metrics` | `FileMetrics` | — | File-level metrics (line counts, byte size, error count). |
| `Structure` | `List<StructureItem>` | `new List<StructureItem>()` | Top-level structural items (functions, classes, etc.). |
| `Imports` | `List<ImportInfo>` | `new List<ImportInfo>()` | Import statements extracted from the source. |
| `Exports` | `List<ExportInfo>` | `new List<ExportInfo>()` | Export statements extracted from the source. |
| `Comments` | `List<CommentInfo>` | `new List<CommentInfo>()` | Comments extracted from the source. |
| `Docstrings` | `List<DocstringInfo>` | `new List<DocstringInfo>()` | Docstrings extracted from the source. |
| `Symbols` | `List<SymbolInfo>` | `new List<SymbolInfo>()` | Symbol definitions (variables, types, functions) extracted from the source. |
| `Diagnostics` | `List<Diagnostic>` | `new List<Diagnostic>()` | Parse diagnostics (syntax errors, missing nodes) from tree-sitter. |
| `Chunks` | `List<CodeChunk>` | `new List<CodeChunk>()` | Syntax-aware code chunks produced when chunking is enabled. |
| `Data` | `DataNode?` | `null` | Hierarchical data tree extracted when `config.data_extraction` is `true`. Populated for supported data-format languages (JSON, YAML, TOML, properties, HCL, INI, XML, CSV, and more). `null` when `data_extraction` is `false` (the default) or when the language is not a recognised data format. See `DataNode` for the shape of the returned tree. |

---

#### Span

Byte and line/column range in source code.

Represents both byte offsets (for slicing) and human-readable line/column
positions (for display and diagnostics).

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `StartByte` | `nuint` | — | Inclusive start byte offset in the source. |
| `EndByte` | `nuint` | — | Exclusive end byte offset in the source. |
| `StartLine` | `nuint` | — | Zero-indexed line number of the span's start. |
| `StartColumn` | `nuint` | — | Zero-indexed column number of the span's start. |
| `EndLine` | `nuint` | — | Zero-indexed line number of the span's end. |
| `EndColumn` | `nuint` | — | Zero-indexed column number of the span's end. |

---

#### StructureItem

A structural item (function, class, struct, etc.) in source code.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Kind` | `StructureKind` | `StructureKind.Function` | The kind of structural item. |
| `Name` | `string?` | `null` | The declared name of the item, if present. |
| `Visibility` | `string?` | `null` | Visibility modifier (e.g., `"pub"`, `"public"`, `"private"`). |
| `Span` | `Span` | — | Source span covering the entire item declaration. |
| `Children` | `List<StructureItem>` | `new List<StructureItem>()` | Nested structural items (e.g., methods within a class). |
| `Decorators` | `List<string>` | `new List<string>()` | Decorator or attribute names applied to the item. |
| `DocComment` | `string?` | `null` | Documentation comment attached to the item, if any. |
| `Signature` | `string?` | `null` | Full signature text of the item (e.g., function parameters and return type). |
| `BodySpan` | `Span?` | `null` | Source span covering only the body of the item, if distinct from the declaration. |

---

#### SymbolInfo

A symbol (variable, function, type, etc.) extracted from source code.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Name` | `string` | — | The name of the symbol. |
| `Kind` | `SymbolKind` | `SymbolKind.Variable` | The kind of symbol (variable, function, class, etc.). |
| `Span` | `Span` | — | Source span covering the symbol definition. |
| `TypeAnnotation` | `string?` | `null` | Explicit type annotation, if present in the source. |
| `Doc` | `string?` | `null` | Documentation comment associated with this symbol. |

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
| `KeyValue` | A key/value pair or mapping (json/toml/properties/yaml/hcl/cue/kdl pair, or a wrapper "object"/"mapping" container). |
| `Element` | An XML element with a tag name in `key` and attributes in `attributes`. |
| `Sequence` | A positional sequence item (JSON array element, YAML block sequence item, CSV/PSV row or cell). |

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
| `Other` | A language-specific construct that does not fit any standard category. — Fields: `0`: `string` |

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

### Wire format (public JSON contract)

Unit variants serialize as a bare string (`"JSDoc"`); the `Other`
variant serializes as a single-keyed object (`{"Other": "rst"}`). DO
NOT add `#[serde(tag = "...")]`. Covered by `tests/wire_format.rs`.

| Value | Description |
|-------|-------------|
| `PythonTripleQuote` | Python triple-quoted string docstring (`"""..."""`). |
| `JsDoc` | JavaScript/TypeScript JSDoc block comment (opens with two stars, closes with star-slash). |
| `Rustdoc` | Rust `///` or `//!` doc comment. |
| `GoDoc` | Go doc comment (a comment block immediately preceding a declaration). |
| `JavaDoc` | Java Javadoc block comment (opens with two stars, closes with star-slash). |
| `Other` | A language-specific docstring format not covered by the standard variants. — Fields: `0`: `string` |

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

### Wire format (public JSON contract)

Unit variants serialize as a bare string (`"Function"`); the `Other`
variant serializes as a single-keyed object (`{"Other": "macro"}`). DO
NOT add `#[serde(tag = "...")]`. Covered by `tests/wire_format.rs`.

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
| `Other` | A symbol kind not covered by the standard variants. — Fields: `0`: `string` |

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
