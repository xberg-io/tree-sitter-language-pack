# Attributions

This GraphQL grammar is a vendored, locally-maintained fork of
[`bkegley/tree-sitter-graphql`](https://github.com/bkegley/tree-sitter-graphql)
(© 2021 bkegley, MIT License — see [`LICENSE`](./LICENSE)).

It was imported at upstream commit
[`5e66e961eee421786bdda8495ed1db045e06b5fe`](https://github.com/bkegley/tree-sitter-graphql/commit/5e66e961eee421786bdda8495ed1db045e06b5fe). Upstream is effectively unmaintained,
so the grammar is now owned and maintained in-tree under `grammars/graphql/`.

## Local modifications

- **`field` directives fix** — the `field` rule accepted only a single optional
  `directive`, so a field with more than one directive failed to parse
  (`root.hasError()` was `true`). Changed `optional($.directive)` to
  `optional($.directives)` so a field accepts a directives list, matching the
  GraphQL specification and the grammar's other 22 directive-bearing rules.
  See <https://github.com/xberg-io/tree-sitter-language-pack/issues/172>.
- Added `test/corpus/directives.txt` covering single, multiple, and repeated
  field directives.

The generated `src/` artifacts are produced by `tree-sitter generate` from
`grammar.js`; the build pipeline regenerates them from source.
