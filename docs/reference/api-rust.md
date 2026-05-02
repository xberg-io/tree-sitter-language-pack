---
title: "Rust API Reference"
---

## Rust API Reference <span class="version-badge">v1.8.0-rc.26</span>

### Functions

#### detect_language_from_extension()

Detect language name from a file extension (without leading dot).

Returns `None` for unrecognized extensions. The match is case-insensitive.

**Signature:**

```rust
pub fn detect_language_from_extension(ext: &str) -> Option<String>
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `ext` | `String` | Yes | The ext |

**Returns:** `Option<String>`


---

#### detect_language_from_path()

Detect language name from a file path.

Extracts the file extension and looks it up. Returns `None` if the
path has no extension or the extension is not recognized.

**Signature:**

```rust
pub fn detect_language_from_path(path: &str) -> Option<String>
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `path` | `String` | Yes | Path to the file |

**Returns:** `Option<String>`


---

#### detect_language_from_content()

Detect language name from file content using the shebang line (`#!`).

Inspects only the first line of `content`. If it begins with `#!`, the
interpreter name is extracted and mapped to a language name.

Handles common patterns:

- `#!/usr/bin/env python3` → `"python"`
- `#!/bin/bash` → `"bash"`
- `#!/usr/bin/env node` → `"javascript"`

The `-S` flag accepted by some `env` implementations is skipped automatically.
Version suffixes (e.g. `python3.11`, `ruby3.2`) are stripped before matching.

Returns `None` when content does not start with `#!`, the shebang is
malformed, or the interpreter is not recognised.

**Signature:**

```rust
pub fn detect_language_from_content(content: &str) -> Option<String>
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `content` | `String` | Yes | The content to process |

**Returns:** `Option<String>`


---

#### parse_string()

Parse source code with the named language, returning the syntax tree.

Uses the global registry to look up the language by name.
Caches parsers per-thread so repeated calls for the same language avoid
re-creating the parser.

**Signature:**

```rust
pub fn parse_string(language: &str, source: &[u8]) -> Result<Tree, Error>
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `language` | `String` | Yes | The language |
| `source` | `Vec<u8>` | Yes | The source |

**Returns:** `Tree`

**Errors:** Returns `Err(Error)`.


---

#### get_highlights_query()

Get the highlights query for a language, if bundled.

Returns the contents of `highlights.scm` as a static string, or `None`
if no highlights query is bundled for this language.

**Signature:**

```rust
pub fn get_highlights_query(language: &str) -> Option<String>
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `language` | `String` | Yes | The language |

**Returns:** `Option<String>`


---

#### get_injections_query()

Get the injections query for a language, if bundled.

Returns the contents of `injections.scm` as a static string, or `None`
if no injections query is bundled for this language.

**Signature:**

```rust
pub fn get_injections_query(language: &str) -> Option<String>
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `language` | `String` | Yes | The language |

**Returns:** `Option<String>`


---

#### get_locals_query()

Get the locals query for a language, if bundled.

Returns the contents of `locals.scm` as a static string, or `None`
if no locals query is bundled for this language.

**Signature:**

```rust
pub fn get_locals_query(language: &str) -> Option<String>
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `language` | `String` | Yes | The language |

**Returns:** `Option<String>`


---

#### get_language()

Get a tree-sitter `Language` by name using the global registry.

Resolves language aliases (e.g., `"shell"` maps to `"bash"`).
When the `download` feature is enabled (default), automatically downloads
the parser from GitHub releases if not found locally.

**Errors:**

Returns `Error.LanguageNotFound` if the language is not recognized,
or `Error.Download` if auto-download fails.

**Signature:**

```rust
pub fn get_language(name: &str) -> Result<Language, Error>
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `name` | `String` | Yes | The name |

**Returns:** `Language`

**Errors:** Returns `Err(Error)`.


---

#### get_parser()

Get a tree-sitter `Parser` pre-configured for the given language.

This is a convenience function that calls `get_language` and configures
a new parser in one step.

**Errors:**

Returns `Error.LanguageNotFound` if the language is not recognized, or
`Error.ParserSetup` if the language cannot be applied to the parser.

**Signature:**

```rust
pub fn get_parser(name: &str) -> Result<Parser, Error>
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `name` | `String` | Yes | The name |

**Returns:** `Parser`

**Errors:** Returns `Err(Error)`.


---

#### detect_language()

Detect language name from a file path or extension.

This compatibility alias matches the pre-Alef Python binding API.

**Signature:**

```rust
pub fn detect_language(path: &str) -> Option<String>
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `path` | `String` | Yes | Path to the file |

