---
priority: high
---

# Parser Testing Standards

- Every included grammar must have at least one test file with representative syntax.
- Test parsing round-trips: source -> tree -> verify node types and structure.
- Test error recovery: parsers should produce partial trees for invalid input.
- Benchmark parsing performance for large files (>10KB) per language.
- CI must test all parsers on Linux, macOS, and Windows.
