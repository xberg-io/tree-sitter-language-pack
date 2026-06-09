---
name: tree-sitter-language-pack
description: >-
  Parse and extract code intelligence from 306+ programming languages using tree-sitter grammars.
  Provides Rust core with native bindings for Python, Node.js, TypeScript, Ruby, Go, Java, C#, Elixir, PHP, and WebAssembly.
  Use when writing code that parses source code, extracts structure/imports/exports, or analyzes code with syntax-aware chunking.
license: Elastic-2.0
metadata:
  author: kreuzberg-dev
  version: "1.0"
  repository: https://github.com/kreuzberg-dev/tree-sitter-language-pack
---

# Tree-Sitter Language Pack

tree-sitter-language-pack is a polyglot code parsing and analysis library with a high-performance Rust core and bindings for Python, Node.js/TypeScript, Ruby, Go, Java, C#, PHP, Elixir, and WebAssembly. It compiles 306+ tree-sitter grammars into efficient parsers and provides code intelligence extraction: structure (functions, classes), imports, exports, comments, docstrings, diagnostics, and syntax-aware chunking for LLM ingestion.

Use this skill when writing code that:

- Parses source code in any of 306 supported languages
- Extracts code structure, imports, exports, and metadata
- Detects syntax errors and generates diagnostics
- Chunks code intelligently for LLM context windows
- Performs language detection from file paths or content
- Validates custom tree-sitter query patterns
- Integrates tree-sitter parsing into polyglot applications

## Installation

### Python

```bash
pip install tree-sitter-language-pack
# or with uv:
uv add tree-sitter-language-pack
```

### Node.js/TypeScript

```bash
npm install @kreuzberg/tree-sitter-language-pack
# or with pnpm:
pnpm add @kreuzberg/tree-sitter-language-pack
```

### Rust

```toml
[dependencies]
tree-sitter-language-pack = "1"
# With download feature (default):
# tree-sitter-language-pack = { version = "1", features = ["download"] }
```

### CLI

```bash
# From source
cargo install --path crates/ts-pack-cli

# Or download pre-built from GitHub releases
```

### Other Bindings

- **Ruby**: `gem install tree_sitter_language_pack`
- **Go**: `go get github.com/kreuzberg-dev/tree-sitter-language-pack/packages/go`
- **Java**: Maven: add `dev.kreuzberg.treesitterlanguagepack:tree-sitter-language-pack`
- **C#**: `dotnet add package TreeSitterLanguagePack`
- **PHP**: `composer require kreuzberg-dev/tree-sitter-language-pack`
- **Elixir**: Mix: `{:tree_sitter_language_pack, "~> 1.0"}`
- **WebAssembly**: `npm install @kreuzberg/tree-sitter-language-pack-wasm`

## Quick Start

### Python

```python
from tree_sitter_language_pack import (
    parse_string, process, ProcessConfig, available_languages
)

# List available languages
print(f"{len(available_languages())} languages supported")

# Parse source code
tree = parse_string("python", "def hello(): pass")
print(tree.root_node_type())         # "module"
print(tree.has_error_nodes())         # False
print(tree.contains_node_type("function_definition"))  # True

# Extract code intelligence
config = ProcessConfig("python", structure=True, imports=True, docstrings=True)
result = process("def hello(): pass", config)
print(f"Functions: {len(result['structure'])}")
```

### Node.js/TypeScript

```typescript
import {
  parseString,
  process,
  availableLanguages,
  treeRootNodeType,
} from "@kreuzberg/tree-sitter-language-pack";

// List languages
console.log(`${availableLanguages().length} languages supported`);

// Parse code
const tree = parseString("python", "def hello(): pass");
console.log(treeRootNodeType(tree)); // "module"

// Extract intelligence
const result = process("def hello(): pass", { language: "python" });
console.log(`Functions: ${result.structure.length}`);
```

### Rust

```rust
use tree_sitter_language_pack::{
    parse_string, process, ProcessConfig, available_languages
};

// List languages
println!("{} languages available", available_languages().len());

// Parse code
let tree = parse_string("python", b"def hello(): pass")?;
println!("{}", tree.root_node().kind());  // "module"

// Extract intelligence
let config = ProcessConfig::new("python").all();
let result = process("def hello(): pass", &config)?;
println!("Functions: {}", result.structure.len());
```

### CLI

```bash
# Parse a file
ts-pack parse src/main.py

# Extract code intelligence
ts-pack process src/main.py --all

# Detect language
ts-pack detect src/main.rs

# List available languages
ts-pack list

# Download specific languages
ts-pack download python javascript typescript rust
```

## Key APIs

### Language Discovery & Detection

| Function                                | Purpose                                       |
| --------------------------------------- | --------------------------------------------- |
| `available_languages()`                 | List all 306 supported language names         |
| `has_language(name)`                    | Check if a language is available              |
| `language_count()`                      | Return total language count                   |
| `detect_language(path)`                 | Detect language from file path/extension      |
| `detect_language_from_content(content)` | Detect from shebang or file content           |
| `detect_language_from_extension(ext)`   | Detect from bare file extension               |
| `extension_ambiguity(ext)`              | Check if extension maps to multiple languages |

### Parsing

