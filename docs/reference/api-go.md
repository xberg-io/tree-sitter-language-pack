---
title: "Go API Reference"
---

## Go API Reference <span class="version-badge">v1.10.3</span>

### Functions

#### DetectLanguageFromExtension()

Detect language name from a file extension (without leading dot).

Returns `nil` for unrecognized extensions. The match is case-insensitive.

**Signature:**

```go
func DetectLanguageFromExtension(ext string) *string
```

**Example:**

```go
result := DetectLanguageFromExtension("value")
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

**Example:**

```go
result := DetectLanguageFromPath("value")
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

**Example:**

```go
result := DetectLanguageFromContent("value")
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

**Example:**

```go
result := GetHighlightsQuery("value")
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

**Example:**

```go
result := GetInjectionsQuery("value")
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

**Example:**

```go
result := GetLocalsQuery("value")
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `Language` | `string` | Yes | The language |

**Returns:** `*string`

---

#### GetTagsQuery()

Get the tags query for a language, if bundled.

Returns the contents of `tags.scm` as a static string, or `nil`
if no tags query is bundled for this language.

**Signature:**

```go
func GetTagsQuery(language string) *string
```

**Example:**

```go
result := GetTagsQuery("value")
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

**Example:**

```go
result, err := GetLanguage("value")
if err != nil {
    return err
}
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

**Example:**

```go
result, err := GetParser("value")
if err != nil {
    return err
}
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

**Example:**

```go
result := DetectLanguage("value")
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

**Example:**

```go
result := AvailableLanguages()
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

**Example:**

```go
result := HasLanguage("value")
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

**Example:**

```go
result := LanguageCount()
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

**Example:**

```go
result, err := Process("value", ProcessConfig{})
if err != nil {
    return err
}
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

**Example:**

```go
if err := Init(PackConfig{}); err != nil {
    return err
}
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `Config` | `PackConfig` | Yes | The configuration options |

**Returns:** No return value.

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

**Example:**

```go
if err := Configure(PackConfig{}); err != nil {
    return err
}
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `Config` | `PackConfig` | Yes | The configuration options |

**Returns:** No return value.

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

**Example:**

```go
result, err := Download(nil)
if err != nil {
    return err
}
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `Names` | `\[\]string` | Yes | The names |

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

**Example:**

```go
result, err := DownloadAll()
if err != nil {
    return err
}
```

**Returns:** `int`

**Errors:** Returns `error`.

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

```go
func DownloadGroup(name string) (int, error)
```

**Example:**

```go
result, err := DownloadGroup("value")
if err != nil {
    return err
}
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

**Example:**

```go
result, err := ManifestLanguages()
if err != nil {
    return err
}
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

**Example:**

```go
result := DownloadedLanguages()
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

**Example:**

```go
if err := CleanCache(); err != nil {
    return err
}
```

**Returns:** No return value.

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

**Example:**

```go
result, err := CacheDir()
if err != nil {
    return err
}
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
| `Language` | `string` | — | Language name used to parse this chunk. |
| `ChunkIndex` | `int` | — | Zero-indexed position of this chunk within the file's chunk list. |
| `TotalChunks` | `int` | — | Total number of chunks the file was split into. |
| `NodeTypes` | `\[\]string` | `nil` | Tree-sitter node kinds that appear at the top level of this chunk. |
| `ContextPath` | `\[\]string` | `nil` | Hierarchical path of enclosing structural items (e.g., `\["MyClass", "my_method"\]`). |
| `SymbolsDefined` | `\[\]string` | `nil` | Names of symbols defined within this chunk. |
| `Comments` | `\[\]CommentInfo` | `nil` | Comments contained within this chunk. |
| `Docstrings` | `\[\]DocstringInfo` | `nil` | Docstrings contained within this chunk. |
| `HasErrorNodes` | `bool` | — | Whether this chunk contains any tree-sitter error nodes. |

---

#### CodeChunk

A chunk of source code with rich metadata.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Content` | `string` | — | The raw source text of this chunk. |
| `StartByte` | `int` | — | Inclusive start byte offset of this chunk in the original source. |
| `EndByte` | `int` | — | Exclusive end byte offset of this chunk in the original source. |
| `StartLine` | `int` | — | Zero-indexed start line of this chunk. |
| `EndLine` | `int` | — | Zero-indexed end line of this chunk. |
| `Metadata` | `ChunkContext` | — | Contextual metadata about this chunk. |

