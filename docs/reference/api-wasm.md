---
title: "WebAssembly API Reference"
---

## WebAssembly API Reference <span class="version-badge">v1.11.0-rc.1</span>

### Functions

#### detectLanguageFromExtension()

Detect language name from a file extension (without leading dot).

Returns `null` for unrecognized extensions. The match is case-insensitive.

**Signature:**

```typescript
function detectLanguageFromExtension(ext: string): string | null
```

**Example:**

```typescript
const result = detectLanguageFromExtension("value");
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `ext` | `string` | Yes | The ext |

**Returns:** `string | null`

---

#### detectLanguageFromPath()

Detect language name from a file path.

Extracts the file extension and looks it up. Returns `null` if the
path has no extension or the extension is not recognized.

**Signature:**

```typescript
function detectLanguageFromPath(path: string): string | null
```

**Example:**

```typescript
const result = detectLanguageFromPath("value");
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `path` | `string` | Yes | Path to the file |

**Returns:** `string | null`

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

```typescript
function detectLanguageFromContent(content: string): string | null
```

**Example:**

```typescript
const result = detectLanguageFromContent("value");
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `content` | `string` | Yes | The content to process |

**Returns:** `string | null`

---

#### getHighlightsQuery()

Get the highlights query for a language, if bundled.

Returns the contents of `highlights.scm` as a static string, or `null`
if no highlights query is bundled for this language.

**Signature:**

```typescript
function getHighlightsQuery(language: string): string | null
```

**Example:**

```typescript
const result = getHighlightsQuery("value");
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `language` | `string` | Yes | The language |

**Returns:** `string | null`

---

#### getInjectionsQuery()

Get the injections query for a language, if bundled.

Returns the contents of `injections.scm` as a static string, or `null`
if no injections query is bundled for this language.

**Signature:**

```typescript
function getInjectionsQuery(language: string): string | null
```

**Example:**

```typescript
const result = getInjectionsQuery("value");
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `language` | `string` | Yes | The language |

**Returns:** `string | null`

---

#### getLocalsQuery()

Get the locals query for a language, if bundled.

Returns the contents of `locals.scm` as a static string, or `null`
if no locals query is bundled for this language.

**Signature:**

```typescript
function getLocalsQuery(language: string): string | null
```

**Example:**

```typescript
const result = getLocalsQuery("value");
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `language` | `string` | Yes | The language |

**Returns:** `string | null`

---

#### getTagsQuery()

Get the tags query for a language, if bundled.

Returns the contents of `tags.scm` as a static string, or `null`
if no tags query is bundled for this language.

**Signature:**

```typescript
function getTagsQuery(language: string): string | null
```

**Example:**

```typescript
const result = getTagsQuery("value");
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `language` | `string` | Yes | The language |

**Returns:** `string | null`

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

```typescript
function getLanguage(name: string): Language
```

**Example:**

```typescript
const result = getLanguage("value");
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `name` | `string` | Yes | The name |

**Returns:** `Language`

**Errors:** Throws `Error` with a descriptive message.

---

#### getParser()

Get a `Parser` pre-configured for the given language.

This is a convenience function that calls `get_language` and configures
a new parser in one step.

**Errors:**

Returns `Error.LanguageNotFound` if the language is not recognized, or
`Error.ParserSetup` if the language cannot be applied to the parser.

**Signature:**

```typescript
function getParser(name: string): Parser
```

**Example:**

```typescript
const result = getParser("value");
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `name` | `string` | Yes | The name |

**Returns:** `Parser`

**Errors:** Throws `Error` with a descriptive message.

---

#### detectLanguage()

Detect language name from a file path or extension.

This compatibility alias matches the pre-Alef Python binding API.

**Signature:**

```typescript
function detectLanguage(path: string): string | null
```

**Example:**

```typescript
const result = detectLanguage("value");
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `path` | `string` | Yes | Path to the file |

**Returns:** `string | null`

---

#### availableLanguages()

List all available language names (sorted, deduplicated, includes aliases).

Returns names of both statically compiled and dynamically loadable languages,
plus any configured aliases.

**Signature:**

```typescript
function availableLanguages(): Array<string>
```

**Example:**

```typescript
const result = availableLanguages();
```

**Returns:** `Array<string>`

---

#### hasLanguage()

Check if a language is available by name or alias.

Returns `true` if the language can be loaded (statically compiled,
dynamically available, or a known alias for one of these).

