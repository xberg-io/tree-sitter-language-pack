# Attributions

This Yul grammar is a vendored, locally-maintained fork of
[`emilbayes/tree-sitter-yul`](https://github.com/emilbayes/tree-sitter-yul)
(© 2020 Emil Bay, ISC License — see [`LICENSE`](./LICENSE)).

It was imported at upstream commit
[`08af8a3d1621794f9ca7feadc5bfcb49081281e9`](https://github.com/emilbayes/tree-sitter-yul/commit/08af8a3d1621794f9ca7feadc5bfcb49081281e9). Upstream is effectively
unmaintained, so the grammar is now owned and maintained in-tree under
`grammars/yul/`.

## Local notes

- Generated cleanly with the current tree-sitter CLI (0.26.x); no patches
  required.

The generated `src/` artifacts are produced by `tree-sitter generate` from
`grammar.js`; the build pipeline regenerates them from source.
