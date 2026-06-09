---
description: "Extract functions, imports, docstrings, and other structured information from source code."
---

# Code intelligence

`process()` runs built-in tree-sitter queries against a file and returns structured data: the functions and classes defined in it, what it imports, its docstrings, comments, and more. You configure what to extract; the library handles the queries.

Here's what a typical result looks like:

=== "Python"

    ```python
    from tree_sitter_language_pack import process, ProcessConfig

    source = '''
    import os
    from pathlib import Path

    def read_file(path: str) -> str:
        """Read and return file contents."""
        return Path(path).read_text()

    class FileCache:
        """Cache for file contents."""

        def __init__(self, root: str):
            self.root = root

        def get(self, name: str) -> str:
            """Return cached file contents."""
            return read_file(os.path.join(self.root, name))
    '''

    result = process(source, ProcessConfig(
        language="python",
        structure=True,
        imports=True,
        docstrings=True,
    ))

    for item in result["structure"]:
        doc = f" — {item['docstring']}" if item.get("docstring") else ""
        print(f"{item['kind']:8} {item['name']:20} lines {item['start_line']}-{item['end_line']}{doc}")

    print()
    for imp in result["imports"]:
        names = ", ".join(imp["names"]) or "*"
        print(f"from {imp['source']} import {names}")
    ```

    Output:
    ```
    function read_file            lines 5-7  — Read and return file contents.
    class    FileCache             lines 9-18 — Cache for file contents.
    method   __init__              lines 12-13
    method   get                   lines 15-17 — Return cached file contents.

    from os import *
    from pathlib import Path
    ```

=== "Node.js"

    ```typescript
    import { process } from "@kreuzberg/tree-sitter-language-pack";

    const result = await process(source, {
      language: "typescript",
      structure: true,
      imports: true,
      docstrings: true,
    });

    result.structure.forEach(item => {
      const doc = item.docstring ? ` — ${item.docstring}` : "";
      console.log(`${item.kind.padEnd(8)} ${item.name.padEnd(20)} lines ${item.startLine}-${item.endLine}${doc}`);
    });
    ```

=== "Rust"

    ```rust
    use tree_sitter_language_pack::{process, ProcessConfig};

    let mut config = ProcessConfig::new("rust");
    config.structure = true;
    config.imports = true;
    config.docstrings = true;

    let result = process(source, &config)?;

    for item in &result.structure {
        println!("{:8} {:20} lines {}-{}",
            item.kind, item.name, item.start_line, item.end_line);
    }
    ```

=== "CLI"

    ```bash
    # Extract structure and docstrings
    ts-pack process src/app.py --structure --docstrings

    # All fields, JSON output
    ts-pack process src/app.py --all | jq '.structure'
    ```

## ProcessConfig fields

Pass `language` plus any of these fields:

| Field            | Default | What it extracts                                                  |
| ---------------- | ------- | ----------------------------------------------------------------- |
| `structure`      | `True`  | Functions, classes, methods, interfaces, structs, traits, enums   |
| `imports`        | `True`  | Import/require statements — source module and imported names      |
| `exports`        | `True`  | Exported symbols                                                  |
| `comments`       | `False` | All comments with text and location                               |
| `docstrings`     | `False` | Docstrings attached to declarations (requires `structure=True`)   |
| `symbols`        | `False` | Deduplicated list of all identifiers, for search indexing         |
| `diagnostics`    | `False` | Syntax error nodes from the parse                                 |
| `chunk_max_size` | `None`  | Maximum chunk size in bytes; see [Chunking for LLMs](chunking.md) |

Enable everything at once: `ProcessConfig.all("python")`.

## Result fields

### `structure`

Each item has:

| Field        | Type        | Description                                                                                         |
| ------------ | ----------- | --------------------------------------------------------------------------------------------------- |
| `kind`       | str         | `function`, `class`, `method`, `interface`, `struct`, `trait`, `enum`, `impl`, `module`, and so on. |
| `name`       | str         | Declaration name                                                                                    |
| `start_line` | int         | First line (1-indexed)                                                                              |
| `end_line`   | int         | Last line (1-indexed)                                                                               |
| `docstring`  | str \| None | Attached docstring — only present when `docstrings=True`                                            |

Kinds vary by language:

| Language              | Kinds                                                                |
| --------------------- | -------------------------------------------------------------------- |
| Python                | `function`, `class`, `method`, `async_function`                      |
| JavaScript/TypeScript | `function`, `class`, `method`, `async_function`, `interface`, `enum` |
| Rust                  | `function`, `struct`, `impl`, `trait`, `enum`, `type_alias`, `mod`   |
| Java                  | `class`, `interface`, `method`, `constructor`, `enum`                |
| Go                    | `function`, `struct`, `interface`, `method`                          |

### `imports`

Each import has `source` (module path), `names` (list of imported identifiers — empty for wildcard or bare imports), and `start_line`.

Covers both `import x` and `from x import y` in Python, both `import` and `require()` in JavaScript.

### `exports`

Language-specific:

- Python: module-level items not prefixed with `_`, or listed in `__all__`
- JavaScript/TypeScript: explicit `export` declarations
- Rust: items with `pub` visibility

Each export has `name` and `kind`.

### `comments`

Each comment has `text`, `start_line`, and `is_block` (True for `/* ... */`, False for line comments).

### `docstrings`

Docstrings attach to their parent item in `structure` as the `docstring` field. Extraction understands each language's convention:

| Language              | Convention                                     |
| --------------------- | ---------------------------------------------- |
| Python                | `"""..."""` immediately after `def`/`class`    |
| Rust                  | `///` or `//!` above the item                  |
| JavaScript/TypeScript | `/** ... */` JSDoc above the function          |
| Java                  | `/** ... */` Javadoc                           |
| Ruby                  | `# ...` lines immediately before `def`/`class` |
| Go                    | `// FuncName ...` comment block above the func |
| Elixir                | `@doc "..."` or `@moduledoc "..."`             |

### `symbols`

A deduplicated list of all identifiers in the file. Useful for search indexing:

```python
result = process(source, ProcessConfig(language="python", symbols=True))
print(sorted(result["symbols"])[:10])
# ['FileCache', 'Path', 'get', 'name', 'os', 'path', 'read_file', 'root', 'str']
```

### `diagnostics`

Syntax error nodes. A non-empty list does not mean the file has a parse error — tree-sitter recovers and produces a partial tree.

```python
result = process(source, ProcessConfig(language="python", diagnostics=True))
for err in result["diagnostics"]:
    print(f"Line {err['start_line']}, col {err['start_col']}: {err['message']}")
```

### `metrics`

File-level statistics, independent of the other fields:

| Field           | Type | Description                              |
| --------------- | ---- | ---------------------------------------- |
| `total_lines`   | int  | All lines                                |
| `code_lines`    | int  | Non-blank, non-comment lines             |
| `comment_lines` | int  | Comment lines                            |
| `blank_lines`   | int  | Empty lines                              |
| `max_depth`     | int  | Maximum nesting depth of the syntax tree |

```python
result = process(source, ProcessConfig(language="python"))
m = result["metrics"]
print(f"{m['total_lines']} lines total, {m['code_lines']} code, {m['comment_lines']} comments")
```

### `chunks`

When `chunk_max_size` has a value, `result["chunks"]` contains syntax-aware splits ready for LLM ingestion. See [Chunking for LLMs](chunking.md) for full documentation.

## Next steps

- [Chunking for LLMs](chunking.md) — split code at natural boundaries for LLM ingestion
- [Parsing code](parsing.md) — raw syntax trees and low-level node traversal
