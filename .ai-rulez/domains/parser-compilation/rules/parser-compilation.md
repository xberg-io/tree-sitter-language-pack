---
type: Decision
title: Parser Compilation
paths:
  - "crates/ts-pack-core/build.rs"
  - "crates/ts-pack-core/build_support/**"
  - "grammars/**"
  - "patches/**"
  - "sources/**"
x-ai-rulez:
  kind: rule
  id: parser-compilation
  domain: parser-compilation
  metadata:
    priority: high
---

- The pack compiles 372 tree-sitter grammars into one Rust crate; `crates/ts-pack-core/build.rs` and `build_support/` compile the C sources with the `cc` crate.
- Grammar-to-C uses `tree-sitter generate`; validate the `parser.c` output before inclusion.
- Pin each grammar to a commit or tag; ABI target is version 14.
- Compile `scanner.c` (external scanner) when a grammar has one; include only the C sources needed, to keep binary size down.
- Each parser exports uniquely prefixed C symbols; avoid collisions between grammars and keep symbol visibility correct.
- Handle allocation and deallocation correctly across the FFI boundary; test parser loading and unloading for leaks.
- Builds and compiled libraries (.so/.dylib/.dll) must work on Linux, macOS and Windows, including cross-compilation.
- Keep compiled parser artifacts out of version control; regenerate in CI.
