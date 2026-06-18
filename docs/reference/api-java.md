---
title: "Java API Reference"
---

## Java API Reference <span class="version-badge">v1.9.1</span>

### Functions

#### detectLanguageFromExtension()

Detect language name from a file extension (without leading dot).

Returns `null` for unrecognized extensions. The match is case-insensitive.

**Signature:**

```java
public static Optional<String> detectLanguageFromExtension(String ext)
```

**Example:**

```java
var result = detectLanguageFromExtension("value");
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `ext` | `String` | Yes | The ext |

**Returns:** `Optional<String>`

---

#### detectLanguageFromPath()

Detect language name from a file path.

Extracts the file extension and looks it up. Returns `null` if the
path has no extension or the extension is not recognized.

**Signature:**

```java
public static Optional<String> detectLanguageFromPath(String path)
```

**Example:**

```java
var result = detectLanguageFromPath("value");
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `path` | `String` | Yes | Path to the file |

**Returns:** `Optional<String>`

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

```java
public static Optional<String> detectLanguageFromContent(String content)
```

**Example:**

```java
var result = detectLanguageFromContent("value");
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `content` | `String` | Yes | The content to process |

**Returns:** `Optional<String>`

---

#### getHighlightsQuery()

Get the highlights query for a language, if bundled.

Returns the contents of `highlights.scm` as a static string, or `null`
if no highlights query is bundled for this language.

**Signature:**

```java
public static Optional<String> getHighlightsQuery(String language)
```

**Example:**

```java
var result = getHighlightsQuery("value");
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `language` | `String` | Yes | The language |

**Returns:** `Optional<String>`

---

#### getInjectionsQuery()

Get the injections query for a language, if bundled.

Returns the contents of `injections.scm` as a static string, or `null`
if no injections query is bundled for this language.

**Signature:**

```java
public static Optional<String> getInjectionsQuery(String language)
```

**Example:**

```java
var result = getInjectionsQuery("value");
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `language` | `String` | Yes | The language |

**Returns:** `Optional<String>`

---

#### getLocalsQuery()

Get the locals query for a language, if bundled.

Returns the contents of `locals.scm` as a static string, or `null`
if no locals query is bundled for this language.

**Signature:**

```java
public static Optional<String> getLocalsQuery(String language)
```

**Example:**

```java
var result = getLocalsQuery("value");
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `language` | `String` | Yes | The language |

**Returns:** `Optional<String>`

---

#### getTagsQuery()

Get the tags query for a language, if bundled.

Returns the contents of `tags.scm` as a static string, or `null`
if no tags query is bundled for this language.

**Signature:**

```java
public static Optional<String> getTagsQuery(String language)
```

**Example:**

```java
var result = getTagsQuery("value");
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `language` | `String` | Yes | The language |

**Returns:** `Optional<String>`

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

```java
public static Language getLanguage(String name) throws Error
```

**Example:**

```java
var result = getLanguage("value");
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `name` | `String` | Yes | The name |

**Returns:** `Language`

**Errors:** Throws `ErrorException`.

---

#### getParser()

Get a `Parser` pre-configured for the given language.

This is a convenience function that calls `get_language` and configures
a new parser in one step.

**Errors:**

Returns `Error.LanguageNotFound` if the language is not recognized, or
`Error.ParserSetup` if the language cannot be applied to the parser.

**Signature:**

```java
public static Parser getParser(String name) throws Error
```

**Example:**

```java
var result = getParser("value");
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `name` | `String` | Yes | The name |

**Returns:** `Parser`

**Errors:** Throws `ErrorException`.

---

#### detectLanguage()

Detect language name from a file path or extension.

This compatibility alias matches the pre-Alef Python binding API.

**Signature:**

```java
public static Optional<String> detectLanguage(String path)
```

**Example:**

```java
var result = detectLanguage("value");
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `path` | `String` | Yes | Path to the file |

**Returns:** `Optional<String>`

---

#### availableLanguages()

List all available language names (sorted, deduplicated, includes aliases).

Returns names of both statically compiled and dynamically loadable languages,
plus any configured aliases.

**Signature:**

```java
public static List<String> availableLanguages()
```

**Example:**

```java
var result = availableLanguages();
```

**Returns:** `List<String>`

---

#### hasLanguage()

Check if a language is available by name or alias.

Returns `true` if the language can be loaded (statically compiled,
dynamically available, or a known alias for one of these).

**Signature:**

```java
public static boolean hasLanguage(String name)
```

**Example:**

