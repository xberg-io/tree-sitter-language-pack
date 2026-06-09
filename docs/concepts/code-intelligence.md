---
title: Code Intelligence
description: "What tree-sitter-language-pack extracts from source code: structure, imports, exports, comments, docstrings, and chunks."
---

The `process` function goes beyond raw syntax trees. It runs tree-sitter queries against the parsed AST to extract structured information useful for code analysis, search, documentation, and LLM ingestion.

---

## ProcessConfig

All intelligence extraction is opt-in via `ProcessConfig`. Enable what you need:

=== "Python"

    ```python
    from tree_sitter_language_pack import ProcessConfig

    config = ProcessConfig(
        language="python",
        structure=True,    # functions, classes, methods
        imports=True,      # import statements
        exports=True,      # exported symbols
        comments=True,     # inline comments
        docstrings=True,   # docstring extraction
        symbols=True,      # all identifiers
        diagnostics=True,  # syntax errors / error nodes
        # chunk_max_size=1000  # uncomment to enable chunking
    )
    ```

=== "Node.js"

    ```typescript
    import { process } from "@kreuzberg/tree-sitter-language-pack";

    const result = await process(source, {
      language: "typescript",
      structure: true,
      imports: true,
      exports: true,
      comments: true,
      docstrings: true,
      symbols: true,
      diagnostics: true,
    });
    ```

=== "Rust"

    ```rust
    use tree_sitter_language_pack::{process, ProcessConfig};

    let config = ProcessConfig::new("rust").all();

    let result = process(source, &config)?;
    ```

Use `.all()` in Rust or `ProcessConfig.all("python")` in Python to enable everything at once.

---

## ProcessResult Fields

### `structure` - Functions, Classes, and Methods

A list of top-level code constructs with their names, kinds, ranges, and optionally their docstrings.

```python
for item in result["structure"]:
    print(item["kind"])       # "function" | "class" | "method" | "interface" | ...
    print(item["name"])       # "greet"
    print(item["start_line"]) # 3
    print(item["end_line"])   # 6
    print(item["docstring"])  # "Greet a user by name."  (if docstrings=True)
```

Supported kinds vary by language:

| Kind         | Languages                                     |
| ------------ | --------------------------------------------- |
| `function`   | All languages                                 |
| `class`      | Python, JS/TS, Java, C#, Ruby, PHP, Kotlin, … |
| `method`     | Same as class                                 |
| `interface`  | TypeScript, Java, C#, Go, Kotlin, …           |
| `struct`     | Rust, Go, C, C++, C#, …                       |
| `impl`       | Rust                                          |
| `module`     | Elixir, Ruby, Rust, …                         |
| `enum`       | Rust, Java, C#, TypeScript, Kotlin, …         |
| `trait`      | Rust                                          |
| `type_alias` | TypeScript, Rust                              |
| `decorator`  | Python, TypeScript                            |

---

### `imports` - Import Statements

All import declarations with their source module and imported names.

```python
for imp in result["imports"]:
    print(imp["source"])    # "os"  or  "pathlib"
    print(imp["names"])     # ["path", "getcwd"]  (empty = wildcard or bare import)
    print(imp["start_line"])
```

Example output as JSON:

```json
[
  { "source": "os", "names": [], "start_line": 1 },
  { "source": "pathlib", "names": ["Path"], "start_line": 2 },
  { "source": "./utils", "names": ["readFile", "writeFile"], "start_line": 3 }
]
```

---

### `exports` — Exported Symbols

Symbols that are part of the module's public API.

```python
for exp in result["exports"]:
    print(exp["name"])  # "readFile"
    print(exp["kind"])  # "function" | "class" | "const" | ...
```

!!! Note Export detection is language-specific. For Python, everything defined at module level counts as exported unless prefixed with `_`. For JavaScript/TypeScript, explicit `export` declarations determine what the module exposes.

---

### `comments` - Inline Comments

All comments in the file with their text and location.

```python
for comment in result["comments"]:
    print(comment["text"])       # "// TODO: handle edge case"
    print(comment["start_line"]) # 42
    print(comment["is_block"])   # False
```

---

### `docstrings` - Documentation Strings

Docstrings appear under their parent construct in `structure`. When `docstrings=True`, each `structure` item gains a `docstring` field:

```python
func = result["structure"][0]
print(func["docstring"])
# "Read and return the contents of a file.\n\nArgs:\n    path: Path to the file."
```

