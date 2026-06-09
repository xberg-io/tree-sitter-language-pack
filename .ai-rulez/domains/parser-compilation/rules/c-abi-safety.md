---
priority: high
---

# C ABI Safety

- All tree-sitter parsers expose C ABI. Ensure symbol visibility is correct.
- Avoid symbol name collisions between grammars — each parser has unique prefixed symbols.
- Handle memory allocation/deallocation correctly across FFI boundaries.
- Test parser loading and unloading to prevent memory leaks.
- Validate that compiled shared libraries (.so/.dylib/.dll) work across target platforms.
