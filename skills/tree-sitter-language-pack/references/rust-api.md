# Rust API Quick Reference

## Installation

```toml
[dependencies]
tree-sitter-language-pack = "1"
# Default features include "download" for auto-downloading parsers
# Minimal (no download):
# tree-sitter-language-pack = { version = "1", default-features = false }
```

## Language Discovery

```rust
available_languages() -> Vec<String>
has_language(name: &str) -> bool
language_count() -> usize
detect_language_from_extension(ext: &str) -> Option<&'static str>
detect_language_from_path(path: &str) -> Option<&'static str>
detect_language_from_content(content: &str) -> Option<&'static str>
extension_ambiguity(ext: &str) -> Option<(&'static str, &'static [&'static str])>
```

## Parsing

```rust
parse_string(language: &str, source: &[u8]) -> Result<Tree, Error>

// Tree methods
tree.root_node().kind() -> &str
tree.root_node().child_count() -> usize
tree_contains_node_type(&tree, node_type: &str) -> bool
tree_has_error_nodes(&tree) -> bool
tree_error_count(&tree) -> usize
tree_to_sexp(&tree) -> String

// Node inspection
root_node_info(&tree) -> NodeInfo
find_nodes_by_type(&tree, node_type: &str) -> Vec<NodeInfo>
named_children_info(&tree) -> Vec<NodeInfo>

struct NodeInfo {
    pub kind: Cow<'static, str>,
    pub is_named: bool,
    pub start_byte: usize,
    pub end_byte: usize,
    pub start_row: usize,
    pub start_col: usize,
    pub end_row: usize,
    pub end_col: usize,
    pub named_child_count: usize,
    pub is_error: bool,
    pub is_missing: bool,
}
```

Example:

```rust
let tree = parse_string("python", b"x = 1")?;
assert_eq!(tree.root_node().kind(), "module");
assert!(!tree_has_error_nodes(&tree));
```

## Code Intelligence Processing

```rust
process(source: &str, config: &ProcessConfig) -> Result<ProcessResult, Error>

pub struct ProcessConfig {
    pub language: Cow<'static, str>,
    pub structure: bool,          // default: true
    pub imports: bool,            // default: true
    pub exports: bool,            // default: true
    pub comments: bool,           // default: false
    pub docstrings: bool,         // default: false
    pub symbols: bool,            // default: false
    pub diagnostics: bool,        // default: false
    pub chunk_max_size: Option<usize>,  // default: None
    pub extractions: Option<AHashMap<String, ExtractionPattern>>,
}

// Constructors
ProcessConfig::new(language: impl Into<String>) -> Self
config.with_chunking(max_size: usize) -> Self
config.all() -> Self
config.minimal() -> Self

pub struct ProcessResult {
    pub language: String,
    pub metrics: FileMetrics,
    pub structure: Vec<StructureItem>,
    pub imports: Vec<ImportInfo>,
    pub exports: Vec<ExportInfo>,
    pub comments: Vec<CommentInfo>,
    pub docstrings: Vec<DocstringInfo>,
    pub symbols: Vec<SymbolInfo>,
    pub diagnostics: Vec<Diagnostic>,
    pub chunks: Vec<CodeChunk>,
    pub parse_errors: usize,
}
```

Example:

```rust
let config = ProcessConfig::new("python")
    .all()
    .with_chunking(2000);

let result = process("def hello(): pass", &config)?;
println!("Functions: {}", result.structure.len());
println!("Lines: {}", result.metrics.total_lines);
```

## Extraction Queries

```rust
extract_patterns(source: &str, config: &ExtractionConfig) -> Result<ExtractionResult, Error>
validate_extraction(config: &ExtractionConfig) -> Result<ValidationResult, Error>

pub struct ExtractionConfig {
    pub language: String,
    pub patterns: AHashMap<String, ExtractionPattern>,
}

pub struct ExtractionPattern {
    pub query: String,
    pub capture_output: CaptureOutput,
    pub child_fields: Vec<String>,
    pub max_results: Option<usize>,
    pub byte_range: Option<(usize, usize)>,
}

pub enum CaptureOutput {
    Text,    // Text only
    Node,    // NodeInfo only
    Full,    // Both text and NodeInfo (default)
}

pub struct ExtractionResult {
    pub language: String,
    pub results: AHashMap<String, PatternResult>,
}

pub struct PatternResult {
    pub matches: Vec<MatchResult>,
    pub total_count: usize,
}
```

