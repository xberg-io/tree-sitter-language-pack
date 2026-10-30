---
status: accepted
date: 2026-04-14
decision-makers: ["Na'aman Hirschfeld"]
---

# Build-time grammar selection through TSLP_LANGUAGES

## Context and Problem Statement

Builds that compile grammars in statically (CI, the WebAssembly package, the C library) must choose which grammars to
compile. The core crate offered one Cargo feature per language (`lang-*`) plus group features (`all`, `web`,
`systems`, `scripting`, `data`, `jvm`, `functional`, `wasm`), and CI also set a `TSLP_LANGUAGES` environment
variable (c25fba23cf, 2026-03-14). With 305 languages the crate had 319 features, more than the 300 that crates.io
accepts, which blocked the crates.io publish (f9cb0c0520, 2026-04-14).

## Considered Options

* The `TSLP_LANGUAGES` environment variable, read by the build script, as the only selection mechanism
* One Cargo feature per language plus group features (removed in f9cb0c0520)

## Decision Outcome

Chosen option: "The `TSLP_LANGUAGES` environment variable, read by the build script, as the only selection
mechanism", because it unblocked the crates.io publish, and, in the words of f9cb0c0520, "the download feature
(default) fetches parsers at runtime, making per-language features unnecessary for most users"
([ADR 0001](0001-native-packages-download-parser-bundles-at-runtime.md)).

`TSLP_LANGUAGES` takes a comma-separated list of grammar names, or `all` or `*` for every grammar. Unset, a native
build compiles no grammar and a `wasm32` build compiles the curated default subset
([ADR 0003](0003-curated-static-grammar-subset-for-webassembly.md)).

### Consequences

* Good, because the crate's feature count no longer grows with the grammar count.
* Bad, because grammar selection is invisible to Cargo: changing `TSLP_LANGUAGES` does not appear in a dependency
  declaration, and a downstream crate cannot request a grammar through features.

### Confirmation

* `crates/ts-pack-core/build_support/definitions.rs:84` (`selected_languages`) reads `TSLP_LANGUAGES` and handles
  `all` and `*`.
* `crates/ts-pack-core/Cargo.toml:30` to `crates/ts-pack-core/Cargo.toml:49` define the features; none is a `lang-*`
  or group feature.
* The `parser-compilation` rule in `.ai-rulez/domains/parser-compilation/rules/parser-compilation.md` documents
  `TSLP_LANGUAGES` as the way to pick grammars.
