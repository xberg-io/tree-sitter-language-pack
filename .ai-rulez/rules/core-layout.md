---
type: Reference
title: Core Crate Layout
paths:
  - "crates/ts-pack-core/src/**"
  - "crates/ts-pack-cli/**"
x-ai-rulez:
  kind: rule
  id: core-layout
  metadata:
    priority: medium
---

- All parsing and compilation logic lives in `crates/ts-pack-core`; bindings are thin wrappers.
- `registry.rs` language registry and lookup; `definitions.rs` grammar definitions; `extensions.rs` extension mapping; `download.rs` grammar source fetching.
- `pack_config.rs` / `process_config.rs` configuration; `parse.rs` parsing; `queries.rs` query support; `text_splitter.rs` code splitting.
- `intel/` holds the code-intelligence layer (extraction, chunking, data extraction).
- CLI entry point: `crates/ts-pack-cli`.
