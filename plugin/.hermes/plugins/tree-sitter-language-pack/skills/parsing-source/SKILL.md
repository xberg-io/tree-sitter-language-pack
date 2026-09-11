---
name: parsing-source
description: >-
  Use when the user wants a tree-sitter syntax tree for a source file —
  an s-expression dump or JSON tree. Covers `ts-pack parse`, language
  auto-detection vs `--language`, stdin input, and reading `has_errors`.
---

<!--
AI-RULEZ :: GENERATED FILE — DO NOT EDIT
Content-Hash: blake3:9033e2c6803385f116531b227b5f1274b5ec6fc035cd2ebf562f5eaf1f1c17fb
Source-Hash: blake3:9e073d427a178d533cfbe4d3d43f29d564c0bda35ef83d901c3958931fb60dc0
Schema-Version: v1
-->

# Parsing source into a syntax tree

`ts-pack parse <file>` parses one source file with the matching
tree-sitter grammar and prints the resulting concrete syntax tree. Use it
when the user wants the tree itself — to inspect node structure, debug a
grammar, or feed an s-expression to another tool.

## Quick recipe

```bash
# S-expression (default) — auto-detects language from the file extension
ts-pack parse src/main.rs

# JSON form: { "language", "sexp", "has_errors" }
ts-pack parse src/main.rs --format json
```

## Flag surface

| Flag | Short | Default | Purpose |
| ---- | ----- | ------- | ------- |
| `--language <name>` | `-l` | auto | Override the language (skip extension detection). |
| `--format <fmt>` | `-f` | `sexp` | `sexp` or `json`. |

The parser library for the language downloads on first use and is cached.

## Language detection

The language is auto-detected from the file extension. Override it when the
extension is missing, misleading, or you are reading from stdin:

```bash
# Misleading extension or no extension:
ts-pack parse script --language bash

# stdin (use "-" as the file):
cat snippet.py | ts-pack parse - --language python
```

stdin cannot be auto-detected by path; always pass `--language` with `-`.
For content-based detection without a path, use the SDK
(`detect_language_from_content`) — see the `detecting-languages` skill.

## Output shapes

### S-expression

A nested `(node_type child ...)` tree. Compact, good for eyeballing
structure or piping to a grammar tool.

### JSON

```json
{
  "language": "rust",
  "sexp": "(source_file (function_item ...))",
  "has_errors": false
}
```

Read `has_errors` to know whether the parse produced any error nodes — a
fast syntax check. For positioned, per-error detail use
`ts-pack process <file> --diagnostics` (see `extracting-code-structure`).

## Examples

```bash
# Inspect a TypeScript file's tree
ts-pack parse src/app.ts

# Machine-readable, then check for syntax errors with jq
ts-pack parse src/app.ts --format json | jq '.has_errors'

# Parse a heredoc as Python
printf 'def f():\n    return 1\n' | ts-pack parse - -l python
```

## When to reach for process instead

`parse` gives you the raw tree. If the user wants structured metadata
(functions, imports, exports, symbols, docstrings) rather than the tree,
use `ts-pack process` — see the `extracting-code-structure` skill.
