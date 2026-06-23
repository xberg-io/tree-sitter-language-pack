---
title: "Rust API Reference"
---

## Rust API Reference <span class="version-badge">v1.10.5</span>

### Functions

#### detect_language_from_extension()

Detect language name from a file extension (without leading dot).

Returns `None` for unrecognized extensions. The match is case-insensitive.

**Signature:**

```rust
pub fn detect_language_from_extension(ext: &str) -> Option<String>
```

**Example:**

```rust
let result = detect_language_from_extension("value");
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

**Example:**

```rust
let result = detect_language_from_path("value");
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

**Example:**

```rust
let result = detect_language_from_content("value");
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `content` | `String` | Yes | The content to process |

**Returns:** `Option<String>`

---

#### get_highlights_query()

Get the highlights query for a language, if bundled.

Returns the contents of `highlights.scm` as a static string, or `None`
if no highlights query is bundled for this language.

**Signature:**

```rust
pub fn get_highlights_query(language: &str) -> Option<String>
```

**Example:**

```rust
use tree_sitter_language_pack::get_highlights_query;

// Returns Some(...) for languages with bundled queries
let query = get_highlights_query("python");
// Returns None for languages without bundled highlights queries
let missing = get_highlights_query("nonexistent_lang");
assert!(missing.is_none());
```rust

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

**Example:**

```rust
use tree_sitter_language_pack::get_injections_query;

let query = get_injections_query("markdown");
// Returns None for languages without bundled injections queries
let missing = get_injections_query("nonexistent_lang");
assert!(missing.is_none());
```rust

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

**Example:**

```rust
use tree_sitter_language_pack::get_locals_query;

let query = get_locals_query("python");
// Returns None for languages without bundled locals queries
let missing = get_locals_query("nonexistent_lang");
assert!(missing.is_none());
```rust

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `language` | `String` | Yes | The language |

**Returns:** `Option<String>`

---

#### get_tags_query()

Get the tags query for a language, if bundled.

Returns the contents of `tags.scm` as a static string, or `None`
if no tags query is bundled for this language.

**Signature:**

```rust
pub fn get_tags_query(language: &str) -> Option<String>
```

**Example:**

```rust
use tree_sitter_language_pack::get_tags_query;

let query = get_tags_query("rust");
// Returns None for languages without bundled tags queries
let missing = get_tags_query("nonexistent_lang");
assert!(missing.is_none());
```rust

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

Returns `Error::LanguageNotFound` if the language is not recognized,
or `Error::Download` if auto-download fails.

**Signature:**

```rust
pub fn get_language(name: &str) -> Result<Language, Error>
```

**Example:**

```rust
use tree_sitter_language_pack::{get_language, Parser};

let _lang = get_language("python")?;
let mut parser = Parser::new();
parser.set_language("python")?;
let tree = parser.parse("x = 1").expect("parse failed");
assert_eq!(tree.root_node().kind(), "module");
# Ok::<(), tree_sitter_language_pack::Error>(())
```rust

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `name` | `String` | Yes | The name |

**Returns:** `Language`

**Errors:** Returns `Err(Error)`.

---

#### get_parser()

Get a `Parser` pre-configured for the given language.

This is a convenience function that calls `get_language` and configures
a new parser in one step.

**Errors:**

Returns `Error::LanguageNotFound` if the language is not recognized, or
`Error::ParserSetup` if the language cannot be applied to the parser.

**Signature:**

```rust
pub fn get_parser(name: &str) -> Result<Parser, Error>
```

**Example:**

```rust
use tree_sitter_language_pack::get_parser;

let mut parser = get_parser("rust")?;
let tree = parser.parse("fn main() {}").expect("parse failed");
assert!(!tree.root_node().has_error());
# Ok::<(), tree_sitter_language_pack::Error>(())
```rust

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

**Example:**

```rust
let result = detect_language("value");
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

**Example:**

```rust
use tree_sitter_language_pack::available_languages;

let langs = available_languages();
for name in &langs {
    println!("{}", name);
}
```rust

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

**Example:**

```rust
use tree_sitter_language_pack::has_language;

assert!(has_language("python"));
assert!(has_language("shell")); // alias for "bash"
assert!(!has_language("nonexistent_language"));
```rust

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

**Example:**

```rust
use tree_sitter_language_pack::language_count;

let count = language_count();
println!("{} languages available", count);
```rust

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

**Example:**