---

#### CommentInfo

A comment extracted from source code.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Text` | `string` | — | The raw text content of the comment. |
| `Kind` | `CommentKind` | `CommentKind.Line` | The kind of comment (line, block, or doc). |
| `Span` | `Span` | — | Source span covering the comment. |
| `AssociatedNode` | `*string` | `nil` | Name of the syntax node this comment is directly associated with. |

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
| `Key` | `*string` | `nil` | Key, attribute name, tag name, or positional index (`"0"`, `"1"`, …). `nil` at the document root. |
| `Value` | `*string` | `nil` | Leaf scalar value, if any. `nil` for containers (objects, arrays, XML elements with child elements). |
| `Attributes` | `\[\]DataAttribute` | `nil` | Attributes on element-shape nodes (XML `STag` attributes). Empty for all other kinds. |
| `Children` | `\[\]DataNode` | `nil` | Children for nested containers and XML element bodies. |
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
| `Name` | `*string` | `nil` | Parameter or return value name, if applicable. |
| `Description` | `string` | — | Description text for this section. |

---

#### DocstringInfo

A docstring extracted from source code.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Text` | `string` | — | The raw text of the docstring. |
| `Format` | `DocstringFormat` | `DocstringFormat.PythonTripleQuote` | The docstring format (Python, JSDoc, Rustdoc, etc.). |
| `Span` | `Span` | — | Source span covering the docstring. |
| `AssociatedItem` | `*string` | `nil` | Name of the item this docstring documents. |
| `ParsedSections` | `\[\]DocSection` | `nil` | Parsed sections of the docstring (Args, Returns, Raises, etc.). |

---

#### DownloadManager

Manages downloading and caching of pre-built parser shared libraries.

##### Methods

###### New()

Create a new download manager for the given version.

**Signature:**

```go
func (o *DownloadManager) New(version string) (DownloadManager, error)
```

**Example:**

```go
result, err := DownloadManager.New("value")
if err != nil {
    return err
}
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `Version` | `string` | Yes | The version |

**Returns:** `DownloadManager`

**Errors:** Returns `error`.

###### InstalledLanguages()

List languages that are already downloaded and cached.

**Signature:**

```go
func (o *DownloadManager) InstalledLanguages() []string
```

**Example:**

```go
result := instance.InstalledLanguages()
```

**Returns:** `[]string`

###### DownloadAllBestEffort()

Download the platform bundle and extract every library file it contains.

Unlike `Self.ensure_languages`, this does not check the manifest language list
against archive contents — it simply extracts all `.so`/`.dylib`/`.dll` files
from the bundle. Languages in the manifest that are missing from the archive
are silently ignored rather than returning an error.

Returns the number of library files extracted (including those already cached).

**Signature:**

```go
func (o *DownloadManager) DownloadAllBestEffort() (int, error)
```

**Example:**

```go
result, err := instance.DownloadAllBestEffort()
if err != nil {
    return err
}
```

**Returns:** `int`

**Errors:** Returns `error`.

###### CleanCache()

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

**Example:**

```go
if err := instance.CleanCache(); err != nil {
    return err
}
```

**Returns:** No return value.

**Errors:** Returns `error`.

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
| `TotalLines` | `int` | — | Total number of lines (including blank and comment lines). |
| `CodeLines` | `int` | — | Number of lines containing non-blank, non-comment source code. |
| `CommentLines` | `int` | — | Number of lines that are entirely comments. |
| `BlankLines` | `int` | — | Number of blank (whitespace-only) lines. |
| `TotalBytes` | `int` | — | Total byte length of the source file. |
| `NodeCount` | `int` | — | Total number of nodes in the syntax tree. |
| `ErrorCount` | `int` | — | Number of error nodes in the syntax tree (parse errors). |
| `MaxDepth` | `int` | — | Maximum nesting depth reached in the syntax tree. |

---

#### ImportInfo

An import statement extracted from source code.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Source` | `string` | — | The module or path being imported from. |
| `Items` | `\[\]string` | `nil` | Specific names imported from the source module. |
| `Alias` | `*string` | `nil` | Alias assigned to the import (e.g., `import numpy as np`). |
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

