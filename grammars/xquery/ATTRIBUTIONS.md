# Attributions

This XQuery grammar is a vendored, locally-maintained fork of
[`grantmacken/tree-sitter-xquery`](https://github.com/grantmacken/tree-sitter-xquery)
(© 2021 Stanislav Chernov, MIT License — see [`LICENSE`](./LICENSE)).

It was imported at upstream commit
[`85f15e8f6388981678838407c25f9222a7abe14b`](https://github.com/grantmacken/tree-sitter-xquery/commit/85f15e8f6388981678838407c25f9222a7abe14b). Upstream is effectively
unmaintained, so the grammar is now owned and maintained in-tree under
`grammars/xquery/`.

## Local modifications

- **Removed the top-level `reserved` property** — upstream declared
  `reserved` as an array of XQuery keywords. The modern tree-sitter CLI
  (0.26.x) requires `reserved` to be an object and would otherwise begin
  reserving those keywords, changing parse behaviour. XQuery keywords are
  **not** reserved (any lexical QName may duplicate one, as upstream's own
  header comment states), so the property was non-functional under the old
  CLI that generated the shipped parser. Removing it keeps the regenerated
  parser faithful to upstream's behaviour while building cleanly on 0.26.x.

`src/` is generated from `grammar.js` by the build pipeline
(`"generate": true`).
