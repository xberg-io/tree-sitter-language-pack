# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- E2E fixture coverage for: language alias resolution (`shell→bash`) via `has_language` / `get_language` / `get_parser` (3 fixtures); `download` edge cases — empty list, multiple-language, and unknown-language error path (3 fixtures); error-handling for 120KB sources and `get_language("")` (2 fixtures); and TypeScript function parsing (1 fixture). Brings fixture count from 403 to 412, covering 100% of the public `download`, `get_*`, and `has_language` surface across all 10 language bindings.

## [1.8.1] - 2026-05-11

### Fixed

- Python: restore `SupportedLanguage` as `Literal[...]` of all 306 grammars at `tree_sitter_language_pack.SupportedLanguage`. The symbol was dropped during the alef 0.15.x codegen migration and re-importing it raised `ImportError` in 1.8.0 (#121).
- Python: `get_parser("python").parse(b"...")` returns a real `tree_sitter.Tree` again instead of raising `AttributeError`. `get_parser` / `get_language` now return native `tree_sitter.Parser` / `tree_sitter.Language` instances via PyO3 capsule pass-through (alef v0.15.39 wires `capsule_types` through `gen_bindings`) (#121).

## [1.8.0] - 2026-05-09

### Added

- macOS x86_64 native binaries across all polyglot bindings (Python wheels, npm napi, Ruby gem, Maven JAR, NuGet, C FFI, Go FFI, libts-pack bottle) — restores Intel Mac coverage that was missing under the alef 0.11 transition
- Real Homebrew bottle protocol for both `ts-pack` (CLI) and `libts-pack` (FFI library) via `brew install --build-bottle` + `brew bottle --json`, replacing the prior synthetic tarball approach. Eight bottles per release across `arm64_sequoia`, `sequoia`, `arm64_linux`, `x86_64_linux`. `brew install` now pours instead of source-building
- `libts-pack` Homebrew formula bundling tree-sitter language pack as a C library (headers + dylib/so + static archive)
- Python sdist published to PyPI alongside the existing platform wheels
- E2E fixtures covering Kotlin package + class structure (`kotlin_package_class_intel.json`), Java package declarations (`java_package_intel.json`), and a process call exercising the typed `extractions` map (`process_with_extractions.json`)

### Changed

- Migrated to alef 0.15.x (Jinja-based codegen) for all polyglot bindings — Python, TypeScript, Ruby, Go, Java, C#, Elixir, PHP, WASM
- WASM now ships the `--target nodejs` build to npm so consumers no longer hit the bundler-only `import * from "env"` failure on `require()`
- WASM coverage scoped to a curated 32-language subset to fit the 16 GB GitHub runner during builds

### Fixed

- Intel: emit `StructureKind::Module` for Kotlin `package_header` and Java `package_declaration` so callers can build fully-qualified names for JVM languages (#112)
- Intel: resolve structure names via a fallback chain (`name` field → `type_identifier` → `identifier` → `scoped_identifier`) so Kotlin classes and Java/Kotlin packages no longer surface with null names (#111)
- Java: ship `natives/{rid}/` entries inside the published JAR — `actions/download-artifact` produces nested artifact paths, and the previous staging loop preserved them, so every platform hit `UnsatisfiedLinkError` on load. Flatten via `find` and add presence/`jar tf` guard steps so the regression cannot ship silently again (#114)
- Bindings: surface `extractions` as a typed `Map<String, ExtractionPattern>` / `Map<String, PatternResult>` across Java, Python, Go, TypeScript, Ruby, PHP, C#, Elixir, FFI, and WASM (was `Optional<String>` on Java, blocking pattern extractions through the high-level API). Driven by the alef 0.12.4 codegen fix for `AHashMap`-typed fields (#115)
- C#: strip duplicate `{` lines emitted by alef 0.14.33 codegen so generated `.cs` files compile
- Ruby: regenerated `native.rb` no longer recurses into itself via `define_singleton_method` — magnus codegen now skips re-export when binding name matches the native module method
- Node: `index.js` now contains real platform-dispatch logic so `require()` resolves the correct `.darwin-arm64.node`/`.linux-x64-gnu.node`/etc. instead of failing on the un-suffixed bundle name
- WASM: drop bundler-only output, removing spurious `'env'` module imports that broke `require()` from Node consumers
- Maven JAR previously missed `linux-x86_64` natives because of stage-loop path mishandling; flatten artifact downloads and add a `jar tf` guard
- Hex.pm `metadata.config` size limit — exclude the parser sources tarball from the package
- PHP: fix broken `crates/ts-pack-php/README.md` links in root `README.md` — path moved to `packages/php/README.md` after alef migration (#106)
- PHP: fix `.task/php.yml` `build`, `build:dev`, and `clean` tasks pointing to removed `crates/ts-pack-php/` — corrected to `crates/ts-pack-core-php/` (#106)
- PHP: align `packages/php/composer.json` and `packages/php/README.md` package name to canonical Packagist vendor slug (`kreuzberg/` not `kreuzberg-dev/`) (#106)
- PHP: document `mlocati/php-extension-installer` prerequisite in install docs and correct minimum PHP version to 8.4+ (#106)
- Go: regenerate stale `binding.go` with current alef generator

## [1.7.0] - 2026-04-22

### Added

- Migrate to [alef](https://github.com/kreuzberg-dev/alef) polyglot binding generator — all language bindings (Python, TypeScript, Ruby, Go, Java, C#, Elixir, PHP, WASM) are now generated from a single `alef.toml` configuration
- `Default`, `Hash`, `PartialEq`, `Eq` derives on all public types
- 18 new e2e test fixtures closing testing gaps across all binding languages
- Consolidated CI: 12 language-specific workflows merged into a single `ci.yaml`
- Registry-mode e2e test apps under `test_apps/` (generated via `alef e2e generate --registry`)

### Changed

- Public API locked down with `pub(crate)` — only functions and types that were in the pre-alef Python bindings are exported; internal modules (`json_utils`, `intel` submodules, `config`, `definitions`) are no longer public
- Workspace lints applied to all binding crates (`clippy::all = "deny"`, `unsafe_code = "deny"`)
- `test_apps/` moved from `tests/test_apps/` to project root

### Fixed

- `available_languages()`, `has_language()`, and `language_count()` now register the download cache directory before querying the registry — fixes empty results when using the `download` feature (#90)
- `process()` auto-downloads missing parsers instead of returning `LanguageNotFound` (#94)
- C# task references updated from `.sln` to `.csproj`
- Maven version plugin pinned to exclude alpha/beta/RC versions
- Docker CI: `uv run` changed to `uv run --no-project` to avoid triggering root pyproject.toml build
- Ruby CI: removed stale `working-directory` that pointed to wrong path

## [1.6.3] - 2026-04-20

### Fixed

- Go: fix FFI build defaults — add `TSLP_LINK_MODE` and `TSLP_LANGUAGES` env vars to Go task (#102)
- Go: fix CGO `LDFLAGS` paths — point to workspace `target/release/` instead of crate-local path (#102)
- Go: remove duplicate forward declarations from `ffi.go` (already in `ts_pack.h`) (#102)
- Go: fix README examples — proper error handling, correct API signatures (`Init`, `Download`) (#102)
- FFI: add extra libs dir from `cache_dir()` to registry on creation (#102)
- Docs: fix textlint pre-commit hook — add `additional_dependencies` for all textlint plugins (#102)

## [1.6.2] - 2026-04-18

### Fixed

- Compile bundled grammars with `-fno-strict-aliasing` to prevent undefined behavior (#100)

### Changed

- Update dependencies across lockfiles
- Regenerate READMEs for 1.6.1 version bump (#101)

## [1.6.1] - 2026-04-17

### Fixed

- Go: move package root from `packages/go/v1/` to `packages/go/` so the Go module proxy can resolve `go.mod` at the correct path — `go get github.com/kreuzberg-dev/tree-sitter-language-pack/packages/go` now works (#97)
- Go: fix CGO `SRCDIR`-relative include/lib paths (one fewer `../` after directory restructure)
- Remove `features = ["all"]` from e2e Rust test `Cargo.toml` — use `download` feature for runtime parser fetching
- Remove 305 `lang-*` features to unblock crates.io publish (300 feature limit)
- Regenerate READMEs for v1.6.0, fix Windows query cache test flake
- Bump `rustls-webpki` to patch RUSTSEC-2026-0098 and RUSTSEC-2026-0099 (#99)
- Fix MIME type inference in core build by embedding `language_definitions.json` in crate

### Changed

- Update dependencies across Python, Node.js, PHP, and Rust lockfiles
- Replace feature group docs with `download`/`TSLP_LANGUAGES` documentation in READMEs

## [1.6.0] - 2026-04-14

### Added

- Thread-local parser cache in `parse_string()` — avoids re-creating parsers on repeated calls for the same language
- Two-level compiled query cache (thread-local + global) in `run_query()` — avoids recompiling tree-sitter queries
- `parse_with_language()` internal API for callers that already have a `Language` object
- Pre-computed capture names in `CompiledExtraction` — avoids rebuilding on every extraction call
- Go `type_spec` declarations extracted as symbols with correct `SymbolKind` (struct, interface, type)
- Dedicated "Download Parsers" section in quickstart docs covering CLI, programmatic APIs, groups, Docker/CI, and config files
- Tests for parser cache reuse, query cache sharing across threads, cursor byte-range isolation, and capture name correctness

### Fixed

- `compiled_query()` now propagates `Error::LockPoisoned` instead of silently ignoring poisoned RwLock
- `QueryCursor` byte-range no longer leaks between patterns when reusing the cursor in `extract_from_tree()`
- Replaced `std::collections::HashMap` with `ahash::AHashMap` in parser cache for consistency
- Redundant `get_language()` call removed from `parse_string()` hot path — only called on cache miss

### Changed

- `CompiledExtraction::extract()` and `intel::parse_source()` now use the thread-local parser cache
- `QueryCursor` reused across patterns within a single `extract_from_tree()` call
- Unnecessary `String` allocation removed from `node_types.contains()` check in chunking

### Removed

- All 305 `lang-*` Cargo features and group features (`all`, `web`, `systems`, `scripting`, `data`, `jvm`, `functional`, `wasm`) — language selection is now via `TSLP_LANGUAGES` env var at build time; the `download` feature (default) fetches parsers at runtime

## [1.5.0] - 2026-04-08

### Added

- 57 new permissively-licensed grammars — 305 languages total
  - abl, c3, cel, cfml, chuck, cst, dhall, elvish, gap, gdshader, glimmer, gnuplot, gotmpl, gowork, gpg, hjson, hocon, hoon, htmldjango, jai, javadoc, json5, kcl, mlir, nasm, norg_meta, ocamllex, openscad, phpdoc, poe_filter, prql, rasi, razor, rbs, roc, rtf, slang, smalltalk, sml, snakemake, souffle, sourcepawn, sql_bigquery, stan, superhtml, sway, systemverilog, tact, tera, typespec, typoscript, vhs, vrl, wgsl_bevy, x86asm, ziggy, ziggy_schema
- CI license validation job in `ci-validate.yaml` — blocks PRs that introduce non-permissive (GPL/AGPL/LGPL/MPL) grammars

### Fixed

- `less` grammar: regenerated parser from ABI 11 to ABI 14 (was incompatible with tree-sitter 0.26)
- `corn` smoke fixture: replaced invalid `"x"` snippet with valid corn syntax

## [1.4.1] - 2026-03-31

### Fixed

- Include `language_definitions.json` in the published crate so `build.rs` can find extension mappings, ambiguity data, and C symbol overrides when installed from crates.io

### Changed

- Updated dependencies across all language ecosystems

## [1.4.0] - 2026-03-29

### Fixed

- Expose `detect_language` in Python public API (#85)
- PHP extension name corrected to `ts-pack-php` (hyphens)

### Changed

- All language snippet READMEs and documentation corrected
- Removed automated grammar updates workflow

## [1.3.3] - 2026-03-27

### Fixed

- `C_SYMBOL_OVERRIDES` table now includes ALL languages from `language_definitions.json`, not just compiled ones — fixes download and loading of `csharp`, `vb`, `embeddedtemplate`, `nushell` from PyPI/npm/RubyGems packages
- `downloaded_languages()` returns canonical names (`csharp`) instead of c_symbol names (`c_sharp`)
- Elixir NIF publish: upload both hyphen and underscore artifact names so RustlerPrecompiled can find them
- Elixir NIF 2.17 packaging: fix stale variable names from dual-name refactor
- Ruby comprehensive test: remove `JSON.parse` on native Hash return from `process()`
- Go comprehensive test: access flat `ProcessResult` fields directly (no `metadata` wrapper)
- Homebrew bottle and PHP PIE packages now included in release artifacts

### Changed

- Dependency updates across all language ecosystems
- `rustler_precompiled` updated to 0.9.0 (Elixir)

## [1.3.2] - 2026-03-26

### Fixed

- Dynamic parser loading for languages with `c_symbol` overrides (`csharp`, `vb`, `embeddedtemplate`, `nushell`) — build was naming libraries with the raw name but runtime loader expected the `c_symbol` name (#80)
- Go E2E generator: unused `tspack` import in non-process test files
- Elixir: add missing `extract/2` and `validate_extraction/1` NIF declarations
- PHP E2E generator: use double-quoted strings for source code so `\n` is interpreted correctly
- Nim grammar: switch from abandoned `paranim/tree-sitter-nim` (ABI v11) to `aMOPel/tree-sitter-nim` (MIT, ABI v14)

### Added

- Smoke test fixtures for all `c_symbol` override languages (csharp, vb, embeddedtemplate, nushell)
- Dynamic-linking CI step in `ci-all-grammars.yaml` to catch `c_symbol` naming mismatches

## [1.3.1] - 2026-03-26

### Fixed

- Ruby binding: `process()`, `extract()`, `validate_extraction()` now return native Ruby Hash instead of raw JSON string
- WASM binding: output keys now use camelCase (matching Node.js binding convention), input config accepts both camelCase and snake_case
- Go E2E generator: use typed `*ProcessResult` struct fields instead of invalid `json.Unmarshal` on non-string return
- Elixir CI: stage NIF with both hyphenated and underscored filenames to satisfy Rustler force-build check and `load_from` loader

## [1.3.0] - 2026-03-26

### Added

- Extraction query API: run user-defined tree-sitter queries and get structured results
  - `extract_patterns()` / `extract()` across Python, Node.js, Rust, Ruby, Elixir, PHP, WASM, C FFI
  - `validate_extraction()` for config validation without execution
  - `CompiledExtraction` for pre-compiled query reuse (Rust)
  - `ProcessConfig.extractions` for combining custom queries with standard analysis
  - Types: ExtractionConfig, ExtractionPattern, CaptureOutput, CaptureResult, MatchResult, PatternResult, ExtractionResult
- Criterion benchmarks: 9 groups, 23 benchmarks across Python, TypeScript, Rust, Go
- Extraction queries guide and documentation across all API references

### Fixed

- E2E generator: `process_imports_contains_source` assertion uses contains instead of equality
- WASM: language list matches actual compiled features (30 languages)
- WASM: add missing `detectLanguageFromPath` and `detectLanguageFromExtension` exports
- PHP generator: null array handling in `process()` result assertions
- Elixir: RustlerPrecompiled `crate` field resolution with `load_from` override
- Predicate evaluation: remove redundant re-evaluation (tree-sitter 0.26 handles internally)
- Documentation: stale version numbers, incomplete API references, incorrect function signatures
- Java version requirement standardized to JDK 25+

## [1.2.1] - 2026-03-25

### Fixed

- Nushell grammar `c_symbol` override — linker error `undefined symbol: tree_sitter_nushell`
- E2E generator calling `.as_deref()` on `String` type (compile error on CI)
- WASM build: gate `c_symbol_for` behind `dynamic-loading`/`download` features (dead code warning)
- Elixir publish: align RustlerPrecompiled `crate:` field with Cargo `[lib]` name (underscores, not hyphens)
- Elixir publish: add `--cfg` flag patch to publish workflow for Rustler 0.37.3 compatibility
- Python `without_gil()`: add `catch_unwind` to ensure GIL is reacquired on panic
- Text splitter: prevent zero-width chunks in pathological UTF-8 edge case
- Comment kind detection: handle `//!`, `/*!`, and `doc_comment` node types
- Import detection: restrict fallback to explicitly supported languages only
- Export detection: use field-based AST matching instead of fragile `text.contains()`

### Changed

- Registry: `Arc<Vec<PathBuf>>` for extra lib dirs (avoids Vec clone per language lookup)
- Registry: `AHashSet<&str>` in `available_languages()` (avoids 248+ String allocations)
- `NodeInfo.kind` uses `Cow::Borrowed` (zero-copy from tree-sitter's `&'static str`)
- Python: `with_tree()`/`try_with_tree()` helpers replace 9 duplicate lock patterns
- Python: `without_gil()` helper replaces 5 duplicate GIL release patterns
- Core: `extension_ambiguity_json()` helper replaces duplicated JSON serialization in 4 bindings
- Chunking: `MetadataCollector` struct reduces function from 11 to 7 parameters
- FFI: 25 SAFETY comments added to unsafe blocks
- Docs: rewrite all 12 API references to match actual binding source code
- Docs: add JSON-LD structured data and Open Graph metadata for crawlers

## [1.2.0] - 2026-03-25

### Added

- 49 new permissively-licensed grammars — 248 languages total
  - angular, bass, blade, brightscript, circom, cooklang, corn, crystal, cue, cylc, desktop, djot, earthfile, ebnf, editorconfig, eds, eex, elsa, enforce, facility, faust, fidl, foam, forth, git_config, git_rebase, godot_resource, http, hurl, just, ledger, less, liquid, mojo, move, nickel, nginx, norg, nushell, promql, pug, ql, robot, teal, templ, tmux, todotxt, turtle, vimdoc, wolfram
- Grammar updater automation (`scripts/check_grammar_updates.py`) with weekly CI workflow
- Generated supported languages table (`docs/supported-languages.md`) integrated into docs CI
- Node.js NAPI exports: `detectLanguageFromExtension`, `detectLanguageFromPath`, `getHighlightsQuery`, `extensionAmbiguity`
- E2E `process` test category with `process()` API coverage across all 11 language bindings

### Fixed

- Download/load filename mismatch for languages with c_symbol overrides (csharp, embeddedtemplate, vb) — fixes [#80](https://github.com/kreuzberg-dev/tree-sitter-language-pack/issues/80)
- E2E fixture system: merged stale `intel/` and `metadata/` directories into unified `process/` category
- TypeScript and WASM e2e generators now use camelCase for metrics keys
- Docker CI grammar fixture updated to include all languages
- Elixir publish workflow: checksum file verification, increased retry timeout
- Missing Node.js `index.js` exports for detection and query functions

### Changed

- Renamed e2e fixture assertions from `intel_*`/`meta_*` to `process_*`
- All documentation and package descriptions updated to reflect 248 languages

## [1.1.4] - 2026-03-24

### Added

- New language: `al` (AL / Business Central) — 198 languages total
- Grammar license linter (`scripts/lint_grammar_licenses.py`, `task lint:licenses`) verifies all grammars use permissive licenses
- Permissive license policy documented in CONTRIBUTING.md, docs, and README

### Fixed

- Replace `nim` grammar (alaviss, MPL-2.0 copyleft) with paranim/tree-sitter-nim (MIT)
- Replace `prolog` grammar (codeberg foxy, AGPL-3.0 copyleft) with Rukiza/tree-sitter-prolog (ISC)
- Docs: align mkdocs config with kreuzberg branding; mermaid diagrams now render (fixes [#81](https://github.com/kreuzberg-dev/tree-sitter-language-pack/issues/81))

## [1.1.3] - 2026-03-24

### Fixed

- Dynamic loader: resolve `c_symbol` overrides for csharp, embeddedtemplate, and vb so `get_language()` works for dynamically loaded grammars (fixes [#80](https://github.com/kreuzberg-dev/tree-sitter-language-pack/issues/80))
- E2E generator: enable all ProcessConfig features (structure, imports, exports, comments, docstrings, symbols, diagnostics) for intel tests so diagnostics assertions pass

### Added

- 23 new smoke test fixtures for languages missing coverage: asciidoc, awk, batch, caddy, cedar, cedarschema, csharp, devicetree, diff, dot, embeddedtemplate, idris, jinja2, jq, lean, pkl, postscript, prolog, rescript, ssh_config, textproto, tlaplus, vb, wit, zsh
- CI workflow (`ci-all-grammars.yaml`) that tests all 197 grammars end-to-end, preventing regressions like #80
- `rust:e2e:all-grammars` task for running the full grammar suite locally

## [1.1.2] - 2026-03-23

### Fixed

- Elixir NIF: fix Rustler crate name mismatch (`ts_pack_elixir` → `ts-pack-elixir`) causing compilation failure
- Rust crate publish: embed query file contents at build time instead of using `include_str!` with relative paths that break in the cargo package tarball

## [1.1.1] - 2026-03-23

### Fixed

- WASM build: ahash uses compile-time-rng instead of runtime-rng (avoids getrandom on wasm32)
- Docker/static build: add `c_symbol` override for grammars with non-standard C symbol names (csharp, vb, embeddedtemplate)
- Unused imports when `dynamic-loading` feature disabled (WASM builds)
- Python sdist: `.pyi` and `py.typed` now included in both wheel and sdist
- C# build: add missing `ExtensionAmbiguityResult` model class
- Set `generate: true` for csharp, vb, embeddedtemplate grammars

### Changed

- Switch from `std::HashMap`/`HashSet` to `ahash::AHashMap`/`AHashSet` for faster hashing in registry

## [1.1.0] - 2026-03-23

### Added

- 20 new languages from arborium: asciidoc, awk, caddy, cedar, cedarschema, devicetree, dot, idris, jinja2, jq, lean, postscript, prolog, rescript, ssh_config, textproto, tlaplus, vb, wasm-interface-types, zsh (197 total)
- Centralized extension-to-language mapping: `sources/language_definitions.json` is the single source of truth for 239 file extensions across 197 languages
- Build-time code generation: `build.rs` generates extension lookup with strict validation (panics on duplicates, non-ASCII, uppercase, dots)
- `detect_language_from_content(content)`: shebang-based language detection (`#!/usr/bin/env python3` → "python")
- `extension_ambiguity(ext)`: query whether a file extension is ambiguous (e.g. `.m` → objc with matlab alternative)
- Highlight query bundling: `get_highlights_query(lang)`, `get_injections_query(lang)`, `get_locals_query(lang)` — embed .scm queries at build time
- `ambiguous` field in `language_definitions.json` for declaring known extension ambiguities
- E2E test fixtures and generators for detect-language, ambiguity, and highlights across all 11 language targets
- New APIs exposed in all bindings: Python, Node.js, Ruby, WASM, Elixir, PHP, C FFI, Go, C#

### Changed

- `LanguageRegistry` uses `Arc<RwLock<Vec<PathBuf>>>` for interior mutability — no more global `RwLock` wrapper, eliminates lock poisoning risk
- `ProcessConfig.language`: `String` → `Cow<'static, str>` (zero allocation for string literals)
- `NodeInfo.kind`, `QueryMatch.captures`: `String` → `Cow<'static, str>`
- `available_languages()` uses `HashSet` for O(1) dedup instead of O(n) Vec contains
- Chunking line counting uses precomputed newline table with binary search (O(log n) per chunk vs O(n))
- Added `memchr` dependency for fast byte scanning in text splitter and chunking
- Extension/ambiguity lookups generated from JSON at build time
- `clone_vendors.py` now copies `queries/` directories alongside `src/`

### Fixed

- Strong types in all binding stubs: Python `.pyi` (TypedDicts), TypeScript `.d.ts` (interfaces), Ruby `.rbs` (record types), C# `Models.cs` (string enums replace `object`)
- Pre-existing registry test failures from global `RwLock` poisoning — test helpers now use local `LanguageRegistry::new()`
- Removed ambiguous `.os` (bsl) and `.cls` (apex/LaTeX conflict) extensions

## [1.0.0] - 2026-03-21

### Changed

- Docker: separated publish-docker workflow from main publish (180-minute timeout for multiplatform builds)
- Docker: publish-docker now triggers on `release` events and includes full smoke tests before push
- Test apps: all bindings now download languages before running tests (Ruby, Go, Elixir)
- Test apps: Rust test app adds parse_string validation tests
- Test apps: CLI smoke test adds chunking test
- Test apps: added Homebrew smoke test suite

### Fixed

- npm publish authentication and registry configuration
- Elixir NIF binary build and checksum generation
- Ruby CI and WASM build timeout
- Version sync across binding manifests

---

## Pre-1.0 Releases (Python-only)

### [0.12.0]

#### Added

- tree-sitter-cobol grammar support

#### Fixed

- MSVC build compatibility for cobol grammar
- Alpine Linux (musl) wheel platform tag support (PEP 656)
- Wheel file discovery in CI test action

### [0.11.0]

#### Added

- tree-sitter-bsl (1C:Enterprise) grammar support

#### Changed

- Updated all dependencies and relocked

### [0.10.0]

#### Added

- tree-sitter 0.25 support

#### Changed

- Dropped Python 3.9 support
- Adopted prek pre-commit workflow
- CI: cancel superseded workflow runs

### [0.9.1]

#### Added

- WASM (wast & wat) grammar support
- F# and F# signature grammar support

### [0.9.0]

#### Added

- tree-sitter-nim grammar support
- tree-sitter-ini grammar support
- Swift grammar update (trailing comma support)

### [0.8.0]

#### Fixed

- sdist build issues resolved

### [0.7.4]

#### Added

- GraphQL grammar support
- Kotlin grammar support (SAM conversions)
- Netlinx grammar support

### [0.7.3]

#### Changed

- Swift grammar update (macros + copyable)

### [0.7.2]

#### Added

- Apex grammar support

#### Fixed

- MSYS2 GCC build issues

### [0.7.1]

#### Added

- OCaml and OCaml Interface grammar support
- Markdown inline parser support

#### Fixed

- Pinned elm and rust grammar versions
- Pinned tree-sitter-tcl to known-good revision

### [0.6.1]

#### Added

- ARM64 Linux CI builds

#### Fixed

- Build issue resolved

### [0.6.0]

#### Fixed

- Windows DLL loading compatibility issues

### [0.5.0]

#### Fixed

- Windows compatibility and encoding issues for non-English locales

### [0.4.0]

#### Added

- PyCapsule-based language loading
- Protocol Buffers (proto) grammar support
- SPARQL grammar support

### [0.3.0]

#### Changed

- Updated generation setup and build matrix
- Removed magik and swift grammars (temporarily)

### [0.2.0]

#### Changed

- Version bump with dependency updates

### [0.1.2]

#### Fixed

- Added MANIFEST.in for sdist packaging

### [0.1.1]

#### Fixed

- Missing parsers in package data

### [0.1.0]

#### Added

- Initial release with 100+ tree-sitter language grammars
- Python package with pre-compiled parsers
- Multi-platform wheel builds (Linux, macOS, Windows)