```rust
use tree_sitter_language_pack::{ProcessConfig, process};

let config = ProcessConfig::new("python").all();
let result = process("def hello(): pass", &config).unwrap();
println!("Language: {}", result.language);
println!("Lines: {}", result.metrics.total_lines);
println!("Structures: {}", result.structure.len());
```rust

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

**Example:**

```rust
use tree_sitter_language_pack::{PackConfig, init};

let config = PackConfig {
    cache_dir: None,
    languages: Some(vec!["python".to_string(), "rust".to_string()]),
    groups: None,
};
init(&config).unwrap();
```rust

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `config` | `PackConfig` | Yes | The configuration options |

**Returns:** No return value.

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

**Example:**

```rust
use std::path::PathBuf;
use tree_sitter_language_pack::{PackConfig, configure};

let config = PackConfig {
    cache_dir: Some(PathBuf::from("/tmp/my-parsers")),
    languages: None,
    groups: None,
};
configure(&config).unwrap();
```rust

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `config` | `PackConfig` | Yes | The configuration options |

**Returns:** No return value.

**Errors:** Returns `Err(Error)`.

---

#### download()

Download specific languages to the local cache.

Returns the number of requested languages available after the call. Already
compiled or cached languages are included in the count.

**Errors:**

Returns an error if any language is not available in the manifest or if
the download fails.

**Signature:**

```rust
pub fn download(names: Vec<String>) -> Result<usize, Error>
```

**Example:**

```rust
use tree_sitter_language_pack::download;

let count = download(&["python", "rust", "typescript"]).unwrap();
println!("Ensured {} languages", count);
```rust

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `names` | `Vec<String>` | Yes | The names |

**Returns:** `usize`

**Errors:** Returns `Err(Error)`.

---

#### download_all()

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

```rust
pub fn download_all() -> Result<usize, Error>
```

**Example:**

```rust
use tree_sitter_language_pack::download_all;

let count = download_all().unwrap();
println!("{} languages available", count);
```rust

**Returns:** `usize`

**Errors:** Returns `Err(Error)`.

---

#### download_group()

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

```rust
pub fn download_group(name: &str) -> Result<usize, Error>
```

**Example:**

```rust
use tree_sitter_language_pack::download_group;

let count = download_group("web").unwrap();
println!("{} languages available", count);
```rust

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `name` | `String` | Yes | The name |

**Returns:** `usize`

**Errors:** Returns `Err(Error)`.

---

#### manifest_languages()

Return all language names available in the remote manifest (306).

Fetches (and caches) the remote manifest to discover the full list of
downloadable languages. Use `downloaded_languages` to list what is
already cached locally.

**Errors:**

Returns an error if the manifest cannot be fetched.

**Signature:**

```rust
pub fn manifest_languages() -> Result<Vec<String>, Error>
```

**Example:**

```rust
use tree_sitter_language_pack::manifest_languages;

let langs = manifest_languages().unwrap();
println!("{} languages available for download", langs.len());
```rust

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

**Example:**

```rust
use tree_sitter_language_pack::downloaded_languages;

let langs = downloaded_languages();
println!("{} languages already cached", langs.len());
```rust

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

**Example:**

```rust
use tree_sitter_language_pack::clean_cache;

clean_cache().unwrap();
println!("Cache cleared");
```rust

**Returns:** No return value.

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
pub fn cache_dir() -> Result<String, Error>
```

**Example:**

```rust
use tree_sitter_language_pack::cache_dir;