```java
var result = hasLanguage("value");
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `name` | `String` | Yes | The name |

**Returns:** `boolean`

---

#### languageCount()

Return the number of available languages.

Includes statically compiled languages, dynamically loadable languages,
and aliases.

**Signature:**

```java
public static long languageCount()
```

**Example:**

```java
var result = languageCount();
```

**Returns:** `long`

---

#### process()

Process source code and extract file intelligence using the global registry.

Parses the source with tree-sitter and extracts metrics, structure, imports,
exports, comments, docstrings, symbols, diagnostics, and/or chunks based on
the flags set in `ProcessConfig`.

**Errors:**

Returns an error if the language is not found or parsing fails.

**Signature:**

```java
public static ProcessResult process(String source, ProcessConfig config) throws Error
```

**Example:**

```java
var result = process("value", new ProcessConfig());
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `source` | `String` | Yes | The source |
| `config` | `ProcessConfig` | Yes | The configuration options |

**Returns:** `ProcessResult`

**Errors:** Throws `ErrorException`.

---

#### init()

Initialize the language pack with the given configuration.

Applies any custom cache directory, then downloads all languages and groups
specified in the config. This is the recommended entry point when you want
to pre-warm the cache before use.

**Errors:**

Returns an error if configuration cannot be applied or if downloads fail.

**Signature:**

```java
public static void init(PackConfig config) throws Error
```

**Example:**

```java
init(new PackConfig());
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `config` | `PackConfig` | Yes | The configuration options |

**Returns:** No return value.

**Errors:** Throws `ErrorException`.

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

```java
public static void configure(PackConfig config) throws Error
```

**Example:**

```java
configure(new PackConfig());
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `config` | `PackConfig` | Yes | The configuration options |

**Returns:** No return value.

**Errors:** Throws `ErrorException`.

---

#### download()

Download specific languages to the local cache.

Returns the number of requested languages available after the call. Already
compiled or cached languages are included in the count.

**Errors:**

Returns an error if any language is not available in the manifest or if
the download fails.

**Signature:**

```java
public static long download(List<String> names) throws Error
```

**Example:**

```java
var result = download(List.of());
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `names` | `List<String>` | Yes | The names |

**Returns:** `long`

**Errors:** Throws `ErrorException`.

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

```java
public static long downloadAll() throws Error
```

**Example:**

```java
var result = downloadAll();
```

**Returns:** `long`

**Errors:** Throws `ErrorException`.

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

```java
public static long downloadGroup(String name) throws Error
```

**Example:**

```java
var result = downloadGroup("value");
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `name` | `String` | Yes | The name |

**Returns:** `long`

**Errors:** Throws `ErrorException`.

---

#### manifestLanguages()

Return all language names available in the remote manifest (306).

Fetches (and caches) the remote manifest to discover the full list of
downloadable languages. Use `downloaded_languages` to list what is
already cached locally.

**Errors:**

Returns an error if the manifest cannot be fetched.

**Signature:**

```java
public static List<String> manifestLanguages() throws Error
```

**Example:**

```java
var result = manifestLanguages();
```

**Returns:** `List<String>`

**Errors:** Throws `ErrorException`.

---

#### downloadedLanguages()

Return languages that are already downloaded and cached locally.

Does not perform any network requests. Returns an empty list if the
cache directory does not exist or cannot be read.

**Signature:**

```java
public static List<String> downloadedLanguages()
```

**Example:**

```java
var result = downloadedLanguages();
```

**Returns:** `List<String>`

---

#### cleanCache()

Delete all cached parser shared libraries.

Resets the cache registration so the next call to `get_language` or
a download function will re-register the (now empty) cache directory.

**Errors:**

Returns an error if the cache directory cannot be removed.

**Signature:**

```java
public static void cleanCache() throws Error
```

**Example:**

```java
cleanCache();
```

**Returns:** No return value.

**Errors:** Throws `ErrorException`.

---

#### cacheDir()

Return the effective cache directory path.

This is either the custom path set via `configure` / `init` or the
default: `~/.cache/tree-sitter-language-pack/v{version}/libs/`.

**Errors:**

Returns an error if the system cache directory cannot be determined.

**Signature:**

```java
public static String cacheDir() throws Error
```

**Example:**

```java
var result = cacheDir();
```

**Returns:** `String`

**Errors:** Throws `ErrorException`.

---

### Types

#### ByteRange

A byte range — start (inclusive) to end (exclusive).

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `start` | `long` | — | Inclusive start byte offset. |
| `end` | `long` | — | Exclusive end byte offset. |

---

#### ChunkContext

