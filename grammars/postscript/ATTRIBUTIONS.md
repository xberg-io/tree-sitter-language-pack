# Attributions

This PostScript grammar is a vendored, locally-maintained fork of
[`smoeding/tree-sitter-postscript`](https://github.com/smoeding/tree-sitter-postscript)
(© 2024 Stefan Möding, BSD 2-Clause License — see [`LICENSE`](./LICENSE)).

It was imported at upstream commit
[`d352ed652a2b39cfa1567c1b77a44e275399bacf`](https://github.com/smoeding/tree-sitter-postscript/commit/d352ed652a2b39cfa1567c1b77a44e275399bacf).
It is maintained in-tree under `grammars/postscript/` because the change below
alters the tree shape that upstream's consumers rely on.

## Local notes

- Upstream ships an external scanner (`src/scanner.c`), vendored without
  functional changes. `scanner.c` is hand-written source (not generated), so
  it is tracked in git even though the rest of `src/` is not.
- `grammar.js` and `src/scanner.c` are reformatted with the repository
  formatter (`poly fmt`); apart from that and the change below they are as
  imported.

## Local modifications

- **`literal` has a `name` child** (`grammar.js`). Upstream lexed a literal
  name such as `/drawbox` as a single token, so the only text available for a
  definition was `/drawbox` while its uses are the bare operator `drawbox`.
  `literal` is now the leading `/` (or `//`) followed by an immediate token
  aliased to `name` (field `name`), so tags and other queries can capture the
  name without the slash. The `literal` node keeps its full span and text.
  Against upstream's `test/corpus`, the only changes are `(literal)` becoming
  `(literal (name))`.

`src/parser.c` is regenerated from `grammar.js` by the build pipeline
(`"generate": true`); the hand-written `src/scanner.c` is the compiled source.
