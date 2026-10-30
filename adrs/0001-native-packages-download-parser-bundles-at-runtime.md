---
status: accepted
date: 2026-03-10
decision-makers: ["Na'aman Hirschfeld"]
---

# Native packages download parser bundles at runtime

## Context and Problem Statement

The pack covers several hundred grammars (372 today). The Rust rewrite (7adfeb0597, 2026-03-07) set the core
crate's default feature set to `["all"]`, which compiled every grammar into every build and every native package, and
`dynamic-loading` joined the default the next day (ce92df6d5b, 2026-03-09). On 2026-03-10 (602d29602c) the default became `["dynamic-loading", "download"]`, with a new
`download` feature. Release builds then generated a `parsers.json` manifest (d673b667bf, 2026-03-16), and a
download and configure API reached every binding (f0976ce016, 2026-03-18).

## Considered Options

* Native packages ship no grammars by default and download a per-platform parser bundle, listed in a `parsers.json`
  release manifest, on first use
* Every grammar compiled into every build through the `all` default feature (replaced in 602d29602c)

## Decision Outcome

Chosen option: "Native packages ship no grammars by default and download a per-platform parser bundle, listed in a
`parsers.json` release manifest, on first use", because, as the architecture docs written later state, it "keeps installation fast
and download sizes minimal" (`docs-site/src/content/docs/concepts/architecture.md:122`).

The release publishes one `parsers-<platform>.tar.zst` bundle per platform and a `parsers.json` manifest. On first
use, the core fetches the manifest, downloads and extracts the bundle for the current platform into a versioned
cache directory, and opens each grammar with `dlopen` or `LoadLibrary`. Later calls load from the cache without
network access. Grammars can still be compiled in statically
([ADR 0002](0002-build-time-grammar-selection-through-tslp-languages.md)), and the WebAssembly package does so
([ADR 0003](0003-curated-static-grammar-subset-for-webassembly.md)).

### Consequences

* Good, because a native package stays small, whatever the number of grammars.
* Bad, because the first use of a grammar needs network access to the GitHub release, or a cache prepared in
  advance.
* Bad, because the loader opens shared libraries from the cache directory, so the cache's ownership and the
  integrity of the download are security boundaries that the core must check.

### Confirmation

* `crates/ts-pack-core/Cargo.toml:31` sets `default = ["dynamic-loading", "download"]`.
* `crates/ts-pack-core/build_support/definitions.rs:86` documents that an unset `TSLP_LANGUAGES` "means runtime
  download", and `crates/ts-pack-core/build_support/definitions.rs:130` compiles no grammar on native targets then.
* `crates/ts-pack-core/src/download.rs:56` builds the release URL of `parsers.json`.
* `.github/workflows/publish.yaml:2167` generates the manifest, and `.github/workflows/publish.yaml:2300` uploads the
  bundles and the manifest to the release.
