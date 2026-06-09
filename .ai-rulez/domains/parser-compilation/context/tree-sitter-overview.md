---
priority: high
---

# Tree-Sitter Overview

Tree-sitter is an incremental parsing library that builds concrete syntax trees for source code.

Key concepts:

- **Grammar**: A JSON/JS definition describing a language's syntax
- **Parser**: C code generated from the grammar by `tree-sitter generate`
- **External Scanner**: Optional C/C++ code for context-sensitive parsing (e.g., indentation)
- **ABI Version**: Binary interface version for runtime compatibility
- **Language Pack**: A collection of pre-compiled parsers bundled for multi-language use

This project compiles 306 grammars into a single Rust crate with bindings for Python, Node.js, Ruby, Go, Java, Elixir, and WebAssembly.
