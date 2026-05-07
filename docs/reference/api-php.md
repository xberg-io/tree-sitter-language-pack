---
title: "PHP API Reference"
---

## PHP API Reference <span class="version-badge">v1.8.0-rc.35</span>

### Functions

#### detectLanguageFromExtension()

Detect language name from a file extension (without leading dot).

Returns `null` for unrecognized extensions. The match is case-insensitive.

**Signature:**

```php
public static function detectLanguageFromExtension(string $ext): ?string
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `ext` | `string` | Yes | The ext |

**Returns:** `?string`


---

#### detectLanguageFromPath()

Detect language name from a file path.

Extracts the file extension and looks it up. Returns `null` if the
path has no extension or the extension is not recognized.

**Signature:**

```php
public static function detectLanguageFromPath(string $path): ?string
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `path` | `string` | Yes | Path to the file |

**Returns:** `?string`


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

```php
public static function detectLanguageFromContent(string $content): ?string
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `content` | `string` | Yes | The content to process |

**Returns:** `?string`


---

#### getHighlightsQuery()

Get the highlights query for a language, if bundled.

Returns the contents of `highlights.scm` as a static string, or `null`
if no highlights query is bundled for this language.

**Signature:**

```php
public static function getHighlightsQuery(string $language): ?string
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `language` | `string` | Yes | The language |

**Returns:** `?string`


---

#### getInjectionsQuery()

Get the injections query for a language, if bundled.

Returns the contents of `injections.scm` as a static string, or `null`
if no injections query is bundled for this language.

**Signature:**

```php
public static function getInjectionsQuery(string $language): ?string
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `language` | `string` | Yes | The language |

**Returns:** `?string`


---

#### getLocalsQuery()

Get the locals query for a language, if bundled.

Returns the contents of `locals.scm` as a static string, or `null`
if no locals query is bundled for this language.

**Signature:**

```php
public static function getLocalsQuery(string $language): ?string
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `language` | `string` | Yes | The language |

**Returns:** `?string`


---

#### getLanguage()

Get a tree-sitter `Language` by name using the global registry.

Resolves language aliases (e.g., `"shell"` maps to `"bash"`).
When the `download` feature is enabled (default), automatically downloads
the parser from GitHub releases if not found locally.

**Errors:**

Returns `Error::LanguageNotFound` if the language is not recognized,
or `Error::Download` if auto-download fails.

**Signature:**

```php
public static function getLanguage(string $name): Language
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `name` | `string` | Yes | The name |

**Returns:** `Language`

**Errors:** Throws `Error`.


---

#### getParser()

Get a tree-sitter `Parser` pre-configured for the given language.

This is a convenience function that calls `get_language` and configures
a new parser in one step.

**Errors:**

Returns `Error::LanguageNotFound` if the language is not recognized, or
`Error::ParserSetup` if the language cannot be applied to the parser.

**Signature:**

```php
public static function getParser(string $name): Parser
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `name` | `string` | Yes | The name |

**Returns:** `Parser`

**Errors:** Throws `Error`.


---

#### detectLanguage()

Detect language name from a file path or extension.

This compatibility alias matches the pre-Alef Python binding API.

**Signature:**

```php
public static function detectLanguage(string $path): ?string
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `path` | `string` | Yes | Path to the file |

**Returns:** `?string`


---

#### availableLanguages()

List all available language names (sorted, deduplicated, includes aliases).

Returns names of both statically compiled and dynamically loadable languages,
plus any configured aliases.

**Signature:**

```php
public static function availableLanguages(): array<string>
```

**Returns:** `array<string>`


---

#### hasLanguage()

Check if a language is available by name or alias.

Returns `true` if the language can be loaded (statically compiled,
dynamically available, or a known alias for one of these).

**Signature:**

```php
public static function hasLanguage(string $name): bool
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `name` | `string` | Yes | The name |

**Returns:** `bool`


---

#### languageCount()

Return the number of available languages.

Includes statically compiled languages, dynamically loadable languages,
and aliases.

**Signature:**

