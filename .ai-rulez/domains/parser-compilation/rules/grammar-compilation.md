---
priority: critical
---

# Grammar Compilation Standards

- Tree-sitter grammars must compile to valid C sources before inclusion.
- Use `tree-sitter generate` for grammar-to-C compilation. Validate parser.c output.
- Pin grammar versions to specific commits/tags for reproducibility.
- Test parser correctness with representative source code samples per language.
- Handle ABI version compatibility — currently target ABI version 14.
- Keep compiled parser artifacts out of version control; regenerate in CI.
