---
title: "Java API Reference"
---

## Java API Reference <span class="version-badge">v1.8.0-rc.24</span>

### Functions

#### DetectLanguageFromExtension()

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

#### DetectLanguageFromPath()

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

```java
public static Optional<String> detectLanguageFromContent(String content)
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `content` | `String` | Yes | The content to process |

**Returns:** `Optional<String>`


---

#### RootNodeInfo()

Get a `NodeInfo` snapshot of the root node.

**Signature:**

```java
public static NodeInfo rootNodeInfo(Tree tree)
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `tree` | `Tree` | Yes | The tree |

**Returns:** `NodeInfo`


---

#### FindNodesByType()

Find all nodes matching the given type name, returning their `NodeInfo`.

Performs a depth-first traversal. Returns an empty vec if no matches.

**Signature:**

```java
public static List<NodeInfo> findNodesByType(Tree tree, String nodeType)
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `tree` | `Tree` | Yes | The tree |
| `nodeType` | `String` | Yes | The node type |

**Returns:** `List<NodeInfo>`


---

#### NamedChildrenInfo()

Get `NodeInfo` for all named children of the root node.

Useful for understanding the top-level structure of a file
(e.g., list of function definitions, class declarations, imports).

**Signature:**

```java
public static List<NodeInfo> namedChildrenInfo(Tree tree)
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `tree` | `Tree` | Yes | The tree |

**Returns:** `List<NodeInfo>`


---

#### ParseString()

Parse source code with the named language, returning the syntax tree.

Uses the global registry to look up the language by name.
Caches parsers per-thread so repeated calls for the same language avoid
re-creating the parser.

**Signature:**

```java
public static Tree parseString(String language, byte[] source) throws Error
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `language` | `String` | Yes | The language |
| `source` | `byte[]` | Yes | The source |

**Returns:** `Tree`

**Errors:** Throws `ErrorException`.


---

#### TreeContainsNodeType()

Check whether any node in the tree matches the given type name.

Performs a depth-first traversal using `TreeCursor`.

**Signature:**

```java
public static boolean treeContainsNodeType(Tree tree, String nodeType)
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `tree` | `Tree` | Yes | The tree |
| `nodeType` | `String` | Yes | The node type |

**Returns:** `boolean`


---

#### TreeHasErrorNodes()

Check whether the tree contains any ERROR or MISSING nodes.

Useful for determining if the parse was clean or had syntax errors.

**Signature:**

```java
public static boolean treeHasErrorNodes(Tree tree)
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `tree` | `Tree` | Yes | The tree |

**Returns:** `boolean`


---

#### TreeToSexp()

Return the S-expression representation of the entire tree.

This is the standard tree-sitter debug format, useful for logging,
snapshot testing, and debugging grammars.

**Signature:**

```java
public static String treeToSexp(Tree tree)
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `tree` | `Tree` | Yes | The tree |

**Returns:** `String`


---

#### TreeErrorCount()

Count the number of ERROR and MISSING nodes in the tree.

Returns 0 for a clean parse.

**Signature:**

```java
public static long treeErrorCount(Tree tree)
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `tree` | `Tree` | Yes | The tree |

**Returns:** `long`


---

#### GetHighlightsQuery()

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

#### GetInjectionsQuery()

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

#### GetLocalsQuery()

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

#### RunQuery()

Execute a tree-sitter query pattern against a parsed tree.

The `query_source` is an S-expression pattern like:

```text
(function_definition name: (identifier) @name)
```

Returns all matches with their captured nodes.

**Signature:**

```java
public static List<QueryMatch> runQuery(Tree tree, String language, String querySource, byte[] source) throws Error
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `tree` | `Tree` | Yes | The parsed syntax tree to query. |
| `language` | `String` | Yes | Language name (used to compile the query pattern). |
| `querySource` | `String` | Yes | The tree-sitter query pattern string. |
| `source` | `byte[]` | Yes | The original source code bytes (needed for capture resolution). |

**Returns:** `List<QueryMatch>`

**Errors:** Throws `ErrorException`.


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

#### GetParser()