```php
public static function languageCount(): int
```

**Returns:** `int`


---

#### process()

Process source code and extract file intelligence using the global registry.

Parses the source with tree-sitter and extracts metrics, structure, imports,
exports, comments, docstrings, symbols, diagnostics, and/or chunks based on
the flags set in `ProcessConfig`.

**Errors:**

Returns an error if the language is not found or parsing fails.

**Signature:**

```php
public static function process(string $source, ProcessConfig $config): ProcessResult
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `source` | `string` | Yes | The source |
| `config` | `ProcessConfig` | Yes | The configuration options |

**Returns:** `ProcessResult`

**Errors:** Throws `Error`.


---

#### init()

Initialize the language pack with the given configuration.

Applies any custom cache directory, then downloads all languages and groups
specified in the config. This is the recommended entry point when you want
to pre-warm the cache before use.

**Errors:**

Returns an error if configuration cannot be applied or if downloads fail.

**Signature:**

```php
public static function init(PackConfig $config): void
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `config` | `PackConfig` | Yes | The configuration options |

**Returns:** `void`

**Errors:** Throws `Error`.


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

```php
public static function configure(PackConfig $config): void
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `config` | `PackConfig` | Yes | The configuration options |

**Returns:** `void`

**Errors:** Throws `Error`.


---

#### download()

Download specific languages to the local cache.

Returns the number of requested languages available after the call. Already
compiled or cached languages are included in the count.

**Errors:**

Returns an error if any language is not available in the manifest or if
the download fails.

**Signature:**

```php
public static function download(array<string> $names): int
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `names` | `array<string>` | Yes | The names |

**Returns:** `int`

**Errors:** Throws `Error`.


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

```php
public static function downloadAll(): int
```

**Returns:** `int`

**Errors:** Throws `Error`.


---

#### manifestLanguages()

Return all language names available in the remote manifest (305).

Fetches (and caches) the remote manifest to discover the full list of
downloadable languages. Use `downloaded_languages` to list what is
already cached locally.

**Errors:**

Returns an error if the manifest cannot be fetched.

**Signature:**

```php
public static function manifestLanguages(): array<string>
```

**Returns:** `array<string>`

**Errors:** Throws `Error`.


---

#### downloadedLanguages()

Return languages that are already downloaded and cached locally.

Does not perform any network requests. Returns an empty list if the
cache directory does not exist or cannot be read.

**Signature:**

```php
public static function downloadedLanguages(): array<string>
```

**Returns:** `array<string>`


---

#### cleanCache()

Delete all cached parser shared libraries.

Resets the cache registration so the next call to `get_language` or
a download function will re-register the (now empty) cache directory.

**Errors:**

Returns an error if the cache directory cannot be removed.

**Signature:**

```php
public static function cleanCache(): void
```

**Returns:** `void`

**Errors:** Throws `Error`.


---

#### cacheDir()

Return the effective cache directory path.

This is either the custom path set via `configure` / `init` or the
default: `~/.cache/tree-sitter-language-pack/v{version}/libs/`.

**Errors:**

Returns an error if the system cache directory cannot be determined.

**Signature:**

```php
public static function cacheDir(): string
```

**Returns:** `string`

**Errors:** Throws `Error`.


---

### Types

#### ChunkContext

Metadata for a single chunk of source code.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `language` | `string` | — | Language |
| `chunkIndex` | `int` | — | Chunk index |
| `totalChunks` | `int` | — | Total chunks |
| `nodeTypes` | `array<string>` | `[]` | Node types |
| `contextPath` | `array<string>` | `[]` | Context path |
| `symbolsDefined` | `array<string>` | `[]` | Symbols defined |
| `comments` | `array<CommentInfo>` | `[]` | Comments |
| `docstrings` | `array<DocstringInfo>` | `[]` | Docstrings |
| `hasErrorNodes` | `bool` | — | Whether error nodes |


---

#### CodeChunk

