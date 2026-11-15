# Attributions

This PlantUML grammar is a vendored, locally-maintained fork of
[`Decodetalkers/tree_sitter_plantuml`](https://github.com/Decodetalkers/tree_sitter_plantuml)
(© 2018 Benjamin Gray, MIT License — see [`LICENSE`](./LICENSE)).

It was imported at upstream commit
[`c7361a1d481dc1ff6700b14ea1d5efc549b72713`](https://github.com/Decodetalkers/tree_sitter_plantuml/commit/c7361a1d481dc1ff6700b14ea1d5efc549b72713). Upstream is effectively
unmaintained, so the grammar is now owned and maintained in-tree under
`grammars/plantuml/`.

## Local notes

- `grammar.js` requires `./grammer/baseuml.js`, `./grammer/jsonuml.js`, and
  `./grammer/yamluml.js` (the upstream directory name is `grammer`, not
  `grammar` — kept as-is to match the `require()` paths). These helper files
  are vendored alongside `grammar.js`.

## Local modifications

- **`escape_sequence` regex** — upstream's `grammar.js` wrote the
  hex-escape rule as `/u{[0-9a-fA-F]+}/`, whose unescaped `{` the modern
  tree-sitter CLI (0.26.x) rejects as an invalid repetition quantifier.
  Escaped the braces to `/u\{[0-9a-fA-F]+\}/` — semantically identical (the
  braces were always intended as literals) but accepted by the current CLI.

Like the other vendored grammars, `src/` is generated from `grammar.js` by
the build pipeline (`"generate": true`); only `grammar.js` and the
`grammer/` helpers are the source of truth.
