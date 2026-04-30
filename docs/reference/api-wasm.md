---
title: "WebAssembly API Reference"
---

## WebAssembly API Reference <span class="version-badge">v1.8.0-rc.24</span>

### Functions

#### DetectLanguageFromExtension()

Detect language name from a file extension (without leading dot).

Returns `null` for unrecognized extensions. The match is case-insensitive.

**Signature:**

```typescript
function detectLanguageFromExtension(ext: string): string | null
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `ext` | `string` | Yes | The ext |

**Returns:** `string | null`


---

#### DetectLanguageFromPath()

Detect language name from a file path.

Extracts the file extension and looks it up. Returns `null` if the
path has no extension or the extension is not recognized.

**Signature:**

```typescript
function detectLanguageFromPath(path: string): string | null
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `path` | `string` | Yes | Path to the file |

**Returns:** `string | null`


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

```typescript
function detectLanguageFromContent(content: string): string | null
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `content` | `string` | Yes | The content to process |

**Returns:** `string | null`


---

#### RootNodeInfo()

Get a `NodeInfo` snapshot of the root node.

**Signature:**

```typescript
function rootNodeInfo(tree: Tree): NodeInfo
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

```typescript
function findNodesByType(tree: Tree, nodeType: string): Array<NodeInfo>
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `tree` | `Tree` | Yes | The tree |
| `nodeType` | `string` | Yes | The node type |

**Returns:** `Array<NodeInfo>`


---

#### NamedChildrenInfo()

Get `NodeInfo` for all named children of the root node.

Useful for understanding the top-level structure of a file
(e.g., list of function definitions, class declarations, imports).

**Signature:**

```typescript
function namedChildrenInfo(tree: Tree): Array<NodeInfo>
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `tree` | `Tree` | Yes | The tree |

**Returns:** `Array<NodeInfo>`


---

#### ParseString()

Parse source code with the named language, returning the syntax tree.

Uses the global registry to look up the language by name.
Caches parsers per-thread so repeated calls for the same language avoid
re-creating the parser.

**Signature:**

```typescript
function parseString(language: string, source: Buffer): Tree
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `language` | `string` | Yes | The language |
| `source` | `Buffer` | Yes | The source |

**Returns:** `Tree`

**Errors:** Throws `Error` with a descriptive message.


---

#### TreeContainsNodeType()

Check whether any node in the tree matches the given type name.

Performs a depth-first traversal using `TreeCursor`.

**Signature:**

```typescript
function treeContainsNodeType(tree: Tree, nodeType: string): boolean
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `tree` | `Tree` | Yes | The tree |
| `nodeType` | `string` | Yes | The node type |

**Returns:** `boolean`


---

#### TreeHasErrorNodes()

Check whether the tree contains any ERROR or MISSING nodes.

Useful for determining if the parse was clean or had syntax errors.

**Signature:**

```typescript
function treeHasErrorNodes(tree: Tree): boolean
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

```typescript
function treeToSexp(tree: Tree): string
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `tree` | `Tree` | Yes | The tree |

**Returns:** `string`


---

#### TreeErrorCount()

Count the number of ERROR and MISSING nodes in the tree.

Returns 0 for a clean parse.

**Signature:**

```typescript
function treeErrorCount(tree: Tree): number
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `tree` | `Tree` | Yes | The tree |

**Returns:** `number`


---

#### GetHighlightsQuery()

Get the highlights query for a language, if bundled.

Returns the contents of `highlights.scm` as a static string, or `null`
if no highlights query is bundled for this language.

**Signature:**

```typescript
function getHighlightsQuery(language: string): string | null
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `language` | `string` | Yes | The language |

**Returns:** `string | null`


---

#### GetInjectionsQuery()

Get the injections query for a language, if bundled.

Returns the contents of `injections.scm` as a static string, or `null`
if no injections query is bundled for this language.

**Signature:**

```typescript
function getInjectionsQuery(language: string): string | null
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `language` | `string` | Yes | The language |

**Returns:** `string | null`


---

#### GetLocalsQuery()

Get the locals query for a language, if bundled.

Returns the contents of `locals.scm` as a static string, or `null`
if no locals query is bundled for this language.

**Signature:**

```typescript
function getLocalsQuery(language: string): string | null
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `language` | `string` | Yes | The language |