A chunk of source code with rich metadata.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `content` | `string` | — | The extracted text content |
| `startByte` | `int` | — | Start byte |
| `endByte` | `int` | — | End byte |
| `startLine` | `int` | — | Start line |
| `endLine` | `int` | — | End line |
| `metadata` | `ChunkContext` | — | Document metadata |


---

#### CommentInfo

A comment extracted from source code.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `text` | `string` | — | Text |
| `kind` | `CommentKind` | `CommentKind::Line` | Kind (comment kind) |
| `span` | `Span` | — | Span (span) |
| `associatedNode` | `?string` | `null` | Associated node |


---

#### Diagnostic

A diagnostic (syntax error, missing node, etc.) from parsing.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `message` | `string` | — | Message |
| `severity` | `DiagnosticSeverity` | `DiagnosticSeverity::Error` | Severity (diagnostic severity) |
| `span` | `Span` | — | Span (span) |


---

#### DocSection

A section within a docstring (e.g., Args, Returns, Raises).

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `kind` | `string` | — | Kind |
| `name` | `?string` | `null` | The name |
| `description` | `string` | — | Human-readable description |


---

#### DocstringInfo

A docstring extracted from source code.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `text` | `string` | — | Text |
| `format` | `DocstringFormat` | `DocstringFormat::PythonTripleQuote` | Format (docstring format) |
| `span` | `Span` | — | Span (span) |
| `associatedItem` | `?string` | `null` | Associated item |
| `parsedSections` | `array<DocSection>` | `[]` | Parsed sections |


---

#### DownloadManager

Manages downloading and caching of pre-built parser shared libraries.

##### Methods

###### new()

Create a new download manager for the given version.

**Signature:**

```php
public static function new(string $version): DownloadManager
```

###### withCacheDir()

Create a download manager with a custom cache directory.

**Signature:**

```php
public static function withCacheDir(string $version, string $cacheDir): DownloadManager
```

###### defaultCacheDir()

Default cache directory: `~/.cache/tree-sitter-language-pack/v{version}/libs/`

**Signature:**

```php
public static function defaultCacheDir(string $version): string
```

###### cacheDir()

Return the path to the libs cache directory.

**Signature:**

```php
public function cacheDir(): string
```

###### installedLanguages()

List languages that are already downloaded and cached.

**Signature:**

```php
public function installedLanguages(): array<string>
```

###### ensureLanguages()

Ensure the specified languages are available in the cache.
Downloads the platform bundle if any requested languages are missing.

**Signature:**

```php
public function ensureLanguages(array<string> $names): void
```

###### ensureGroup()

Ensure all languages in a named group are available.

**Signature:**

```php
public function ensureGroup(string $group): void
```

###### libPath()

Get the expected path for a language's shared library in the cache.

**Signature:**

```php
public function libPath(string $name): string
```

###### fetchManifest()

Fetch the parser manifest from GitHub Releases.

**Signature:**

```php
public function fetchManifest(): ParserManifest
```

###### downloadAllBestEffort()

Download the platform bundle and extract every library file it contains.

Unlike `ensure_languages`, this does not check the manifest language list
against archive contents — it simply extracts all `.so`/`.dylib`/`.dll` files
from the bundle. Languages in the manifest that are missing from the archive
are silently ignored rather than returning an error.

Returns the number of library files extracted (including those already cached).

**Signature:**

```php
public function downloadAllBestEffort(): int
```

###### cleanCache()

Remove all cached parser libraries.

**Signature:**

```php
public function cleanCache(): void
```


---

#### ExportInfo

An export statement extracted from source code.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `name` | `string` | — | The name |
| `kind` | `ExportKind` | `ExportKind::Named` | Kind (export kind) |
| `span` | `Span` | — | Span (span) |


---

#### FileMetrics

Aggregate metrics for a source file.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `totalLines` | `int` | — | Total lines |
| `codeLines` | `int` | — | Code lines |
| `commentLines` | `int` | — | Comment lines |
| `blankLines` | `int` | — | Blank lines |
| `totalBytes` | `int` | — | Total bytes |
| `nodeCount` | `int` | — | Number of nodes |
| `errorCount` | `int` | — | Number of errors |
| `maxDepth` | `int` | — | Maximum depth |