**Returns:** `Option<String>`


---

#### available_languages()

List all available language names (sorted, deduplicated, includes aliases).

Returns names of both statically compiled and dynamically loadable languages,
plus any configured aliases.

**Signature:**

```rust
pub fn available_languages() -> Vec<String>
```

**Returns:** `Vec<String>`


---

#### has_language()

Check if a language is available by name or alias.

Returns `true` if the language can be loaded (statically compiled,
dynamically available, or a known alias for one of these).

**Signature:**

```rust
pub fn has_language(name: &str) -> bool
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `name` | `String` | Yes | The name |

**Returns:** `bool`


---

#### language_count()

Return the number of available languages.

Includes statically compiled languages, dynamically loadable languages,
and aliases.

**Signature:**

```rust
pub fn language_count() -> usize
```

**Returns:** `usize`


---

#### process()

Process source code and extract file intelligence using the global registry.

Parses the source with tree-sitter and extracts metrics, structure, imports,
exports, comments, docstrings, symbols, diagnostics, and/or chunks based on
the flags set in `ProcessConfig`.

**Errors:**

Returns an error if the language is not found or parsing fails.

**Signature:**

```rust
pub fn process(source: &str, config: ProcessConfig) -> Result<ProcessResult, Error>
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `source` | `String` | Yes | The source |
| `config` | `ProcessConfig` | Yes | The configuration options |

**Returns:** `ProcessResult`

**Errors:** Returns `Err(Error)`.


---

#### init()

Initialize the language pack with the given configuration.

Applies any custom cache directory, then downloads all languages and groups
specified in the config. This is the recommended entry point when you want
to pre-warm the cache before use.

**Errors:**

Returns an error if configuration cannot be applied or if downloads fail.

**Signature:**

```rust
pub fn init(config: PackConfig) -> Result<(), Error>
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `config` | `PackConfig` | Yes | The configuration options |

**Returns:** `()`

**Errors:** Returns `Err(Error)`.


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

```rust
pub fn configure(config: PackConfig) -> Result<(), Error>
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `config` | `PackConfig` | Yes | The configuration options |

**Returns:** `()`

**Errors:** Returns `Err(Error)`.


---

#### download()

Download specific languages to the local cache.

Returns the number of newly downloaded languages (languages that were
already cached are not counted).

**Errors:**

Returns an error if any language is not available in the manifest or if
the download fails.

**Signature:**

