# Attributions

This Motoko grammar is a vendored, locally-maintained fork of
[`polychromatist/tree-sitter-motoko`](https://github.com/polychromatist/tree-sitter-motoko)
(© polychromatist, Apache License 2.0 — see [`LICENSE`](./LICENSE)).

It was imported at upstream commit
[`a172fd1ddc09b9c3806c0d6ecd71ee2f5160aebc`](https://github.com/polychromatist/tree-sitter-motoko/commit/a172fd1ddc09b9c3806c0d6ecd71ee2f5160aebc). Upstream is effectively
unmaintained, so the grammar is now owned and maintained in-tree under
`grammars/motoko/`.

## Local notes

- `queries/highlights.scm` is upstream's `helix-queries/highlights.scm`.
- `grammar.js`, `src/scanner.c` and `queries/highlights.scm` are converted from
  CRLF to LF line endings where upstream used CRLF and reformatted with the
  repository formatter (`poly fmt`); apart from that and the change below they
  are as imported.
- `src/scanner.c` is hand-written source (not generated), so it is tracked in
  git even though the rest of `src/` is not.

## Local modifications

- **Spaced binary operators moved to the external scanner** (`grammar.js`,
  `src/scanner.c`). Upstream lexes the binary `>`, `<` and `>>` as the tokens
  `" > "`, `" < "` and `" >>"`, which is how Motoko tells them apart from type
  argument brackets. A token that begins with a space stops the generated
  lexer from skipping spaces before any token, so names carried the
  preceding space (`actor Main` gave the name `" Main"`; upstream's README
  notes this). The three tokens are now the externals `_spaced_gt`,
  `_spaced_lt` and `_spaced_shr`, aliased to the original `" > "`, `" < "` and
  `" >>"` node types so queries are unchanged. The scanner matches them only
  after skipped whitespace, requires whitespace after `>` and `<`, and leaves
  `>>=` to the grammar. Any whitespace counts where upstream required a
  single space. Parse trees of upstream's `smoke/*.mo` files are unchanged
  apart from node ranges.

`src/parser.c` is regenerated from `grammar.js` by the build pipeline
(`"generate": true`); the hand-written `src/scanner.c` (with the change above)
is the compiled source.
