# Code Intelligence & Chunking Quick Reference

## ProcessConfig Options

Enable only what you need:

| Option           | Type         | Default  | Extracts                              |
| ---------------- | ------------ | -------- | ------------------------------------- |
| `language`       | string       | required | (Language to parse)                   |
| `structure`      | bool         | true     | Functions, classes, methods, modules  |
| `imports`        | bool         | true     | Import statements and their sources   |
| `exports`        | bool         | true     | Exported symbols and their kinds      |
| `comments`       | bool         | false    | Inline and block comments             |
| `docstrings`     | bool         | false    | Docstrings attached to definitions    |
| `symbols`        | bool         | false    | All identifiers (for search indexing) |
| `diagnostics`    | bool         | false    | Syntax errors and error nodes         |
| `chunk_max_size` | int or null  | null     | Syntax-aware code chunks (for LLMs)   |
| `extractions`    | dict or null | null     | Custom tree-sitter query patterns     |

Use `ProcessConfig.all(language)` to enable everything, or `ProcessConfig.minimal(language)` to extract only metrics.

## ProcessResult Structure

```python
result = process(source, config)

result["language"]        # str: language used
result["metrics"]         # FileMetrics dict
result["structure"]       # list[StructureItem]: functions, classes, etc.
result["imports"]         # list[ImportInfo]: import statements
result["exports"]         # list[ExportInfo]: exported symbols
result["comments"]        # list[CommentInfo]: code comments
result["docstrings"]      # list[DocstringInfo]: docstrings
result["symbols"]         # list[SymbolInfo]: all identifiers
result["diagnostics"]     # list[Diagnostic]: syntax errors
result["chunks"]          # list[CodeChunk]: code chunks (if chunking enabled)
```

## FileMetrics

Available when any processing enabled:

```python
metrics = result["metrics"]

metrics["total_lines"]     # int: total lines in file
metrics["code_lines"]      # int: lines with code (no blanks/comments)
metrics["comment_lines"]   # int: lines with comments
metrics["blank_lines"]     # int: blank lines
metrics["total_bytes"]     # int: bytes in file
metrics["error_count"]     # int: number of syntax errors
```

## Structure Items

Functions, classes, methods, modules, etc.

```python
for item in result["structure"]:
    item["kind"]           # str: "function", "class", "method", "module", etc.
    item["name"]           # str: identifier name
    item["start_line"]     # int: 1-indexed
    item["end_line"]       # int
    item["start_byte"]     # int
    item["end_byte"]       # int
    item["parent"]         # str or None: enclosing class/module if nested
    item["docstring"]      # str or None: if docstrings=True
    item["visibility"]     # str or None: "public", "private", etc.
```

Example (Python):

```python
# def greet(name: str) -> str:
#     """Greet a user."""
#     return f"Hello, {name}!"

# item =
{
    "kind": "function",
    "name": "greet",
    "start_line": 1,
    "end_line": 3,
    "docstring": "Greet a user.",
    "visibility": "public",
}
```

## Import/Export Items

```python
for imp in result["imports"]:
    imp["source"]          # str: module name ("os", "pathlib", "./utils")
    imp["names"]           # list[str]: imported items (empty = wildcard)
    imp["alias"]           # str or None: "as" clause
    imp["start_line"]      # int
    imp["is_wildcard"]     # bool

for exp in result["exports"]:
    exp["name"]            # str: exported symbol
    exp["kind"]            # str: "function", "class", "constant", etc.
    exp["start_line"]      # int
```

Examples (Python):

```python
# import os
# {"source": "os", "names": [], "is_wildcard": true}

# from pathlib import Path
# {"source": "pathlib", "names": ["Path"], "is_wildcard": false}

# def greet(): ...
# {"name": "greet", "kind": "function"}
```

## Comment/Docstring Items

```python
for comment in result["comments"]:
    comment["text"]        # str: comment text
    comment["kind"]        # str: "line", "block", "doc"
    comment["start_line"]  # int
    comment["is_block"]    # bool: multi-line comment

for docstring in result["docstrings"]:
    docstring["text"]      # str: full docstring text
    docstring["format"]    # str: "markdown", "restructuredtext", "google", "numpy", etc.
    docstring["start_line"] # int
    docstring["associated_item"]  # str or None: function/class name
    docstring["sections"]  # list[dict]: parsed sections (Args, Returns, etc.)
```

## Symbol Items

All identifiers in the file (variable names, functions, classes):

```python
for symbol in result["symbols"]:
    symbol["name"]         # str: identifier
    symbol["kind"]         # str: "variable", "function", "class", "parameter", etc.
    symbol["start_line"]   # int
    symbol["type_annotation"]  # str or None: type hint if present
```

## Diagnostics

Syntax errors detected by tree-sitter:

```python
for diag in result["diagnostics"]:
    diag["message"]        # str: error description
    diag["severity"]       # str: "error", "warning", "info"
    diag["start_line"]     # int
    diag["start_column"]   # int
    diag["end_line"]       # int
    diag["end_column"]     # int
```

Note: Non-empty diagnostics doesn't mean unparsable — tree-sitter recovers and continues parsing.

