---
name: grammar-engineer
description: Tree-sitter grammar compilation and language pack management
model: haiku
---

When working on tree-sitter-language-pack:

- Core library in `crates/ts-pack-core/src/` manages grammar compilation, language registry, and parser operations. Key modules: `registry.rs` (language registry and lookup), `definitions.rs` (grammar definitions), `config.rs` / `pack_config.rs` / `process_config.rs` (build and runtime configuration), `download.rs` (grammar source fetching).
- Parser operations: `parse.rs` (source code parsing), `extract.rs` (AST extraction), `node.rs` (tree-sitter node handling), `queries.rs` / `query.rs` (tree-sitter query support), `text_splitter.rs` (code splitting).
- Grammar sources live in `sources/` and compiled parsers in `parsers/`. The `build/` directory contains build-time compilation infrastructure.
- Language intelligence features in `crates/ts-pack-core/src/intel/` provide higher-level code analysis.
- JSON utilities in `json_utils.rs` and extension mapping in `extensions.rs`.
- Bindings: Python (`ts-pack-python`), Node.js (`ts-pack-node`), Ruby (`ts-pack-ruby`), PHP (`ts-pack-php`), Go/Java/C# via FFI (`ts-pack-ffi`), Java (`ts-pack-java`), Elixir (`ts-pack-elixir`), WASM (`ts-pack-wasm`).
- CLI entry point in `ts-pack-cli`.
- E2e test fixtures in `fixtures/` drive cross-language test generation via the e2e generator in `tools/`.
- Domain rules in `.ai-rulez/domains/` cover `parser-compilation` and `language-management` -- these are the authoritative references for grammar compilation patterns and C ABI safety.
- When adding a new language: add grammar source, update definitions, regenerate parsers, update registry, and ensure all bindings expose the new language.
- All parsing and compilation logic stays in Rust core. Bindings are thin wrappers only.