let dir = cache_dir().unwrap();
println!("Cache directory: {dir}");
```rust

**Returns:** `String`

**Errors:** Returns `Err(Error)`.

---

### Types

#### ByteRange

A byte range — start (inclusive) to end (exclusive).

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `start` | `usize` | — | Inclusive start byte offset. |
| `end` | `usize` | — | Exclusive end byte offset. |

---

#### ChunkContext

Metadata for a single chunk of source code.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `language` | `String` | — | Language name used to parse this chunk. |
| `chunk_index` | `usize` | — | Zero-indexed position of this chunk within the file's chunk list. |
| `total_chunks` | `usize` | — | Total number of chunks the file was split into. |
| `node_types` | `Vec<String>` | `vec!\[\]` | Tree-sitter node kinds that appear at the top level of this chunk. |
| `context_path` | `Vec<String>` | `vec!\[\]` | Hierarchical path of enclosing structural items (e.g., `\["MyClass", "my_method"\]`). |
| `symbols_defined` | `Vec<String>` | `vec!\[\]` | Names of symbols defined within this chunk. |
| `comments` | `Vec<CommentInfo>` | `vec!\[\]` | Comments contained within this chunk. |
| `docstrings` | `Vec<DocstringInfo>` | `vec!\[\]` | Docstrings contained within this chunk. |
| `has_error_nodes` | `bool` | — | Whether this chunk contains any tree-sitter error nodes. |

---

#### CodeChunk

A chunk of source code with rich metadata.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `content` | `String` | — | The raw source text of this chunk. |
| `start_byte` | `usize` | — | Inclusive start byte offset of this chunk in the original source. |
| `end_byte` | `usize` | — | Exclusive end byte offset of this chunk in the original source. |
| `start_line` | `usize` | — | Zero-indexed start line of this chunk. |
| `end_line` | `usize` | — | Zero-indexed end line of this chunk. |
| `metadata` | `ChunkContext` | — | Contextual metadata about this chunk. |

---

#### CommentInfo

A comment extracted from source code.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `text` | `String` | — | The raw text content of the comment. |
| `kind` | `CommentKind` | `CommentKind::Line` | The kind of comment (line, block, or doc). |
| `span` | `Span` | — | Source span covering the comment. |
| `associated_node` | `Option<String>` | `Default::default()` | Name of the syntax node this comment is directly associated with. |

---

#### DataAttribute

An XML-style attribute attached to an `Element` node.

Populated only for `DataNodeKind::Element`; always empty for `KeyValue` and
`Sequence` nodes.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `name` | `String` | — | Attribute name (e.g. `"class"`, `"href"`). |
| `value` | `String` | — | Attribute value as a raw string (quotes stripped). |
| `span` | `Span` | — | Source span covering the entire `name="value"` attribute token. |

---

#### DataNode

A node in the hierarchical data tree produced by data-format extraction.

When `ProcessConfig::data_extraction` is
`true`, `ProcessResult::data` is populated with a root `DataNode` whose
`children` mirror the structure of the parsed file.

The `kind` field determines which other fields are meaningful:

| `kind`     | `key`                    | `value`       | `attributes` | `children` |
|------------|--------------------------|---------------|--------------|------------|
| `KeyValue` | key / mapping key / index | leaf value   | empty        | nested map |
| `Element`  | XML tag name             | text content  | XML attrs    | child elements |
| `Sequence` | positional index (`"0"`) | leaf value   | empty        | sub-items  |

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `kind` | `DataNodeKind` | `DataNodeKind::KeyValue` | Whether this node is a key/value pair, XML element, or sequence item. |
| `key` | `Option<String>` | `Default::default()` | Key, attribute name, tag name, or positional index (`"0"`, `"1"`, …). `None` at the document root. |
| `value` | `Option<String>` | `Default::default()` | Leaf scalar value, if any. `None` for containers (objects, arrays, XML elements with child elements). |
| `attributes` | `Vec<DataAttribute>` | `vec!\[\]` | Attributes on element-shape nodes (XML `STag` attributes). Empty for all other kinds. |
| `children` | `Vec<DataNode>` | `vec!\[\]` | Children for nested containers and XML element bodies. |
| `span` | `Span` | — | Source span covering this node in the original source file. |

---

#### Diagnostic

A diagnostic (syntax error, missing node, etc.) from parsing.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `message` | `String` | — | Human-readable description of the diagnostic. |
| `severity` | `DiagnosticSeverity` | `DiagnosticSeverity::Error` | Severity of the diagnostic. |
| `span` | `Span` | — | Source span where the diagnostic was detected. |

---

#### DocSection

A section within a docstring (e.g., Args, Returns, Raises).

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `kind` | `String` | — | Section kind (e.g., `"args"`, `"returns"`, `"raises"`). |
| `name` | `Option<String>` | `Default::default()` | Parameter or return value name, if applicable. |
| `description` | `String` | — | Description text for this section. |

---

#### DocstringInfo

A docstring extracted from source code.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `text` | `String` | — | The raw text of the docstring. |
| `format` | `DocstringFormat` | `DocstringFormat::PythonTripleQuote` | The docstring format (Python, JSDoc, Rustdoc, etc.). |
| `span` | `Span` | — | Source span covering the docstring. |
| `associated_item` | `Option<String>` | `Default::default()` | Name of the item this docstring documents. |
| `parsed_sections` | `Vec<DocSection>` | `vec!\[\]` | Parsed sections of the docstring (Args, Returns, Raises, etc.). |

---

#### DownloadManager

Manages downloading and caching of pre-built parser shared libraries.

##### Methods

###### new()

Create a new download manager for the given version.

**Signature:**

```rust
pub fn new(version: &str) -> Result<DownloadManager, Error>
```

**Example:**

```rust
let result = DownloadManager::new("value")?;
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `version` | `String` | Yes | The version |