**Returns:** `string | null`


---

#### RunQuery()

Execute a tree-sitter query pattern against a parsed tree.

The `query_source` is an S-expression pattern like:

```text
(function_definition name: (identifier) @name)
```

Returns all matches with their captured nodes.

**Signature:**

```typescript
function runQuery(tree: Tree, language: string, querySource: string, source: Buffer): Array<QueryMatch>
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `tree` | `Tree` | Yes | The parsed syntax tree to query. |
| `language` | `string` | Yes | Language name (used to compile the query pattern). |
| `querySource` | `string` | Yes | The tree-sitter query pattern string. |
| `source` | `Buffer` | Yes | The original source code bytes (needed for capture resolution). |

**Returns:** `Array<QueryMatch>`

**Errors:** Throws `Error` with a descriptive message.


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

```typescript
function getLanguage(name: string): Language
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `name` | `string` | Yes | The name |

**Returns:** `Language`

**Errors:** Throws `Error` with a descriptive message.


---

#### GetParser()

Get a tree-sitter `Parser` pre-configured for the given language.

This is a convenience function that calls `get_language` and configures
a new parser in one step.

**Errors:**

Returns `Error.LanguageNotFound` if the language is not recognized, or
`Error.ParserSetup` if the language cannot be applied to the parser.

**Signature:**

```typescript
function getParser(name: string): Parser
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `name` | `string` | Yes | The name |

**Returns:** `Parser`

**Errors:** Throws `Error` with a descriptive message.


---

#### AvailableLanguages()

List all available language names (sorted, deduplicated, includes aliases).

Returns names of both statically compiled and dynamically loadable languages,
plus any configured aliases.

**Signature:**

```typescript
function availableLanguages(): Array<string>
```

**Returns:** `Array<string>`


---

#### HasLanguage()

Check if a language is available by name or alias.

Returns `true` if the language can be loaded (statically compiled,
dynamically available, or a known alias for one of these).

**Signature:**

```typescript
function hasLanguage(name: string): boolean
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `name` | `string` | Yes | The name |

**Returns:** `boolean`


---

#### LanguageCount()

Return the number of available languages.

Includes statically compiled languages, dynamically loadable languages,
and aliases.

**Signature:**

```typescript
function languageCount(): number
```

**Returns:** `number`


---

#### Process()

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

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `source` | `string` | Yes | The source |
| `config` | `ProcessConfig` | Yes | The configuration options |

**Returns:** `ProcessResult`

**Errors:** Throws `Error` with a descriptive message.


---

#### ExtractPatterns()

Run extraction patterns against source code.

Convenience wrapper around `extract.extract`.

**Errors:**

Returns an error if the language is not found, parsing fails, or a query
pattern is invalid.

**Signature:**

```typescript
function extractPatterns(source: string, config: ExtractionConfig): ExtractionResult
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `source` | `string` | Yes | The source |
| `config` | `ExtractionConfig` | Yes | The configuration options |

**Returns:** `ExtractionResult`

**Errors:** Throws `Error` with a descriptive message.


---

#### ValidateExtraction()

Validate extraction patterns without running them.

Convenience wrapper around `extract.validate_extraction`.

**Errors:**

Returns an error if the language cannot be loaded.

**Signature:**

```typescript
function validateExtraction(config: ExtractionConfig): ValidationResult
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `config` | `ExtractionConfig` | Yes | The configuration options |

**Returns:** `ValidationResult`

**Errors:** Throws `Error` with a descriptive message.


---

#### Init()

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

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `config` | `PackConfig` | Yes | The configuration options |

**Returns:** `void`

**Errors:** Throws `Error` with a descriptive message.


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

```typescript
function configure(config: PackConfig): void
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `config` | `PackConfig` | Yes | The configuration options |

**Returns:** `void`

**Errors:** Throws `Error` with a descriptive message.


---

#### Download()

Download specific languages to the local cache.

Returns the number of newly downloaded languages (languages that were
already cached are not counted).

**Errors:**

Returns an error if any language is not available in the manifest or if
the download fails.

**Signature:**

```typescript
function download(names: Array<string>): number
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `names` | `Array<string>` | Yes | The names |

**Returns:** `number`

**Errors:** Throws `Error` with a descriptive message.