Docstring extraction understands language-specific conventions:

| Language              | Convention                                                       |
| --------------------- | ---------------------------------------------------------------- |
| Python                | `"""..."""` triple-quoted string immediately after `def`/`class` |
| Rust                  | `///` or `//!` doc comments above item                           |
| JavaScript/TypeScript | `/** ... */` JSDoc block above function                          |
| Java                  | `/** ... */` Javadoc block above method/class                    |
| Ruby                  | `# ...` lines immediately above `def`/`class`                    |
| Go                    | `// FuncName ...` comment block above func                       |
| Elixir                | `@doc "..."` or `@moduledoc "..."`                               |

---

### `symbols` - All Identifiers

A deduplicated list of all identifiers referenced in the file, useful for search indexing.

```python
print(result["symbols"])
# ["os", "Path", "read_file", "FileManager", "base_dir", "get", ...]
```

---

### `diagnostics` - Syntax Errors

Tree-sitter produces partial trees for malformed code, marking error nodes. `diagnostics` surfaces these:

```python
for error in result["diagnostics"]:
    print(error["message"])    # "Unexpected token"
    print(error["start_line"])
    print(error["start_col"])
```

!!! Tip A non-empty `diagnostics` list does not mean the file is unparsable — tree-sitter recovers and continues. Use it to detect broken syntax rather than to gate parsing.

---

### `chunks` - Syntax-Aware Splits

When `chunk_max_size > 0`, the `chunks` field contains the file split into byte-budget segments. See [Chunking for LLMs](../guides/chunking.md) for full documentation.

```python
for chunk in result["chunks"]:
    print(chunk["content"])      # the source code text
    print(chunk["start_byte"])   # start byte offset
    print(chunk["end_byte"])     # end byte offset
    print(chunk["start_line"])   # first line of chunk
    print(chunk["end_line"])     # last line of chunk
    print(chunk["node_types"])   # ["function_definition", "class_definition"]
```

---

### `metrics` - File-Level Statistics

Basic metrics about the file:

```python
m = result["metrics"]
print(m["total_lines"])       # 120
print(m["code_lines"])        # 95   (non-blank, non-comment lines)
print(m["comment_lines"])     # 18
print(m["blank_lines"])       # 7
print(m["max_depth"])         # maximum nesting depth of the syntax tree
```

---

## Full Example

```python
from tree_sitter_language_pack import process, ProcessConfig

source = '''
import os
from pathlib import Path
from typing import Optional

def read_file(path: str, encoding: str = "utf-8") -> Optional[str]:
    """Read and return the contents of a file.

    Args:
        path: Path to the file to read.
        encoding: File encoding. Defaults to utf-8.

    Returns:
        File contents, or None if the file doesn't exist.
    """
    p = Path(path)
    if not p.exists():
        return None
    return p.read_text(encoding=encoding)

class FileCache:
    """In-memory cache for file contents."""

    def __init__(self, root: str):
        self._root = root
        self._cache: dict[str, str] = {}

    def get(self, name: str) -> Optional[str]:
        if name not in self._cache:
            self._cache[name] = read_file(os.path.join(self._root, name))
        return self._cache[name]
'''

config = ProcessConfig(
    language="python",
    structure=True,
    imports=True,
    docstrings=True,
    comments=True,
    diagnostics=True,
)
result = process(source, config)

# Structure
for item in result["structure"]:
    print(f"{item['kind']:12} {item['name']:20} lines {item['start_line']}-{item['end_line']}")

# Output:
# function     read_file            lines 6-20
# class        FileCache            lines 22-33
# method       __init__             lines 26-28
# method       get                  lines 30-33

# Imports
for imp in result["imports"]:
    names = ", ".join(imp["names"]) or "*"
    print(f"from {imp['source']} import {names}")

# Output:
# from os import *
# from pathlib import Path
# from typing import Optional

# Docstrings
func = result["structure"][0]
print(f"\n{func['name']} docstring:\n{func['docstring']}")

# Metrics
m = result["metrics"]
print(f"\nLines: {m['total_lines']} total, {m['code_lines']} code, {m['comment_lines']} comments")
```

---

## Custom Queries

Custom query extraction helpers are not part of the v1.8 public API. In Rust, call `get_parser()` and use the `tree-sitter` query APIs directly when the built-in `process()` fields are not enough.