Get a tree-sitter `Parser` pre-configured for the given language.

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

#### AvailableLanguages()

List all available language names (sorted, deduplicated, includes aliases).

Returns names of both statically compiled and dynamically loadable languages,
plus any configured aliases.

**Signature:**

```java
public static List<String> availableLanguages()
```

**Returns:** `List<String>`


---

#### HasLanguage()

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

#### LanguageCount()

Return the number of available languages.

Includes statically compiled languages, dynamically loadable languages,
and aliases.

**Signature:**

```java
public static long languageCount()
```

**Returns:** `long`


---

#### Process()

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

#### ExtractPatterns()

Run extraction patterns against source code.

Convenience wrapper around `extract.extract`.

**Errors:**

Returns an error if the language is not found, parsing fails, or a query
pattern is invalid.

**Signature:**

```java
public static ExtractionResult extractPatterns(String source, ExtractionConfig config) throws Error
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `source` | `String` | Yes | The source |
| `config` | `ExtractionConfig` | Yes | The configuration options |

**Returns:** `ExtractionResult`

**Errors:** Throws `ErrorException`.


---

#### ValidateExtraction()

Validate extraction patterns without running them.

Convenience wrapper around `extract.validate_extraction`.

**Errors:**

Returns an error if the language cannot be loaded.

**Signature:**

```java
public static ValidationResult validateExtraction(ExtractionConfig config) throws Error
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `config` | `ExtractionConfig` | Yes | The configuration options |

**Returns:** `ValidationResult`

**Errors:** Throws `ErrorException`.


---

#### Init()

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

#### Configure()

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

#### Download()

Download specific languages to the local cache.

Returns the number of newly downloaded languages (languages that were
already cached are not counted).

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

#### DownloadAll()

Download all available languages from the remote manifest.

Returns the number of newly downloaded languages.

**Errors:**

Returns an error if the manifest cannot be fetched or a download fails.

**Signature:**

```java
public static long downloadAll() throws Error
```

**Returns:** `long`

**Errors:** Throws `ErrorException`.


---

#### ManifestLanguages()

Return all language names available in the remote manifest (305).

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

#### DownloadedLanguages()

Return languages that are already downloaded and cached locally.

Does not perform any network requests. Returns an empty list if the
cache directory does not exist or cannot be read.

**Signature:**

```java
public static List<String> downloadedLanguages()
```

**Returns:** `List<String>`


---

#### CleanCache()

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

#### CacheDir()

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

#### CaptureResult

A single captured node within a match.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `name` | `String` | — | The capture name from the query (e.g., `"fn_name"`). |
| `node` | `Optional<NodeInfo>` | `null` | The `NodeInfo` snapshot, present when `CaptureOutput` is `Node` or `Full`. |
| `text` | `Optional<String>` | `null` | The matched source text, present when `CaptureOutput` is `Text` or `Full`. |
| `childFields` | `String` | — | Values of requested child fields, keyed by field name. |
| `startByte` | `long` | — | Byte offset where this capture starts in the source. |


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

##### Methods

###### New()

Create a new download manager for the given version.

**Signature:**

```java
public static DownloadManager new(String version) throws Error
```

###### WithCacheDir()

Create a download manager with a custom cache directory.

**Signature:**

```java
public static DownloadManager withCacheDir(String version, String cacheDir)
```

###### DefaultCacheDir()

Default cache directory: `~/.cache/tree-sitter-language-pack/v{version}/libs/`

**Signature:**

```java
public static String defaultCacheDir(String version) throws Error
```

###### CacheDir()

Return the path to the libs cache directory.

**Signature:**

```java
public String cacheDir()
```

###### InstalledLanguages()

List languages that are already downloaded and cached.

**Signature:**

```java
public List<String> installedLanguages()
```

###### EnsureLanguages()

Ensure the specified languages are available in the cache.
Downloads the platform bundle if any requested languages are missing.

**Signature:**

```java
public void ensureLanguages(List<String> names) throws Error
```

###### EnsureGroup()

Ensure all languages in a named group are available.

**Signature:**

```java
public void ensureGroup(String group) throws Error
```

###### LibPath()