**Returns:** `DownloadManager`

**Errors:** Returns `Err(Error)`.

###### installed_languages()

List languages that are already downloaded and cached.

**Signature:**

```rust
pub fn installed_languages(&self) -> Vec<String>
```

**Example:**

```rust
let result = instance.installed_languages();
```

**Returns:** `Vec<String>`

###### download_all_best_effort()

Download the platform bundle and extract every library file it contains.

Unlike `Self::ensure_languages`, this does not check the manifest language list
against archive contents — it simply extracts all `.so`/`.dylib`/`.dll` files
from the bundle. Languages in the manifest that are missing from the archive
are silently ignored rather than returning an error.

Returns the number of library files extracted (including those already cached).

**Signature:**

```rust
pub fn download_all_best_effort(&self) -> Result<usize, Error>
```

**Example:**

```rust
let result = instance.download_all_best_effort()?;
```

**Returns:** `usize`

**Errors:** Returns `Err(Error)`.

###### clean_cache()

Remove all cached parser libraries.

Acquires the cross-process lock so `clean_cache` cannot race a concurrent
downloader (avoids Windows sharing-violation errors against an in-flight
bundle write). The `.download.lock` file itself is **not** removed — it is
permanent infrastructure; deleting it could allow a concurrent process that
already opened the file to continue holding a stale lock handle while a new
process opens a fresh inode, breaking the mutual-exclusion guarantee.

**Signature:**

```rust
pub fn clean_cache(&self) -> Result<(), Error>
```

**Example:**

```rust
instance.clean_cache()?;
```

**Returns:** No return value.

**Errors:** Returns `Err(Error)`.

---

#### ExportInfo

An export statement extracted from source code.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `name` | `String` | — | The exported name. |
| `kind` | `ExportKind` | `ExportKind::Named` | The kind of export (named, default, or re-export). |
| `span` | `Span` | — | Source span covering the export statement. |

---

#### FileMetrics

Aggregate metrics for a source file.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `total_lines` | `usize` | — | Total number of lines (including blank and comment lines). |
| `code_lines` | `usize` | — | Number of lines containing non-blank, non-comment source code. |
| `comment_lines` | `usize` | — | Number of lines that are entirely comments. |
| `blank_lines` | `usize` | — | Number of blank (whitespace-only) lines. |
| `total_bytes` | `usize` | — | Total byte length of the source file. |
| `node_count` | `usize` | — | Total number of nodes in the syntax tree. |
| `error_count` | `usize` | — | Number of error nodes in the syntax tree (parse errors). |
| `max_depth` | `usize` | — | Maximum nesting depth reached in the syntax tree. |

---

#### ImportInfo

An import statement extracted from source code.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `source` | `String` | — | The module or path being imported from. |
| `items` | `Vec<String>` | `vec!\[\]` | Specific names imported from the source module. |
| `alias` | `Option<String>` | `Default::default()` | Alias assigned to the import (e.g., `import numpy as np`). |
| `is_wildcard` | `bool` | — | Whether this is a wildcard import (e.g., `import *` or `use foo::*`). |
| `span` | `Span` | — | Source span covering the import statement. |

---

#### Language

---

#### LanguageRegistry

Thread-safe registry of tree-sitter language parsers.

Manages both statically compiled and dynamically loaded language grammars.
Use `LanguageRegistry::new()` for the default registry, or access the
global instance via the module-level convenience functions
(`get_language`, `available_languages`, etc.).

##### Methods

###### new()

Create a new registry populated with all statically compiled languages.

When the `dynamic-loading` feature is enabled, the registry also knows
about dynamically loadable grammars and will load them on demand.

**Signature:**

```rust
pub fn new() -> LanguageRegistry
```

**Example:**

```rust
let result = LanguageRegistry::new();
```

**Returns:** `LanguageRegistry`

###### get_language()

Get a tree-sitter `Language` by name.

