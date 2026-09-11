---
name: extracting-code-structure
description: >-
  Use when the user wants structured code metadata from a source file —
  functions, classes, imports, exports, symbols, docstrings, comments, or
  syntax diagnostics. Covers `ts-pack process` feature flags, the JSON
  result shape, and the default feature set.
---

<!--
AI-RULEZ :: GENERATED FILE — DO NOT EDIT
Content-Hash: blake3:dd570b2c1cb806fd7004f737dadfefde3bef71ae1ba233bbaae8121d744b5f0b
Source-Hash: blake3:9e073d427a178d533cfbe4d3d43f29d564c0bda35ef83d901c3958931fb60dc0
Schema-Version: v1
-->

# Extracting code structure

`ts-pack process <file>` runs the code-intelligence pipeline over a source
file and prints JSON. Use it when the user wants *structured metadata* —
"list the functions", "what does this file import", "find the exported
symbols", "pull the docstrings" — rather than a raw syntax tree.

## Quick recipe

```bash
# Default features: structure + imports + exports
ts-pack process src/app.ts

# Pick features explicitly (only what you ask for is computed)
ts-pack process src/app.ts --structure --imports --symbols --docstrings

# Everything
ts-pack process src/app.ts --all
```

When no feature flags are given, `process` defaults to
`--structure --imports --exports`. Passing any single *extraction* flag
(`--structure --imports --exports --comments --docstrings --symbols
--diagnostics`) turns off that default set, so list every feature you want.
`--chunk-size` and `--language` are modifiers, not extraction flags: they do
not disable the defaults (e.g. `--chunk-size 500` alone still emits
structure+imports+exports and adds chunks).

## Feature flags

| Flag | Extracts |
| ---- | -------- |
| `--structure` | Functions, classes, methods, modules (spans, nesting, visibility). |
| `--imports` | Import statements and their sources. |
| `--exports` | Exported symbols and their kinds. |
| `--comments` | Inline and block comments. |
| `--docstrings` | Docstrings attached to definitions. |
| `--symbols` | All identifiers (for search/indexing). |
| `--diagnostics` | Syntax errors and error nodes with positions. |
| `--all` | Enable every feature above. |
| `--chunk-size <bytes>` | Syntax-aware chunks — see `chunking-for-llms`. |
| `--language <name>` (`-l`) | Override language (auto-detected from extension otherwise). |

## Result shape

`process` always prints JSON. Top-level keys:

```json
{
  "language": "python",
  "metrics": { "total_lines": 0, "code_lines": 0, "comment_lines": 0, "blank_lines": 0, "total_bytes": 0, "node_count": 0, "error_count": 0, "max_depth": 0 },
  "structure": [],
  "imports": [],
  "exports": [],
  "comments": [],
  "docstrings": [],
  "symbols": [],
  "diagnostics": [],
  "chunks": []
}
```

`metrics` is populated whenever any processing runs. `chunks` appears only
when `--chunk-size` is set. Empty collections are omitted from the JSON
(the `skip_serializing_if` wire contract), so absent keys mean "nothing
found", not an error.

### Structure items

Each `structure` entry has `kind` (a capitalized string —
`"Function"`, `"Class"`, `"Method"`, `"Module"`, `"Struct"`, `"Trait"`,
…; language-specific kinds serialize as `{"Other": "<kind>"}`), `name`,
`visibility`, and a nested `span` object with `start_byte`/`end_byte` and
**zero-indexed** `start_line`/`start_column`/`end_line`/`end_column`.
Nesting is expressed via `children` (e.g. methods inside a class), not a
`parent` pointer. Optional fields: `decorators`, `doc_comment` (the
attached doc comment), `signature`, and `body_span`.

## Examples

```bash
# Function and class names with line numbers
ts-pack process src/service.py --structure \
  | jq '.structure[] | {kind, name, line: .span.start_line}'

# Import sources only
ts-pack process src/app.ts --imports \
  | jq '.imports[].source'

# Syntax error count (also available via metrics.error_count)
ts-pack process broken.go --diagnostics \
  | jq '.diagnostics | length'

# Build a symbol index across a tree of files
for f in $(git ls-files '*.rs'); do
  ts-pack process "$f" --symbols | jq -c --arg f "$f" '{file: $f, symbols: .symbols}'
done
```

## SDK equivalent

```python
from tree_sitter_language_pack import process, ProcessConfig

# ProcessConfig is a frozen dataclass — set fields in the constructor.
# structure/imports/exports default to True; add the rest you want.
config = ProcessConfig("python", symbols=True, docstrings=True)
result = process(source_code, config)
for item in result.structure:                 # ProcessResult is an object, not a dict
    print(item.kind, item.name, item.span.start_line)
```

The SDK also exposes the parsed tree (`get_parser(...)` → `Parser`/`Tree`/
`Node`) for running your own tree-sitter queries, and a `data_extraction`
config flag that pulls a hierarchical key/value tree from data-format files
(JSON, YAML, TOML, …) into the result's `data` field — neither is exposed
on the CLI.

## When to reach for parse instead

If the user wants the raw syntax tree rather than extracted metadata, use
`ts-pack parse` — see the `parsing-source` skill.
