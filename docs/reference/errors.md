---
title: "Error Reference"
---

## Error Reference

All error types thrown by the library across all languages.

### Error

Errors that can occur when using the tree-sitter language pack.

Covers language lookup failures, parse errors, query errors, and I/O issues.
Feature-gated variants are included when `config`, `download`, or related
features are enabled.

| Variant | Message | Description |
|---------|---------|-------------|
| `LanguageNotFound` | Language '{0}' not found | Language not found errors |
| `DynamicLoad` | Dynamic library load error: {0} | Dynamic load errors |
| `NullLanguagePointer` | Language function returned null pointer for '{0}' | Null language pointer errors |
| `ParserSetup` | Failed to set parser language: {0} | Parser setup errors |
| `LockPoisoned` | Registry lock poisoned: {0} | Lock poisoned errors |
| `Config` | Configuration error: {0} | Config errors |
| `ParseFailed` | Parse failed: parsing returned no tree | Parse failed errors |
| `QueryError` | Query error: {0} | Query error errors |
| `InvalidRange` | Invalid byte range: {0} | Invalid range errors |
| `Io` | IO error: {0} | Io errors |

---