Metadata for a single chunk of source code.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `language` | `String` | — | Language name used to parse this chunk. |
| `chunkIndex` | `long` | — | Zero-indexed position of this chunk within the file's chunk list. |
| `totalChunks` | `long` | — | Total number of chunks the file was split into. |
| `nodeTypes` | `List<String>` | `Collections.emptyList()` | Tree-sitter node kinds that appear at the top level of this chunk. |
| `contextPath` | `List<String>` | `Collections.emptyList()` | Hierarchical path of enclosing structural items (e.g., `\["MyClass", "my_method"\]`). |
| `symbolsDefined` | `List<String>` | `Collections.emptyList()` | Names of symbols defined within this chunk. |
| `comments` | `List<CommentInfo>` | `Collections.emptyList()` | Comments contained within this chunk. |
| `docstrings` | `List<DocstringInfo>` | `Collections.emptyList()` | Docstrings contained within this chunk. |
| `hasErrorNodes` | `boolean` | — | Whether this chunk contains any tree-sitter error nodes. |

---

#### CodeChunk

A chunk of source code with rich metadata.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `content` | `String` | — | The raw source text of this chunk. |
| `startByte` | `long` | — | Inclusive start byte offset of this chunk in the original source. |
| `endByte` | `long` | — | Exclusive end byte offset of this chunk in the original source. |
| `startLine` | `long` | — | Zero-indexed start line of this chunk. |
| `endLine` | `long` | — | Zero-indexed end line of this chunk. |
| `metadata` | `ChunkContext` | — | Contextual metadata about this chunk. |

---

#### CommentInfo

A comment extracted from source code.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `text` | `String` | — | The raw text content of the comment. |
| `kind` | `CommentKind` | `CommentKind.LINE` | The kind of comment (line, block, or doc). |
| `span` | `Span` | — | Source span covering the comment. |
| `associatedNode` | `Optional<String>` | `null` | Name of the syntax node this comment is directly associated with. |

---

#### DataAttribute

An XML-style attribute attached to an `Element` node.

Populated only for `DataNodeKind.Element`; always empty for `KeyValue` and
`Sequence` nodes.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `name` | `String` | — | Attribute name (e.g. `"class"`, `"href"`). |
| `value` | `String` | — | Attribute value as a raw string (quotes stripped). |
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
| `kind` | `DataNodeKind` | `DataNodeKind.KEY_VALUE` | Whether this node is a key/value pair, XML element, or sequence item. |
| `key` | `Optional<String>` | `null` | Key, attribute name, tag name, or positional index (`"0"`, `"1"`, …). `null` at the document root. |
| `value` | `Optional<String>` | `null` | Leaf scalar value, if any. `null` for containers (objects, arrays, XML elements with child elements). |
| `attributes` | `List<DataAttribute>` | `Collections.emptyList()` | Attributes on element-shape nodes (XML `STag` attributes). Empty for all other kinds. |
| `children` | `List<DataNode>` | `Collections.emptyList()` | Children for nested containers and XML element bodies. |
| `span` | `Span` | — | Source span covering this node in the original source file. |

---

#### Diagnostic

A diagnostic (syntax error, missing node, etc.) from parsing.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `message` | `String` | — | Human-readable description of the diagnostic. |
| `severity` | `DiagnosticSeverity` | `DiagnosticSeverity.ERROR` | Severity of the diagnostic. |
| `span` | `Span` | — | Source span where the diagnostic was detected. |

---

#### DocSection

A section within a docstring (e.g., Args, Returns, Raises).

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `kind` | `String` | — | Section kind (e.g., `"args"`, `"returns"`, `"raises"`). |
| `name` | `Optional<String>` | `null` | Parameter or return value name, if applicable. |
| `description` | `String` | — | Description text for this section. |

---

#### DocstringInfo

A docstring extracted from source code.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `text` | `String` | — | The raw text of the docstring. |
| `format` | `DocstringFormat` | `DocstringFormat.PYTHON_TRIPLE_QUOTE` | The docstring format (Python, JSDoc, Rustdoc, etc.). |
| `span` | `Span` | — | Source span covering the docstring. |
| `associatedItem` | `Optional<String>` | `null` | Name of the item this docstring documents. |
| `parsedSections` | `List<DocSection>` | `Collections.emptyList()` | Parsed sections of the docstring (Args, Returns, Raises, etc.). |

---

#### DownloadManager

Manages downloading and caching of pre-built parser shared libraries.

##### Methods

###### new()

Create a new download manager for the given version.

**Signature:**

```java
public static DownloadManager new(String version) throws Error
```

**Example:**