---

#### DownloadAll()

Download all available languages from the remote manifest.

Returns the number of newly downloaded languages.

**Errors:**

Returns an error if the manifest cannot be fetched or a download fails.

**Signature:**

```typescript
function downloadAll(): number
```

**Returns:** `number`

**Errors:** Throws `Error` with a descriptive message.


---

#### ManifestLanguages()

Return all language names available in the remote manifest (305).

Fetches (and caches) the remote manifest to discover the full list of
downloadable languages. Use `downloaded_languages` to list what is
already cached locally.

**Errors:**

Returns an error if the manifest cannot be fetched.

**Signature:**

```typescript
function manifestLanguages(): Array<string>
```

**Returns:** `Array<string>`

**Errors:** Throws `Error` with a descriptive message.


---

#### DownloadedLanguages()

Return languages that are already downloaded and cached locally.

Does not perform any network requests. Returns an empty list if the
cache directory does not exist or cannot be read.

**Signature:**

```typescript
function downloadedLanguages(): Array<string>
```

**Returns:** `Array<string>`


---

#### CleanCache()

Delete all cached parser shared libraries.

Resets the cache registration so the next call to `get_language` or
a download function will re-register the (now empty) cache directory.

**Errors:**

Returns an error if the cache directory cannot be removed.

**Signature:**

```typescript
function cleanCache(): void
```

**Returns:** `void`

**Errors:** Throws `Error` with a descriptive message.


---

#### CacheDir()

Return the effective cache directory path.

This is either the custom path set via `configure` / `init` or the
default: `~/.cache/tree-sitter-language-pack/v{version}/libs/`.

**Errors:**

Returns an error if the system cache directory cannot be determined.

**Signature:**

```typescript
function cacheDir(): string
```

**Returns:** `string`

**Errors:** Throws `Error` with a descriptive message.


---

### Types

#### CaptureResult

A single captured node within a match.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `name` | `string` | — | The capture name from the query (e.g., `"fn_name"`). |
| `node` | `NodeInfo | null` | `null` | The `NodeInfo` snapshot, present when `CaptureOutput` is `Node` or `Full`. |
| `text` | `string | null` | `null` | The matched source text, present when `CaptureOutput` is `Text` or `Full`. |
| `childFields` | `string` | — | Values of requested child fields, keyed by field name. |
| `startByte` | `number` | — | Byte offset where this capture starts in the source. |


---

#### ChunkContext

Metadata for a single chunk of source code.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `language` | `string` | — | Language |
| `chunkIndex` | `number` | — | Chunk index |
| `totalChunks` | `number` | — | Total chunks |
| `nodeTypes` | `Array<string>` | `[]` | Node types |
| `contextPath` | `Array<string>` | `[]` | Context path |
| `symbolsDefined` | `Array<string>` | `[]` | Symbols defined |
| `comments` | `Array<CommentInfo>` | `[]` | Comments |
| `docstrings` | `Array<DocstringInfo>` | `[]` | Docstrings |
| `hasErrorNodes` | `boolean` | — | Whether error nodes |


---

#### CodeChunk

A chunk of source code with rich metadata.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `content` | `string` | — | The extracted text content |
| `startByte` | `number` | — | Start byte |
| `endByte` | `number` | — | End byte |
| `startLine` | `number` | — | Start line |
| `endLine` | `number` | — | End line |
| `metadata` | `ChunkContext` | — | Document metadata |


---

#### CommentInfo

A comment extracted from source code.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `text` | `string` | — | Text |
| `kind` | `CommentKind` | `CommentKind.Line` | Kind (comment kind) |
| `span` | `Span` | — | Span (span) |
| `associatedNode` | `string | null` | `null` | Associated node |


---

#### Diagnostic

A diagnostic (syntax error, missing node, etc.) from parsing.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `message` | `string` | — | Message |
| `severity` | `DiagnosticSeverity` | `DiagnosticSeverity.Error` | Severity (diagnostic severity) |
| `span` | `Span` | — | Span (span) |


---

#### DocSection

A section within a docstring (e.g., Args, Returns, Raises).

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `kind` | `string` | — | Kind |
| `name` | `string | null` | `null` | The name |
| `description` | `string` | — | Human-readable description |


---

