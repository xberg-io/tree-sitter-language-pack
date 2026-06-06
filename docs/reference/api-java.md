---
title: "Java API Reference"
---

## Java API Reference <span class="version-badge">v1.9.0-rc.23</span>

### Functions

#### detectLanguageFromExtension()

Detect language name from a file extension (without leading dot).

Returns `null` for unrecognized extensions. The match is case-insensitive.

**Signature:**

```java
public static Optional<String> detectLanguageFromExtension(String ext)
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

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `config` | `PackConfig` | Yes | The configuration options |

**Returns:** `void`
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

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `config` | `PackConfig` | Yes | The configuration options |

**Returns:** `void`
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

**Returns:** `long`
**Errors:** Throws `ErrorException`.

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

```java
public static long downloadGroup(String name) throws Error
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

**Returns:** `void`
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
| `language` | `String` | — | Language |
| `chunkIndex` | `long` | — | Chunk index |
| `totalChunks` | `long` | — | Total chunks |
| `nodeTypes` | `List<String>` | `Collections.emptyList()` | Node types |
| `contextPath` | `List<String>` | `Collections.emptyList()` | Context path |
| `symbolsDefined` | `List<String>` | `Collections.emptyList()` | Symbols defined |
| `comments` | `List<CommentInfo>` | `Collections.emptyList()` | Comments |
| `docstrings` | `List<DocstringInfo>` | `Collections.emptyList()` | Docstrings |
| `hasErrorNodes` | `boolean` | — | Whether error nodes |

---

#### CodeChunk

A chunk of source code with rich metadata.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `content` | `String` | — | The extracted text content |
| `startByte` | `long` | — | Start byte |
| `endByte` | `long` | — | End byte |
| `startLine` | `long` | — | Start line |
| `endLine` | `long` | — | End line |
| `metadata` | `ChunkContext` | — | Document metadata |

---

#### CommentInfo

A comment extracted from source code.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `text` | `String` | — | Text |
| `kind` | `CommentKind` | `CommentKind.LINE` | Kind (comment kind) |
| `span` | `Span` | — | Span (span) |
| `associatedNode` | `Optional<String>` | `null` | Associated node |

---

#### Diagnostic

A diagnostic (syntax error, missing node, etc.) from parsing.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `message` | `String` | — | Message |
| `severity` | `DiagnosticSeverity` | `DiagnosticSeverity.ERROR` | Severity (diagnostic severity) |
| `span` | `Span` | — | Span (span) |

---

#### DocSection

A section within a docstring (e.g., Args, Returns, Raises).

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `kind` | `String` | — | Kind |
| `name` | `Optional<String>` | `null` | The name |
| `description` | `String` | — | Human-readable description |

---

#### DocstringInfo

A docstring extracted from source code.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `text` | `String` | — | Text |
| `format` | `DocstringFormat` | `DocstringFormat.PYTHON_TRIPLE_QUOTE` | Format (docstring format) |
| `span` | `Span` | — | Span (span) |
| `associatedItem` | `Optional<String>` | `null` | Associated item |
| `parsedSections` | `List<DocSection>` | `Collections.emptyList()` | Parsed sections |

---

#### DownloadManager

Manages downloading and caching of pre-built parser shared libraries.

### Methods

#### new()

Create a new download manager for the given version.

**Signature:**

```java
public static DownloadManager new(String version) throws Error
```

#### installedLanguages()

List languages that are already downloaded and cached.

**Signature:**

```java
public List<String> installedLanguages()
```

#### downloadAllBestEffort()

Download the platform bundle and extract every library file it contains.

Unlike `ensure_languages`, this does not check the manifest language list
against archive contents — it simply extracts all `.so`/`.dylib`/`.dll` files
from the bundle. Languages in the manifest that are missing from the archive
are silently ignored rather than returning an error.

Returns the number of library files extracted (including those already cached).

**Signature:**