Resolves aliases (e.g., `"shell"` -> `"bash"`, `"makefile"` -> `"make"`),
then looks up the language in the static table. When the `dynamic-loading`
feature is enabled, falls back to loading a shared library on demand.

**Errors:**

Returns `Error::LanguageNotFound` if the name (after alias resolution)
does not match any known grammar.

**Signature:**

```rust
pub fn get_language(&self, name: &str) -> Result<Language, Error>
```

**Example:**

```rust
let result = instance.get_language("value")?;
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `name` | `String` | Yes | The name |

**Returns:** `Language`

**Errors:** Returns `Err(Error)`.

###### available_languages()

List all available language names, sorted and deduplicated.

Includes statically compiled languages, dynamically loadable languages
(if the `dynamic-loading` feature is enabled), and all configured aliases.

**Signature:**

```rust
pub fn available_languages(&self) -> Vec<String>
```

**Example:**

```rust
let result = instance.available_languages();
```

**Returns:** `Vec<String>`

###### has_parser()

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

```rust
pub fn has_parser(&self, name: &str) -> bool
```

**Example:**

```rust
let result = instance.has_parser("value");
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `name` | `String` | Yes | The name |

**Returns:** `bool`

###### has_language()

Check whether a language is available by name or alias.

Returns `true` if the language can be loaded, either from the static
table or from a dynamic library on disk.

**Signature:**

```rust
pub fn has_language(&self, name: &str) -> bool
```

**Example:**

```rust
let result = instance.has_language("value");
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `name` | `String` | Yes | The name |

**Returns:** `bool`

###### language_count()

Return the total number of available languages (including aliases).

**Signature:**

```rust
pub fn language_count(&self) -> usize
```

**Example:**

```rust
let result = instance.language_count();
```

**Returns:** `usize`

###### process()

Parse source code and extract file intelligence based on config in a single pass.

**Signature:**

```rust
pub fn process(&self, source: &str, config: ProcessConfig) -> Result<ProcessResult, Error>
```

**Example:**

```rust
let result = instance.process("value", ProcessConfig::default())?;
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `source` | `String` | Yes | The source |
| `config` | `ProcessConfig` | Yes | The configuration options |

**Returns:** `ProcessResult`

**Errors:** Returns `Err(Error)`.

###### default()

**Signature:**

```rust
pub fn default() -> LanguageRegistry
```

**Example:**

```rust
let result = LanguageRegistry::default();
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

```rust
pub fn clone(&self) -> Node
```

**Example:**

```rust
let result = instance.clone();
```

**Returns:** `Node`

###### kind()

Return the node's kind name (e.g. `"function_definition"`).

**Signature:**

```rust
pub fn kind(&self) -> String
```

**Example:**

```rust
let result = instance.kind();
```

**Returns:** `String`

###### kind_id()

Return the node's numeric kind ID.

Tree-sitter assigns a stable `u16` ID to every node kind in a grammar
(e.g. `"function_definition" → 42`). Comparing `kind_id()` is cheaper
than comparing the string `kind()` in tight AST loops.

**Signature:**

```rust
pub fn kind_id(&self) -> u16
```

**Example:**

```rust
let result = instance.kind_id();
```

**Returns:** `u16`

###### start_byte()

Return the inclusive start byte offset of this node.

**Signature:**

```rust
pub fn start_byte(&self) -> usize
```

**Example:**

```rust
let result = instance.start_byte();
```

**Returns:** `usize`

###### end_byte()

Return the exclusive end byte offset of this node.

**Signature:**

```rust
pub fn end_byte(&self) -> usize
```

**Example:**

```rust
let result = instance.end_byte();
```

**Returns:** `usize`

###### byte_range()

Return the node's byte range as a `ByteRange`.

Callers should slice their own source bytes — this is a zero-copy
text accessor.

**Signature:**

```rust
pub fn byte_range(&self) -> ByteRange
```

**Example:**

```rust
let result = instance.byte_range();
```

**Returns:** `ByteRange`

###### start_position()

Return the start `Point` (row, column).

**Signature:**

```rust
pub fn start_position(&self) -> Point
```

**Example:**

```rust
let result = instance.start_position();
```

**Returns:** `Point`

###### end_position()

Return the end `Point` (row, column).

**Signature:**

```rust
pub fn end_position(&self) -> Point
```

**Example:**

```rust
let result = instance.end_position();
```

**Returns:** `Point`

###### is_named()

True when this node is named (not punctuation/whitespace).

**Signature:**

```rust
pub fn is_named(&self) -> bool
```

**Example:**

```rust
let result = instance.is_named();
```

**Returns:** `bool`

###### is_error()

True when this is an error node.

**Signature:**

```rust
pub fn is_error(&self) -> bool
```

**Example:**

```rust
let result = instance.is_error();
```

**Returns:** `bool`

###### is_missing()

True when this is a missing-token node.

**Signature:**

```rust
pub fn is_missing(&self) -> bool
```

**Example:**

```rust
let result = instance.is_missing();
```

**Returns:** `bool`

###### is_extra()

True when this is an "extra" node (e.g. a comment).

**Signature:**

```rust
pub fn is_extra(&self) -> bool
```

**Example:**

```rust
let result = instance.is_extra();
```

**Returns:** `bool`

###### has_error()

True when this node or any descendant is an error.

**Signature:**

```rust
pub fn has_error(&self) -> bool
```

**Example:**

```rust
let result = instance.has_error();
```

**Returns:** `bool`

###### parent()

Return this node's parent, if any.

**Signature:**

```rust
pub fn parent(&self) -> Option<Node>
```

**Example:**

```rust
let result = instance.parent();
```

**Returns:** `Option<Node>`

###### child()

Return the i-th child of this node, if any.

**Signature:**

```rust
pub fn child(&self, index: u32) -> Option<Node>
```

**Example:**

```rust
let result = instance.child(42);
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `index` | `u32` | Yes | The index |