**Signature:**

```typescript
function hasLanguage(name: string): boolean
```

**Example:**

```typescript
const result = hasLanguage("value");
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `name` | `string` | Yes | The name |

**Returns:** `boolean`

---

#### languageCount()

Return the number of available languages.

Includes statically compiled languages, dynamically loadable languages,
and aliases.

**Signature:**

```typescript
function languageCount(): number
```

**Example:**

```typescript
const result = languageCount();
```

**Returns:** `number`

---

#### process()

Process source code and extract file intelligence using the global registry.

Parses the source with tree-sitter and extracts metrics, structure, imports,
exports, comments, docstrings, symbols, diagnostics, and/or chunks based on
the flags set in `ProcessConfig`.

**Errors:**

Returns an error if the language is not found or parsing fails.

**Signature:**

```typescript
function process(source: string, config: ProcessConfig): ProcessResult
```

**Example:**

```typescript
const result = process("value", new ProcessConfig());
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `source` | `string` | Yes | The source |
| `config` | `ProcessConfig` | Yes | The configuration options |

**Returns:** `ProcessResult`

**Errors:** Throws `Error` with a descriptive message.

---

#### init()

Initialize the language pack with the given configuration.

Applies any custom cache directory, then downloads all languages and groups
specified in the config. This is the recommended entry point when you want
to pre-warm the cache before use.

**Errors:**

Returns an error if configuration cannot be applied or if downloads fail.

**Signature:**

```typescript
function init(config: PackConfig): void
```

**Example:**

```typescript
init(new PackConfig());
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `config` | `PackConfig` | Yes | The configuration options |

**Returns:** No return value.

**Errors:** Throws `Error` with a descriptive message.

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

```typescript
function configure(config: PackConfig): void
```

**Example:**

```typescript
configure(new PackConfig());
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `config` | `PackConfig` | Yes | The configuration options |

**Returns:** No return value.

**Errors:** Throws `Error` with a descriptive message.

---

#### download()

Download specific languages to the local cache.

Returns the number of requested languages available after the call. Already
compiled or cached languages are included in the count.

**Errors:**

Returns an error if any language is not available in the manifest or if
the download fails.

**Signature:**

```typescript
function download(names: Array<string>): number
```

**Example:**

```typescript
const result = download([]);
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `names` | `Array<string>` | Yes | The names |

**Returns:** `number`

**Errors:** Throws `Error` with a descriptive message.

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

```typescript
function downloadAll(): number
```

**Example:**

```typescript
const result = downloadAll();
```

**Returns:** `number`

**Errors:** Throws `Error` with a descriptive message.

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

```typescript
function downloadGroup(name: string): number
```

**Example:**

```typescript
const result = downloadGroup("value");
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `name` | `string` | Yes | The name |

**Returns:** `number`

**Errors:** Throws `Error` with a descriptive message.

---

#### manifestLanguages()

Return all language names available in the remote manifest (306).

Fetches (and caches) the remote manifest to discover the full list of
downloadable languages. Use `downloaded_languages` to list what is
already cached locally.

**Errors:**

Returns an error if the manifest cannot be fetched.

**Signature:**

```typescript
function manifestLanguages(): Array<string>
```

**Example:**

```typescript
const result = manifestLanguages();
```

**Returns:** `Array<string>`

**Errors:** Throws `Error` with a descriptive message.

---

#### downloadedLanguages()

Return languages that are already downloaded and cached locally.

Does not perform any network requests. Returns an empty list if the
cache directory does not exist or cannot be read.

**Signature:**

```typescript
function downloadedLanguages(): Array<string>
```

**Example:**

```typescript
const result = downloadedLanguages();
```

**Returns:** `Array<string>`

---

#### cleanCache()

Delete all cached parser shared libraries.

Resets the cache registration so the next call to `get_language` or
a download function will re-register the (now empty) cache directory.

**Errors:**

Returns an error if the cache directory cannot be removed.

**Signature:**

```typescript
function cleanCache(): void
```

**Example:**

```typescript
cleanCache();
```

**Returns:** No return value.

**Errors:** Throws `Error` with a descriptive message.

---

#### cacheDir()

Return the effective cache directory path.

This is either the custom path set via `configure` / `init` or the
default: `~/.cache/tree-sitter-language-pack/v{version}/libs/`.

**Errors:**

Returns an error if the system cache directory cannot be determined.

**Signature:**

```typescript
function cacheDir(): string
```

