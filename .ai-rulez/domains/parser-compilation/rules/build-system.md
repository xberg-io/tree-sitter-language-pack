---
priority: high
---

# Build System

- Use build.rs for compiling tree-sitter C sources into the Rust crate.
- Configure cc crate with appropriate optimization flags per grammar.
- Support cross-compilation for all target platforms (Linux, macOS, Windows).
- Handle scanner.c (external scanner) compilation when grammars require it.
- Include only necessary C source files to minimize binary size.