**Returns:** `Option<Node>`

###### child_count()

Total number of children (including unnamed).

**Signature:**

```rust
pub fn child_count(&self) -> usize
```

**Example:**

```rust
let result = instance.child_count();
```

**Returns:** `usize`

###### named_child()

Return the i-th named child of this node, if any.

**Signature:**

```rust
pub fn named_child(&self, index: u32) -> Option<Node>
```

**Example:**

```rust
let result = instance.named_child(42);
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `index` | `u32` | Yes | The index |

**Returns:** `Option<Node>`

###### named_child_count()

Number of named children of this node.

**Signature:**

```rust
pub fn named_child_count(&self) -> usize
```

**Example:**

```rust
let result = instance.named_child_count();
```

**Returns:** `usize`

###### child_by_field_name()

Look up a child by its grammar-defined field name.

**Signature:**

```rust
pub fn child_by_field_name(&self, name: &str) -> Option<Node>
```

**Example:**

```rust
let result = instance.child_by_field_name("value");
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `name` | `String` | Yes | The name |

**Returns:** `Option<Node>`

###### to_sexp()

Return the S-expression form of this node's subtree.

**Signature:**

```rust
pub fn to_sexp(&self) -> String
```

**Example:**

```rust
let result = instance.to_sexp();
```

**Returns:** `String`

###### walk()

Return a `TreeCursor` positioned at this node.

**Signature:**

```rust
pub fn walk(&self) -> TreeCursor
```

**Example:**

```rust
let result = instance.walk();
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
| `cache_dir` | `Option<PathBuf>` | `Default::default()` | Override default cache directory. Default: `~/.cache/tree-sitter-language-pack/v{version}/libs/` |
| `languages` | `Option<Vec<String>>` | `vec!\[\]` | Languages to pre-download on init. Each entry is a language name (e.g. `"python"`, `"rust"`). |
| `groups` | `Option<Vec<String>>` | `vec!\[\]` | Language groups to pre-download (e.g. `"web"`, `"systems"`, `"scripting"`). |

---

#### Parser

A tree-sitter parser configured for one language at a time.

##### Methods

###### new()

Construct a new parser with no language set.

Call `Parser::set_language` before parsing.

**Signature:**

```rust
pub fn new() -> Parser
```

**Example:**

```rust
let result = Parser::new();
```

**Returns:** `Parser`

###### set_language()

Configure the parser to use the language identified by name (e.g. `"python"`).

Resolves the language through the global registry — auto-downloading
if necessary, when the `download` feature is enabled.

**Errors:**

Returns `Error::LanguageNotFound` if the language is not recognized,
or `Error::ParserSetup` if the language ABI is incompatible.

**Signature:**

```rust
pub fn set_language(&self, name: &str) -> Result<(), Error>
```

**Example:**

```rust
instance.set_language("value")?;
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `name` | `String` | Yes | The name |

**Returns:** No return value.

**Errors:** Returns `Err(Error)`.

###### parse()