**Example:**

```typescript
const result = cacheDir();
```

**Returns:** `string`

**Errors:** Throws `Error` with a descriptive message.

---

### Types

#### ByteRange

A byte range — start (inclusive) to end (exclusive).

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `start` | `number` | — | Inclusive start byte offset. |
| `end` | `number` | — | Exclusive end byte offset. |

---

#### ChunkContext

Metadata for a single chunk of source code.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `language` | `string` | — | Language name used to parse this chunk. |
| `chunkIndex` | `number` | — | Zero-indexed position of this chunk within the file's chunk list. |
| `totalChunks` | `number` | — | Total number of chunks the file was split into. |
| `nodeTypes` | `Array<string>` | `\[\]` | Tree-sitter node kinds that appear at the top level of this chunk. |
| `contextPath` | `Array<string>` | `\[\]` | Hierarchical path of enclosing structural items (e.g., `\["MyClass", "my_method"\]`). |
| `symbolsDefined` | `Array<string>` | `\[\]` | Names of symbols defined within this chunk. |
| `comments` | `Array<CommentInfo>` | `\[\]` | Comments contained within this chunk. |
| `docstrings` | `Array<DocstringInfo>` | `\[\]` | Docstrings contained within this chunk. |
| `hasErrorNodes` | `boolean` | — | Whether this chunk contains any tree-sitter error nodes. |

---

#### CodeChunk

A chunk of source code with rich metadata.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `content` | `string` | — | The raw source text of this chunk. |
| `startByte` | `number` | — | Inclusive start byte offset of this chunk in the original source. |
| `endByte` | `number` | — | Exclusive end byte offset of this chunk in the original source. |
| `startLine` | `number` | — | Zero-indexed start line of this chunk. |
| `endLine` | `number` | — | Zero-indexed end line of this chunk. |
| `metadata` | `ChunkContext` | — | Contextual metadata about this chunk. |

---

#### CommentInfo

A comment extracted from source code.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `text` | `string` | — | The raw text content of the comment. |
| `kind` | `CommentKind` | `CommentKind.Line` | The kind of comment (line, block, or doc). |
| `span` | `Span` | — | Source span covering the comment. |
| `associatedNode` | `string \| null` | `null` | Name of the syntax node this comment is directly associated with. |

---

#### DataAttribute

An XML-style attribute attached to an `Element` node.

Populated only for `DataNodeKind.Element`; always empty for `KeyValue` and
`Sequence` nodes.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `name` | `string` | — | Attribute name (e.g. `"class"`, `"href"`). |
| `value` | `string` | — | Attribute value as a raw string (quotes stripped). |
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
| `kind` | `DataNodeKind` | `DataNodeKind.KeyValue` | Whether this node is a key/value pair, XML element, or sequence item. |
| `key` | `string \| null` | `null` | Key, attribute name, tag name, or positional index (`"0"`, `"1"`, …). `null` at the document root. |
| `value` | `string \| null` | `null` | Leaf scalar value, if any. `null` for containers (objects, arrays, XML elements with child elements). |
| `attributes` | `Array<DataAttribute>` | `\[\]` | Attributes on element-shape nodes (XML `STag` attributes). Empty for all other kinds. |
| `children` | `Array<DataNode>` | `\[\]` | Children for nested containers and XML element bodies. |
| `span` | `Span` | — | Source span covering this node in the original source file. |

---

#### Diagnostic

A diagnostic (syntax error, missing node, etc.) from parsing.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `message` | `string` | — | Human-readable description of the diagnostic. |
| `severity` | `DiagnosticSeverity` | `DiagnosticSeverity.Error` | Severity of the diagnostic. |
| `span` | `Span` | — | Source span where the diagnostic was detected. |

---

#### DocSection

A section within a docstring (e.g., Args, Returns, Raises).

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `kind` | `string` | — | Section kind (e.g., `"args"`, `"returns"`, `"raises"`). |
| `name` | `string \| null` | `null` | Parameter or return value name, if applicable. |
| `description` | `string` | — | Description text for this section. |

---

#### DocstringInfo

A docstring extracted from source code.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `text` | `string` | — | The raw text of the docstring. |
| `format` | `DocstringFormat` | `DocstringFormat.PythonTripleQuote` | The docstring format (Python, JSDoc, Rustdoc, etc.). |
| `span` | `Span` | — | Source span covering the docstring. |
| `associatedItem` | `string \| null` | `null` | Name of the item this docstring documents. |
| `parsedSections` | `Array<DocSection>` | `\[\]` | Parsed sections of the docstring (Args, Returns, Raises, etc.). |