##### Methods

###### New()

Create a new registry populated with all statically compiled languages.

When the `dynamic-loading` feature is enabled, the registry also knows
about dynamically loadable grammars and will load them on demand.

**Signature:**

```go
func (o *LanguageRegistry) New() LanguageRegistry
```

**Example:**

```go
result := LanguageRegistry.New()
```

**Returns:** `LanguageRegistry`

###### GetLanguage()

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

**Example:**

```go
result, err := instance.GetLanguage("value")
if err != nil {
    return err
}
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `Name` | `string` | Yes | The name |

**Returns:** `Language`

**Errors:** Returns `error`.

###### AvailableLanguages()

List all available language names, sorted and deduplicated.

Includes statically compiled languages, dynamically loadable languages
(if the `dynamic-loading` feature is enabled), and all configured aliases.

**Signature:**

```go
func (o *LanguageRegistry) AvailableLanguages() []string
```

**Example:**

```go
result := instance.AvailableLanguages()
```

**Returns:** `[]string`

###### HasParser()

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

```go
func (o *LanguageRegistry) HasParser(name string) bool
```

**Example:**

```go
result := instance.HasParser("value")
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `Name` | `string` | Yes | The name |

**Returns:** `bool`

###### HasLanguage()

Check whether a language is available by name or alias.

Returns `true` if the language can be loaded, either from the static
table or from a dynamic library on disk.

**Signature:**

```go
func (o *LanguageRegistry) HasLanguage(name string) bool
```

**Example:**

```go
result := instance.HasLanguage("value")
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `Name` | `string` | Yes | The name |

**Returns:** `bool`

###### LanguageCount()

Return the total number of available languages (including aliases).

**Signature:**

```go
func (o *LanguageRegistry) LanguageCount() int
```

**Example:**

```go
result := instance.LanguageCount()
```

**Returns:** `int`

###### Process()

Parse source code and extract file intelligence based on config in a single pass.

**Signature:**

```go
func (o *LanguageRegistry) Process(source string, config ProcessConfig) (ProcessResult, error)
```

**Example:**

```go
result, err := instance.Process("value", ProcessConfig{})
if err != nil {
    return err
}
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `Source` | `string` | Yes | The source |
| `Config` | `ProcessConfig` | Yes | The configuration options |

**Returns:** `ProcessResult`

**Errors:** Returns `error`.

###### Default()

**Signature:**

```go
func (o *LanguageRegistry) Default() LanguageRegistry
```

**Example:**

```go
result := LanguageRegistry.Default()
```

**Returns:** `LanguageRegistry`

---

#### Node

A single syntax node within a `Tree`.

Nodes hold a strong reference to their parent tree so they remain valid
regardless of how the tree is moved or stored at the FFI boundary.

##### Methods

###### Clone()

**Signature:**

```go
func (o *Node) Clone() Node
```

**Example:**

```go
result := instance.Clone()
```

**Returns:** `Node`

###### Kind()

Return the node's kind name (e.g. `"function_definition"`).

**Signature:**

```go
func (o *Node) Kind() string
```

**Example:**

```go
result := instance.Kind()
```

**Returns:** `string`

###### KindId()

Return the node's numeric kind ID.

Tree-sitter assigns a stable `u16` ID to every node kind in a grammar
(e.g. `"function_definition" → 42`). Comparing `kind_id()` is cheaper
than comparing the string `kind()` in tight AST loops.

**Signature:**

```go
func (o *Node) KindId() uint16
```

**Example:**

```go
result := instance.KindId()
```

**Returns:** `uint16`

###### StartByte()

Return the inclusive start byte offset of this node.

**Signature:**

```go
func (o *Node) StartByte() int
```

**Example:**

```go
result := instance.StartByte()
```

**Returns:** `int`

###### EndByte()

Return the exclusive end byte offset of this node.

**Signature:**

```go
func (o *Node) EndByte() int
```

**Example:**

```go
result := instance.EndByte()
```

**Returns:** `int`

###### ByteRange()

Return the node's byte range as a `ByteRange`.

Callers should slice their own source bytes — this is a zero-copy
text accessor.

**Signature:**