## Code Chunking for LLMs

Enable with `chunk_max_size` (in bytes):

```python
config = ProcessConfig(
    "python",
    structure=True,
    chunk_max_size=1000,    # ~1000 tokens per chunk
)
result = process(large_source, config)

for chunk in result["chunks"]:
    chunk["content"]        # str: source code text
    chunk["start_line"]     # int: first line (1-indexed)
    chunk["end_line"]       # int: last line
    chunk["start_byte"]     # int
    chunk["end_byte"]       # int
    chunk["metadata"]       # ChunkContext
```

ChunkContext fields:

```python
metadata = chunk["metadata"]

metadata["language"]       # str: "python"
metadata["chunk_index"]    # int: position in sequence
metadata["total_chunks"]   # int: total number of chunks
metadata["node_types"]     # list[str]: AST node types in this chunk
metadata["symbols_defined"]  # list[str]: names defined in this chunk
metadata["comments"]       # list[str]: comments in this chunk
metadata["docstrings"]     # list[str]: docstrings in this chunk
metadata["has_error_nodes"]  # bool: syntax errors in this chunk
metadata["context_path"]   # list[str]: scope path (e.g., ["MyClass", "method"])
```

### Chunking Algorithm

1. **Collect units**: Walk AST, collect top-level declarations (functions, classes) as atomic units
2. **Pack greedily**: Fit units into chunks without exceeding `chunk_max_size`
3. **Split oversized**: If a single unit exceeds budget, split at sub-boundaries (methods in class, statement blocks in function)

Guarantees:

- Functions never split unless individually too large
- Decorators/docstrings stay with function
- Classes keep method lists together where possible
- Imports grouped at top

## Custom Extraction Queries

Use arbitrary tree-sitter S-expression patterns:

```python
config = {
    "language": "python",
    "patterns": {
        "pattern_name": {
            "query": "(identifier) @name",
            "capture_output": "Text",  # or "Node", "Full"
            "child_fields": [],
            "max_results": None,       # None = unlimited
            "byte_range": None,        # None = entire file
        }
    }
}

result = extract(source, config)

for match in result["results"]["pattern_name"]["matches"]:
    match["pattern_index"]  # int: index in query
    for capture in match["captures"]:
        capture["name"]     # str: capture name from query
        capture["text"]     # str or None: matched text
        capture["node"]     # dict or None: node info
        capture["child_fields"]  # dict: extracted child field values
        capture["start_byte"]  # int
```

Example:

```python
# Extract all decorator names
extract(source, {
    "language": "python",
    "patterns": {
        "decorators": {
            "query": "(decorator (identifier) @name)",
            "capture_output": "Text",
        }
    }
})
```

## Pattern Validation

Validate patterns before expensive extraction:

```python
result = validate_extraction({
    "language": "python",
    "patterns": {
        "decorators": {
            "query": "(decorator (identifier) @name)",
        }
    }
})

if result["valid"]:
    print("Ready to extract")
else:
    for name, info in result["patterns"].items():
        if not info["valid"]:
            print(f"{name}: {', '.join(info['errors'])}")
```

## Common Extraction Patterns

### Functions with Type Hints

```python
{
    "language": "python",
    "patterns": {
        "typed_functions": {
            "query": "(function_definition name: (identifier) @fn_name return_type: (type) @type)",
            "capture_output": "Text",
        }
    }
}
```

### Class Definitions with Decorators

```python
{
    "language": "python",
    "patterns": {
        "decorated_classes": {
            "query": "((decorator (identifier) @decorator) @dec)+ (class_definition name: (identifier) @class_name)",
            "capture_output": "Full",
            "child_fields": ["decorator", "class_name"],
        }
    }
}
```

### All Function Calls

```python
{
    "language": "python",
    "patterns": {
        "function_calls": {
            "query": "(call function: (identifier) @func_name arguments: (argument_list) @args)",
            "capture_output": "Text",
            "max_results": 100,  # Limit results for large files
        }
    }
}
```

## Token Counting

Chunking uses `cl100k_base` approximation: 4 characters ≈ 1 token (matches GPT-4, Claude, Llama).

```python
chunk["token_count"]       # int: estimated tokens in this chunk
metadata["token_count"]    # int: same as above
```

The `chunk_max_size` parameter is an **upper bound**, not exact. Chunks may be smaller (natural boundaries) or slightly exceed (only split point is past limit).

## Language-Specific Structure Kinds

| Kind         | Languages                                  |
| ------------ | ------------------------------------------ |
| `function`   | All languages                              |
| `class`      | Python, JS/TS, Java, C#, Ruby, PHP, Kotlin |
| `method`     | Same as class                              |
| `interface`  | TS, Java, C#, Go, Kotlin                   |
| `struct`     | Rust, Go, C, C++, C#, Zig                  |
| `module`     | Elixir, Ruby, Rust, Go                     |
| `enum`       | Rust, Java, C#, TypeScript, Kotlin         |
| `trait`      | Rust                                       |
| `type_alias` | TypeScript, Rust                           |
| `impl`       | Rust                                       |
| `namespace`  | C#, C++, Java                              |