#### DocstringInfo

A docstring extracted from source code.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `text` | `string` | — | Text |
| `format` | `DocstringFormat` | `DocstringFormat.PythonTripleQuote` | Format (docstring format) |
| `span` | `Span` | — | Span (span) |
| `associatedItem` | `string | null` | `null` | Associated item |
| `parsedSections` | `Array<DocSection>` | `[]` | Parsed sections |


---

#### DownloadManager

Manages downloading and caching of pre-built parser shared libraries.

##### Methods

###### New()

Create a new download manager for the given version.

**Signature:**

```typescript
static new(version: string): DownloadManager
```

###### WithCacheDir()

Create a download manager with a custom cache directory.

**Signature:**

```typescript
static withCacheDir(version: string, cacheDir: string): DownloadManager
```

###### DefaultCacheDir()

Default cache directory: `~/.cache/tree-sitter-language-pack/v{version}/libs/`

**Signature:**

```typescript
static defaultCacheDir(version: string): string
```

###### CacheDir()

Return the path to the libs cache directory.

**Signature:**

```typescript
cacheDir(): string
```

###### InstalledLanguages()

List languages that are already downloaded and cached.

**Signature:**

```typescript
installedLanguages(): Array<string>
```

###### EnsureLanguages()

Ensure the specified languages are available in the cache.
Downloads the platform bundle if any requested languages are missing.

**Signature:**

```typescript
ensureLanguages(names: Array<string>): void
```

###### EnsureGroup()

Ensure all languages in a named group are available.

**Signature:**

```typescript
ensureGroup(group: string): void
```

###### LibPath()

Get the expected path for a language's shared library in the cache.

**Signature:**

```typescript
libPath(name: string): string
```

###### FetchManifest()

Fetch the parser manifest from GitHub Releases.

**Signature:**

```typescript
fetchManifest(): ParserManifest
```

###### CleanCache()

Remove all cached parser libraries.

**Signature:**

```typescript
cleanCache(): void
```


---

#### ExportInfo

An export statement extracted from source code.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `name` | `string` | — | The name |
| `kind` | `ExportKind` | `ExportKind.Named` | Kind (export kind) |
| `span` | `Span` | — | Span (span) |


---

#### ExtractionConfig

Configuration for an extraction run against a single language.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `language` | `string` | — | The language name (e.g., `"python"`). |
| `patterns` | `string` | — | Named patterns to run. Keys become the keys in `ExtractionResult.results`. |


---

#### ExtractionPattern

Defines a single extraction pattern and its configuration.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `query` | `string` | — | The tree-sitter query string (S-expression). |
| `captureOutput` | `CaptureOutput` | `CaptureOutput.Full` | What to include in each capture result. |
| `childFields` | `Array<string>` | `[]` | Field names to extract from child nodes of each capture. Maps a label to a tree-sitter field name used with `child_by_field_name`. |
| `maxResults` | `number | null` | `null` | Maximum number of matches to return. `null` means unlimited. |
| `byteRange` | `Array<number> | null` | `[]` | Restrict matches to a byte range `(start, end)`. |


---

#### ExtractionResult

Complete extraction results for all patterns.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `language` | `string` | — | The language that was used. |
| `results` | `string` | — | Results keyed by pattern name. |


---

#### FileMetrics

Aggregate metrics for a source file.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `totalLines` | `number` | — | Total lines |
| `codeLines` | `number` | — | Code lines |
| `commentLines` | `number` | — | Comment lines |
| `blankLines` | `number` | — | Blank lines |
| `totalBytes` | `number` | — | Total bytes |
| `nodeCount` | `number` | — | Number of nodes |
| `errorCount` | `number` | — | Number of errors |
| `maxDepth` | `number` | — | Maximum depth |


---

#### ImportInfo

An import statement extracted from source code.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `source` | `string` | — | Source |
| `items` | `Array<string>` | `[]` | Items |
| `alias` | `string | null` | `null` | Alias |
| `isWildcard` | `boolean` | — | Whether wildcard |
| `span` | `Span` | — | Span (span) |


---

#### Language


---

#### LanguageInfo

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `group` | `string` | — | Group |
| `size` | `number` | — | Size in bytes |


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