---

#### DownloadManager

Manages downloading and caching of pre-built parser shared libraries.

##### Methods

###### new()

Create a new download manager for the given version.

**Signature:**

```typescript
static new(version: string): DownloadManager
```

**Example:**

```typescript
const result = DownloadManager.new("value");
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `version` | `string` | Yes | The version |

**Returns:** `DownloadManager`

**Errors:** Throws `Error` with a descriptive message.

###### installedLanguages()

List languages that are already downloaded and cached.

**Signature:**

```typescript
installedLanguages(): Array<string>
```

**Example:**

```typescript
const result = instance.installedLanguages();
```

**Returns:** `Array<string>`

###### downloadAllBestEffort()

Download the platform bundle and extract every library file it contains.

Unlike `Self.ensure_languages`, this does not check the manifest language list
against archive contents — it simply extracts all `.so`/`.dylib`/`.dll` files
from the bundle. Languages in the manifest that are missing from the archive
are silently ignored rather than returning an error.

Returns the number of library files extracted (including those already cached).

**Signature:**

```typescript
downloadAllBestEffort(): number
```

**Example:**

```typescript
const result = instance.downloadAllBestEffort();
```

**Returns:** `number`

**Errors:** Throws `Error` with a descriptive message.

###### cleanCache()

Remove all cached parser libraries.

Acquires the cross-process lock so `clean_cache` cannot race a concurrent
downloader (avoids Windows sharing-violation errors against an in-flight
bundle write). The `.download.lock` file itself is **not** removed — it is
permanent infrastructure; deleting it could allow a concurrent process that
already opened the file to continue holding a stale lock handle while a new
process opens a fresh inode, breaking the mutual-exclusion guarantee.

**Signature:**

```typescript
cleanCache(): void
```

**Example:**

```typescript
instance.cleanCache();
```

**Returns:** No return value.

**Errors:** Throws `Error` with a descriptive message.

---

#### ExportInfo

An export statement extracted from source code.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `name` | `string` | — | The exported name. |
| `kind` | `ExportKind` | `ExportKind.Named` | The kind of export (named, default, or re-export). |
| `span` | `Span` | — | Source span covering the export statement. |

---

#### FileMetrics

Aggregate metrics for a source file.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `totalLines` | `number` | — | Total number of lines (including blank and comment lines). |
| `codeLines` | `number` | — | Number of lines containing non-blank, non-comment source code. |
| `commentLines` | `number` | — | Number of lines that are entirely comments. |
| `blankLines` | `number` | — | Number of blank (whitespace-only) lines. |
| `totalBytes` | `number` | — | Total byte length of the source file. |
| `nodeCount` | `number` | — | Total number of nodes in the syntax tree. |
| `errorCount` | `number` | — | Number of error nodes in the syntax tree (parse errors). |
| `maxDepth` | `number` | — | Maximum nesting depth reached in the syntax tree. |

---

#### ImportInfo

An import statement extracted from source code.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `source` | `string` | — | The module or path being imported from. |
| `items` | `Array<string>` | `\[\]` | Specific names imported from the source module. |
| `alias` | `string \| null` | `null` | Alias assigned to the import (e.g., `import numpy as np`). |
| `isWildcard` | `boolean` | — | Whether this is a wildcard import (e.g., `import *` or `use foo.*`). |
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

##### Methods

###### new()

Create a new registry populated with all statically compiled languages.

When the `dynamic-loading` feature is enabled, the registry also knows
about dynamically loadable grammars and will load them on demand.

**Signature:**

```typescript
static new(): LanguageRegistry
```

**Example:**

```typescript
const result = LanguageRegistry.new();
```

**Returns:** `LanguageRegistry`

###### getLanguage()

Get a tree-sitter `Language` by name.

Resolves aliases (e.g., `"shell"` -> `"bash"`, `"makefile"` -> `"make"`),
then looks up the language in the static table. When the `dynamic-loading`
feature is enabled, falls back to loading a shared library on demand.

**Errors:**

Returns `Error.LanguageNotFound` if the name (after alias resolution)
does not match any known grammar.

**Signature:**

```typescript
getLanguage(name: string): Language
```

**Example:**

```typescript
const result = instance.getLanguage("value");
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `name` | `string` | Yes | The name |