```java
var result = DownloadManager.new("value");
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `version` | `String` | Yes | The version |

**Returns:** `DownloadManager`

**Errors:** Throws `ErrorException`.

###### installedLanguages()

List languages that are already downloaded and cached.

**Signature:**

```java
public List<String> installedLanguages()
```

**Example:**

```java
var result = instance.installedLanguages();
```

**Returns:** `List<String>`

###### downloadAllBestEffort()

Download the platform bundle and extract every library file it contains.

Unlike `Self.ensure_languages`, this does not check the manifest language list
against archive contents — it simply extracts all `.so`/`.dylib`/`.dll` files
from the bundle. Languages in the manifest that are missing from the archive
are silently ignored rather than returning an error.

Returns the number of library files extracted (including those already cached).

**Signature:**

```java
public long downloadAllBestEffort() throws Error
```

**Example:**

```java
var result = instance.downloadAllBestEffort();
```

**Returns:** `long`

**Errors:** Throws `ErrorException`.

###### cleanCache()

Remove all cached parser libraries.

Acquires the cross-process lock so `clean_cache` cannot race a concurrent
downloader (avoids Windows sharing-violation errors against an in-flight
bundle write). The `.download.lock` file itself is **not** removed — it is
permanent infrastructure; deleting it could allow a concurrent process that
already opened the file to continue holding a stale lock handle while a new
process opens a fresh inode, breaking the mutual-exclusion guarantee.

**Signature:**

```java
public void cleanCache() throws Error
```

**Example:**

```java
instance.cleanCache();
```

**Returns:** No return value.

**Errors:** Throws `ErrorException`.

---

#### ExportInfo

An export statement extracted from source code.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `name` | `String` | — | The exported name. |
| `kind` | `ExportKind` | `ExportKind.NAMED` | The kind of export (named, default, or re-export). |
| `span` | `Span` | — | Source span covering the export statement. |

---

#### FileMetrics

Aggregate metrics for a source file.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `totalLines` | `long` | — | Total number of lines (including blank and comment lines). |
| `codeLines` | `long` | — | Number of lines containing non-blank, non-comment source code. |
| `commentLines` | `long` | — | Number of lines that are entirely comments. |
| `blankLines` | `long` | — | Number of blank (whitespace-only) lines. |
| `totalBytes` | `long` | — | Total byte length of the source file. |
| `nodeCount` | `long` | — | Total number of nodes in the syntax tree. |
| `errorCount` | `long` | — | Number of error nodes in the syntax tree (parse errors). |
| `maxDepth` | `long` | — | Maximum nesting depth reached in the syntax tree. |

---

#### ImportInfo

An import statement extracted from source code.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `source` | `String` | — | The module or path being imported from. |
| `items` | `List<String>` | `Collections.emptyList()` | Specific names imported from the source module. |
| `alias` | `Optional<String>` | `null` | Alias assigned to the import (e.g., `import numpy as np`). |
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

```java
public static LanguageRegistry new()
```

**Example:**

```java
var result = LanguageRegistry.new();
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

```java
public Language getLanguage(String name) throws Error
```

**Example:**

```java
var result = instance.getLanguage("value");
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `name` | `String` | Yes | The name |

**Returns:** `Language`

**Errors:** Throws `ErrorException`.

###### availableLanguages()

List all available language names, sorted and deduplicated.

Includes statically compiled languages, dynamically loadable languages
(if the `dynamic-loading` feature is enabled), and all configured aliases.

**Signature:**

```java
public List<String> availableLanguages()
```

**Example:**

```java
var result = instance.availableLanguages();
```

**Returns:** `List<String>`

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

```java
public boolean hasParser(String name)
```

**Example:**

```java
var result = instance.hasParser("value");
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `name` | `String` | Yes | The name |

**Returns:** `boolean`

###### hasLanguage()

Check whether a language is available by name or alias.

Returns `true` if the language can be loaded, either from the static
table or from a dynamic library on disk.

**Signature:**

```java
public boolean hasLanguage(String name)
```

**Example:**

```java
var result = instance.hasLanguage("value");
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `name` | `String` | Yes | The name |

**Returns:** `boolean`

###### languageCount()

Return the total number of available languages (including aliases).

**Signature:**

```java
public long languageCount()
```

**Example:**

```java
var result = instance.languageCount();
```

**Returns:** `long`

###### process()

Parse source code and extract file intelligence based on config in a single pass.

**Signature:**

```java
public ProcessResult process(String source, ProcessConfig config) throws Error
```

**Example:**

```java
var result = instance.process("value", new ProcessConfig());
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `source` | `String` | Yes | The source |
| `config` | `ProcessConfig` | Yes | The configuration options |

**Returns:** `ProcessResult`

**Errors:** Throws `ErrorException`.

###### defaultOptions()

**Signature:**

```java
public static LanguageRegistry defaultOptions()
```

**Example:**

