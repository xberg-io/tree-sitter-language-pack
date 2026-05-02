---
title: "WebAssembly API Reference"
---

## WebAssembly API Reference <span class="version-badge">v1.8.0-rc.26</span>

### Functions

#### detectLanguageFromExtension()

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

#### detectLanguageFromPath()

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

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `content` | `string` | Yes | The content to process |

**Returns:** `string | null`


---

#### parseString()

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

#### getHighlightsQuery()

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

#### getInjectionsQuery()

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

#### getLocalsQuery()

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

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `name` | `string` | Yes | The name |

**Returns:** `Language`

**Errors:** Throws `Error` with a descriptive message.


---

#### getParser()

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

#### detectLanguage()

Detect language name from a file path or extension.

This compatibility alias matches the pre-Alef Python binding API.

**Signature:**

```typescript
function detectLanguage(path: string): string | null
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

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `config` | `PackConfig` | Yes | The configuration options |

**Returns:** `void`

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

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `config` | `PackConfig` | Yes | The configuration options |

**Returns:** `void`

**Errors:** Throws `Error` with a descriptive message.


---

#### download()

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

#### downloadAll()

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

#### manifestLanguages()

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

#### downloadedLanguages()

Return languages that are already downloaded and cached locally.

Does not perform any network requests. Returns an empty list if the
cache directory does not exist or cannot be read.

**Signature:**

```typescript
function downloadedLanguages(): Array<string>
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

**Returns:** `void`

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

**Returns:** `string`

**Errors:** Throws `Error` with a descriptive message.


---

### Types

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

###### new()

Create a new download manager for the given version.

**Signature:**

```typescript
static new(version: string): DownloadManager
```

###### withCacheDir()

Create a download manager with a custom cache directory.

**Signature:**

```typescript
static withCacheDir(version: string, cacheDir: string): DownloadManager
```

###### defaultCacheDir()

Default cache directory: `~/.cache/tree-sitter-language-pack/v{version}/libs/`

**Signature:**

```typescript
static defaultCacheDir(version: string): string
```

###### cacheDir()

Return the path to the libs cache directory.

**Signature:**

```typescript
cacheDir(): string
```

###### installedLanguages()

List languages that are already downloaded and cached.

**Signature:**

```typescript
installedLanguages(): Array<string>
```

###### ensureLanguages()

Ensure the specified languages are available in the cache.
Downloads the platform bundle if any requested languages are missing.

**Signature:**

```typescript
ensureLanguages(names: Array<string>): void
```

###### ensureGroup()

Ensure all languages in a named group are available.

**Signature:**

```typescript
ensureGroup(group: string): void
```

###### libPath()

Get the expected path for a language's shared library in the cache.

**Signature:**

```typescript
libPath(name: string): string
```

###### fetchManifest()

Fetch the parser manifest from GitHub Releases.

**Signature:**

```typescript
fetchManifest(): ParserManifest
```

###### cleanCache()

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

###### withLibsDir()

Create a registry with a custom directory for dynamic libraries.

Overrides the default build-time library directory. Useful when
dynamic grammar shared libraries are stored in a non-standard location.

**Signature:**

```typescript
static withLibsDir(libsDir: string): LanguageRegistry
```

###### addExtraLibsDir()

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

###### availableLanguages()

List all available language names, sorted and deduplicated.

Includes statically compiled languages, dynamically loadable languages
(if the `dynamic-loading` feature is enabled), and all configured aliases.

**Signature:**

```typescript
availableLanguages(): Array<string>
```

###### hasLanguage()

Check whether a language is available by name or alias.

Returns `true` if the language can be loaded, either from the static
table or from a dynamic library on disk.

**Signature:**

```typescript
hasLanguage(name: string): boolean
```

###### languageCount()

Return the total number of available languages (including aliases).

**Signature:**

```typescript
languageCount(): number
```

###### process()

Parse source code and extract file intelligence based on config in a single pass.

**Signature:**

```typescript
process(source: string, config: ProcessConfig): ProcessResult
```

###### default()

**Signature:**

```typescript
static default(): LanguageRegistry
```


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

###### fromTomlFile()

Load configuration from a TOML file.

**Errors:**

Returns an error if the file cannot be read or the TOML is invalid.

**Signature:**

```typescript
static fromTomlFile(path: string): PackConfig
```

###### discover()

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

##### Methods

###### default()

**Signature:**

```typescript
static default(): ProcessConfig
```

###### withChunking()

Enable chunking with the given maximum chunk size in bytes.

**Signature:**

```typescript
withChunking(maxSize: number): ProcessConfig
```

###### all()

Enable all analysis features.

**Signature:**

```typescript
all(): ProcessConfig
```

###### minimal()

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