```rust
pub fn download(names: Vec<String>) -> Result<usize, Error>
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `names` | `Vec<String>` | Yes | The names |

**Returns:** `usize`

**Errors:** Returns `Err(Error)`.


---

#### download_all()

Download all available languages from the remote manifest.

Returns the number of newly downloaded languages.

**Errors:**

Returns an error if the manifest cannot be fetched or a download fails.

**Signature:**

```rust
pub fn download_all() -> Result<usize, Error>
```

**Returns:** `usize`

**Errors:** Returns `Err(Error)`.


---

#### manifest_languages()

Return all language names available in the remote manifest (305).

Fetches (and caches) the remote manifest to discover the full list of
downloadable languages. Use `downloaded_languages` to list what is
already cached locally.

**Errors:**

Returns an error if the manifest cannot be fetched.

**Signature:**

```rust
pub fn manifest_languages() -> Result<Vec<String>, Error>
```

**Returns:** `Vec<String>`

**Errors:** Returns `Err(Error)`.


---

#### downloaded_languages()

Return languages that are already downloaded and cached locally.

Does not perform any network requests. Returns an empty list if the
cache directory does not exist or cannot be read.

**Signature:**

```rust
pub fn downloaded_languages() -> Vec<String>
```

**Returns:** `Vec<String>`


---

#### clean_cache()

Delete all cached parser shared libraries.

Resets the cache registration so the next call to `get_language` or
a download function will re-register the (now empty) cache directory.

**Errors:**

Returns an error if the cache directory cannot be removed.

**Signature:**

```rust
pub fn clean_cache() -> Result<(), Error>
```

**Returns:** `()`

**Errors:** Returns `Err(Error)`.


---

#### cache_dir()

Return the effective cache directory path.

This is either the custom path set via `configure` / `init` or the
default: `~/.cache/tree-sitter-language-pack/v{version}/libs/`.

**Errors:**

Returns an error if the system cache directory cannot be determined.

**Signature:**

```rust
pub fn cache_dir() -> Result<PathBuf, Error>
```

**Returns:** `PathBuf`

**Errors:** Returns `Err(Error)`.


---

### Types

#### ChunkContext

Metadata for a single chunk of source code.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `language` | `String` | — | Language |
| `chunk_index` | `usize` | — | Chunk index |
| `total_chunks` | `usize` | — | Total chunks |
| `node_types` | `Vec<String>` | `vec![]` | Node types |
| `context_path` | `Vec<String>` | `vec![]` | Context path |
| `symbols_defined` | `Vec<String>` | `vec![]` | Symbols defined |
| `comments` | `Vec<CommentInfo>` | `vec![]` | Comments |
| `docstrings` | `Vec<DocstringInfo>` | `vec![]` | Docstrings |
| `has_error_nodes` | `bool` | — | Whether error nodes |


---

#### CodeChunk

A chunk of source code with rich metadata.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `content` | `String` | — | The extracted text content |
| `start_byte` | `usize` | — | Start byte |
| `end_byte` | `usize` | — | End byte |
| `start_line` | `usize` | — | Start line |
| `end_line` | `usize` | — | End line |
| `metadata` | `ChunkContext` | — | Document metadata |


---

#### CommentInfo

A comment extracted from source code.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `text` | `String` | — | Text |
| `kind` | `CommentKind` | `CommentKind::Line` | Kind (comment kind) |
| `span` | `Span` | — | Span (span) |
| `associated_node` | `Option<String>` | `Default::default()` | Associated node |


---

#### Diagnostic

A diagnostic (syntax error, missing node, etc.) from parsing.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `message` | `String` | — | Message |
| `severity` | `DiagnosticSeverity` | `DiagnosticSeverity::Error` | Severity (diagnostic severity) |
| `span` | `Span` | — | Span (span) |


---

#### DocSection

A section within a docstring (e.g., Args, Returns, Raises).

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `kind` | `String` | — | Kind |
| `name` | `Option<String>` | `Default::default()` | The name |
| `description` | `String` | — | Human-readable description |


---

#### DocstringInfo

A docstring extracted from source code.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `text` | `String` | — | Text |
| `format` | `DocstringFormat` | `DocstringFormat::PythonTripleQuote` | Format (docstring format) |
| `span` | `Span` | — | Span (span) |
| `associated_item` | `Option<String>` | `Default::default()` | Associated item |
| `parsed_sections` | `Vec<DocSection>` | `vec![]` | Parsed sections |


---

#### DownloadManager

Manages downloading and caching of pre-built parser shared libraries.

##### Methods

###### new()

Create a new download manager for the given version.

**Signature:**

```rust
pub fn new(version: &str) -> DownloadManager
```

###### with_cache_dir()

Create a download manager with a custom cache directory.

**Signature:**

```rust
pub fn with_cache_dir(version: &str, cache_dir: PathBuf) -> DownloadManager
```

###### default_cache_dir()

Default cache directory: `~/.cache/tree-sitter-language-pack/v{version}/libs/`

**Signature:**

```rust
pub fn default_cache_dir(version: &str) -> PathBuf
```

###### cache_dir()

Return the path to the libs cache directory.

**Signature:**

```rust
pub fn cache_dir(&self) -> PathBuf
```

###### installed_languages()

List languages that are already downloaded and cached.

**Signature:**

```rust
pub fn installed_languages(&self) -> Vec<String>
```

###### ensure_languages()

Ensure the specified languages are available in the cache.
Downloads the platform bundle if any requested languages are missing.

**Signature:**

```rust
pub fn ensure_languages(&self, names: Vec<String>)
```

###### ensure_group()

Ensure all languages in a named group are available.

**Signature:**

```rust
pub fn ensure_group(&self, group: &str)
```

###### lib_path()

Get the expected path for a language's shared library in the cache.

**Signature:**

```rust
pub fn lib_path(&self, name: &str) -> PathBuf
```

###### fetch_manifest()

Fetch the parser manifest from GitHub Releases.

**Signature:**

```rust
pub fn fetch_manifest(&self) -> ParserManifest
```

###### clean_cache()

Remove all cached parser libraries.

**Signature:**

```rust
pub fn clean_cache(&self)
```


---

#### ExportInfo

An export statement extracted from source code.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `name` | `String` | — | The name |
| `kind` | `ExportKind` | `ExportKind::Named` | Kind (export kind) |
| `span` | `Span` | — | Span (span) |


---

#### FileMetrics

Aggregate metrics for a source file.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `total_lines` | `usize` | — | Total lines |
| `code_lines` | `usize` | — | Code lines |
| `comment_lines` | `usize` | — | Comment lines |
| `blank_lines` | `usize` | — | Blank lines |
| `total_bytes` | `usize` | — | Total bytes |
| `node_count` | `usize` | — | Number of nodes |
| `error_count` | `usize` | — | Number of errors |
| `max_depth` | `usize` | — | Maximum depth |


---

#### ImportInfo

An import statement extracted from source code.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `source` | `String` | — | Source |
| `items` | `Vec<String>` | `vec![]` | Items |
| `alias` | `Option<String>` | `Default::default()` | Alias |
| `is_wildcard` | `bool` | — | Whether wildcard |
| `span` | `Span` | — | Span (span) |


---

#### Language


---

#### LanguageInfo

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `group` | `String` | — | Group |
| `size` | `u64` | — | Size in bytes |


---

#### LanguageRegistry

Thread-safe registry of tree-sitter language parsers.

Manages both statically compiled and dynamically loaded language grammars.
Use `LanguageRegistry.new()` for the default registry, or access the
global instance via the module-level convenience functions
(`crate.get_language`, `crate.available_languages`, etc.).

##### Methods

###### with_libs_dir()

Create a registry with a custom directory for dynamic libraries.

Overrides the default build-time library directory. Useful when
dynamic grammar shared libraries are stored in a non-standard location.

**Signature:**

```rust
pub fn with_libs_dir(libs_dir: PathBuf) -> LanguageRegistry
```

###### add_extra_libs_dir()

Add an additional directory to search for dynamic libraries.

When `get_language` cannot find a grammar in the
primary library directory, it searches these extra directories in order.
Typically used by the download system to register its cache directory.

Takes `&self` (not `&mut self`) because `extra_lib_dirs` uses interior
mutability via an `Arc<RwLock<...>>`, so the outer registry can remain
immutable while the directory list is updated.

**Signature:**

```rust
pub fn add_extra_libs_dir(&self, dir: PathBuf)
```

###### get_language()

Get a tree-sitter `Language` by name.

Resolves aliases (e.g., `"shell"` -> `"bash"`, `"makefile"` -> `"make"`),
then looks up the language in the static table. When the `dynamic-loading`
feature is enabled, falls back to loading a shared library on demand.

**Errors:**

Returns `Error.LanguageNotFound` if the name (after alias resolution)
does not match any known grammar.

**Signature:**

```rust
pub fn get_language(&self, name: &str) -> Language
```

###### available_languages()

List all available language names, sorted and deduplicated.

Includes statically compiled languages, dynamically loadable languages
(if the `dynamic-loading` feature is enabled), and all configured aliases.

**Signature:**

```rust
pub fn available_languages(&self) -> Vec<String>
```

###### has_language()

Check whether a language is available by name or alias.

Returns `true` if the language can be loaded, either from the static
table or from a dynamic library on disk.

**Signature:**

```rust
pub fn has_language(&self, name: &str) -> bool
```

###### language_count()

Return the total number of available languages (including aliases).

**Signature:**

```rust
pub fn language_count(&self) -> usize
```

###### process()

Parse source code and extract file intelligence based on config in a single pass.

**Signature:**

```rust
pub fn process(&self, source: &str, config: ProcessConfig) -> ProcessResult
```

###### default()

**Signature:**

```rust
pub fn default() -> LanguageRegistry
```


---

#### PackConfig

Configuration for the tree-sitter language pack.

Controls cache directory and which languages to pre-download.
Can be loaded from a TOML file, constructed programmatically,
or passed as a dict/object from language bindings.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `cache_dir` | `Option<PathBuf>` | `Default::default()` | Override default cache directory. Default: `~/.cache/tree-sitter-language-pack/v{version}/libs/` |
| `languages` | `Option<Vec<String>>` | `vec![]` | Languages to pre-download on init. Each entry is a language name (e.g. `"python"`, `"rust"`). |
| `groups` | `Option<Vec<String>>` | `vec![]` | Language groups to pre-download (e.g. `"web"`, `"systems"`, `"scripting"`). |

##### Methods

###### from_toml_file()

Load configuration from a TOML file.

**Errors:**

Returns an error if the file cannot be read or the TOML is invalid.

**Signature:**

```rust
pub fn from_toml_file(path: PathBuf) -> PackConfig
```

###### discover()

Discover configuration by searching for `language-pack.toml` in:

1. Current directory and up to 10 parent directories
2. `$XDG_CONFIG_HOME/tree-sitter-language-pack/config.toml`
3. `~/.config/tree-sitter-language-pack/config.toml`

Returns `None` if no configuration file is found.

**Signature:**

```rust
pub fn discover() -> Option<PackConfig>
```


---

#### Parser


---

#### ParserManifest

Manifest describing available parser downloads for a specific version.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `version` | `String` | — | Version string |
| `platforms` | `HashMap<String, PlatformBundle>` | — | Platforms |
| `languages` | `HashMap<String, LanguageInfo>` | — | Languages |
| `groups` | `HashMap<String, Vec<String>>` | — | Groups |


---

#### PlatformBundle

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `url` | `String` | — | Url |
| `sha256` | `String` | — | Sha256 |
| `size` | `u64` | — | Size in bytes |


---

#### ProcessConfig

Configuration for the `process()` function.

Controls which analysis features are enabled and whether chunking is performed.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `language` | `String` | — | Language name (required). |
| `structure` | `bool` | `true` | Extract structural items (functions, classes, etc.). Default: true. |
| `imports` | `bool` | `true` | Extract import statements. Default: true. |
| `exports` | `bool` | `true` | Extract export statements. Default: true. |
| `comments` | `bool` | `false` | Extract comments. Default: false. |
| `docstrings` | `bool` | `false` | Extract docstrings. Default: false. |
| `symbols` | `bool` | `false` | Extract symbol definitions. Default: false. |
| `diagnostics` | `bool` | `false` | Include parse diagnostics. Default: false. |
| `chunk_max_size` | `Option<usize>` | `None` | Maximum chunk size in bytes. `None` disables chunking. |

##### Methods

###### default()

**Signature:**

```rust
pub fn default() -> ProcessConfig
```

###### with_chunking()

Enable chunking with the given maximum chunk size in bytes.

**Signature:**

```rust
pub fn with_chunking(&self, max_size: usize) -> ProcessConfig
```

###### all()

Enable all analysis features.

**Signature:**

```rust
pub fn all(&self) -> ProcessConfig
```

###### minimal()

Disable all analysis features (only metrics computed).

**Signature:**

```rust
pub fn minimal(&self) -> ProcessConfig
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
| `structure` | `Vec<StructureItem>` | `vec![]` | Structure |
| `imports` | `Vec<ImportInfo>` | `vec![]` | Imports |
| `exports` | `Vec<ExportInfo>` | `vec![]` | Exports |
| `comments` | `Vec<CommentInfo>` | `vec![]` | Comments |
| `docstrings` | `Vec<DocstringInfo>` | `vec![]` | Docstrings |
| `symbols` | `Vec<SymbolInfo>` | `vec![]` | Symbols |
| `diagnostics` | `Vec<Diagnostic>` | `vec![]` | Diagnostics |
| `chunks` | `Vec<CodeChunk>` | `vec![]` | Text chunks for chunking/embedding |


