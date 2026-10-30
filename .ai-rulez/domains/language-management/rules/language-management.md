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

- Entries in `sources/language_definitions.json` carry `repo` + `rev` (required unless `local`) and `extensions`, plus optional `branch`, `directory`, `ambiguous`, `c_symbol`, `generate`, `abi_version`, `license`. `rev` is a full 40-hex commit SHA; field table: `docs-site/src/content/docs/contributing.md`.
- Add a language: the entry (permissive licence only), then `mise run lint:licenses:check`, `mise run notices`, `mise run clone:if-missing`, `TSLP_LANGUAGES=<lang> mise run build`, a fixture under `fixtures/smoke/`, `mise run e2e:generate`, `mise run docs:generate:languages`.
- `crates/ts-pack-core/language_definitions.json` is the crate-local copy `build_support/definitions.rs` reads when `sources/` is absent (crates.io builds). It is byte-identical to `sources/` today and no task syncs it: change both.
- Versioning: breaking grammar updates (ABI changes, removed languages) are major, new languages minor, grammar fixes patch.