```java
public long downloadAllBestEffort() throws Error
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

```java
public void cleanCache() throws Error
```

---

#### ExportInfo

An export statement extracted from source code.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `name` | `String` | — | The name |
| `kind` | `ExportKind` | `ExportKind.NAMED` | Kind (export kind) |
| `span` | `Span` | — | Span (span) |

---

#### FileMetrics

Aggregate metrics for a source file.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `totalLines` | `long` | — | Total lines |
| `codeLines` | `long` | — | Code lines |
| `commentLines` | `long` | — | Comment lines |
| `blankLines` | `long` | — | Blank lines |
| `totalBytes` | `long` | — | Total bytes |
| `nodeCount` | `long` | — | Number of nodes |
| `errorCount` | `long` | — | Number of errors |
| `maxDepth` | `long` | — | Maximum depth |

---

#### ImportInfo

An import statement extracted from source code.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `source` | `String` | — | Source |
| `items` | `List<String>` | `Collections.emptyList()` | Items |
| `alias` | `Optional<String>` | `null` | Alias |
| `isWildcard` | `boolean` | — | Whether wildcard |
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

#### new()

Create a new registry populated with all statically compiled languages.

When the `dynamic-loading` feature is enabled, the registry also knows
about dynamically loadable grammars and will load them on demand.

**Signature:**

```java
public static LanguageRegistry new()
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

```java
public Language getLanguage(String name) throws Error
```

#### availableLanguages()

List all available language names, sorted and deduplicated.

Includes statically compiled languages, dynamically loadable languages
(if the `dynamic-loading` feature is enabled), and all configured aliases.

**Signature:**

```java
public List<String> availableLanguages()
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

```java
public boolean hasParser(String name)
```

#### hasLanguage()

Check whether a language is available by name or alias.

Returns `true` if the language can be loaded, either from the static
table or from a dynamic library on disk.

**Signature:**

```java
public boolean hasLanguage(String name)
```

#### languageCount()

Return the total number of available languages (including aliases).

**Signature:**

```java
public long languageCount()
```

#### process()

Parse source code and extract file intelligence based on config in a single pass.

**Signature:**

```java
public ProcessResult process(String source, ProcessConfig config) throws Error
```

#### defaultOptions()

**Signature:**

```java
public static LanguageRegistry defaultOptions()
```

---

#### Node

A single syntax node within a `Tree`.

Nodes hold a strong reference to their parent tree so they remain valid
regardless of how the tree is moved or stored at the FFI boundary.

### Methods

#### clone()

**Signature:**

```java
public Node clone()
```

#### kind()

Return the node's kind name (e.g. `"function_definition"`).

**Signature:**

```java
public String kind()
```

#### kindId()

Return the node's numeric kind ID.

Tree-sitter assigns a stable `u16` ID to every node kind in a grammar
(e.g. `"function_definition" → 42`). Comparing `kind_id()` is cheaper
than comparing the string `kind()` in tight AST loops.

**Signature:**

```java
public short kindId()
```

#### startByte()

Return the inclusive start byte offset of this node.

**Signature:**

```java
public long startByte()
```

#### endByte()

Return the exclusive end byte offset of this node.

**Signature:**

```java
public long endByte()
```

#### byteRange()

Return the node's byte range as a `ByteRange`.

Callers should slice their own source bytes — this is a zero-copy
text accessor.

**Signature:**

```java
public ByteRange byteRange()
```

#### startPosition()

Return the start `Point` (row, column).

**Signature:**

```java
public Point startPosition()
```

#### endPosition()

Return the end `Point` (row, column).

**Signature:**

```java
public Point endPosition()
```

#### isNamed()

True when this node is named (not punctuation/whitespace).

**Signature:**

```java
public boolean isNamed()
```

#### isError()

True when this is an error node.

**Signature:**

```java
public boolean isError()
```

#### isMissing()

True when this is a missing-token node.

**Signature:**

```java
public boolean isMissing()
```

#### isExtra()

True when this is an "extra" node (e.g. a comment).

**Signature:**

```java
public boolean isExtra()
```

#### hasError()

True when this node or any descendant is an error.

**Signature:**

```java
public boolean hasError()
```

#### parent()

Return this node's parent, if any.

**Signature:**

```java
public Optional<Node> parent()
```

#### child()

Return the i-th child of this node, if any.

**Signature:**

```java
public Optional<Node> child(int index)
```

#### childCount()

Total number of children (including unnamed).

**Signature:**

```java
public long childCount()
```

#### namedChild()

Return the i-th named child of this node, if any.

**Signature:**

```java
public Optional<Node> namedChild(int index)
```

#### namedChildCount()

Number of named children of this node.

**Signature:**

```java
public long namedChildCount()
```

#### childByFieldName()

Look up a child by its grammar-defined field name.

**Signature:**

```java
public Optional<Node> childByFieldName(String name)
```

#### toSexp()

Return the S-expression form of this node's subtree.

**Signature:**

```java
public String toSexp()
```

#### walk()

Return a `TreeCursor` positioned at this node.

**Signature:**

```java
public TreeCursor walk()
```

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

### Methods

#### new()

Construct a new parser with no language set.

Call `Parser.set_language` before parsing.

**Signature:**

```java
public static Parser new()
```

#### setLanguage()

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

#### parse()

Parse a UTF-8 source string. Returns `null` if parsing was cancelled
or no language is set.

**Signature:**

```java
public Optional<Tree> parse(String source)
```

#### parseBytes()

Parse a raw byte slice. Returns `null` if parsing was cancelled or
no language is set.

**Signature:**

```java
public Optional<Tree> parseBytes(byte[] source)
```

#### reset()

Reset internal state. The next call to `parse` will
not be incremental.

**Signature:**

```java
public void reset()
```

#### defaultOptions()

**Signature:**

```java
public static Parser defaultOptions()
```

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

### Methods

#### defaultOptions()

**Signature:**

```java
public static ProcessConfig defaultOptions()
```

#### withChunking()

Enable chunking with the given maximum chunk size in bytes.

**Signature:**

```java
public ProcessConfig withChunking(long maxSize)
```

#### all()

Enable all analysis features.

**Signature:**

```java
public ProcessConfig all()
```

#### minimal()

Disable all analysis features (only metrics computed).

**Signature:**

```java
public ProcessConfig minimal()
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
| `structure` | `List<StructureItem>` | `Collections.emptyList()` | Structure |
| `imports` | `List<ImportInfo>` | `Collections.emptyList()` | Imports |
| `exports` | `List<ExportInfo>` | `Collections.emptyList()` | Exports |
| `comments` | `List<CommentInfo>` | `Collections.emptyList()` | Comments |
| `docstrings` | `List<DocstringInfo>` | `Collections.emptyList()` | Docstrings |
| `symbols` | `List<SymbolInfo>` | `Collections.emptyList()` | Symbols |
| `diagnostics` | `List<Diagnostic>` | `Collections.emptyList()` | Diagnostics |
| `chunks` | `List<CodeChunk>` | `Collections.emptyList()` | Text chunks for chunking/embedding |