```java
var result = LanguageRegistry.defaultOptions();
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

```java
public Node clone()
```

**Example:**

```java
var result = instance.clone();
```

**Returns:** `Node`

###### kind()

Return the node's kind name (e.g. `"function_definition"`).

**Signature:**

```java
public String kind()
```

**Example:**

```java
var result = instance.kind();
```

**Returns:** `String`

###### kindId()

Return the node's numeric kind ID.

Tree-sitter assigns a stable `u16` ID to every node kind in a grammar
(e.g. `"function_definition" → 42`). Comparing `kind_id()` is cheaper
than comparing the string `kind()` in tight AST loops.

**Signature:**

```java
public short kindId()
```

**Example:**

```java
var result = instance.kindId();
```

**Returns:** `short`

###### startByte()

Return the inclusive start byte offset of this node.

**Signature:**

```java
public long startByte()
```

**Example:**

```java
var result = instance.startByte();
```

**Returns:** `long`

###### endByte()

Return the exclusive end byte offset of this node.

**Signature:**

```java
public long endByte()
```

**Example:**

```java
var result = instance.endByte();
```

**Returns:** `long`

###### byteRange()

Return the node's byte range as a `ByteRange`.

Callers should slice their own source bytes — this is a zero-copy
text accessor.

**Signature:**

```java
public ByteRange byteRange()
```

**Example:**

```java
var result = instance.byteRange();
```

**Returns:** `ByteRange`

###### startPosition()

Return the start `Point` (row, column).

**Signature:**

```java
public Point startPosition()
```

**Example:**

```java
var result = instance.startPosition();
```

**Returns:** `Point`

###### endPosition()

Return the end `Point` (row, column).

**Signature:**

```java
public Point endPosition()
```

**Example:**

```java
var result = instance.endPosition();
```

**Returns:** `Point`

###### isNamed()

True when this node is named (not punctuation/whitespace).

**Signature:**

```java
public boolean isNamed()
```

**Example:**

```java
var result = instance.isNamed();
```

**Returns:** `boolean`

###### isError()

True when this is an error node.

**Signature:**

```java
public boolean isError()
```

**Example:**

```java
var result = instance.isError();
```

**Returns:** `boolean`

###### isMissing()

True when this is a missing-token node.

**Signature:**

```java
public boolean isMissing()
```

**Example:**

```java
var result = instance.isMissing();
```

**Returns:** `boolean`

###### isExtra()

True when this is an "extra" node (e.g. a comment).

**Signature:**

```java
public boolean isExtra()
```

**Example:**

```java
var result = instance.isExtra();
```

**Returns:** `boolean`

###### hasError()

True when this node or any descendant is an error.

**Signature:**

```java
public boolean hasError()
```

**Example:**

```java
var result = instance.hasError();
```

**Returns:** `boolean`

###### parent()

Return this node's parent, if any.

**Signature:**

```java
public Optional<Node> parent()
```

**Example:**

```java
var result = instance.parent();
```

**Returns:** `Optional<Node>`

###### child()

Return the i-th child of this node, if any.

**Signature:**

```java
public Optional<Node> child(int index)
```

**Example:**

```java
var result = instance.child(42);
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `index` | `int` | Yes | The index |

**Returns:** `Optional<Node>`

###### childCount()

Total number of children (including unnamed).

**Signature:**

```java
public long childCount()
```

**Example:**

```java
var result = instance.childCount();
```

**Returns:** `long`

###### namedChild()

Return the i-th named child of this node, if any.

**Signature:**

```java
public Optional<Node> namedChild(int index)
```

**Example:**

```java
var result = instance.namedChild(42);
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `index` | `int` | Yes | The index |

**Returns:** `Optional<Node>`

###### namedChildCount()

Number of named children of this node.

**Signature:**

```java
public long namedChildCount()
```

**Example:**

```java
var result = instance.namedChildCount();
```

**Returns:** `long`

###### childByFieldName()

Look up a child by its grammar-defined field name.

**Signature:**

```java
public Optional<Node> childByFieldName(String name)
```

**Example:**

```java
var result = instance.childByFieldName("value");
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `name` | `String` | Yes | The name |

**Returns:** `Optional<Node>`

###### toSexp()

Return the S-expression form of this node's subtree.

**Signature:**

```java
public String toSexp()
```

**Example:**

```java
var result = instance.toSexp();
```

**Returns:** `String`

###### walk()

Return a `TreeCursor` positioned at this node.

**Signature:**

```java
public TreeCursor walk()
```

**Example:**

```java
var result = instance.walk();
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
| `cacheDir` | `Optional<String>` | `null` | Override default cache directory. Default: `~/.cache/tree-sitter-language-pack/v{version}/libs/` |
| `languages` | `Optional<List<String>>` | `Collections.emptyList()` | Languages to pre-download on init. Each entry is a language name (e.g. `"python"`, `"rust"`). |
| `groups` | `Optional<List<String>>` | `Collections.emptyList()` | Language groups to pre-download (e.g. `"web"`, `"systems"`, `"scripting"`). |

---

#### Parser

A tree-sitter parser configured for one language at a time.

##### Methods

###### new()

Construct a new parser with no language set.

Call `Parser.set_language` before parsing.

**Signature:**

```java
public static Parser new()
```

**Example:**

```java
var result = Parser.new();
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

