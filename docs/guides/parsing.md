---
description: "Parse any of 306 languages into a concrete syntax tree with tree-sitter-language-pack."
---

# Parsing code

`parse_string` turns source code into a concrete syntax tree. The tree contains every token in the file — identifiers, operators, keywords — as a navigable node you can inspect, traverse, and query.

=== "Python"

    ```python
    from tree_sitter_language_pack import parse_string

    tree = parse_string("""
    def greet(name: str) -> str:
        return f"Hello {name}"
    """, "python")

    root = tree.root_node
    print(root.sexp())
    ```

    Output:
    ```
    (module
      (function_definition
        name: (identifier)
        parameters: (parameters (identifier))
        return_type: (type (identifier))
        body: (block (return_statement (string)))))
    ```

    ```python
    # Access the function name directly via field navigation
    func = root.child(0)
    name_node = func.child_by_field_name("name")
    print(name_node.text.decode())  # "greet"
    ```

=== "Node.js"

    ```typescript
    import { parseString } from "@kreuzberg/tree-sitter-language-pack";

    const tree = await parseString(`
    function greet(name) {
      return \`Hello \${name}\`;
    }
    `, "javascript");

    const root = tree.rootNode;
    console.log(root.toString());
    // (program (function_declaration name: (identifier) ...))
    ```

=== "CLI"

    ```bash
    # Print the syntax tree for a file
    ts-pack parse main.py

    # Output as JSON
    ts-pack parse main.py --format json
    ```

## Getting a parser

Three entry points — they differ in how much setup you control:

```python
from tree_sitter_language_pack import parse_string, get_parser, get_language

# One call for everything — good for scripts and one-off parses
tree = parse_string("x = 1", "python")

# Reusable parser — more efficient when parsing many files of the same language
parser = get_parser("python")
tree = parser.parse(b"x = 1")

# Raw language object — for direct tree-sitter integration
lang = get_language("python")
```

For batch processing, reuse the parser object. Creating one per parse works fine at small scale but adds overhead when processing thousands of files.

!!! Tip "Language names" Names are case-insensitive. Aliases exist: `shell` → `bash`, `makefile` → `make`, `bazel` → `starlark`. See [Languages](../languages.md) for the full list.

## The syntax tree

### Root node

Every parse returns a `Tree` with a single `root_node`. Its `kind` is the top-level grammar node for the language: `module` for Python, `program` for JavaScript, `source_file` for Rust and Go.

```python
tree = parse_string("def foo(): pass\ndef bar(): pass", "python")
root = tree.root_node

print(root.kind)          # "module"
print(root.start_point)   # (0, 0)  — (line, column), zero-indexed
print(root.end_point)     # (1, 14)
print(root.child_count)   # 2
print(root.has_error)     # False
```

### Field names

Grammars assign named fields to semantically meaningful children. A Python `function_definition` has `name`, `parameters`, `return_type`, and `body`. Use `child_by_field_name` to reach them directly:

```python
tree = parse_string("def add(a, b):\n    return a + b", "python")
func = tree.root_node.child(0)

name   = func.child_by_field_name("name")
params = func.child_by_field_name("parameters")
body   = func.child_by_field_name("body")

print(name.text.decode())    # "add"
print(params.text.decode())  # "(a, b)"
```

Field names are grammar-specific. To discover them, run `ts-pack parse file.py` and read the labelled sexp output — field names appear as `name:`, `parameters:`, `body:` before each child.

### Named vs. anonymous nodes

Named nodes carry semantic meaning (`identifier`, `call_expression`, `string`). Anonymous nodes are punctuation and keywords (`(`, `)`, `def`, `:`). `node.children` returns both; `node.named_children` skips anonymous nodes.

```python
for child in tree.root_node.named_children:
    print(child.kind, child.text.decode())
```

### Node text

Tree-sitter returns text as bytes. Decode with `utf-8` unless you're working with non-UTF-8 source:

```python
tree = parse_string("x = 42 + y", "python")
stmt = tree.root_node.child(0)
print(stmt.text.decode())  # "x = 42 + y"
```

## Traversing the tree

A recursive generator visits every node:

```python
def walk(node):
    yield node
    for child in node.children:
        yield from walk(child)

# Find all identifiers in the file
ids = [n for n in walk(tree.root_node) if n.kind == "identifier"]
print(f"{len(ids)} identifiers")
```

For structured extraction — functions, classes, imports, docstrings — use [`process()`](intelligence.md). It runs built-in queries and returns the results as clean data structures so you don't have to write your own walker.

## Syntax errors

Tree-sitter never raises on malformed syntax. It marks problem areas with `ERROR` or `MISSING` nodes and keeps parsing.

```python
tree = parse_string("def broken(", "python")
print(tree.root_node.has_error)  # True

def find_errors(node):
    if node.kind in ("ERROR", "MISSING"):
        line, col = node.start_point
        print(f"Error at line {line}, col {col}: {node.text.decode()!r}")
    for child in node.children:
        find_errors(child)

find_errors(tree.root_node)
```

`has_error` on the root is a fast way to check whether any errors exist before walking. For structured error data, set `diagnostics=True` in `ProcessConfig` instead.

## Node properties

| Property | Type | Description |
|----------|------|-------------|
| `kind` | str | Grammar node type, for example `function_definition` |
| `start_point` | (int, int) | `(line, column)`, zero-indexed |
| `end_point` | (int, int) | `(line, column)`, zero-indexed |
| `start_byte` | int | Byte offset in source |
| `end_byte` | int | Byte offset in source |
| `child_count` | int | All children including anonymous nodes |
| `named_child_count` | int | Named children only |
| `text` | bytes | Raw source text for this node |
| `has_error` | bool | True if any error nodes exist in this subtree |
| `is_named` | bool | False for anonymous nodes like `(` or `def` |
| `parent` | Node? | Enclosing node, or None for the root |

## Next steps

- [Code intelligence](intelligence.md) — extract functions, imports, docstrings, and symbols without writing a tree walker
- [Extraction queries](extraction.md) — run custom tree-sitter S-expression queries for language-specific patterns
- [Languages](../languages.md) — all 306 supported languages and their aliases