Parse a UTF-8 source string. Returns `None` if parsing was cancelled
or no language is set.

**Signature:**

```rust
pub fn parse(&self, source: &str) -> Option<Tree>
```

**Example:**

```rust
let result = instance.parse("value");
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `source` | `String` | Yes | The source |

**Returns:** `Option<Tree>`

###### parse_bytes()

Parse a raw byte slice. Returns `None` if parsing was cancelled or
no language is set.

**Signature:**

```rust
pub fn parse_bytes(&self, source: &[u8]) -> Option<Tree>
```

**Example:**

```rust
let result = instance.parse_bytes(b"data");
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `source` | `Vec<u8>` | Yes | The source |

**Returns:** `Option<Tree>`

###### reset()

Reset internal state. The next call to `parse` will
not be incremental.

**Signature:**

```rust
pub fn reset(&self)
```

**Example:**

```rust
instance.reset();
```

**Returns:** No return value.

###### default()

**Signature:**

```rust
pub fn default() -> Parser
```

**Example:**

```rust
let result = Parser::default();
```

**Returns:** `Parser`

---

#### Point

A source position — row + column, zero-indexed.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `row` | `usize` | — | Zero-indexed row number. |
| `column` | `usize` | — | Zero-indexed column number, in UTF-16 code units. |

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
| `data_extraction` | `bool` | `false` | Extract hierarchical key/value data tree from data-format files. Default: false. When `true`, `ProcessResult::data` is populated with a `DataNode` tree for supported languages: JSON, YAML, TOML, `.properties`, HCL/HOCON, INI, editorconfig, KDL, CUE, CSV, PSV, PO, nginx config, Caddy config, XML, and DTD. For languages outside this set the field is left as `None`. |

##### Methods

###### default()

**Signature:**

```rust
pub fn default() -> ProcessConfig
```

**Example:**

```rust
let result = ProcessConfig::default();
```

**Returns:** `ProcessConfig`

###### with_chunking()

Enable chunking with the given maximum chunk size in bytes.

**Signature:**

```rust
pub fn with_chunking(&self, max_size: usize) -> ProcessConfig
```

**Example:**

```rust
let result = instance.with_chunking(42);
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `max_size` | `usize` | Yes | The max size |

**Returns:** `ProcessConfig`

###### all()

Enable all analysis features.

**Signature:**

```rust
pub fn all(&self) -> ProcessConfig
```

**Example:**

```rust
let result = instance.all();
```

**Returns:** `ProcessConfig`

###### minimal()

Disable all analysis features (only metrics computed).

**Signature:**

```rust
pub fn minimal(&self) -> ProcessConfig
```

**Example:**

```rust
let result = instance.minimal();
```

**Returns:** `ProcessConfig`

###### with_data_extraction()

Enable or disable hierarchical data extraction for data-format files.

When `true`, `ProcessResult::data` is
populated with a key/value tree for supported data-format languages.

**Signature:**

```rust
pub fn with_data_extraction(&self, enabled: bool) -> ProcessConfig
```

**Example:**

```rust
let result = instance.with_data_extraction(true);
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `enabled` | `bool` | Yes | The enabled |

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
| `structure` | `Vec<StructureItem>` | `vec!\[\]` | Top-level structural items (functions, classes, etc.). |
| `imports` | `Vec<ImportInfo>` | `vec!\[\]` | Import statements extracted from the source. |
| `exports` | `Vec<ExportInfo>` | `vec!\[\]` | Export statements extracted from the source. |
| `comments` | `Vec<CommentInfo>` | `vec!\[\]` | Comments extracted from the source. |
| `docstrings` | `Vec<DocstringInfo>` | `vec!\[\]` | Docstrings extracted from the source. |
| `symbols` | `Vec<SymbolInfo>` | `vec!\[\]` | Symbol definitions (variables, types, functions) extracted from the source. |
| `diagnostics` | `Vec<Diagnostic>` | `vec!\[\]` | Parse diagnostics (syntax errors, missing nodes) from tree-sitter. |
| `chunks` | `Vec<CodeChunk>` | `vec!\[\]` | Syntax-aware code chunks produced when chunking is enabled. |
| `data` | `Option<DataNode>` | `Default::default()` | Hierarchical data tree extracted when `config.data_extraction` is `true`. Populated for supported data-format languages (JSON, YAML, TOML, properties, HCL, INI, XML, CSV, and more). `None` when `data_extraction` is `false` (the default) or when the language is not a recognised data format. See `DataNode` for the shape of the returned tree. |