```go
func (o *Node) ByteRange() ByteRange
```

**Example:**

```go
result := instance.ByteRange()
```

**Returns:** `ByteRange`

###### StartPosition()

Return the start `Point` (row, column).

**Signature:**

```go
func (o *Node) StartPosition() Point
```

**Example:**

```go
result := instance.StartPosition()
```

**Returns:** `Point`

###### EndPosition()

Return the end `Point` (row, column).

**Signature:**

```go
func (o *Node) EndPosition() Point
```

**Example:**

```go
result := instance.EndPosition()
```

**Returns:** `Point`

###### IsNamed()

True when this node is named (not punctuation/whitespace).

**Signature:**

```go
func (o *Node) IsNamed() bool
```

**Example:**

```go
result := instance.IsNamed()
```

**Returns:** `bool`

###### IsError()

True when this is an error node.

**Signature:**

```go
func (o *Node) IsError() bool
```

**Example:**

```go
result := instance.IsError()
```

**Returns:** `bool`

###### IsMissing()

True when this is a missing-token node.

**Signature:**

```go
func (o *Node) IsMissing() bool
```

**Example:**

```go
result := instance.IsMissing()
```

**Returns:** `bool`

###### IsExtra()

True when this is an "extra" node (e.g. a comment).

**Signature:**

```go
func (o *Node) IsExtra() bool
```

**Example:**

```go
result := instance.IsExtra()
```

**Returns:** `bool`

###### HasError()

True when this node or any descendant is an error.

**Signature:**

```go
func (o *Node) HasError() bool
```

**Example:**

```go
result := instance.HasError()
```

**Returns:** `bool`

###### Parent()

Return this node's parent, if any.

**Signature:**

```go
func (o *Node) Parent() *Node
```

**Example:**

```go
result := instance.Parent()
```

**Returns:** `*Node`

###### Child()

Return the i-th child of this node, if any.

**Signature:**

```go
func (o *Node) Child(index uint32) *Node
```

**Example:**

```go
result := instance.Child(42)
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `Index` | `uint32` | Yes | The index |

**Returns:** `*Node`

###### ChildCount()

Total number of children (including unnamed).

**Signature:**

```go
func (o *Node) ChildCount() int
```

**Example:**

```go
result := instance.ChildCount()
```

**Returns:** `int`

###### NamedChild()

Return the i-th named child of this node, if any.

**Signature:**

```go
func (o *Node) NamedChild(index uint32) *Node
```

**Example:**

```go
result := instance.NamedChild(42)
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `Index` | `uint32` | Yes | The index |

**Returns:** `*Node`

###### NamedChildCount()

Number of named children of this node.

**Signature:**

```go
func (o *Node) NamedChildCount() int
```

**Example:**

```go
result := instance.NamedChildCount()
```

**Returns:** `int`

###### ChildByFieldName()

Look up a child by its grammar-defined field name.

**Signature:**

```go
func (o *Node) ChildByFieldName(name string) *Node
```

**Example:**

```go
result := instance.ChildByFieldName("value")
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `Name` | `string` | Yes | The name |

**Returns:** `*Node`

###### ToSexp()

Return the S-expression form of this node's subtree.

**Signature:**

```go
func (o *Node) ToSexp() string
```

**Example:**

```go
result := instance.ToSexp()
```

**Returns:** `string`

###### Walk()

Return a `TreeCursor` positioned at this node.

**Signature:**

```go
func (o *Node) Walk() TreeCursor
```

**Example:**

```go
result := instance.Walk()
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
| `CacheDir` | `*string` | `nil` | Override default cache directory. Default: `~/.cache/tree-sitter-language-pack/v{version}/libs/` |
| `Languages` | `*\[\]string` | `nil` | Languages to pre-download on init. Each entry is a language name (e.g. `"python"`, `"rust"`). |
| `Groups` | `*\[\]string` | `nil` | Language groups to pre-download (e.g. `"web"`, `"systems"`, `"scripting"`). |

---

#### Parser

A tree-sitter parser configured for one language at a time.

##### Methods

###### New()

Construct a new parser with no language set.

Call `Parser.set_language` before parsing.

**Signature:**

```go
func (o *Parser) New() Parser
```

**Example:**

```go
result := Parser.New()
```

