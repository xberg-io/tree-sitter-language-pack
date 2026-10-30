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

The subset began as a `wasm` feature group and now lives in the build script as `WASM_DEFAULT_LANGUAGES`, the
default for `wasm32` when `TSLP_LANGUAGES` is unset. The WebAssembly package exposes no download or cache helpers.

### Consequences

* Good, because the WebAssembly package builds on a standard runner and stays loadable in a browser.
* Bad, because a WebAssembly user cannot parse a language outside the subset without a custom build.
* Bad, because the generated e2e suites must skip fixtures whose grammar is outside the subset.

### Confirmation

* `crates/ts-pack-core/build_support/definitions.rs:121` selects `WASM_DEFAULT_LANGUAGES` for `wasm32`, and
  `crates/ts-pack-core/build_support/definitions.rs:140` defines the subset (49 grammars).
* `crates/ts-pack-core/build_support/wasm.rs:57` caps the size of a single `parser.c` in a `wasm32` build.
* `docs-site/src/content/docs/concepts/architecture.md:124` documents the curated static subset.