```java
public void setLanguage(String name) throws Error
```

**Example:**

```java
instance.setLanguage("value");
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `name` | `String` | Yes | The name |

**Returns:** No return value.

**Errors:** Throws `ErrorException`.

###### parse()

Parse a UTF-8 source string. Returns `null` if parsing was cancelled
or no language is set.

**Signature:**

```java
public Optional<Tree> parse(String source)
```

**Example:**

```java
var result = instance.parse("value");
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `source` | `String` | Yes | The source |

**Returns:** `Optional<Tree>`

###### parseBytes()

Parse a raw byte slice. Returns `null` if parsing was cancelled or
no language is set.

**Signature:**

```java
public Optional<Tree> parseBytes(byte[] source)
```

**Example:**

```java
var result = instance.parseBytes("data".getBytes());
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `source` | `byte\[\]` | Yes | The source |

**Returns:** `Optional<Tree>`

###### reset()

Reset internal state. The next call to `parse` will
not be incremental.

**Signature:**

```java
public void reset()
```

**Example:**

```java
instance.reset();
```

**Returns:** No return value.

###### defaultOptions()

**Signature:**

```java
public static Parser defaultOptions()
```

**Example:**

```java
var result = Parser.defaultOptions();
```

**Returns:** `Parser`

---

#### Point

A source position — row + column, zero-indexed.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `row` | `long` | — | Zero-indexed row number. |
| `column` | `long` | — | Zero-indexed column number, in UTF-16 code units. |

---

#### ProcessConfig

Configuration for the `process()` function.

Controls which analysis features are enabled and whether chunking is performed.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `language` | `String` | — | Language name (required). |
| `structure` | `boolean` | `true` | Extract structural items (functions, classes, etc.). Default: true. |
| `imports` | `boolean` | `true` | Extract import statements. Default: true. |
| `exports` | `boolean` | `true` | Extract export statements. Default: true. |
| `comments` | `boolean` | `false` | Extract comments. Default: false. |
| `docstrings` | `boolean` | `false` | Extract docstrings. Default: false. |
| `symbols` | `boolean` | `false` | Extract symbol definitions. Default: false. |
| `diagnostics` | `boolean` | `false` | Include parse diagnostics. Default: false. |
| `chunkMaxSize` | `Optional<Long>` | `null` | Maximum chunk size in bytes. `null` disables chunking. |
| `dataExtraction` | `boolean` | `false` | Extract hierarchical key/value data tree from data-format files. Default: false. When `true`, `ProcessResult.data` is populated with a `DataNode` tree for supported languages: JSON, YAML, TOML, `.properties`, HCL/HOCON, INI, editorconfig, KDL, CUE, CSV, PSV, PO, nginx config, Caddy config, XML, and DTD. For languages outside this set the field is left as `null`. |

##### Methods

###### defaultOptions()

**Signature:**

```java
public static ProcessConfig defaultOptions()
```

**Example:**

```java
var result = ProcessConfig.defaultOptions();
```

**Returns:** `ProcessConfig`

###### withChunking()

Enable chunking with the given maximum chunk size in bytes.

**Signature:**

```java
public ProcessConfig withChunking(long maxSize)
```

**Example:**

```java
var result = instance.withChunking(42);
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `maxSize` | `long` | Yes | The max size |

**Returns:** `ProcessConfig`

###### all()

Enable all analysis features.

**Signature:**

```java
public ProcessConfig all()
```

**Example:**

```java
var result = instance.all();
```

**Returns:** `ProcessConfig`

###### minimal()

Disable all analysis features (only metrics computed).

**Signature:**

```java
public ProcessConfig minimal()
```

**Example:**

```java
var result = instance.minimal();
```

**Returns:** `ProcessConfig`

###### withDataExtraction()

Enable or disable hierarchical data extraction for data-format files.

When `true`, `ProcessResult.data` is
populated with a key/value tree for supported data-format languages.

**Signature:**

```java
public ProcessConfig withDataExtraction(boolean enabled)
```

**Example:**