**Returns:** `Language`

**Errors:** Throws `Error` with a descriptive message.

###### availableLanguages()

List all available language names, sorted and deduplicated.

Includes statically compiled languages, dynamically loadable languages
(if the `dynamic-loading` feature is enabled), and all configured aliases.

**Signature:**

```typescript
availableLanguages(): Array<string>
```

**Example:**

```typescript
const result = instance.availableLanguages();
```

**Returns:** `Array<string>`

###### hasParser()

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

```typescript
hasParser(name: string): boolean
```

**Example:**

```typescript
const result = instance.hasParser("value");
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `name` | `string` | Yes | The name |

**Returns:** `boolean`

###### hasLanguage()

Check whether a language is available by name or alias.

Returns `true` if the language can be loaded, either from the static
table or from a dynamic library on disk.

**Signature:**

```typescript
hasLanguage(name: string): boolean
```

**Example:**

```typescript
const result = instance.hasLanguage("value");
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `name` | `string` | Yes | The name |

**Returns:** `boolean`

###### languageCount()

Return the total number of available languages (including aliases).

**Signature:**

```typescript
languageCount(): number
```

**Example:**

```typescript
const result = instance.languageCount();
```

**Returns:** `number`

###### process()

Parse source code and extract file intelligence based on config in a single pass.

**Signature:**

```typescript
process(source: string, config: ProcessConfig): ProcessResult
```

**Example:**

```typescript
const result = instance.process("value", new ProcessConfig());
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `source` | `string` | Yes | The source |
| `config` | `ProcessConfig` | Yes | The configuration options |

**Returns:** `ProcessResult`

**Errors:** Throws `Error` with a descriptive message.

###### default()

**Signature:**

```typescript
static default(): LanguageRegistry
```

**Example:**

```typescript
const result = LanguageRegistry.default();
```

**Returns:** `LanguageRegistry`

---

#### Node

A single syntax node within a `Tree`.

Nodes hold a strong reference to their parent tree so they remain valid
regardless of how the tree is moved or stored at the FFI boundary.

##### Methods

###### clone()

**Signature:**

```typescript
clone(): Node
```

**Example:**

```typescript
const result = instance.clone();
```

**Returns:** `Node`

###### kind()

Return the node's kind name (e.g. `"function_definition"`).

**Signature:**

```typescript
kind(): string
```

**Example:**

```typescript
const result = instance.kind();
```

**Returns:** `string`

###### kindId()

Return the node's numeric kind ID.

Tree-sitter assigns a stable `u16` ID to every node kind in a grammar
(e.g. `"function_definition" → 42`). Comparing `kind_id()` is cheaper
than comparing the string `kind()` in tight AST loops.

**Signature:**

```typescript
kindId(): number
```

**Example:**

```typescript
const result = instance.kindId();
```

**Returns:** `number`

###### startByte()

Return the inclusive start byte offset of this node.

**Signature:**

```typescript
startByte(): number
```

**Example:**

```typescript
const result = instance.startByte();
```

**Returns:** `number`

###### endByte()

Return the exclusive end byte offset of this node.

**Signature:**

```typescript
endByte(): number
```

**Example:**

```typescript
const result = instance.endByte();
```

**Returns:** `number`

###### byteRange()

Return the node's byte range as a `ByteRange`.

Callers should slice their own source bytes — this is a zero-copy
text accessor.

**Signature:**

```typescript
byteRange(): ByteRange
```

**Example:**

```typescript
const result = instance.byteRange();
```

**Returns:** `ByteRange`

###### startPosition()

Return the start `Point` (row, column).

**Signature:**

```typescript
startPosition(): Point
```

**Example:**

```typescript
const result = instance.startPosition();
```

**Returns:** `Point`

###### endPosition()

Return the end `Point` (row, column).

**Signature:**

```typescript
endPosition(): Point
```

**Example:**

```typescript
const result = instance.endPosition();
```

**Returns:** `Point`

###### isNamed()

True when this node is named (not punctuation/whitespace).

**Signature:**

```typescript
isNamed(): boolean
```

**Example:**

```typescript
const result = instance.isNamed();
```

**Returns:** `boolean`

###### isError()

True when this is an error node.

**Signature:**

```typescript
isError(): boolean
```

**Example:**

```typescript
const result = instance.isError();
```

**Returns:** `boolean`

###### isMissing()

True when this is a missing-token node.

**Signature:**

```typescript
isMissing(): boolean
```

**Example:**

```typescript
const result = instance.isMissing();
```

**Returns:** `boolean`

###### isExtra()

True when this is an "extra" node (e.g. a comment).

**Signature:**

```typescript
isExtra(): boolean
```

**Example:**

```typescript
const result = instance.isExtra();
```

**Returns:** `boolean`

###### hasError()

True when this node or any descendant is an error.

**Signature:**

```typescript
hasError(): boolean
```

**Example:**

```typescript
const result = instance.hasError();
```

**Returns:** `boolean`

###### parent()

Return this node's parent, if any.

**Signature:**

```typescript
parent(): Node | null
```

**Example:**

```typescript
const result = instance.parent();
```

**Returns:** `Node | null`

###### child()

Return the i-th child of this node, if any.

**Signature:**

```typescript
child(index: number): Node | null
```

**Example:**

```typescript
const result = instance.child(42);
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `index` | `number` | Yes | The index |