Example:

```rust
use ahash::AHashMap;

let mut patterns = AHashMap::new();
patterns.insert("functions".to_string(), ExtractionPattern {
    query: "(function_definition name: (identifier) @fn_name)".to_string(),
    capture_output: CaptureOutput::Full,
    child_fields: vec!["name".to_string()],
    max_results: None,
    byte_range: None,
});

let config = ExtractionConfig {
    language: "python".to_string(),
    patterns,
};

let result = extract_patterns("def hello(): pass", &config)?;
assert_eq!(result.results["functions"].total_count, 1);
```

## Compiled Extraction (Reusable)

```rust
pub struct CompiledExtraction { /* opaque */ }

CompiledExtraction::compile(config: &ExtractionConfig) -> Result<Self, Error>
CompiledExtraction::compile_with_language(
    language: Language,
    language_name: &str,
    patterns: &AHashMap<String, ExtractionPattern>
) -> Result<Self, Error>

compiled.extract(&self, source: &str) -> Result<ExtractionResult, Error>
compiled.extract_from_tree(&self, tree: &Tree, source: &[u8]) -> Result<ExtractionResult, Error>
```

## Bundled Queries

```rust
get_highlights_query(language: &str) -> Option<&'static str>
get_injections_query(language: &str) -> Option<&'static str>
get_locals_query(language: &str) -> Option<&'static str>
```

## Language Pointers (Interop)

```rust
get_language(name: &str) -> Result<Language, Error>
get_parser(name: &str) -> Result<Parser, Error>
```

These re-export `tree_sitter::{Language, Parser, Tree}` for interop with the upstream tree-sitter crate.

## Download & Configuration (requires "download" feature)

```rust
pub struct PackConfig {
    pub cache_dir: Option<PathBuf>,
    pub languages: Option<Vec<String>>,
    pub groups: Option<Vec<String>>,
}

init(config: &PackConfig) -> Result<(), Error>
configure(config: &PackConfig) -> Result<(), Error>
download(names: &[&str]) -> Result<usize, Error>
download_all() -> Result<usize, Error>
manifest_languages() -> Result<Vec<String>, Error>
downloaded_languages() -> Vec<String>
clean_cache() -> Result<(), Error>
cache_dir() -> Result<PathBuf, Error>
```

Example:

```rust
let config = PackConfig {
    cache_dir: Some(PathBuf::from("/opt/ts-pack")),
    languages: Some(vec!["python".to_string(), "rust".to_string()]),
    groups: None,
};

init(&config)?;
let count = download(&["python", "typescript"])?;
println!("Downloaded {} new languages", count);
```

## Error Handling

```rust
pub enum Error {
    LanguageNotFound(String),
    DynamicLoad(String),
    NullLanguagePointer(String),
    ParserSetup(String),
    LockPoisoned(String),
    Config(String),
    ParseFailed,
    QueryError(String),
    InvalidRange(String),
    Io(std::io::Error),
    Json(serde_json::Error),           // with "config" or "download"
    Toml(toml::de::Error),             // with "config"
    Download(String),                   // with "download"
    ChecksumMismatch { file, expected, actual },  // with "download"
}
```

Match specific error types:

```rust
match get_language("python") {
    Ok(lang) => println!("Got Python"),
    Err(Error::LanguageNotFound(name)) => println!("Not found: {}", name),
    Err(e) => println!("Error: {:?}", e),
}
```

## Common Patterns

### Parse and Query

```rust
use tree_sitter_language_pack::{parse_string, run_query};

let tree = parse_string("python", b"def hello(): pass")?;
let matches = run_query(
    &tree,
    "python",
    "(function_definition name: (identifier) @fn_name)",
    b"def hello(): pass",
)?;
```

### Chunked Processing

```rust
let config = ProcessConfig::new("python")
    .with_chunking(2000)
    .structure(true);

let result = process(source, &config)?;
for chunk in &result.chunks {
    println!("Chunk: lines {}-{}", chunk.start_line, chunk.end_line);
}
```

### Feature Flags

Crate features:

- `download` (default): Auto-download and cache parsers
- `serde`: Serialization for ProcessConfig/ProcessResult
- `config`: Load PackConfig from TOML files

Minimal build (no download):

```toml
tree-sitter-language-pack = { version = "1", default-features = false }
```
