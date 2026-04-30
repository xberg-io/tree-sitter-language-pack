---
title: Extraction queries
description: "Run custom tree-sitter queries against parsed code and get structured results with text, position, and child fields."
---

Extraction queries let you run arbitrary [tree-sitter S-expression queries](https://tree-sitter.github.io/tree-sitter/using-parsers/queries/1-syntax.html) against source code. Each match returns captured text, node metadata, and optionally the text of named child fields.

Use `extract()` when the built-in fields in `process()` don't cover your use case — finding all calls to a specific function, listing decorator names, extracting test method names matching a pattern. The two can also run together: `ProcessConfig.extractions` runs custom patterns alongside the standard analysis pass.

## Basic usage

The simplest case: find all function names.

=== "Python"

    ```python
    from tree_sitter_language_pack import extract

    source = "def hello(): pass\ndef world(): pass\n"

    result = extract(source, {
        "language": "python",
        "patterns": {
            "functions": {
                "query": "(function_definition name: (identifier) @fn_name)",
            }
        }
    })

    for match in result["functions"]["matches"]:
        for capture in match["captures"]:
            print(capture["text"])
    # hello
    # world
    ```

=== "Node.js"

    ```typescript
    import { extract } from "@kreuzberg/tree-sitter-language-pack";

    const source = "def hello(): pass\ndef world(): pass\n";

    const result = extract(source, {
      language: "python",
      patterns: {
        functions: {
          query: "(function_definition name: (identifier) @fn_name)",
        },
      },
    });

    for (const match of result.functions.matches) {
      for (const capture of match.captures) {
        console.log(capture.text);
      }
    }
    ```

=== "Rust"

    ```rust
    use tree_sitter_language_pack::{ExtractionConfig, ExtractionPattern, extract_patterns};
    use ahash::AHashMap;

    let mut patterns = AHashMap::new();
    patterns.insert("functions".to_string(), ExtractionPattern {
        query: "(function_definition name: (identifier) @fn_name)".to_string(),
        ..Default::default()
    });

    let result = extract_patterns(
        "def hello(): pass\ndef world(): pass\n",
        &ExtractionConfig { language: "python".to_string(), patterns },
    )?;

    assert_eq!(result.results["functions"].total_count, 2);
    ```

## Extracting child fields

When you capture a parent node, `child_fields` pulls the text of its named children without needing to write extra captures:

```python
result = extract("def greet(name): pass\n", {
    "language": "python",
    "patterns": {
        "functions": {
            "query": "(function_definition) @fn_def",
            "child_fields": ["name", "parameters"],
        }
    }
})

capture = result["functions"]["matches"][0]["captures"][0]
print(capture["child_fields"]["name"])        # "greet"
print(capture["child_fields"]["parameters"])  # "(name)"
```

Field names depend on the grammar. Run `ts-pack parse file.py` to see the sexp with field labels, or check the grammar's `node-types.json`.

## Capture output modes

`capture_output` controls how much data each match returns:

| Mode | `text` | `node` | Use when |
|------|--------|--------|----------|
| `"Text"` (default for most cases) | present | null | You only need the matched text |
| `"Node"` | null | present | You only need position/type info |
| `"Full"` | present | present | You need both |

The `node` field contains `type`, `start_byte`, `end_byte`, `start_point`, and `end_point`.

```python
result = extract(source, {
    "language": "python",
    "patterns": {
        "names": {
            "query": "(function_definition name: (identifier) @fn_name)",
            "capture_output": "Text",
        }
    }
})

capture = result["names"]["matches"][0]["captures"][0]
print(capture["text"])  # "hello"
print(capture["node"])  # None
```

## Limiting results

`max_results` caps the `matches` list. `total_count` always shows the true match count, so you can detect truncation:

```python
result = extract(source, {
    "language": "python",
    "patterns": {
        "fns": {
            "query": "(function_definition name: (identifier) @fn_name)",
            "max_results": 5,
        }
    }
})

pattern = result["fns"]
print(len(pattern["matches"]))  # at most 5
print(pattern["total_count"])   # actual count, e.g. 42
```

## Restricting to a byte range

`byte_range` limits extraction to a `[start, end]` byte offset range. Matches return when the root node falls within the range:

```python
source = "def a(): pass\ndef b(): pass\ndef c(): pass\n"

result = extract(source, {
    "language": "python",
    "patterns": {
        "fns": {
            "query": "(function_definition name: (identifier) @fn_name)",
            "byte_range": [14, 28],
        }
    }
})

# Only "b" falls in bytes 14-28
print(result["fns"]["matches"][0]["captures"][0]["text"])  # "b"
```

## Validating queries

`validate_extraction()` checks query syntax without running extraction. Use it to catch mistakes before running extractions:

```python
from tree_sitter_language_pack import validate_extraction

result = validate_extraction({
    "language": "python",
    "patterns": {
        "good": {
            "query": "(function_definition name: (identifier) @fn_name)",
        },
        "bad": {
            "query": "((((not valid syntax",
        }
    }
})

print(result["valid"])                    # False
print(result["patterns"]["good"]["valid"])  # True
print(result["patterns"]["good"]["capture_names"])  # ["fn_name"]
print(result["patterns"]["bad"]["errors"])          # ["<syntax error>"]
```

Each pattern result has:

| Field | Type | Description |
|-------|------|-------------|
| `valid` | bool | Whether the query compiled |
| `capture_names` | list[str] | Capture names in the query |
| `pattern_count` | int | Number of patterns |
| `warnings` | list[str] | Non-fatal warnings |
| `errors` | list[str] | Fatal errors |

## Combined with process()

`ProcessConfig.extractions` runs custom patterns alongside the standard analysis pass. Results appear in `result["extractions"]`, keyed by pattern name:

```python
from tree_sitter_language_pack import process, ProcessConfig

result = process(source, ProcessConfig(
    language="python",
    structure=True,
    extractions={
        "decorators": {
            "query": "(decorator) @dec",
            "capture_output": "Text",
        }
    }
))

print(result["structure"])                              # standard results
print(result["extractions"]["decorators"]["matches"])   # custom results
```

## Compiled extraction (Rust)

`CompiledExtraction` pre-compiles queries once for reuse across inputs — useful when processing files with the same patterns:

```rust
use tree_sitter_language_pack::CompiledExtraction;

let compiled = CompiledExtraction::compile(&config)?;

// Reuse across many files
let r1 = compiled.extract("def a(): pass\n")?;
let r2 = compiled.extract("def x(): pass\ndef y(): pass\n")?;

assert_eq!(r1.results["fns"].total_count, 1);
assert_eq!(r2.results["fns"].total_count, 2);
```

`CompiledExtraction` is `Send + Sync`. To skip re-parsing when you already have a tree:

```rust
let tree = parse_string("python", source.as_bytes())?;
let result = compiled.extract_from_tree(&tree, source.as_bytes())?;
```

## Binding support

| Binding | `extract()` | `validate_extraction()` |
|---------|:-----------:|:-----------------------:|
| Python | yes | yes |
| Node.js | yes | yes |
| Rust | yes | yes |
| Ruby | yes | yes |
| Elixir | yes | yes |
| PHP | yes | yes |
| Wasm | yes | yes |
| C FFI | yes | yes |
| Go | not yet | not yet |
| C# | not yet | not yet |
| Java | not yet | not yet |

## Next steps

- [Code intelligence](intelligence.md) — built-in extraction for common patterns (structure, imports, exports)
- [Parsing code](parsing.md) — understanding the syntax tree your queries run against