**Returns:** `Node | null`

###### childCount()

Total number of children (including unnamed).

**Signature:**

```typescript
childCount(): number
```

**Example:**

```typescript
const result = instance.childCount();
```

**Returns:** `number`

###### namedChild()

Return the i-th named child of this node, if any.

**Signature:**

```typescript
namedChild(index: number): Node | null
```

**Example:**

```typescript
const result = instance.namedChild(42);
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `index` | `number` | Yes | The index |

**Returns:** `Node | null`

###### namedChildCount()

Number of named children of this node.

**Signature:**

```typescript
namedChildCount(): number
```

**Example:**

```typescript
const result = instance.namedChildCount();
```

**Returns:** `number`

###### childByFieldName()

Look up a child by its grammar-defined field name.

**Signature:**

```typescript
childByFieldName(name: string): Node | null
```

**Example:**

```typescript
const result = instance.childByFieldName("value");
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `name` | `string` | Yes | The name |

**Returns:** `Node | null`

###### toSexp()

Return the S-expression form of this node's subtree.

**Signature:**

```typescript
toSexp(): string
```

**Example:**

```typescript
const result = instance.toSexp();
```

**Returns:** `string`

###### walk()

Return a `TreeCursor` positioned at this node.

**Signature:**

```typescript
walk(): TreeCursor
```

**Example:**

```typescript
const result = instance.walk();
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
| `cacheDir` | `string \| null` | `null` | Override default cache directory. Default: `~/.cache/tree-sitter-language-pack/v{version}/libs/` |
| `languages` | `Array<string> \| null` | `\[\]` | Languages to pre-download on init. Each entry is a language name (e.g. `"python"`, `"rust"`). |
| `groups` | `Array<string> \| null` | `\[\]` | Language groups to pre-download (e.g. `"web"`, `"systems"`, `"scripting"`). |

---

#### Parser

A tree-sitter parser configured for one language at a time.

##### Methods

###### new()

Construct a new parser with no language set.

Call `Parser.set_language` before parsing.

**Signature:**

```typescript
static new(): Parser
```

**Example:**

```typescript
const result = Parser.new();
```

**Returns:** `Parser`

###### setLanguage()

Configure the parser to use the language identified by name (e.g. `"python"`).

Resolves the language through the global registry — auto-downloading
if necessary, when the `download` feature is enabled.

**Errors:**

Returns `Error.LanguageNotFound` if the language is not recognized,
or `Error.ParserSetup` if the language ABI is incompatible.

**Signature:**

```typescript
setLanguage(name: string): void
```

**Example:**

```typescript
instance.setLanguage("value");
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `name` | `string` | Yes | The name |

**Returns:** No return value.

**Errors:** Throws `Error` with a descriptive message.

###### parse()

Parse a UTF-8 source string. Returns `null` if parsing was cancelled
or no language is set.

**Signature:**

```typescript
parse(source: string): Tree | null
```

**Example:**

```typescript
const result = instance.parse("value");
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `source` | `string` | Yes | The source |

**Returns:** `Tree | null`

###### parseBytes()

Parse a raw byte slice. Returns `null` if parsing was cancelled or
no language is set.

**Signature:**

```typescript
parseBytes(source: Buffer): Tree | null
```

**Example:**

```typescript
const result = instance.parseBytes(new Uint8Array([100, 97, 116, 97]));
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `source` | `Buffer` | Yes | The source |

**Returns:** `Tree | null`

###### reset()