```java
var result = instance.withDataExtraction(true);
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
| `language` | `String` | — | The language name used to parse the source file. |
| `metrics` | `FileMetrics` | — | File-level metrics (line counts, byte size, error count). |
| `structure` | `List<StructureItem>` | `Collections.emptyList()` | Top-level structural items (functions, classes, etc.). |
| `imports` | `List<ImportInfo>` | `Collections.emptyList()` | Import statements extracted from the source. |
| `exports` | `List<ExportInfo>` | `Collections.emptyList()` | Export statements extracted from the source. |
| `comments` | `List<CommentInfo>` | `Collections.emptyList()` | Comments extracted from the source. |
| `docstrings` | `List<DocstringInfo>` | `Collections.emptyList()` | Docstrings extracted from the source. |
| `symbols` | `List<SymbolInfo>` | `Collections.emptyList()` | Symbol definitions (variables, types, functions) extracted from the source. |
| `diagnostics` | `List<Diagnostic>` | `Collections.emptyList()` | Parse diagnostics (syntax errors, missing nodes) from tree-sitter. |
| `chunks` | `List<CodeChunk>` | `Collections.emptyList()` | Syntax-aware code chunks produced when chunking is enabled. |
| `data` | `Optional<DataNode>` | `null` | Hierarchical data tree extracted when `config.data_extraction` is `true`. Populated for supported data-format languages (JSON, YAML, TOML, properties, HCL, INI, XML, CSV, and more). `null` when `data_extraction` is `false` (the default) or when the language is not a recognised data format. See `DataNode` for the shape of the returned tree. |

---

#### Span

Byte and line/column range in source code.

Represents both byte offsets (for slicing) and human-readable line/column
positions (for display and diagnostics).

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `startByte` | `long` | — | Inclusive start byte offset in the source. |
| `endByte` | `long` | — | Exclusive end byte offset in the source. |
| `startLine` | `long` | — | Zero-indexed line number of the span's start. |
| `startColumn` | `long` | — | Zero-indexed column number of the span's start. |
| `endLine` | `long` | — | Zero-indexed line number of the span's end. |
| `endColumn` | `long` | — | Zero-indexed column number of the span's end. |

---

#### StructureItem

A structural item (function, class, struct, etc.) in source code.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `kind` | `StructureKind` | `StructureKind.FUNCTION` | The kind of structural item. |
| `name` | `Optional<String>` | `null` | The declared name of the item, if present. |
| `visibility` | `Optional<String>` | `null` | Visibility modifier (e.g., `"pub"`, `"public"`, `"private"`). |
| `span` | `Span` | — | Source span covering the entire item declaration. |
| `children` | `List<StructureItem>` | `Collections.emptyList()` | Nested structural items (e.g., methods within a class). |
| `decorators` | `List<String>` | `Collections.emptyList()` | Decorator or attribute names applied to the item. |
| `docComment` | `Optional<String>` | `null` | Documentation comment attached to the item, if any. |
| `signature` | `Optional<String>` | `null` | Full signature text of the item (e.g., function parameters and return type). |
| `bodySpan` | `Optional<Span>` | `null` | Source span covering only the body of the item, if distinct from the declaration. |

---

#### SymbolInfo

A symbol (variable, function, type, etc.) extracted from source code.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `name` | `String` | — | The name of the symbol. |
| `kind` | `SymbolKind` | `SymbolKind.VARIABLE` | The kind of symbol (variable, function, class, etc.). |
| `span` | `Span` | — | Source span covering the symbol definition. |
| `typeAnnotation` | `Optional<String>` | `null` | Explicit type annotation, if present in the source. |
| `doc` | `Optional<String>` | `null` | Documentation comment associated with this symbol. |

---

#### Tree

A parsed syntax tree. Cheap to clone (refcount bump).

##### Methods

###### rootNode()

Return the root `Node` of this tree.

**Signature:**

```java
public Node rootNode()
```

**Example:**

```java
var result = instance.rootNode();
```

**Returns:** `Node`

###### walk()

Return a `TreeCursor` positioned at the root.

**Signature:**

```java
public TreeCursor walk()
```

**Example:**

```java
var result = instance.walk();
```

**Returns:** `TreeCursor`

---

#### TreeCursor

A cursor for traversing a `Tree`.

##### Methods

###### node()

Return the `Node` at the cursor's current position.

**Signature:**

```java
public Node node()
```

**Example:**

```java
var result = instance.node();
```

**Returns:** `Node`

###### gotoFirstChild()

Move the cursor to the first child of the current node.
Returns `true` if a child existed.

**Signature:**

```java
public boolean gotoFirstChild()
```

**Example:**

```java
var result = instance.gotoFirstChild();
```

**Returns:** `boolean`

###### gotoParent()

Move the cursor to the parent of the current node.
Returns `true` if a parent existed.

**Signature:**

```java
public boolean gotoParent()
```

**Example:**

```java
var result = instance.gotoParent();
```

**Returns:** `boolean`

###### gotoNextSibling()

Move the cursor to the next sibling of the current node.
Returns `true` if a sibling existed.

**Signature:**

```java
public boolean gotoNextSibling()
```

**Example:**

```java
var result = instance.gotoNextSibling();
```

**Returns:** `boolean`

###### fieldName()

Return the field name for the current node, if any.

**Signature:**

```java
public Optional<String> fieldName()
```

**Example:**

```java
var result = instance.fieldName();
```

**Returns:** `Optional<String>`

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
| `KEY_VALUE` | A key/value pair or mapping (json/toml/properties/yaml/hcl/cue/kdl pair, or a wrapper "object"/"mapping" container). |
| `ELEMENT` | An XML element with a tag name in `key` and attributes in `attributes`. |
| `SEQUENCE` | A positional sequence item (JSON array element, YAML block sequence item, CSV/PSV row or cell). |

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
| `FUNCTION` | A free-standing or associated function. |
| `METHOD` | A method defined inside a class, struct, trait, or impl block. |
| `CLASS` | A class definition. |
| `STRUCT` | A struct definition. |
| `INTERFACE` | An interface or protocol definition. |
| `ENUM` | An enum definition. |
| `MODULE` | A module or package declaration. |
| `TRAIT` | A trait definition. |
| `IMPL` | An impl block (Rust) or similar implementation block. |
| `NAMESPACE` | A namespace declaration. |
| `OTHER` | A language-specific construct that does not fit any standard category. — Fields: `0`: `String` |

---

#### CommentKind

The kind of a comment found in source code.

Distinguishes between single-line comments, block (multi-line) comments,
and documentation comments.

| Value | Description |
|-------|-------------|
| `LINE` | A single-line comment (e.g., `// ...` or `# ...`). |
| `BLOCK` | A block or multi-line comment using slash-star delimiters. |
| `DOC` | A documentation comment such as `/// ...` or slash-double-star block. |

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
| `PYTHON_TRIPLE_QUOTE` | Python triple-quoted string docstring (`"""..."""`). |
| `JS_DOC` | JavaScript/TypeScript JSDoc block comment (opens with two stars, closes with star-slash). |
| `RUSTDOC` | Rust `///` or `//!` doc comment. |
| `GO_DOC` | Go doc comment (a comment block immediately preceding a declaration). |
| `JAVA_DOC` | Java Javadoc block comment (opens with two stars, closes with star-slash). |
| `OTHER` | A language-specific docstring format not covered by the standard variants. — Fields: `0`: `String` |

