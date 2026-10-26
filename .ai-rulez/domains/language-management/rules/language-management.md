---
type: Decision
title: Language Management
paths:
  - "sources/language_definitions.json"
  - "crates/ts-pack-core/language_definitions.json"
  - "crates/ts-pack-core/src/registry.rs"
  - "crates/ts-pack-core/src/definitions.rs"
  - "crates/ts-pack-core/src/extensions.rs"
  - "fixtures/parsing/**"
  - "fixtures/registry/**"
x-ai-rulez:
  kind: rule
  id: language-management
  domain: language-management
  metadata:
    priority: high
---

- Each language entry specifies its grammar repo, commit or tag, and whether it has an external scanner.
- The language registry lists every supported language and its metadata; document language-specific quirks and known parsing limits.
- Adding a language: add the grammar source, update the definitions, regenerate parsers, update the registry, and expose it in every binding.
- Every grammar has at least one test file with representative syntax; test source -> tree -> node types and structure.
- Test error recovery: invalid input still yields a partial tree.
- Grammar version bumps run the full test suite against real source files; CI tests all parsers on Linux, macOS and Windows.
- Versioning: breaking grammar updates (ABI changes, removed languages) are major, new languages minor, grammar fixes patch; keep the version in sync across all binding manifests.