Reset internal state. The next call to `parse` will
not be incremental.

**Signature:**

```typescript
reset(): void
```

**Example:**

```typescript
instance.reset();
```

**Returns:** No return value.

###### default()

**Signature:**

```typescript
static default(): Parser
```

**Example:**

```typescript
const result = Parser.default();
```

**Returns:** `Parser`

---

#### Point

A source position — row + column, zero-indexed.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `row` | `number` | — | Zero-indexed row number. |
| `column` | `number` | — | Zero-indexed column number, in UTF-16 code units. |

---

#### ProcessConfig

Configuration for the `process()` function.

Controls which analysis features are enabled and whether chunking is performed.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `language` | `string` | — | Language name (required). |
| `structure` | `boolean` | `true` | Extract structural items (functions, classes, etc.). Default: true. |
| `imports` | `boolean` | `true` | Extract import statements. Default: true. |
| `exports` | `boolean` | `true` | Extract export statements. Default: true. |
| `comments` | `boolean` | `false` | Extract comments. Default: false. |
| `docstrings` | `boolean` | `false` | Extract docstrings. Default: false. |
| `symbols` | `boolean` | `false` | Extract symbol definitions. Default: false. |
| `diagnostics` | `boolean` | `false` | Include parse diagnostics. Default: false. |
| `chunkMaxSize` | `number \| null` | `null` | Maximum chunk size in bytes. `null` disables chunking. |
| `dataExtraction` | `boolean` | `false` | Extract hierarchical key/value data tree from data-format files. Default: false. When `true`, `ProcessResult.data` is populated with a `DataNode` tree for supported languages: JSON, YAML, TOML, `.properties`, HCL/HOCON, INI, editorconfig, KDL, CUE, CSV, PSV, PO, nginx config, Caddy config, XML, and DTD. For languages outside this set the field is left as `null`. |

##### Methods

###### default()

**Signature:**

```typescript
static default(): ProcessConfig
```

**Example:**

```typescript
const result = ProcessConfig.default();
```

**Returns:** `ProcessConfig`

###### withChunking()

Enable chunking with the given maximum chunk size in bytes.

**Signature:**

```typescript
withChunking(maxSize: number): ProcessConfig
```

**Example:**

```typescript
const result = instance.withChunking(42);
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `maxSize` | `number` | Yes | The max size |

**Returns:** `ProcessConfig`

###### all()

Enable all analysis features.

**Signature:**

```typescript
all(): ProcessConfig
```

**Example:**

```typescript
const result = instance.all();
```

**Returns:** `ProcessConfig`

###### minimal()

Disable all analysis features (only metrics computed).

**Signature:**

```typescript
minimal(): ProcessConfig
```

**Example:**

```typescript
const result = instance.minimal();
```

**Returns:** `ProcessConfig`

###### withDataExtraction()

Enable or disable hierarchical data extraction for data-format files.

When `true`, `ProcessResult.data` is
populated with a key/value tree for supported data-format languages.

**Signature:**

```typescript
withDataExtraction(enabled: boolean): ProcessConfig
```

**Example:**

```typescript
const result = instance.withDataExtraction(true);
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `enabled` | `boolean` | Yes | The enabled |

**Returns:** `ProcessConfig`

---

#### ProcessResult

Complete analysis result from processing a source file.

Contains metrics, structural analysis, imports/exports, comments,
docstrings, symbols, diagnostics, and optionally chunked code segments.
Fields are populated based on the `ProcessConfig` flags.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `language` | `string` | — | The language name used to parse the source file. |
| `metrics` | `FileMetrics` | — | File-level metrics (line counts, byte size, error count). |
| `structure` | `Array<StructureItem>` | `\[\]` | Top-level structural items (functions, classes, etc.). |
| `imports` | `Array<ImportInfo>` | `\[\]` | Import statements extracted from the source. |
| `exports` | `Array<ExportInfo>` | `\[\]` | Export statements extracted from the source. |
| `comments` | `Array<CommentInfo>` | `\[\]` | Comments extracted from the source. |
| `docstrings` | `Array<DocstringInfo>` | `\[\]` | Docstrings extracted from the source. |
| `symbols` | `Array<SymbolInfo>` | `\[\]` | Symbol definitions (variables, types, functions) extracted from the source. |
| `diagnostics` | `Array<Diagnostic>` | `\[\]` | Parse diagnostics (syntax errors, missing nodes) from tree-sitter. |
| `chunks` | `Array<CodeChunk>` | `\[\]` | Syntax-aware code chunks produced when chunking is enabled. |
| `data` | `DataNode \| null` | `null` | Hierarchical data tree extracted when `config.data_extraction` is `true`. Populated for supported data-format languages (JSON, YAML, TOML, properties, HCL, INI, XML, CSV, and more). `null` when `data_extraction` is `false` (the default) or when the language is not a recognised data format. See `DataNode` for the shape of the returned tree. |

