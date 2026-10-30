---
title: "Decision records"
description: "Architecture decisions recorded as MADR records under adrs/ in the tree-sitter-language-pack repository."
---

Lasting decisions about tree-sitter-language-pack are recorded as [MADR](https://adr.github.io/madr/) records under
[`adrs/`](https://github.com/xberg-io/tree-sitter-language-pack/tree/main/adrs), checked by
[adrlint](https://github.com/Goldziher/adrlint). An accepted record is never edited: a change is a new record that
supersedes or amends it. Organization-wide decisions live in the
[agent-conventions log](https://github.com/xberg-io/agent-conventions/tree/main/adrs). Add a row here in the same
change as a new record.

| ADR | Title | Status | Date |
| --- | ----- | ------ | ---- |
| [0000](https://github.com/xberg-io/tree-sitter-language-pack/blob/main/adrs/0000-record-architecture-decisions.md) | Record architecture decisions | accepted | 2026-10-30 |
| [0001](https://github.com/xberg-io/tree-sitter-language-pack/blob/main/adrs/0001-native-packages-download-parser-bundles-at-runtime.md) | Native packages download parser bundles at runtime | accepted | 2026-03-10 |
| [0002](https://github.com/xberg-io/tree-sitter-language-pack/blob/main/adrs/0002-build-time-grammar-selection-through-tslp-languages.md) | Build-time grammar selection through TSLP_LANGUAGES | accepted | 2026-04-14 |
| [0003](https://github.com/xberg-io/tree-sitter-language-pack/blob/main/adrs/0003-curated-static-grammar-subset-for-webassembly.md) | Curated static grammar subset for WebAssembly | accepted | 2026-03-22 |
| [0004](https://github.com/xberg-io/tree-sitter-language-pack/blob/main/adrs/0004-generate-grammars-at-tree-sitter-abi-14.md) | Generate grammars at tree-sitter ABI 14 | accepted | 2026-06-19 |
| [0005](https://github.com/xberg-io/tree-sitter-language-pack/blob/main/adrs/0005-permissively-licensed-grammars-only.md) | Permissively licensed grammars only | accepted | 2026-03-24 |
| [0006](https://github.com/xberg-io/tree-sitter-language-pack/blob/main/adrs/0006-grammar-pins-validated-as-commit-shas-on-allowlisted-forges.md) | Grammar pins validated as commit SHAs on allowlisted forges | accepted | 2026-08-12 |