**Returns:** `Parser`

###### SetLanguage()

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

**Example:**

```go
if err := instance.SetLanguage("value"); err != nil {
    return err
}
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `Name` | `string` | Yes | The name |

**Returns:** No return value.

**Errors:** Returns `error`.

###### Parse()

Parse a UTF-8 source string. Returns `nil` if parsing was cancelled
or no language is set.

**Signature:**

```go
func (o *Parser) Parse(source string) *Tree
```

**Example:**

```go
result := instance.Parse("value")
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `Source` | `string` | Yes | The source |

**Returns:** `*Tree`

###### ParseBytes()

Parse a raw byte slice. Returns `nil` if parsing was cancelled or
no language is set.

**Signature:**

```go
func (o *Parser) ParseBytes(source []byte) *Tree
```

**Example:**

```go
result := instance.ParseBytes([]byte("data"))
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `Source` | `\[\]byte` | Yes | The source |

**Returns:** `*Tree`

###### Reset()

Reset internal state. The next call to `parse` will
not be incremental.

**Signature:**

```go
func (o *Parser) Reset()
```

**Example:**

```go
instance.Reset()
```

**Returns:** No return value.

###### Default()

**Signature:**

```go
func (o *Parser) Default() Parser
```

**Example:**

```go
result := Parser.Default()
```

**Returns:** `Parser`

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
| `DataExtraction` | `bool` | `false` | Extract hierarchical key/value data tree from data-format files. Default: false. When `true`, `ProcessResult.data` is populated with a `DataNode` tree for supported languages: JSON, YAML, TOML, `.properties`, HCL/HOCON, INI, editorconfig, KDL, CUE, CSV, PSV, PO, nginx config, Caddy config, XML, and DTD. For languages outside this set the field is left as `nil`. |

##### Methods

###### Default()

**Signature:**

```go
func (o *ProcessConfig) Default() ProcessConfig
```

**Example:**

```go
result := ProcessConfig.Default()
```

**Returns:** `ProcessConfig`

###### WithChunking()

Enable chunking with the given maximum chunk size in bytes.

**Signature:**

```go
func (o *ProcessConfig) WithChunking(maxSize int) ProcessConfig
```

**Example:**

```go
result := instance.WithChunking(42)
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `MaxSize` | `int` | Yes | The max size |

**Returns:** `ProcessConfig`

###### All()

Enable all analysis features.

**Signature:**

```go
func (o *ProcessConfig) All() ProcessConfig
```

**Example:**

```go
result := instance.All()
```

**Returns:** `ProcessConfig`

###### Minimal()

Disable all analysis features (only metrics computed).

**Signature:**

```go
func (o *ProcessConfig) Minimal() ProcessConfig
```

**Example:**

```go
result := instance.Minimal()
```

**Returns:** `ProcessConfig`

###### WithDataExtraction()

Enable or disable hierarchical data extraction for data-format files.

When `true`, `ProcessResult.data` is
populated with a key/value tree for supported data-format languages.

**Signature:**

```go
func (o *ProcessConfig) WithDataExtraction(enabled bool) ProcessConfig
```

**Example:**