---

#### ExportKind

The kind of an export statement found in source code.

Covers named exports, default exports, and re-exports from other modules.

| Value | Description |
|-------|-------------|
| `NAMED` | A named export (e.g., `export { foo }`). |
| `DEFAULT` | A default export (e.g., `export default foo`). |
| `RE_EXPORT` | A re-export from another module (e.g., `export { foo } from 'bar'`). |

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
| `VARIABLE` | A variable binding. |
| `CONSTANT` | A constant (immutable binding). |
| `FUNCTION` | A function definition. |
| `CLASS` | A class definition. |
| `TYPE` | A type alias or typedef. |
| `INTERFACE` | An interface definition. |
| `ENUM` | An enum definition. |
| `MODULE` | A module declaration. |
| `OTHER` | A symbol kind not covered by the standard variants. — Fields: `0`: `String` |

---

#### DiagnosticSeverity

Severity level of a diagnostic produced during parsing.

Used to classify parse errors, warnings, and informational messages
found in the syntax tree.

| Value | Description |
|-------|-------------|
| `ERROR` | A parse error (e.g., an `ERROR` or `MISSING` node in the tree). |
| `WARNING` | A warning-level diagnostic. |
| `INFO` | An informational diagnostic. |

---

### Errors

#### Error

Errors that can occur when using the tree-sitter language pack.

Covers language lookup failures, parse errors, query errors, and I/O issues.
Feature-gated variants are included when `config`, `download`, or related
features are enabled.

| Variant | Description |
|---------|-------------|
| `LANGUAGE_NOT_FOUND` | The requested language name (or alias) was not found in the registry. |
| `DYNAMIC_LOAD` | A dynamic shared library could not be loaded at runtime. |
| `NULL_LANGUAGE_POINTER` | The tree-sitter language function returned a null pointer for the given language name. |
| `PARSER_SETUP` | The language could not be applied to the parser (e.g., ABI version mismatch). |
| `LOCK_POISONED` | An internal `RwLock` or `Mutex` was poisoned by a previous panic. |
| `CONFIG` | A configuration file or value was invalid or could not be applied. |
| `PARSE_FAILED` | The tree-sitter parser returned no tree for the given source input. |
| `QUERY_ERROR` | A tree-sitter query could not be compiled or executed. |
| `INVALID_RANGE` | A byte range was invalid (e.g., end before start, or out of bounds). |
| `DOWNLOAD` | A parser download from GitHub releases failed. |
| `CHECKSUM_MISMATCH` | The downloaded file's SHA-256 digest did not match the manifest's expected value. |
| `CACHE_LOCK` | The cross-process download cache lock file could not be acquired or created. |

---