---

#### Span

Byte and line/column range in source code.

Represents both byte offsets (for slicing) and human-readable line/column
positions (for display and diagnostics).

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `startByte` | `long` | — | Start byte |
| `endByte` | `long` | — | End byte |
| `startLine` | `long` | — | Start line |
| `startColumn` | `long` | — | Start column |
| `endLine` | `long` | — | End line |
| `endColumn` | `long` | — | End column |

---

#### StructureItem

A structural item (function, class, struct, etc.) in source code.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `kind` | `StructureKind` | `StructureKind.FUNCTION` | Kind (structure kind) |
| `name` | `Optional<String>` | `null` | The name |
| `visibility` | `Optional<String>` | `null` | Visibility |
| `span` | `Span` | — | Span (span) |
| `children` | `List<StructureItem>` | `Collections.emptyList()` | Children |
| `decorators` | `List<String>` | `Collections.emptyList()` | Decorators |
| `docComment` | `Optional<String>` | `null` | Doc comment |
| `signature` | `Optional<String>` | `null` | Signature |
| `bodySpan` | `Optional<Span>` | `null` | Body span (span) |

---

#### SymbolInfo

A symbol (variable, function, type, etc.) extracted from source code.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `name` | `String` | — | The name |
| `kind` | `SymbolKind` | `SymbolKind.VARIABLE` | Kind (symbol kind) |
| `span` | `Span` | — | Span (span) |
| `typeAnnotation` | `Optional<String>` | `null` | Type annotation |
| `doc` | `Optional<String>` | `null` | Doc |

---

#### Tree

A parsed syntax tree. Cheap to clone (refcount bump).

### Methods

#### rootNode()

Return the root `Node` of this tree.

**Signature:**

```java
public Node rootNode()
```

#### walk()

Return a `TreeCursor` positioned at the root.

**Signature:**

```java
public TreeCursor walk()
```