---

#### Span

Byte and line/column range in source code.

Represents both byte offsets (for slicing) and human-readable line/column
positions (for display and diagnostics).

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `startByte` | `number` | — | Inclusive start byte offset in the source. |
| `endByte` | `number` | — | Exclusive end byte offset in the source. |
| `startLine` | `number` | — | Zero-indexed line number of the span's start. |
| `startColumn` | `number` | — | Zero-indexed column number of the span's start. |
| `endLine` | `number` | — | Zero-indexed line number of the span's end. |
| `endColumn` | `number` | — | Zero-indexed column number of the span's end. |

---

#### StructureItem

A structural item (function, class, struct, etc.) in source code.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `kind` | `StructureKind` | `StructureKind.Function` | The kind of structural item. |
| `name` | `string \| null` | `null` | The declared name of the item, if present. |
| `visibility` | `string \| null` | `null` | Visibility modifier (e.g., `"pub"`, `"public"`, `"private"`). |
| `span` | `Span` | — | Source span covering the entire item declaration. |
| `children` | `Array<StructureItem>` | `\[\]` | Nested structural items (e.g., methods within a class). |
| `decorators` | `Array<string>` | `\[\]` | Decorator or attribute names applied to the item. |
| `docComment` | `string \| null` | `null` | Documentation comment attached to the item, if any. |
| `signature` | `string \| null` | `null` | Full signature text of the item (e.g., function parameters and return type). |
| `bodySpan` | `Span \| null` | `null` | Source span covering only the body of the item, if distinct from the declaration. |

---

#### SymbolInfo

A symbol (variable, function, type, etc.) extracted from source code.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `name` | `string` | — | The name of the symbol. |
| `kind` | `SymbolKind` | `SymbolKind.Variable` | The kind of symbol (variable, function, class, etc.). |
| `span` | `Span` | — | Source span covering the symbol definition. |
| `typeAnnotation` | `string \| null` | `null` | Explicit type annotation, if present in the source. |
| `doc` | `string \| null` | `null` | Documentation comment associated with this symbol. |

---

#### Tree

A parsed syntax tree. Cheap to clone (refcount bump).

##### Methods

###### rootNode()

Return the root `Node` of this tree.

**Signature:**

```typescript
rootNode(): Node
```

**Example:**

```typescript
const result = instance.rootNode();
```

**Returns:** `Node`

###### walk()

Return a `TreeCursor` positioned at the root.

**Signature:**

```typescript
walk(): TreeCursor
```

**Example:**

```typescript
const result = instance.walk();
```

**Returns:** `TreeCursor`

---

#### TreeCursor

A cursor for traversing a `Tree`.

##### Methods

###### node()

Return the `Node` at the cursor's current position.

**Signature:**

```typescript
node(): Node
```

**Example:**

```typescript
const result = instance.node();
```

**Returns:** `Node`

###### gotoFirstChild()

Move the cursor to the first child of the current node.
Returns `true` if a child existed.

**Signature:**

```typescript
gotoFirstChild(): boolean
```

**Example:**

```typescript
const result = instance.gotoFirstChild();
```

**Returns:** `boolean`

###### gotoParent()

Move the cursor to the parent of the current node.
Returns `true` if a parent existed.

**Signature:**

```typescript
gotoParent(): boolean
```

**Example:**

```typescript
const result = instance.gotoParent();
```

**Returns:** `boolean`

###### gotoNextSibling()

Move the cursor to the next sibling of the current node.
Returns `true` if a sibling existed.

**Signature:**

```typescript
gotoNextSibling(): boolean
```

**Example:**

```typescript
const result = instance.gotoNextSibling();
```

**Returns:** `boolean`

###### fieldName()

Return the field name for the current node, if any.

**Signature:**

```typescript
fieldName(): string | null
```

**Example:**

```typescript
const result = instance.fieldName();
```

**Returns:** `string | null`

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

Errors are thrown as plain `Error` objects with descriptive messages.

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