---

#### Span

Byte and line/column range in source code.

Represents both byte offsets (for slicing) and human-readable line/column
positions (for display and diagnostics).

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `start_byte` | `usize` | — | Start byte |
| `end_byte` | `usize` | — | End byte |
| `start_line` | `usize` | — | Start line |
| `start_column` | `usize` | — | Start column |
| `end_line` | `usize` | — | End line |
| `end_column` | `usize` | — | End column |


---

#### StructureItem

A structural item (function, class, struct, etc.) in source code.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `kind` | `StructureKind` | `StructureKind::Function` | Kind (structure kind) |
| `name` | `Option<String>` | `Default::default()` | The name |
| `visibility` | `Option<String>` | `Default::default()` | Visibility |
| `span` | `Span` | — | Span (span) |
| `children` | `Vec<StructureItem>` | `vec![]` | Children |
| `decorators` | `Vec<String>` | `vec![]` | Decorators |
| `doc_comment` | `Option<String>` | `Default::default()` | Doc comment |
| `signature` | `Option<String>` | `Default::default()` | Signature |
| `body_span` | `Option<Span>` | `Default::default()` | Body span (span) |


---

#### SymbolInfo

A symbol (variable, function, type, etc.) extracted from source code.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `name` | `String` | — | The name |
| `kind` | `SymbolKind` | `SymbolKind::Variable` | Kind (symbol kind) |
| `span` | `Span` | — | Span (span) |
| `type_annotation` | `Option<String>` | `Default::default()` | Type annotation |
| `doc` | `Option<String>` | `Default::default()` | Doc |


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
| `Other` | Other — Fields: `0`: `String` |


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
| `Other` | Other — Fields: `0`: `String` |


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
| `Other` | Other — Fields: `0`: `String` |


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