Get the expected path for a language's shared library in the cache.

**Signature:**

```java
public String libPath(String name)
```

###### FetchManifest()

Fetch the parser manifest from GitHub Releases.

**Signature:**

```java
public ParserManifest fetchManifest() throws Error
```

###### CleanCache()

Remove all cached parser libraries.

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

#### ExtractionConfig

Configuration for an extraction run against a single language.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `language` | `String` | — | The language name (e.g., `"python"`). |
| `patterns` | `String` | — | Named patterns to run. Keys become the keys in `ExtractionResult.results`. |


---

#### ExtractionPattern

Defines a single extraction pattern and its configuration.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `query` | `String` | — | The tree-sitter query string (S-expression). |
| `captureOutput` | `CaptureOutput` | `CaptureOutput.FULL` | What to include in each capture result. |
| `childFields` | `List<String>` | `Collections.emptyList()` | Field names to extract from child nodes of each capture. Maps a label to a tree-sitter field name used with `child_by_field_name`. |
| `maxResults` | `Optional<Long>` | `null` | Maximum number of matches to return. `null` means unlimited. |
| `byteRange` | `Optional<List<Long>>` | `Collections.emptyList()` | Restrict matches to a byte range `(start, end)`. |


---

#### ExtractionResult

Complete extraction results for all patterns.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `language` | `String` | — | The language that was used. |
| `results` | `String` | — | Results keyed by pattern name. |


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

#### LanguageInfo

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `group` | `String` | — | Group |
| `size` | `long` | — | Size in bytes |


---

#### LanguageRegistry

Thread-safe registry of tree-sitter language parsers.

Manages both statically compiled and dynamically loaded language grammars.
Use `LanguageRegistry.new()` for the default registry, or access the
global instance via the module-level convenience functions
(`crate.get_language`, `crate.available_languages`, etc.).

##### Methods

###### WithLibsDir()

Create a registry with a custom directory for dynamic libraries.

Overrides the default build-time library directory. Useful when
dynamic grammar shared libraries are stored in a non-standard location.

**Signature:**

```java
public static LanguageRegistry withLibsDir(String libsDir)
```

###### AddExtraLibsDir()

Add an additional directory to search for dynamic libraries.

When `get_language` cannot find a grammar in the
primary library directory, it searches these extra directories in order.
Typically used by the download system to register its cache directory.

Takes `&self` (not `&mut self`) because `extra_lib_dirs` uses interior
mutability via an `Arc<RwLock<...>>`, so the outer registry can remain
immutable while the directory list is updated.

**Signature:**

```java
public void addExtraLibsDir(String dir)
```

###### GetLanguage()

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

###### AvailableLanguages()

List all available language names, sorted and deduplicated.

Includes statically compiled languages, dynamically loadable languages
(if the `dynamic-loading` feature is enabled), and all configured aliases.

**Signature:**

```java
public List<String> availableLanguages()
```

###### HasLanguage()

Check whether a language is available by name or alias.

Returns `true` if the language can be loaded, either from the static
table or from a dynamic library on disk.

**Signature:**

```java
public boolean hasLanguage(String name)
```

###### LanguageCount()

Return the total number of available languages (including aliases).

**Signature:**

```java
public long languageCount()
```

###### Process()

Parse source code and extract file intelligence based on config in a single pass.

**Signature:**

```java
public ProcessResult process(String source, ProcessConfig config) throws Error
```

###### DefaultOptions()

**Signature:**

```java
public static LanguageRegistry defaultOptions()
```


---

#### MatchResult

A single query match containing one or more captures.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `patternIndex` | `long` | — | The pattern index within the query that produced this match. |
| `captures` | `List<CaptureResult>` | `Collections.emptyList()` | The captures for this match. |


---

#### NodeInfo

Lightweight snapshot of a tree-sitter node's properties.

