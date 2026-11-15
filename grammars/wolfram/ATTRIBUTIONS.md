# Attributions

This Wolfram Language grammar is a vendored, locally-maintained fork of
[`LumaKernel/tree-sitter-wolfram`](https://github.com/LumaKernel/tree-sitter-wolfram)
(© 2021 Luma, MIT License — see [`LICENSE`](./LICENSE)).

It was imported at upstream commit
[`ab3506a5b49b7d76a8ed06958d0b2b7be91a5d34`](https://github.com/LumaKernel/tree-sitter-wolfram/commit/ab3506a5b49b7d76a8ed06958d0b2b7be91a5d34). Upstream is effectively
unmaintained, so the grammar is now owned and maintained in-tree under
`grammars/wolfram/`.

## Local notes

- Generated cleanly with the current tree-sitter CLI (0.26.x); no patches
  required. `tree-sitter generate` emits benign "unnecessary conflicts"
  warnings from the upstream grammar's declared conflict list — these do not
  affect parser correctness.

The generated `src/` artifacts are produced by `tree-sitter generate` from
`grammar.js`; the build pipeline regenerates them from source.
