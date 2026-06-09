---
title: Extraction queries
description: "Custom extraction queries are not part of the v1.8 public API — use process() or call tree-sitter queries directly via get_parser()."
---

## Extraction queries

Custom query extraction helpers are internal in v1.8 and are not exported by the Rust crate or generated language bindings.

Use [`process()`](intelligence.md) for supported code intelligence fields such as structure, imports, exports, comments, docstrings, symbols, diagnostics, metrics, and chunks. If you need raw tree-sitter queries in Rust, use [`get_parser()`](parsing.md) and the `tree-sitter` query APIs directly.

### Next steps

- [Code intelligence](intelligence.md) — built-in extraction for common patterns
- [Parsing code](parsing.md) — raw syntax trees and low-level node traversal