Contains only primitive types for easy cross-language serialization.
This is an owned type that can be passed across FFI boundaries, unlike
`tree_sitter.Node` which borrows from the tree.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `kind` | `String` | — | The grammar type name (e.g., "function_definition", "identifier"). |
| `isNamed` | `boolean` | — | Whether this is a named node (vs anonymous like punctuation). |
| `startByte` | `long` | — | Start byte offset in source. |
| `endByte` | `long` | — | End byte offset in source. |
| `startRow` | `long` | — | Start row (zero-indexed). |
| `startCol` | `long` | — | Start column (zero-indexed). |
| `endRow` | `long` | — | End row (zero-indexed). |
| `endCol` | `long` | — | End column (zero-indexed). |
| `namedChildCount` | `long` | — | Number of named children. |
| `isError` | `boolean` | — | Whether this node is an ERROR node. |
| `isMissing` | `boolean` | — | Whether this node is a MISSING node. |


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

##### Methods

###### FromTomlFile()

Load configuration from a TOML file.

**Errors:**

Returns an error if the file cannot be read or the TOML is invalid.

**Signature:**

```java
public static PackConfig fromTomlFile(String path) throws Error
```

###### Discover()

Discover configuration by searching for `language-pack.toml` in:

1. Current directory and up to 10 parent directories
2. `$XDG_CONFIG_HOME/tree-sitter-language-pack/config.toml`
3. `~/.config/tree-sitter-language-pack/config.toml`

Returns `null` if no configuration file is found.

**Signature:**

```java
public static Optional<PackConfig> discover()
```


---

#### Parser


---

#### ParserManifest

Manifest describing available parser downloads for a specific version.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `version` | `String` | — | Version string |
| `platforms` | `Map<String, PlatformBundle>` | — | Platforms |
| `languages` | `Map<String, LanguageInfo>` | — | Languages |
| `groups` | `Map<String, List<String>>` | — | Groups |


---

#### PatternResult

Results for a single named pattern.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `matches` | `List<MatchResult>` | `Collections.emptyList()` | The individual matches. |
| `totalCount` | `long` | — | Total number of matches before `max_results` truncation. |


---

#### PatternValidation

Validation information for a single pattern.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `valid` | `boolean` | — | Whether the pattern compiled successfully. |
| `captureNames` | `List<String>` | `Collections.emptyList()` | Names of captures defined in the query. |
| `patternCount` | `long` | — | Number of patterns in the query. |
| `warnings` | `List<String>` | `Collections.emptyList()` | Non-fatal warnings (e.g., unused captures). |
| `errors` | `List<String>` | `Collections.emptyList()` | Fatal errors (e.g., query syntax errors). |


---

#### PlatformBundle

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `url` | `String` | — | Url |
| `sha256` | `String` | — | Sha256 |
| `size` | `long` | — | Size in bytes |


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
| `extractions` | `Optional<String>` | `null` | Custom extraction patterns to run against the parsed tree. Keys become the keys in `ProcessResult.extractions`. |

##### Methods

###### DefaultOptions()

**Signature:**

```java
public static ProcessConfig defaultOptions()
```

###### WithChunking()

Enable chunking with the given maximum chunk size in bytes.

**Signature:**

```java
public ProcessConfig withChunking(long maxSize)
```

###### All()

Enable all analysis features.

**Signature:**

```java
public ProcessConfig all()
```

###### Minimal()

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
Fields are populated based on the `crate.ProcessConfig` flags.

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
| `extractions` | `String` | — | Results of custom extraction patterns (when `config.extractions` is set). |


---

#### QueryMatch

A single match from a tree-sitter query, with captured nodes.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `patternIndex` | `long` | — | The pattern index that matched (position in the query string). |
| `captures` | `List<String>` | `Collections.emptyList()` | Captures: list of (capture_name, node_info) pairs. |


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


---

#### ValidationResult

Validation results for an entire extraction config.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `valid` | `boolean` | — | Whether all patterns are valid. |
| `patterns` | `String` | — | Per-pattern validation details. |


---

### Enums

#### CaptureOutput

Controls what data is captured for each query match.

| Value | Description |
|-------|-------------|
| `TEXT` | Capture only the matched text. |
| `NODE` | Capture only the `NodeInfo`. |
| `FULL` | Capture both text and `NodeInfo` (default). |


---

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
| `IO` | IO error: {0} |


---
