---
status: accepted
date: 2026-11-15
decision-makers: ["Na'aman Hirschfeld"]
amends: ["0003"]
---

# Compact SQL parser tables during vendoring

## Context and Problem Statement

The updated SQL grammar generates 48.73 MiB of C, exceeding the 48 MiB WebAssembly compiler guard in
[ADR 0003](0003-curated-static-grammar-subset-for-webassembly.md). Dense parse tables also make the compiled
module larger. The pack generates parser sources directly from the pinned fork.

## Considered Options

* Run the pinned SQL fork's table compactor during vendoring
* Keep the higher C-source cutoff for the uncompressed parser

## Decision Outcome

Chosen option: "Run the pinned SQL fork's table compactor during vendoring", because Tree-sitter's existing
sparse representation lowers both the source and compiled module sizes while preserving the grammar and ABI.

### Consequences

* Good, because the same vendored SQL source fits the standard cutoff and works for native and WebAssembly builds.
* Bad, because the compactor depends on the generator's C table layout. A format change may require updating the
  pinned fork before upgrading the generator.

### Confirmation

* `scripts/ci/test_clone_vendors_cli.py` checks that vendoring runs compaction before publishing the source
  and propagates compaction failures.
* The [SQL fork's workflow](https://github.com/Goldziher/tree-sitter-sql/blob/main/.github/workflows/ci.yml)
  builds WebAssembly and checks the compacted C against the 48 MiB cutoff.