---

#### TreeCursor

A cursor for traversing a `Tree`.

### Methods

#### node()

Return the `Node` at the cursor's current position.

**Signature:**

```java
public Node node()
```

#### gotoFirstChild()

Move the cursor to the first child of the current node.
Returns `true` if a child existed.

**Signature:**

```java
public boolean gotoFirstChild()
```

#### gotoParent()

Move the cursor to the parent of the current node.
Returns `true` if a parent existed.

**Signature:**

```java
public boolean gotoParent()
```

#### gotoNextSibling()

Move the cursor to the next sibling of the current node.
Returns `true` if a sibling existed.

**Signature:**

```java
public boolean gotoNextSibling()
```

#### fieldName()

Return the field name for the current node, if any.

**Signature:**

```java
public Optional<String> fieldName()
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
| `FUNCTION` | Function |
| `METHOD` | Method |
| `CLASS` | Class |
| `STRUCT` | Struct |
| `INTERFACE` | Interface |
| `ENUM` | Enum |
| `MODULE` | Module |
| `TRAIT` | Trait |
| `IMPL` | Impl |
| `NAMESPACE` | Namespace |
| `OTHER` | Other — Fields: `0`: `String` |

---

#### CommentKind

The kind of a comment found in source code.

Distinguishes between single-line comments, block (multi-line) comments,
and documentation comments.

| Value | Description |
|-------|-------------|
| `LINE` | Line |
| `BLOCK` | Block |
| `DOC` | Doc |

---

#### DocstringFormat

The format of a docstring extracted from source code.

Identifies the docstring convention used, which varies by language
(e.g., Python triple-quoted strings, JSDoc, Rustdoc `///` comments).

| Value | Description |
|-------|-------------|
| `PYTHON_TRIPLE_QUOTE` | Python triple quote |
| `JS_DOC` | J s doc |
| `RUSTDOC` | Rustdoc |
| `GO_DOC` | Go doc |
| `JAVA_DOC` | Java doc |
| `OTHER` | Other — Fields: `0`: `String` |

---

#### ExportKind

The kind of an export statement found in source code.

Covers named exports, default exports, and re-exports from other modules.

| Value | Description |
|-------|-------------|
| `NAMED` | Named |
| `DEFAULT` | Default |
| `RE_EXPORT` | Re export |

---

#### SymbolKind

The kind of a symbol definition found in source code.

Categorizes symbol definitions such as variables, constants, functions,
classes, types, interfaces, enums, and modules.

| Value | Description |
|-------|-------------|
| `VARIABLE` | Variable |
| `CONSTANT` | Constant |
| `FUNCTION` | Function |
| `CLASS` | Class |
| `TYPE` | Type |
| `INTERFACE` | Interface |
| `ENUM` | Enum |
| `MODULE` | Module |
| `OTHER` | Other — Fields: `0`: `String` |

---

#### DiagnosticSeverity

Severity level of a diagnostic produced during parsing.

Used to classify parse errors, warnings, and informational messages
found in the syntax tree.

| Value | Description |
|-------|-------------|
| `ERROR` | Error |
| `WARNING` | Warning |
| `INFO` | Info |

---

### Errors

#### Error

Errors that can occur when using the tree-sitter language pack.

Covers language lookup failures, parse errors, query errors, and I/O issues.
Feature-gated variants are included when `config`, `download`, or related
features are enabled.

| Variant | Description |
|---------|-------------|
| `LANGUAGE_NOT_FOUND` | Language '{0}' not found |
| `DYNAMIC_LOAD` | Dynamic library load error: {0} |
| `NULL_LANGUAGE_POINTER` | Language function returned null pointer for '{0}' |
| `PARSER_SETUP` | Failed to set parser language: {0} |
| `LOCK_POISONED` | Registry lock poisoned: {0} |
| `CONFIG` | Configuration error: {0} |
| `PARSE_FAILED` | Parse failed: parsing returned no tree |
| `QUERY_ERROR` | Query error: {0} |
| `INVALID_RANGE` | Invalid byte range: {0} |
| `DOWNLOAD` | Download error: {0} |
| `CHECKSUM_MISMATCH` | Checksum mismatch for '{file}': expected {expected}, got {actual} |
| `CACHE_LOCK` | Download cache lock error: {0} |

---