---

#### Span

Byte and line/column range in source code.

Represents both byte offsets (for slicing) and human-readable line/column
positions (for display and diagnostics).

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `start_byte` | `usize` | — | Inclusive start byte offset in the source. |
| `end_byte` | `usize` | — | Exclusive end byte offset in the source. |
| `start_line` | `usize` | — | Zero-indexed line number of the span's start. |
| `start_column` | `usize` | — | Zero-indexed column number of the span's start. |
| `end_line` | `usize` | — | Zero-indexed line number of the span's end. |
| `end_column` | `usize` | — | Zero-indexed column number of the span's end. |

---

#### StructureItem

A structural item (function, class, struct, etc.) in source code.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `kind` | `StructureKind` | `StructureKind::Function` | The kind of structural item. |
| `name` | `Option<String>` | `Default::default()` | The declared name of the item, if present. |
| `visibility` | `Option<String>` | `Default::default()` | Visibility modifier (e.g., `"pub"`, `"public"`, `"private"`). |
| `span` | `Span` | — | Source span covering the entire item declaration. |
| `children` | `Vec<StructureItem>` | `vec!\[\]` | Nested structural items (e.g., methods within a class). |
| `decorators` | `Vec<String>` | `vec!\[\]` | Decorator or attribute names applied to the item. |
| `doc_comment` | `Option<String>` | `Default::default()` | Documentation comment attached to the item, if any. |
| `signature` | `Option<String>` | `Default::default()` | Full signature text of the item (e.g., function parameters and return type). |
| `body_span` | `Option<Span>` | `Default::default()` | Source span covering only the body of the item, if distinct from the declaration. |

---

#### SymbolInfo

A symbol (variable, function, type, etc.) extracted from source code.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `name` | `String` | — | The name of the symbol. |
| `kind` | `SymbolKind` | `SymbolKind::Variable` | The kind of symbol (variable, function, class, etc.). |
| `span` | `Span` | — | Source span covering the symbol definition. |
| `type_annotation` | `Option<String>` | `Default::default()` | Explicit type annotation, if present in the source. |
| `doc` | `Option<String>` | `Default::default()` | Documentation comment associated with this symbol. |

---

#### Tree

A parsed syntax tree. Cheap to clone (refcount bump).

##### Methods

###### root_node()

Return the root `Node` of this tree.

**Signature:**

```rust
pub fn root_node(&self) -> Node
```

**Example:**

```rust
let result = instance.root_node();
```

**Returns:** `Node`

###### walk()

Return a `TreeCursor` positioned at the root.

**Signature:**

```rust
pub fn walk(&self) -> TreeCursor
```

**Example:**

```rust
let result = instance.walk();
```

**Returns:** `TreeCursor`

---

#### TreeCursor

A cursor for traversing a `Tree`.

##### Methods

###### node()

Return the `Node` at the cursor's current position.

**Signature:**

```rust
pub fn node(&self) -> Node
```

**Example:**

```rust
let result = instance.node();
```

**Returns:** `Node`

###### goto_first_child()

Move the cursor to the first child of the current node.
Returns `true` if a child existed.

**Signature:**

```rust
pub fn goto_first_child(&self) -> bool
```

**Example:**

```rust
let result = instance.goto_first_child();
```

**Returns:** `bool`

###### goto_parent()

Move the cursor to the parent of the current node.
Returns `true` if a parent existed.

**Signature:**

```rust
pub fn goto_parent(&self) -> bool
```

**Example:**

```rust
let result = instance.goto_parent();
```

**Returns:** `bool`

###### goto_next_sibling()

Move the cursor to the next sibling of the current node.
Returns `true` if a sibling existed.

**Signature:**

```rust
pub fn goto_next_sibling(&self) -> bool
```

**Example:**

```rust
let result = instance.goto_next_sibling();
```

**Returns:** `bool`

###### field_name()

Return the field name for the current node, if any.

**Signature:**

```rust
pub fn field_name(&self) -> Option<String>
```

**Example:**

```rust
let result = instance.field_name();
```

**Returns:** `Option<String>`

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
| `Other` | A language-specific construct that does not fit any standard category. — Fields: `0`: `String` |

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
| `Other` | A language-specific docstring format not covered by the standard variants. — Fields: `0`: `String` |

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
| `Other` | A symbol kind not covered by the standard variants. — Fields: `0`: `String` |

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
