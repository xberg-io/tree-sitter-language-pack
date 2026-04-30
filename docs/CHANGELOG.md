# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [1.0.0-rc.13] - 2026-03-19

### Fixed

- Docs: mermaid diagrams now render (fixed broken code fence closings in architecture.md)
- Docs: Elixir icon renders in Supported Ecosystems table (`:simple-elixir:`)
- Docs: mkdocs.yaml aligned with Kreuzberg (custom palette, toc.integrate, repo icons)
- Publish: Ruby platform-specific native gems (build-native-gem.rb)
- Publish: PHP extension binaries uploaded to GitHub release
- Publish: Elixir uses rustler_precompiled for binary NIF distribution
- Publish: workflow triggers on release event (not just workflow_dispatch)
- Publish: npm tag computed in prepare job (rc→next, stable→latest)

## [1.0.0-rc.12] - 2026-03-19

### Added

- Config files aligned with Kreuzberg: biome.json, rust-toolchain.toml, deny.toml, .golangci.yml, .clang-format, .taplo.toml, .shellcheckrc, tsconfig.json, .npmrc, go.work
- CI CLI workflow (`ci-cli.yaml`) with clone-vendors, build, smoke tests, grammar tests
- Vendor scripts for PHP and Ruby (`scripts/ci/php/vendor-core.py`, `scripts/ci/ruby/vendor-core.py`)
- Docker + C FFI badges in all READMEs
- `composer.lock` for reproducible PHP builds
- `tsconfig.json` for ts-pack-node, ts-pack-wasm, and tests/test_apps/node crates
- 8 missing Elixir NIF stubs: init, configure, download, download_all, manifest_languages, downloaded_languages, clean_cache, cache_dir

### Fixed

- CI: `rustfmt` and `clippy` added to `rust-toolchain.toml` components (fixes 6 CI workflows)
- CI: Go E2E tests — added `e2e/go` to `go.work` for module resolution
- CI: C# E2E tests — `setup-dotnet@v4` → `@v5`, e2e-generator C# template `net9.0` → `net10.0`
- CI: WASM E2E tests — missing tsconfig.json files caused vitest ENOENT
- CI: CLI smoke tests — added clone-vendors job so parsers are available
- Publish: npm tag computed once in prepare job (Kreuzberg pattern: rc/alpha/beta → `next`, stable → `latest`)
- README badges: reordered to match Kreuzberg, license badge `.svg`, docs link to kreuzberg.dev

### Removed

- Dead code: `sources/language_extension.c`, `scripts/publish/go/tag-and-push-go-module.sh`, `tools/e2e-generator/src/generators/mod.rs.bak`

## [1.0.0-rc.11] - 2026-03-19

### Added