---

#### ImportInfo

An import statement extracted from source code.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `source` | `string` | — | Source |
| `items` | `array<string>` | `[]` | Items |
| `alias` | `?string` | `null` | Alias |
| `isWildcard` | `bool` | — | Whether wildcard |
| `span` | `Span` | — | Span (span) |


---

#### Language


---

#### LanguageInfo

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `group` | `string` | — | Group |
| `size` | `int` | — | Size in bytes |


---

#### LanguageRegistry

Thread-safe registry of tree-sitter language parsers.

Manages both statically compiled and dynamically loaded language grammars.
Use `LanguageRegistry::new()` for the default registry, or access the
global instance via the module-level convenience functions
(`get_language`, `available_languages`, etc.).

##### Methods

###### withLibsDir()

Create a registry with a custom directory for dynamic libraries.

Overrides the default build-time library directory. Useful when
dynamic grammar shared libraries are stored in a non-standard location.

**Signature:**

```php
public static function withLibsDir(string $libsDir): LanguageRegistry
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

```php
public function addExtraLibsDir(string $dir): void
```

###### getLanguage()

Get a tree-sitter `Language` by name.

Resolves aliases (e.g., `"shell"` -> `"bash"`, `"makefile"` -> `"make"`),
then looks up the language in the static table. When the `dynamic-loading`
feature is enabled, falls back to loading a shared library on demand.

**Errors:**

Returns `Error::LanguageNotFound` if the name (after alias resolution)
does not match any known grammar.

**Signature:**

```php
public function getLanguage(string $name): Language
```

###### availableLanguages()

List all available language names, sorted and deduplicated.

Includes statically compiled languages, dynamically loadable languages
(if the `dynamic-loading` feature is enabled), and all configured aliases.

**Signature:**

```php
public function availableLanguages(): array<string>
```

###### hasLanguage()

Check whether a language is available by name or alias.

Returns `true` if the language can be loaded, either from the static
table or from a dynamic library on disk.

**Signature:**

```php
public function hasLanguage(string $name): bool
```

###### languageCount()

Return the total number of available languages (including aliases).

**Signature:**

```php
public function languageCount(): int
```

###### process()

Parse source code and extract file intelligence based on config in a single pass.

**Signature:**

```php
public function process(string $source, ProcessConfig $config): ProcessResult
```

###### default()

**Signature:**

```php
public static function default(): LanguageRegistry
```


---

#### PackConfig

Configuration for the tree-sitter language pack.

Controls cache directory and which languages to pre-download.
Can be loaded from a TOML file, constructed programmatically,
or passed as a dict/object from language bindings.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `cacheDir` | `?string` | `null` | Override default cache directory. Default: `~/.cache/tree-sitter-language-pack/v{version}/libs/` |
| `languages` | `?array<string>` | `[]` | Languages to pre-download on init. Each entry is a language name (e.g. `"python"`, `"rust"`). |
| `groups` | `?array<string>` | `[]` | Language groups to pre-download (e.g. `"web"`, `"systems"`, `"scripting"`). |

##### Methods

###### fromTomlFile()

Load configuration from a TOML file.

**Errors:**

Returns an error if the file cannot be read or the TOML is invalid.

**Signature:**

```php
public static function fromTomlFile(string $path): PackConfig
```

###### discover()

Discover configuration by searching for `language-pack.toml` in:

1. Current directory and up to 10 parent directories
2. `$XDG_CONFIG_HOME/tree-sitter-language-pack/config.toml`
3. `~/.config/tree-sitter-language-pack/config.toml`

Returns `null` if no configuration file is found.

**Signature:**

```php
public static function discover(): ?PackConfig
```


---

#### Parser


---

#### ParserManifest

Manifest describing available parser downloads for a specific version.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `version` | `string` | — | Version string |
| `platforms` | `array<string, PlatformBundle>` | — | Platforms |
| `languages` | `array<string, LanguageInfo>` | — | Languages |
| `groups` | `array<string, array<string>>` | — | Groups |


---

#### PlatformBundle

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `url` | `string` | — | Url |
| `sha256` | `string` | — | Sha256 |
| `size` | `int` | — | Size in bytes |


---

#### ProcessConfig

Configuration for the `process()` function.

Controls which analysis features are enabled and whether chunking is performed.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `language` | `string` | — | Language name (required). |
| `structure` | `bool` | `true` | Extract structural items (functions, classes, etc.). Default: true. |
| `imports` | `bool` | `true` | Extract import statements. Default: true. |
| `exports` | `bool` | `true` | Extract export statements. Default: true. |
| `comments` | `bool` | `false` | Extract comments. Default: false. |
| `docstrings` | `bool` | `false` | Extract docstrings. Default: false. |
| `symbols` | `bool` | `false` | Extract symbol definitions. Default: false. |
| `diagnostics` | `bool` | `false` | Include parse diagnostics. Default: false. |
| `chunkMaxSize` | `?int` | `null` | Maximum chunk size in bytes. `null` disables chunking. |

##### Methods

###### default()

**Signature:**

```php
public static function default(): ProcessConfig
```

###### withChunking()

Enable chunking with the given maximum chunk size in bytes.

**Signature:**

```php
public function withChunking(int $maxSize): ProcessConfig
```

###### all()

Enable all analysis features.

**Signature:**

```php
public function all(): ProcessConfig
```

###### minimal()

Disable all analysis features (only metrics computed).

**Signature:**

```php
public function minimal(): ProcessConfig
```


---

#### ProcessResult

Complete analysis result from processing a source file.

Contains metrics, structural analysis, imports/exports, comments,
docstrings, symbols, diagnostics, and optionally chunked code segments.
Fields are populated based on the `ProcessConfig` flags.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `language` | `string` | — | Language |
| `metrics` | `FileMetrics` | — | Metrics (file metrics) |
| `structure` | `array<StructureItem>` | `[]` | Structure |
| `imports` | `array<ImportInfo>` | `[]` | Imports |
| `exports` | `array<ExportInfo>` | `[]` | Exports |
| `comments` | `array<CommentInfo>` | `[]` | Comments |
| `docstrings` | `array<DocstringInfo>` | `[]` | Docstrings |
| `symbols` | `array<SymbolInfo>` | `[]` | Symbols |
| `diagnostics` | `array<Diagnostic>` | `[]` | Diagnostics |
| `chunks` | `array<CodeChunk>` | `[]` | Text chunks for chunking/embedding |


---

#### Span

Byte and line/column range in source code.

Represents both byte offsets (for slicing) and human-readable line/column
positions (for display and diagnostics).

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `startByte` | `int` | — | Start byte |
| `endByte` | `int` | — | End byte |
| `startLine` | `int` | — | Start line |
| `startColumn` | `int` | — | Start column |
| `endLine` | `int` | — | End line |
| `endColumn` | `int` | — | End column |


---

#### StructureItem

A structural item (function, class, struct, etc.) in source code.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `kind` | `StructureKind` | `StructureKind::Function` | Kind (structure kind) |
| `name` | `?string` | `null` | The name |
| `visibility` | `?string` | `null` | Visibility |
| `span` | `Span` | — | Span (span) |
| `children` | `array<StructureItem>` | `[]` | Children |
| `decorators` | `array<string>` | `[]` | Decorators |
| `docComment` | `?string` | `null` | Doc comment |
| `signature` | `?string` | `null` | Signature |
| `bodySpan` | `?Span` | `null` | Body span (span) |


---

#### SymbolInfo

A symbol (variable, function, type, etc.) extracted from source code.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `name` | `string` | — | The name |
| `kind` | `SymbolKind` | `SymbolKind::Variable` | Kind (symbol kind) |
| `span` | `Span` | — | Span (span) |
| `typeAnnotation` | `?string` | `null` | Type annotation |
| `doc` | `?string` | `null` | Doc |


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
