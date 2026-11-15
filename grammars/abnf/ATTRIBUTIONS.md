# Attributions

This ABNF grammar is a vendored, locally-maintained fork of
[`jmitchell/tree-sitter-abnf`](https://github.com/jmitchell/tree-sitter-abnf)
(© 2019 Jacob Mitchell, MIT License — see [`LICENSE`](./LICENSE)).

It was imported at upstream commit
[`f68bb6e3cfcc2a3eebf11f47e27efffb128c001f`](https://github.com/jmitchell/tree-sitter-abnf/commit/f68bb6e3cfcc2a3eebf11f47e27efffb128c001f). Upstream is effectively
unmaintained, so the grammar is now owned and maintained in-tree under
`grammars/abnf/`.

## Local notes

- Generated cleanly with the current tree-sitter CLI (0.26.x); no patches
  required.

The generated `src/` artifacts are produced by `tree-sitter generate` from
`grammar.js`; the build pipeline regenerates them from source.
