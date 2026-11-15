# Attributions

This Promela grammar is a vendored, locally-maintained fork of
[`siraben/tree-sitter-promela`](https://github.com/siraben/tree-sitter-promela)
(© 2022 Ben Siraphob, MIT License — see [`LICENSE`](./LICENSE)).

It was imported at upstream commit
[`91da8f141c3c4c695eb71018c8a7b2e7ea39c167`](https://github.com/siraben/tree-sitter-promela/commit/91da8f141c3c4c695eb71018c8a7b2e7ea39c167). Upstream is effectively
unmaintained, so the grammar is now owned and maintained in-tree under
`grammars/promela/`.

## Local notes

- Generated cleanly with the current tree-sitter CLI (0.26.x); no patches
  required.

The generated `src/` artifacts are produced by `tree-sitter generate` from
`grammar.js`; the build pipeline regenerates them from source.
