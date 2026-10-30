---
status: accepted
date: 2026-03-22
decision-makers: ["Na'aman Hirschfeld"]
---

# Curated static grammar subset for WebAssembly

## Context and Problem Statement

A `wasm32` build cannot load shared libraries or download parser bundles at runtime
([ADR 0001](0001-native-packages-download-parser-bundles-at-runtime.md)), so the WebAssembly package has to compile
its grammars in. It built with the `all` feature, 173 grammars at the time, which "exceeds runner memory when
compiled to WASM" (c38b1db674, 2026-03-22). The same day the subset shrank from 55 to 30 languages "to fit in 7GB
runner" (66b5ea8c41).

## Considered Options

* A curated subset of common languages compiled into the `.wasm` module, overridable with `TSLP_LANGUAGES`
* Every grammar compiled into the `.wasm` module (the `all` feature, replaced in c38b1db674)

## Decision Outcome

Chosen option: "A curated subset of common languages compiled into the `.wasm` module, overridable with
`TSLP_LANGUAGES`", because the full set exceeded runner memory when compiled to WebAssembly (c38b1db674). The current
source adds that the full `parser.c` sources total about 1.7 GB and the resulting bundle "is far too large for
browsers" (`crates/ts-pack-core/build_support/definitions.rs:135`).

The subset began as a `wasm` feature group. Today the published WebAssembly package is built with an explicit
31-language `TSLP_LANGUAGES` (`.github/workflows/publish.yaml:1282`), and the wasm builds in
`.github/workflows/ci-e2e.yaml` and `.github/workflows/ci.yaml` set the same list. The build script's
`WASM_DEFAULT_LANGUAGES` (49 grammars) is only the fallback for a `wasm32` build that leaves `TSLP_LANGUAGES` unset,
as `alef.toml:887` to `alef.toml:889` records. The WebAssembly package exposes no download or cache helpers.

### Consequences

* Good, because the WebAssembly package builds on a standard runner and stays loadable in a browser.
* Bad, because a WebAssembly user cannot parse a language outside the subset without a custom build.
* Bad, because the generated e2e suites must skip fixtures whose grammar is outside the subset.

### Confirmation

* `.github/workflows/publish.yaml:1282` sets the 31-language `TSLP_LANGUAGES` for the published WebAssembly build;
  `.github/workflows/ci-e2e.yaml:552` and `.github/workflows/ci.yaml:1131` set the same list.
* `crates/ts-pack-core/build_support/definitions.rs:121` falls back to `WASM_DEFAULT_LANGUAGES` for `wasm32` when
  `TSLP_LANGUAGES` is unset, and `crates/ts-pack-core/build_support/definitions.rs:140` defines that fallback (49
  grammars).
* `crates/ts-pack-core/build_support/wasm.rs:57` caps the size of a single `parser.c` in a `wasm32` build.
* `docs-site/src/content/docs/concepts/architecture.md:124` documents the curated static subset.