```go
result := instance.WithDataExtraction(true)
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `Enabled` | `bool` | Yes | The enabled |

**Returns:** `ProcessConfig`

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
| `Structure` | `\[\]StructureItem` | `nil` | Top-level structural items (functions, classes, etc.). |
| `Imports` | `\[\]ImportInfo` | `nil` | Import statements extracted from the source. |
| `Exports` | `\[\]ExportInfo` | `nil` | Export statements extracted from the source. |
| `Comments` | `\[\]CommentInfo` | `nil` | Comments extracted from the source. |
| `Docstrings` | `\[\]DocstringInfo` | `nil` | Docstrings extracted from the source. |
| `Symbols` | `\[\]SymbolInfo` | `nil` | Symbol definitions (variables, types, functions) extracted from the source. |
| `Diagnostics` | `\[\]Diagnostic` | `nil` | Parse diagnostics (syntax errors, missing nodes) from tree-sitter. |
| `Chunks` | `\[\]CodeChunk` | `nil` | Syntax-aware code chunks produced when chunking is enabled. |
| `Data` | `*DataNode` | `nil` | Hierarchical data tree extracted when `config.data_extraction` is `true`. Populated for supported data-format languages (JSON, YAML, TOML, properties, HCL, INI, XML, CSV, and more). `nil` when `data_extraction` is `false` (the default) or when the language is not a recognised data format. See `DataNode` for the shape of the returned tree. |

---

#### Span

Byte and line/column range in source code.

Represents both byte offsets (for slicing) and human-readable line/column
positions (for display and diagnostics).

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `StartByte` | `int` | — | Inclusive start byte offset in the source. |
| `EndByte` | `int` | — | Exclusive end byte offset in the source. |
| `StartLine` | `int` | — | Zero-indexed line number of the span's start. |
| `StartColumn` | `int` | — | Zero-indexed column number of the span's start. |
| `EndLine` | `int` | — | Zero-indexed line number of the span's end. |
| `EndColumn` | `int` | — | Zero-indexed column number of the span's end. |

---

#### StructureItem

A structural item (function, class, struct, etc.) in source code.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Kind` | `StructureKind` | `StructureKind.Function` | The kind of structural item. |
| `Name` | `*string` | `nil` | The declared name of the item, if present. |
| `Visibility` | `*string` | `nil` | Visibility modifier (e.g., `"pub"`, `"public"`, `"private"`). |
| `Span` | `Span` | — | Source span covering the entire item declaration. |
| `Children` | `\[\]StructureItem` | `nil` | Nested structural items (e.g., methods within a class). |
| `Decorators` | `\[\]string` | `nil` | Decorator or attribute names applied to the item. |
| `DocComment` | `*string` | `nil` | Documentation comment attached to the item, if any. |
| `Signature` | `*string` | `nil` | Full signature text of the item (e.g., function parameters and return type). |
| `BodySpan` | `*Span` | `nil` | Source span covering only the body of the item, if distinct from the declaration. |

---

#### SymbolInfo

A symbol (variable, function, type, etc.) extracted from source code.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Name` | `string` | — | The name of the symbol. |
| `Kind` | `SymbolKind` | `SymbolKind.Variable` | The kind of symbol (variable, function, class, etc.). |
| `Span` | `Span` | — | Source span covering the symbol definition. |
| `TypeAnnotation` | `*string` | `nil` | Explicit type annotation, if present in the source. |
| `Doc` | `*string` | `nil` | Documentation comment associated with this symbol. |

---

#### Tree

A parsed syntax tree. Cheap to clone (refcount bump).

##### Methods

###### RootNode()

Return the root `Node` of this tree.

**Signature:**

```go
func (o *Tree) RootNode() Node
```

**Example:**

```go
result := instance.RootNode()
```

**Returns:** `Node`

###### Walk()

Return a `TreeCursor` positioned at the root.

**Signature:**

```go
func (o *Tree) Walk() TreeCursor
```

**Example:**

```go
result := instance.Walk()
```

**Returns:** `TreeCursor`

---

#### TreeCursor

A cursor for traversing a `Tree`.

##### Methods

###### Node()

Return the `Node` at the cursor's current position.

**Signature:**

```go
func (o *TreeCursor) Node() Node
```

**Example:**

```go
result := instance.Node()
```

**Returns:** `Node`

###### GotoFirstChild()

Move the cursor to the first child of the current node.
Returns `true` if a child existed.

**Signature:**

```go
func (o *TreeCursor) GotoFirstChild() bool
```

**Example:**

```go
result := instance.GotoFirstChild()
```

**Returns:** `bool`

###### GotoParent()

Move the cursor to the parent of the current node.
Returns `true` if a parent existed.

**Signature:**

```go
func (o *TreeCursor) GotoParent() bool
```

**Example:**

```go
result := instance.GotoParent()
```

**Returns:** `bool`

###### GotoNextSibling()

Move the cursor to the next sibling of the current node.
Returns `true` if a sibling existed.

**Signature:**

```go
func (o *TreeCursor) GotoNextSibling() bool
```

**Example:**

```go
result := instance.GotoNextSibling()
```

**Returns:** `bool`

###### FieldName()

Return the field name for the current node, if any.

**Signature:**

```go
func (o *TreeCursor) FieldName() *string
```

**Example:**

```go
result := instance.FieldName()
```

**Returns:** `*string`

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