| Function                              | Purpose                               |
| ------------------------------------- | ------------------------------------- |
| `parse_string(language, source)`      | Parse source code, return tree handle |
| `tree_root_node_type(tree)`           | Get root node type name               |
| `tree_root_child_count(tree)`         | Count named children of root          |
| `tree_contains_node_type(tree, type)` | Check if tree has node type anywhere  |
| `tree_has_error_nodes(tree)`          | Check for syntax errors               |
| `tree_error_count(tree)`              | Count ERROR and MISSING nodes         |
| `tree_to_sexp(tree)`                  | Return S-expression representation    |

### Code Intelligence & Processing

| Function                      | Purpose                                                                                 |
| ----------------------------- | --------------------------------------------------------------------------------------- |
| `process(source, config)`     | Extract structure, imports, exports, comments, docstrings, symbols, diagnostics, chunks |
| `extract(source, config)`     | Run custom tree-sitter query patterns                                                   |
| `validate_extraction(config)` | Validate query patterns without executing                                               |

### Download Management

| Function                 | Purpose                                                |
| ------------------------ | ------------------------------------------------------ |
| `init(config)`           | Initialize pack with pre-downloads and configuration   |
| `configure(config)`      | Set cache directory without downloading                |
| `download(names)`        | Download specific languages                            |
| `download_all()`         | Download all 306 languages                             |
| `manifest_languages()`   | Fetch list of available languages from remote manifest |
| `downloaded_languages()` | List locally cached languages                          |
| `clean_cache()`          | Delete all cached parsers                              |
| `cache_dir()`            | Get effective cache directory path                     |

## ProcessConfig Options

Control what analysis features are enabled:

| Option           | Type         | Default  | Description                                  |
| ---------------- | ------------ | -------- | -------------------------------------------- |
| `language`       | string       | required | Language name (e.g., "python", "javascript") |
| `structure`      | bool         | true     | Extract functions, classes, methods          |
| `imports`        | bool         | true     | Extract import statements                    |
| `exports`        | bool         | true     | Extract exported symbols                     |
| `comments`       | bool         | false    | Extract inline and block comments            |
| `docstrings`     | bool         | false    | Extract docstrings attached to definitions   |
| `symbols`        | bool         | false    | Extract all identifiers for search indexing  |
| `diagnostics`    | bool         | false    | Include parse errors and syntax diagnostics  |
| `chunk_max_size` | int or null  | null     | Maximum bytes per chunk (enables chunking)   |
| `extractions`    | dict or null | null     | Custom tree-sitter query patterns            |

Use `ProcessConfig.all(language)` to enable all features, or `ProcessConfig.minimal(language)` to disable all extractions (metrics only).

## Common Patterns

### Detect and Parse

```python
from tree_sitter_language_pack import detect_language, parse_string

lang = detect_language("src/main.rs")
if lang:
    source = open(f"src/main.rs").read()
    tree = parse_string(lang, source)
    print(tree.has_error_nodes())
```

### Batch Processing

```python
from tree_sitter_language_pack import detect_language, process, ProcessConfig
from pathlib import Path

for filepath in Path("src").glob("**/*"):
    if filepath.is_file():
        lang = detect_language(str(filepath))
        if lang:
            source = filepath.read_text()
            config = ProcessConfig(lang, structure=True, imports=True)
            result = process(source, config)
            print(f"{filepath}: {len(result['structure'])} items")
```

### Code Chunking for LLMs

```python
from tree_sitter_language_pack import process, ProcessConfig

config = ProcessConfig(
    "python",
    structure=True,
    chunk_max_size=1000,  # ~1000 tokens per chunk
)
result = process(open("large_file.py").read(), config)

for i, chunk in enumerate(result["chunks"]):
    print(f"Chunk {i}: lines {chunk['start_line']}-{chunk['end_line']}")
    # Feed chunk["content"] to LLM
```

### Custom Extraction Queries

```python
from tree_sitter_language_pack import extract

config = {
    "language": "python",
    "patterns": {
        "decorators": {
            "query": "(decorator (identifier) @name)",
            "capture_output": "Text",
        },
        "type_hints": {
            "query": "(typed_parameter type: (identifier) @type)",
            "capture_output": "Text",
        },
    },
}

result = extract("@dataclass\ndef process(x: int): pass", config)
for pattern_name, matches in result["results"].items():
    print(f"{pattern_name}: {matches['total_count']} matches")
```

## Common Pitfalls

1. **Forgetting to close/free resources**: Tree handles and registries consume native memory. Always dispose of them when done (use context managers in Python).

2. **Auto-download latency**: First call to parse an uncached language triggers a network request and download. Pre-download languages in `init()` for production.

3. **Chunking without metrics**: Always enable `structure=True` when using chunking to get node type metadata for each chunk.

4. **Query syntax errors**: Validate extraction patterns with `validate_extraction()` before running against large codebases.

5. **Language name casing**: All language names are lowercase (e.g., "python", not "Python").

6. **Extension ambiguity**: Some extensions (e.g., ".h") map to multiple languages. Use `extension_ambiguity()` to check and resolve manually.

## References

- [Python API Reference](./references/python-api.md)
- [TypeScript/Node.js API Reference](./references/typescript-api.md)
- [Rust API Reference](./references/rust-api.md)
- [CLI Reference](./references/cli-reference.md)
- [Other Bindings (Go, Java, C#, Ruby, Elixir, PHP, WASM, C FFI)](./references/other-bindings.md)
- [Code Intelligence Extraction](./references/code-intelligence.md)
- [Configuration & Download Model](./references/configuration.md)
