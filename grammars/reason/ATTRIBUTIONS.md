# Attributions

This Reason grammar is a vendored, locally-maintained fork of
[`danielo515/tree-sitter-reason`](https://github.com/danielo515/tree-sitter-reason)
(© 2021 Victor Nakoryakov, MIT License — see [`LICENSE`](./LICENSE)).

It was imported at upstream commit
[`936958c8c3b2d76cbeb4dffcfe5a8f929c958e7a`](https://github.com/danielo515/tree-sitter-reason/commit/936958c8c3b2d76cbeb4dffcfe5a8f929c958e7a). Upstream is effectively
unmaintained, so the grammar is now owned and maintained in-tree under
`grammars/reason/`.

## Local notes

- Upstream ships an external scanner (`src/scanner.c`), vendored unchanged
  except for the fix noted below. `scanner.c` is hand-written source (not
  generated), so it is tracked in git even though the rest of `src/` is not.

## Local modifications

- **`grammar.js` `escape_sequence` regex** — upstream wrote the hex-escape
  rule as `/u{[0-9a-fA-F]+}/`, whose unescaped `{` the modern tree-sitter
  CLI (0.26.x) rejects as an invalid repetition quantifier. Escaped the
  braces to `/u\{[0-9a-fA-F]+\}/` — semantically identical (literal braces)
  but accepted by the current CLI.
- **`scanner.c` implicit-int fix** — upstream's
  `const in_string = state->in_quotes || state->in_backticks;` omits a type
  specifier (implicit `int`, valid under pre-C99 rules). Modern clang
  rejects this under C99+ (`-Wimplicit-int`). Changed to
  `const bool in_string = ...` with a `// ~keep` comment marking the
  divergence. This is the only intentional difference from upstream
  `src/scanner.c`.

With the `escape_sequence` patch, `src/parser.c` is regenerated from
`grammar.js` by the build pipeline (`"generate": true`); the hand-written
`src/scanner.c` (with the fix above) is the compiled source.