```typescript
static withLibsDir(libsDir: string): LanguageRegistry
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

```typescript
addExtraLibsDir(dir: string): void
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

```typescript
getLanguage(name: string): Language
```

###### AvailableLanguages()

List all available language names, sorted and deduplicated.

Includes statically compiled languages, dynamically loadable languages
(if the `dynamic-loading` feature is enabled), and all configured aliases.

**Signature:**

```typescript
availableLanguages(): Array<string>
```

###### HasLanguage()

Check whether a language is available by name or alias.

Returns `true` if the language can be loaded, either from the static
table or from a dynamic library on disk.

**Signature:**

```typescript
hasLanguage(name: string): boolean
```

###### LanguageCount()

Return the total number of available languages (including aliases).

**Signature:**

```typescript
languageCount(): number
```

###### Process()

Parse source code and extract file intelligence based on config in a single pass.

**Signature:**

```typescript
process(source: string, config: ProcessConfig): ProcessResult
```

###### Default()

**Signature:**

```typescript
static default(): LanguageRegistry
```


---

#### MatchResult

A single query match containing one or more captures.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `patternIndex` | `number` | — | The pattern index within the query that produced this match. |
| `captures` | `Array<CaptureResult>` | `[]` | The captures for this match. |


---

#### NodeInfo

Lightweight snapshot of a tree-sitter node's properties.

Contains only primitive types for easy cross-language serialization.
This is an owned type that can be passed across FFI boundaries, unlike
`tree_sitter.Node` which borrows from the tree.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `kind` | `string` | — | The grammar type name (e.g., "function_definition", "identifier"). |
| `isNamed` | `boolean` | — | Whether this is a named node (vs anonymous like punctuation). |
| `startByte` | `number` | — | Start byte offset in source. |
| `endByte` | `number` | — | End byte offset in source. |
| `startRow` | `number` | — | Start row (zero-indexed). |
| `startCol` | `number` | — | Start column (zero-indexed). |
| `endRow` | `number` | — | End row (zero-indexed). |
| `endCol` | `number` | — | End column (zero-indexed). |
| `namedChildCount` | `number` | — | Number of named children. |
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
| `cacheDir` | `string | null` | `null` | Override default cache directory. Default: `~/.cache/tree-sitter-language-pack/v{version}/libs/` |
| `languages` | `Array<string> | null` | `[]` | Languages to pre-download on init. Each entry is a language name (e.g. `"python"`, `"rust"`). |
| `groups` | `Array<string> | null` | `[]` | Language groups to pre-download (e.g. `"web"`, `"systems"`, `"scripting"`). |

##### Methods

###### FromTomlFile()

Load configuration from a TOML file.

**Errors:**

Returns an error if the file cannot be read or the TOML is invalid.

**Signature:**

```typescript
static fromTomlFile(path: string): PackConfig
```

###### Discover()

Discover configuration by searching for `language-pack.toml` in:

1. Current directory and up to 10 parent directories
2. `$XDG_CONFIG_HOME/tree-sitter-language-pack/config.toml`
3. `~/.config/tree-sitter-language-pack/config.toml`

Returns `null` if no configuration file is found.

**Signature:**

```typescript
static discover(): PackConfig | null
```


---

#### Parser


---

#### ParserManifest

Manifest describing available parser downloads for a specific version.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `version` | `string` | — | Version string |
| `platforms` | `Record<string, PlatformBundle>` | — | Platforms |
| `languages` | `Record<string, LanguageInfo>` | — | Languages |
| `groups` | `Record<string, Array<string>>` | — | Groups |


---

#### PatternResult

Results for a single named pattern.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `matches` | `Array<MatchResult>` | `[]` | The individual matches. |
| `totalCount` | `number` | — | Total number of matches before `max_results` truncation. |


---

#### PatternValidation

Validation information for a single pattern.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `valid` | `boolean` | — | Whether the pattern compiled successfully. |
| `captureNames` | `Array<string>` | `[]` | Names of captures defined in the query. |
| `patternCount` | `number` | — | Number of patterns in the query. |
| `warnings` | `Array<string>` | `[]` | Non-fatal warnings (e.g., unused captures). |
| `errors` | `Array<string>` | `[]` | Fatal errors (e.g., query syntax errors). |


---