- Multi-arch Docker build (linux/amd64 + linux/arm64)
- Full 19-test suite aligned across all 12 test apps (Python, Node, Ruby, Go, Java, C#, Elixir, PHP, WASM, C, Docker, Rust)
- Docker test app

### Fixed

- Node test: remove redundant `init()` call in download API tests
- WASM serde fix

## [1.0.0-rc.10] - 2026-03-19

### Added

- `json_utils` module with `snake_to_camel` / `camel_to_snake` conversion
- Node.js `process()` accepts both camelCase and snake_case config keys
- Expanded test suites: Ruby, Go, Elixir, Java, C#, PHP, WASM, C all at 19 tests

### Fixed

- Homebrew publish: git credentials for tap push
- `json_utils` gated behind `serde` feature flag
- `init()` accepts optional config, `process()` registers cache for download feature
- `cargo fmt` formatting

## [1.0.0-rc.9] - 2026-03-19

### Added

- Full documentation site with MkDocs Material (29 pages), GitHub Pages deploy
- 173-language grammar test fixtures
- Docker image build/publish to GHCR, ci-docker workflow
- `snippet-runner` tool with 33 code snippets for 11 languages
- `llms.txt` for AI assistant context
- All-grammars test suite

### Fixed

- Static linking symbol collisions: prefix scanner functions per language
- Enable grammar generation for batch/diff/pkl
- Homebrew upload checkout, RubyGems action, Docker clone_vendors
- Move Cargo profiles to workspace root
- Docker needs parsers from clone-vendors
- Ci-docker needs tree-sitter-cli for grammar generation
- Test_apps call `download()` before testing

### Removed

- Musl cross-compile from publish workflow

## [1.0.0-rc.8] - 2026-03-18

### Added

- Download/configure API across all bindings (Python, Node.js, Ruby, Go, Java, C#, Elixir, PHP, WASM, C FFI)
  - `init(config)` — configure + pre-download languages
  - `download(languages)` — download specific parsers
  - `download_all()` — download all 170+ parsers
  - `configure(config)` — set cache directory
  - `manifest_languages()` — list all available from remote manifest
  - `downloaded_languages()` — list locally cached parsers
  - `clean_cache()` — remove cached parsers
  - `cache_dir()` — get effective cache directory
- Auto-download in `get_language()` — parsers download on first use
- `PackConfig` struct with TOML file loading and directory discovery
- CLI redesign: `download`, `clean`, `list`, `parse`, `process`, `cache-dir`, `init`, `completions`
- Homebrew publishing pipeline (bottles, formula update via `kreuzberg-dev/homebrew-tap`)
- NAPI-RS multi-platform npm distribution (5 platform packages)
- Test apps for published package validation across 12 ecosystems
- E2E test fixtures for download API surface

### Fixed

- Memory leaks in Go, Java, C# bindings (individual strings in FFI arrays not freed)
- Python GIL not released during download/network I/O operations
- Elixir NIFs not scheduled as DirtyIo for network operations
- Java `parseString`/`process` passing `source.length()` instead of UTF-8 byte count
- Rust core `AtomicBool` ordering: `Relaxed` → `Acquire` in `ensure_cache_registered`
- C FFI `cache_dir` return type `*const c_char` → `*mut c_char`
- Elixir error atom: `parse_error` → `download_error` for download failures
- Platform detection: macOS aarch64 → `arm64` in `DownloadManager`
- Node.js npm publishing: multi-platform packages via `napi artifacts`
- Parser binary builds: `TSLP_LANGUAGES` set, correct output path
- `parsers.json` manifest generated and uploaded to GitHub releases
- `build.rs` graceful fallback for crates.io installs
- Maven GPG signing enabled in publish profile
- Ruby trusted publishing (gem name with underscores)

### Changed

- READMEs overhauled: correct badges (Homebrew, docs), download API docs, language-specific naming
- CLI binary name: `ts-pack`
- Dotnet target: 9.0 → 10.0
- Ruby minimum: 3.2 → 3.4
- Go minimum: 1.22 → 1.26
- Smoke tests removed from publish workflow (replaced by test_apps)

## [1.0.0-rc.7] - 2026-03-17

### Fixed

- NAPI-RS multi-platform npm distribution (5 platform packages)
- Platform detection: macOS aarch64 → `arm64` in `DownloadManager`
- Language discovery: `available_languages()` scans download cache directories
- Python test_app uses `ProcessConfig` instead of raw dicts

## [1.0.0-rc.6] - 2026-03-17

### Fixed

- Platform detection: macOS aarch64 → `arm64` for parser downloads
- Language discovery in extra library directories
- Smoke tests removed from publish workflow

## [1.0.0-rc.5] - 2026-03-16

### Fixed

- Parser binary upload: newline-separated artifact patterns
- Upload artifacts script: empty array initialization, comma-separated pattern support
- Smoke test jobs: added checkout steps

## [1.0.0-rc.4] - 2026-03-16

### Fixed

- Windows parser build: `setup-python` action added
- Maven Central: GPG signing enabled in publish profile (`gpg.skip=false`)
- Elixir Hex.pm: skip docs during publish for NIF packages

## [1.0.0-rc.3] - 2026-03-16

### Fixed

- `build.rs` graceful fallback when `sources/language_definitions.json` missing
- Parser binaries: `TSLP_LANGUAGES` set, correct output path for `.so`/`.dylib`
- `parsers.json` manifest generated and uploaded to GitHub releases
- Elixir Hex.pm: added LICENSE, removed `native` from files list
- Maven Central: profile renamed `release` → `publish`
- Python wheel: `.pyi` type stubs and `py.typed` marker (PEP 561)

### Added

- Test apps (`tests/test_apps/`) for validating published packages across 12 ecosystems
- Shared JSON test fixtures for cross-language test parity
- Version sync for test_app dependency manifests

## [1.0.0-rc.2] - 2026-03-16

### Fixed

- `build.rs` no longer panics on crates.io installs without `sources/language_definitions.json`
- PEP 440 version sync for Python
- `sync_versions.py` gemspec single-quote regex

### Added

- Test apps structure and shared fixtures
- Task definitions for test-apps:smoke and test-apps:comprehensive

## [1.0.0-rc.1] - 2026-03-09

Complete rewrite from Python to Rust with polyglot language bindings.

### Added

- Rust core library (`ts-pack-core`) with `LanguageRegistry` for thread-safe grammar access
- C-FFI layer (`ts-pack-ffi`) with cbindgen-generated headers and panic shields
- Python bindings via PyO3/maturin (`ts-pack-python`) with PyCapsule support
- Node.js bindings via NAPI-RS (`ts-pack-node`) with TypeScript definitions
- Go bindings via cgo (`ts-pack-go`) with platform-specific build directives
- Java bindings via Panama FFM (`ts-pack-java`) targeting JDK 22+
- Elixir bindings via Rustler NIF (`ts-pack-elixir`) with ExUnit tests
- CLI tool (`ts-pack-cli`) for grammar management (init, list, add, remove, info, build)
- E2E test generator with 7 language backends (Rust, Python, TypeScript, Go, Java, Elixir, C)
- 21 test fixtures across 4 categories (smoke, parsing, error handling, registry)
- Dynamic linking mode (`TSLP_LINK_MODE=dynamic`) for per-parser shared libraries
- Feature-gated language selection via `TSLP_LANGUAGES` env var or Cargo features
- Language group features: `web`, `systems`, `scripting`, `data`, `jvm`, `functional`
- Tree-sitter 0.26 support with `Language::into_raw()` / `Language::from_raw()`
- Domain-split CI workflows (ci-validate, ci-rust, ci-python, ci-node, ci-go, ci-java, ci-elixir, ci-c)
- Multi-registry publish workflow (crates.io, PyPI, npm, GitHub Releases for Go FFI)
- 168 language grammars supported

### Changed

- Architecture: Python-only package → Rust core with polyglot bindings
- Parser compilation: pure Python with tree-sitter CLI → Rust `build.rs` with `cc` crate
- Language registry: dictionary-based → typed `LanguageRegistry` with thread-safe `LazyLock` access
- Error handling: Python exceptions → Rust `Result<T, E>` with cross-language error conversion
- Repository moved from `Goldziher/tree-sitter-language-pack` to `kreuzberg-dev/tree-sitter-language-pack`
- Node.js package renamed to `@kreuzberg/tree-sitter-language-pack`
- Java groupId changed from `io.github.tree-sitter` to `dev.kreuzberg`
- Go module path updated to `github.com/kreuzberg-dev/tree-sitter-language-pack/go`
- README branding updated with kreuzberg.dev banner and Discord community link

### Removed

- Python-only implementation (setup.py, MANIFEST.in, tree_sitter_language_pack/)
- Direct tree-sitter Python dependency for parsing (now via native bindings)
- Cython-based build pipeline

---

## Pre-1.0 Releases (Python-only)

### [0.12.0]

#### Added

- Tree-sitter-cobol grammar support

#### Fixed

- MSVC build compatibility for cobol grammar
- Alpine Linux (musl) wheel platform tag support (PEP 656)
- Wheel file discovery in CI test action

### [0.11.0]

#### Added

- Tree-sitter-bsl (1C:Enterprise) grammar support

#### Changed

- Updated all dependencies and relocked

### [0.10.0]

#### Added

- Tree-sitter 0.25 support

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

- Tree-sitter-nim grammar support
- Tree-sitter-ini grammar support
- Swift grammar update (trailing comma support)

### [0.8.0]

#### Fixed

- Sdist build issues resolved

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