#### PlatformBundle

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `url` | `string` | — | Url |
| `sha256` | `string` | — | Sha256 |
| `size` | `number` | — | Size in bytes |


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
| `chunkMaxSize` | `number | null` | `null` | Maximum chunk size in bytes. `null` disables chunking. |
| `extractions` | `string | null` | `null` | Custom extraction patterns to run against the parsed tree. Keys become the keys in `ProcessResult.extractions`. |

##### Methods

###### Default()

**Signature:**

```typescript
static default(): ProcessConfig
```

###### WithChunking()

Enable chunking with the given maximum chunk size in bytes.

**Signature:**

```typescript
withChunking(maxSize: number): ProcessConfig
```

###### All()

Enable all analysis features.

**Signature:**

```typescript
all(): ProcessConfig
```

###### Minimal()

Disable all analysis features (only metrics computed).

**Signature:**

```typescript
minimal(): ProcessConfig
```


---

#### ProcessResult

Complete analysis result from processing a source file.

Contains metrics, structural analysis, imports/exports, comments,
docstrings, symbols, diagnostics, and optionally chunked code segments.
Fields are populated based on the `crate.ProcessConfig` flags.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `language` | `string` | — | Language |
| `metrics` | `FileMetrics` | — | Metrics (file metrics) |
| `structure` | `Array<StructureItem>` | `[]` | Structure |
| `imports` | `Array<ImportInfo>` | `[]` | Imports |
| `exports` | `Array<ExportInfo>` | `[]` | Exports |
| `comments` | `Array<CommentInfo>` | `[]` | Comments |
| `docstrings` | `Array<DocstringInfo>` | `[]` | Docstrings |
| `symbols` | `Array<SymbolInfo>` | `[]` | Symbols |
| `diagnostics` | `Array<Diagnostic>` | `[]` | Diagnostics |
| `chunks` | `Array<CodeChunk>` | `[]` | Text chunks for chunking/embedding |
| `extractions` | `string` | — | Results of custom extraction patterns (when `config.extractions` is set). |


---

#### QueryMatch

A single match from a tree-sitter query, with captured nodes.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `patternIndex` | `number` | — | The pattern index that matched (position in the query string). |
| `captures` | `Array<string>` | `[]` | Captures: list of (capture_name, node_info) pairs. |


---

#### Span

Byte and line/column range in source code.

Represents both byte offsets (for slicing) and human-readable line/column
positions (for display and diagnostics).

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `startByte` | `number` | — | Start byte |
| `endByte` | `number` | — | End byte |
| `startLine` | `number` | — | Start line |
| `startColumn` | `number` | — | Start column |
| `endLine` | `number` | — | End line |
| `endColumn` | `number` | — | End column |


---

#### StructureItem

A structural item (function, class, struct, etc.) in source code.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `kind` | `StructureKind` | `StructureKind.Function` | Kind (structure kind) |
| `name` | `string | null` | `null` | The name |
| `visibility` | `string | null` | `null` | Visibility |
| `span` | `Span` | — | Span (span) |
| `children` | `Array<StructureItem>` | `[]` | Children |
| `decorators` | `Array<string>` | `[]` | Decorators |
| `docComment` | `string | null` | `null` | Doc comment |
| `signature` | `string | null` | `null` | Signature |
| `bodySpan` | `Span | null` | `null` | Body span (span) |


---

#### SymbolInfo

A symbol (variable, function, type, etc.) extracted from source code.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `name` | `string` | — | The name |
| `kind` | `SymbolKind` | `SymbolKind.Variable` | Kind (symbol kind) |
| `span` | `Span` | — | Span (span) |
| `typeAnnotation` | `string | null` | `null` | Type annotation |
| `doc` | `string | null` | `null` | Doc |


---

#### Tree


---

#### ValidationResult

Validation results for an entire extraction config.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `valid` | `boolean` | — | Whether all patterns are valid. |
| `patterns` | `string` | — | Per-pattern validation details. |


---

### Enums

#### CaptureOutput

Controls what data is captured for each query match.

| Value | Description |
|-------|-------------|
| `Text` | Capture only the matched text. |
| `Node` | Capture only the `NodeInfo`. |
| `Full` | Capture both text and `NodeInfo` (default). |


---

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

Errors are thrown as plain `Error` objects with descriptive messages.

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
