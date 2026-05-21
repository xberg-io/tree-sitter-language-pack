# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## Unreleased

### Fixed

- **Download cache is now safe under concurrent multi-process access.** `DOWNLOAD_CACHE_LOCK` in `crates/ts-pack-core/src/lib.rs` was a `Mutex<()>` — intra-process only — so multi-worker servers (gunicorn / Puma / Node cluster), fan-out build pipelines (`make -j8`, parallel test runners), and the zig e2e suite (`zig build test` spawns eight test binaries in parallel) all raced on the same `~/.cache/tree-sitter-language-pack/v{version}/` directory. Partial `entry.unpack` writes were observable to other workers' `libloading::open`, producing intermittent `LanguageNotFound` / segfaults on first request for an uncached language; N processes could also each redundantly pull the 50MB platform bundle. Cache writes are now atomic (write to `<dest_dir>/.<name>.tmp.<pid>.<seq>` then `fs::rename` — readers see old, new, or nothing, never partial) and the bundle-fetch / extract / clean critical section is serialized across processes with an exclusive `fd-lock` on `<version_cache_dir>/.download.lock`. Double-checked locking preserves the lock-free hot path: steady-state `is_cached` lookups never pay the OS file-lock cost. New `Error::CacheLock(String)` variant surfaces lock-acquisition failures cleanly. Affects every binding (Python, Node.js, Ruby, PHP, Go, Java, C#, Elixir, WASM, Dart, Swift, Zig, Kotlin-Android) because the fix lives entirely in the shared `ts-pack-core` Rust crate. New `fd-lock = "4"` dependency (gated under the `download` feature). Cross-process safety relies on `flock` semantics, which are unreliable on NFS — users with `XDG_CACHE_HOME` on NFS should use a local-FS cache or serialize at the application layer. (`crates/ts-pack-core/src/{download.rs,error.rs}`, `crates/ts-pack-core/Cargo.toml`, `Cargo.toml`, new `crates/ts-pack-core/tests/concurrent_download.rs`)
- **Zig e2e auto-omits fixtures outside the static-compiled grammar set (regen on alef `65f1a129`).** Declared `[crates.zig].languages = [<curated 18-grammar list>]` mirroring the `TSLP_LANGUAGES` value in `[crates.test.zig].before`. Alef's new Zig codegen filter consults both `input.language` and `input.config.language` and drops fixtures whose target grammar is not in the list (mirroring the WASM `f9e0ff50` pattern). Eliminates `smoke_bibtex` and every other non-static-set test that previously failed at parser-load time. Also reverts the per-fixture `skip: { languages: ["zig"] }` workaround on `fixtures/smoke/actionscript.json` since the auto-omit subsumes it. (`alef.toml`, `fixtures/smoke/actionscript.json`)
- **swift e2e: `process` `contains` assertions on `Vec<DTO>` fields aggregate every stringy accessor (regen on alef `857c55d1`).** `testProcessPythonImportsDetail` and `testProcessRustStructureName` previously failed because the codegen relied on `result_field_accessor` naming a single "primary" accessor per array field (`imports → source`, `structure → kind`), which misses values surfaced on sibling fields — `"os"` against `ImportInfo.items`, `"MyConfig"` against `StructureItem.name` rather than `StructureKind`. The regenerated tests now emit a `contains(where: { item in … })` closure that gathers every text-bearing accessor (String, Option<String>, Vec<String>, serde-enum) into a `[String]` and substring-matches the expected value, mirroring python's `_alef_e2e_item_texts`. Swift e2e: 411 tests, 0 failures. (`e2e/swift_e2e/Tests/TreeSitterLanguagePackE2ETests/ProcessTests.swift`)
- **Maven JAR native layout collapses every classifier under `natives/native/` ([#128](https://github.com/kreuzberg-dev/tree-sitter-language-pack/issues/128)).** The re-stage loop in `build-maven-package` walked one `dirname` too far when extracting the classifier from each lib's path, so all six platform libs landed at `natives/native/{lib}` instead of `natives/{classifier}/{lib}`. The Maven Central JAR shipped in v1.8.1 contained only three files (one per `.so`/`.dylib`/`.dll` extension) and `TreeSitterLanguagePack.getParser("…")` failed with `UnsatisfiedLinkError: Expected resource: /natives/windows-x86_64/ts_pack_core_ffi.dll`. Fixed the path-walk depth, and hardened both build-side and deploy-side verification steps to require every `linux-x86_64 / linux-arm64 / macos-arm64 / macos-x86_64 / windows-x86_64 / windows-aarch64` classifier directory is present in the staged JAR so the regression cannot ship again. (`.github/workflows/publish.yaml`)
- **WASM e2e local-feasibility + auto-skip wiring.** `[crates.test.wasm].before` previously ran `wasm-pack build` with no `TSLP_LANGUAGES` set, which triggered a full 305-grammar static build — the 97MB `abl/parser.c` alone hangs clang at -O2 for tens of minutes. Mirrored the publish-wasm CI environment locally: `TSLP_LINK_MODE=static TSLP_LANGUAGES=<curated 31-grammar list> CARGO_PROFILE_RELEASE_LTO=false CARGO_PROFILE_RELEASE_CODEGEN_UNITS=16`. Also declared `[crates.wasm].languages = [<same list>]` so alef's wasm e2e auto-skip path correctly elides 268 of the 302 smoke tests for grammars not in the bundle (with the matching alef `f23ae5d3` / `f9e0ff50` fixes that teach the wasm filter to look up both `input.language` and `input.config.language`). (`alef.toml`)
- **Regen on alef HEAD (csharp List<string>, go os import, php deterministic accessor ordering, swift codegen trifecta).** Pulls in upstream alef fixes: `4f6a9056` csharp List<string> emission for `mock_url_list`; `06caa440` go `os` import include guard for `mock_url_list`; `1fde7aae` PHP deterministic accessor extraction order (HashMap→BTreeMap; resolves the recurring `$imports`/`$structure` flip in `e2e/php/tests/ProcessTest.php`); `13717e24` swift e2e — trailing `()` on scalar accessors that bridge through opaque structs, drop spurious `?.map ... ?? []` on non-optional `RustVec` accessors, and camelCase swift-bridge method names (e.g. `asStr()` not `as_str()`); plus the wasm `input.config.language` filter follow-up cited above. (`e2e/php/**`, `e2e/swift_e2e/**`, `e2e/wasm/**`, `e2e/zig/**`)
- **npm darwin-x64 NAPI binary missing ([#127](https://github.com/kreuzberg-dev/tree-sitter-language-pack/issues/127)).** `crates/ts-pack-core-node/package.json#napi.targets` already listed `x86_64-apple-darwin`, but the `build-node-native` matrix in `.github/workflows/publish.yaml` omitted the `macos-15-intel` runner — so v1.8.0 / v1.8.1 npm tarballs shipped without `ts-pack-core-node.darwin-x64.node`, breaking `require('@kreuzberg/tree-sitter-language-pack')` on Intel Macs. Added a `macos-15-intel` / `darwin-x64` / `x86_64-apple-darwin` row to the matrix, mirroring the parity already present in the Python/Ruby/Java/Go publish matrices. The next published version (≥1.8.2) will include the darwin-x64 binary. (`.github/workflows/publish.yaml`)
- **Regen on alef v0.17.13.** Pulls in four upstream fixes since v0.17.11: `fix(alef-e2e/rust): unwrap Option<scalar> leaf fields in numeric comparison assertions` (the three `greater_than` / `less_than` / `less_than_or_equal` operators no longer fail to compile when the leaf field is `Option<T>`), `fix(alef-e2e/rust): use serde_json::from_str instead of json! macro for fixture json_object args` (sidesteps the macro recursion-limit on fixtures with large JSON payloads), `fix(alef-backend-php): emit Box::default() instead of Box::new(Default::default()) for boxed fallback fields` (resolves `clippy::box-default` -D warnings on the PHP umbrella crate), and `feat(alef-core,alef-e2e/wasm,alef-e2e/typescript): auto-skip wasm fixtures outside the static-compiled language set` (foundational for tslp's curated wasm32 builds; no-op for now since `[crates.wasm].languages` is empty, but unlocks the future curated-build flow). Side effects in this regen: a few Rust e2e fixture bodies re-formatted, `e2e/c/main.c` cosmetic update, and `packages/swift/rust/Cargo.toml` deps re-ordered. (`alef.toml`, `e2e/{c,php,rust}/**`, `packages/swift/rust/Cargo.toml`)
- **CI E2E (.NET) lib-path block uses grouped redirect.** `shellcheck SC2129` flagged four consecutive `echo … >> "$GITHUB_ENV"` lines in the Set library paths for .NET step; consolidated into a single grouped `{ … } >> "$GITHUB_ENV"` block to keep actionlint clean on the workflow. (`.github/workflows/ci-e2e.yaml`)
- **Pin alef to v0.17.10.** Bumps `alef_version` in `alef.toml` and the alef pre-commit-hook rev. Lands the Phase-5 leakage-sanitizer chain plus follow-up codegen fixes: v0.17.4 csharp/elixir/kotlin/swift codegen-consumer unblocks; v0.17.5 NAPI/PHP/Java docstring sanitizer wiring; v0.17.7 sanitizer recognises rustdoc test-attribute fences (` ```no_run `, ` ```ignore `, ` ```should_panic `, ` ```compile_fail `, ` ```edition* `) as Rust code (so their bodies are dropped for foreign-language targets); v0.17.8/v0.17.9 csharp U1-bool P/Invoke call-site fix; v0.17.10 Swift free-function forwarder fixes — `Option<String>` returns now use `?.toString()` and host DTO args flow through `.intoRust()` before the bridge call, so `detectLanguageFromExtension/Path/Content`, the `*Query` getters, and `process(_:config:)` compile and execute against the high-level Swift API. Downstream surface: 61 Rust-code-block leaks in `crates/ts-pack-core-node/index.d.ts` and 20+ in `crates/ts-pack-core-php/src/lib.rs` collapse to 0 after this regen.
- **Rust e2e `chunks` undefined.** `e2e/rust/tests/process_test.rs` four `test_*_chunking_*` cases were emitting `assert!(chunks.len() >= 2 as usize, ...)` where `chunks` was undeclared (E0425). Same class of bug as the PHP `$chunks` fix; alef's Rust e2e codegen unconditionally fired the streaming-virtual-field assertion arm for `chunks`/`imports`/`structure` even for non-streaming fixtures. Fix pulled in via alef `a32ca2a0 fix(rust-gen): bind fields_array accessor before len() assertion in e2e tests` — non-streaming fixtures with a colliding `fields_array` field now emit a leading `let {field} = &{result}.{field};` binding.
- **`e2e/node` `tree-sitter` dev-dep restored (recurring).** `alef generate` strips `tree-sitter@^0.25.0` from `e2e/node/package.json` on every regen, but `tests/capsule_passthrough.test.ts` imports it to verify FFI capsule type-tag pass-through between our `Language` object and the upstream tree-sitter Node native module. Hand-restored, alongside the corresponding `pnpm-lock.yaml` rows.
- **Subsequent regen on top of alef Swift API tightening.** Pulls in alef `2eaa260a fix(swift): hide RustVec/RustString/intoRust from public API; convert at forwarder boundaries` plus a handful of smaller adapter fixes (`fix(alef-backend-pyo3)`, `fix(alef-backend-napi,wasm)`, `fix(alef-backend-ffi)` clippy). Public Swift API surface no longer leaks `RustVec`/`RustString`/`intoRust()`; conversion happens at forwarder boundaries inside generated extensions.
- **CI green-up.** Regenerated `pnpm-lock.yaml` to drop the stale `e2e/node → tree-sitter@^0.25.0` devDependency that broke `pnpm install --frozen-lockfile` in `CI Validate`. Regenerated the `docs/reference/api-*.md` set so committed output matches `alef docs` (compact Markdown tables) and `alef verify` stays green on `main`.
- **Full alef regen on top of upstream codegen fixes.** Pulls in three alef fixes: `fix(swift): enum intoRust(), Ref→owned init, Vec<RustString> elem type` (Swift CI was failing on `CommentKind.intoRust()`, `RustStringRef.toString()`, `RustVec<String>` not conforming to `Vectorizable`); `fix(php-gen): bind fields_array accessor before count() assertion in e2e tests` (PHP e2e `test_*_chunking_*` cases were referencing undefined `$chunks`); `fix(alef-backend-go): null-check and box Option<String> returns instead of dereferencing` (generated `packages/go/binding.go` was returning `C.GoString(ptr)` where the signature expected `*string`, breaking `golangci-lint` and `govulncheck`). Side-effect: API docstrings now elide Rust-style `[Type]`-link syntax (e.g. PHP `Node.php` doc comments now read `A single syntax node within a 'Tree'` instead of `A single syntax node within a [`Tree`]`).
- **WASM yuck grammar marked unsupported.** `tree-sitter-yuck` produces `RuntimeError: unreachable` when parsing under wasm32 (same class of bug as zig/ziggy, which already skip on wasm). `fixtures/smoke/yuck.json` now carries `skip: { languages: ["wasm"] }`; `alef e2e generate` removed the corresponding test from `e2e/wasm/tests/smoke.test.ts`. Native bindings remain unaffected.
- **`package.json` pnpm-field cleanup.** Removed the now-ignored `pnpm.onlyBuiltDependencies` block from the root `package.json`. pnpm 11 reads that setting from `pnpm-workspace.yaml` (which already declares the same allowlist); the duplicate field made pnpm emit a warning on every install.
- **Downloader now honours the host OS trust store by default ([#125](https://github.com/kreuzberg-dev/tree-sitter-language-pack/issues/125)).** Manifest and bundle downloads from `github.com/kreuzberg-dev/tree-sitter-language-pack/releases/...` previously used ureq 3.x's default rustls agent, which trusts only the bundled Mozilla webpki roots and ignores the platform store. On Linux/WSL2 hosts where GitHub HTTPS traffic is presented with a chain rooted in a locally trusted (corp / private) CA — and where `curl`, `pip`, and `git` all succeed against the same URL via the OS trust store — first-use parser downloads failed with `DownloadError: ... io: invalid peer certificate: UnknownIssuer`. The downloader now constructs a configured `ureq::Agent` with `RootCerts::PlatformVerifier` by default (via `rustls-platform-verifier`), matching the behaviour of every other host-trust-aware HTTP client on the system. Set `TREE_SITTER_LANGUAGE_PACK_TLS_ROOTS=webpki` to opt back into ureq's bundled Mozilla roots; set `TREE_SITTER_LANGUAGE_PACK_TLS_ROOTS=platform` to make the default explicit. Affects every binding (Python, Node.js, Ruby, PHP, Go, Java, C#, Elixir, WASM, Dart, Swift, Zig, Kotlin-Android) because the fix lives entirely in the shared `ts-pack-core` Rust crate. (`crates/ts-pack-core/src/{download.rs,pack_config.rs}`, workspace `Cargo.toml`)

### Removed

- **`wolfram` grammar dropped from the language pack.** `tree-sitter-wolfram` produces glibc heap corruption (`free(): invalid next size`) when parsing trivial input under serial test execution on Linux; macOS allocator silently tolerated the corruption. The entire upstream ecosystem is unmaintained (canonical `bostick/tree-sitter-wolfram` last touched 2021-11-11 with 3 stars; every known fork — `LumaKernel`, `LoganAMorrison`, `JuanG970`, `jakassebaum` — ships the same `LANGUAGE_VERSION 13` parser tables and is inactive). Rather than fork-and-maintain a Wolfram grammar in-house for marginal demand, the entry is removed from `language_definitions.json`, all CI `TSLP_LANGUAGES` lists, the smoke fixture, the e2e harness, the docs, and the README ecosystem listings. **Total supported grammar count drops from 306 to 305**, which matches the long-standing "305 languages" marketing copy (previously off-by-one due to the broken wolfram entry).

### Changed

- **Split pub.dev publish into a dedicated `publish-pubdev.yaml` workflow triggered by `push: tags: v*`.** pub.dev OIDC trusted publishing rejects tokens from `release` events; only `push` and `workflow_dispatch` events are accepted. The new workflow produces an accepted token. One-time setup required: configure pub.dev → tree_sitter_language_pack package → Admin → Automated publishing with workflow path `.github/workflows/publish-pubdev.yaml`. (`.github/workflows/publish-pubdev.yaml`, `.github/workflows/publish.yaml`)
- Regenerated all alef-managed surfaces (per-binding READMEs, API reference docs, generated bindings, e2e tests) and the script-managed docs/languages.md + `_supported_languages.py` to reflect the 305-grammar count.
- `scripts/generate_grammar_table.py` default output path corrected from `docs/supported-languages.md` to the canonical nav-referenced `docs/languages.md`; Taskfile `docs:generate:languages` `generates:` field updated to match.

## 1.9.0

### Added

- Four new language bindings via alef 0.16.6, taking total binding count from 10 to 14:
  - **Dart / Flutter** — `dart pub add tree_sitter_language_pack`. Built with flutter_rust_bridge for isolate-safe Future APIs.
  - **Kotlin (Android)** — `dev.kreuzberg.tslp:tslp-android` AAR on Maven Central. JNI-based with per-ABI native libraries (arm64-v8a, armeabi-v7a, x86_64, x86). JVM Kotlin users continue to consume the canonical Java / Panama-FFM package.
  - **Swift** — `TreeSitterLanguagePack` via SwiftPM. swift-bridge for macOS, iOS, and Linux.
  - **Zig** — `zig fetch --save <tarball-url>` from GitHub Releases. Direct C FFI via `@cImport`.
- Two new Rust binding crates: `tree-sitter-language-pack-dart` (FRB bridge) and `tree-sitter-language-pack-swift` (swift-bridge).
- Hand-written `crates/ts-pack-core-jni` Rust crate exporting `Java_...` JNI symbols for the Kotlin-Android binding (excluded from the default workspace build because it cross-compiles via `cargo ndk`).
- Per-language CI workflows: `ci-zig.yaml`, `ci-swift.yaml`, `ci-dart.yaml`, plus a combined `ci-mobile.yaml` covering Android cross-compile + iOS cargo check.
- Publish jobs for pub.dev (`publish-pub`), Swift Package Index (`publish-swift`), Zig (`publish-zig` → GitHub Release tarball), and Maven Central kotlin-android (`publish-kotlin-android`).

## 1.8.1 - 2026-05-13

### Added

- E2E fixture coverage for: language alias resolution (`shell→bash`) via `has_language` / `get_language` / `get_parser` (3 fixtures); `download` edge cases — empty list, multiple-language, and unknown-language error path (3 fixtures); error-handling for 120KB sources and `get_language("")` (2 fixtures); and TypeScript function parsing (1 fixture). Brings fixture count from 403 to 412, covering 100% of the public `download`, `get_*`, and `has_language` surface across all 10 language bindings.

### Fixed

- Node: `getLanguage(name)` now returns a real `tree-sitter` `Language` that `new Parser().setLanguage(lang)` accepts at runtime. The previous capsule shim used `napi::bindgen_prelude::External::new` (rejected by `node-tree-sitter`'s `UnwrapLanguage`), wrote the External to `__parser`, and did not type-tag the value. Adopts alef [v0.15.49](https://github.com/kreuzberg-dev/alef/releases/tag/v0.15.49) where the napi capsule codegen emits raw `napi_create_external` + `napi_type_tag_object` and reads `property_name`/`type_tag` from `[crates.node.capsule_types]`.
- Python: `PackConfig` and `ProcessConfig` type hints now resolve to the `.options` dataclasses, fixing `mypy --strict` errors at every `init(...)` / `process(...)` call site (adopts alef [#72](https://github.com/kreuzberg-dev/alef/issues/72)).

- Python: restore `SupportedLanguage` as `Literal[...]` of all 306 grammars at `tree_sitter_language_pack.SupportedLanguage`. The symbol was dropped during the alef 0.15.x codegen migration and re-importing it raised `ImportError` in 1.8.0 (#121).
- Python: `get_parser("python").parse(b"...")` returns a real `tree_sitter.Tree` again instead of raising `AttributeError`. `get_parser` / `get_language` now return native `tree_sitter.Parser` / `tree_sitter.Language` instances via PyO3 capsule pass-through (alef v0.15.39 wires `capsule_types` through `gen_bindings`) (#121).

### Changed

- CI pinned to Node 22 LTS across all workflows. `tree-sitter@0.25.0` (the `tree-sitter` npm package) ships a `binding.cc` written against pre-C++20 stdlib (no `std::ranges`, `concept`, `requires`) and fails to compile against Node 24/26's V8 headers. Node 22 is the latest supported runtime until upstream `node-tree-sitter` updates its `cflags_cc` or ships prebuilds.
- CPD pre-commit hook and `packages/java/pom.xml` `maven-pmd-plugin` minimum-tokens bumped from 100 → 250: alef's java codegen emits ~200-token `try`/`catch` cleanup blocks on `DownloadManager` / `LanguageRegistry`. Refactoring the codegen to share a helper is tracked separately.

## 1.8.0 - 2026-05-09

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

## 1.7.0 - 2026-04-22

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

## 1.6.3 - 2026-04-20

### Fixed

- Go: fix FFI build defaults — add `TSLP_LINK_MODE` and `TSLP_LANGUAGES` env vars to Go task (#102)
- Go: fix CGO `LDFLAGS` paths — point to workspace `target/release/` instead of crate-local path (#102)
- Go: remove duplicate forward declarations from `ffi.go` (already in `ts_pack.h`) (#102)
- Go: fix README examples — proper error handling, correct API signatures (`Init`, `Download`) (#102)
- FFI: add extra libs dir from `cache_dir()` to registry on creation (#102)
- Docs: fix textlint pre-commit hook — add `additional_dependencies` for all textlint plugins (#102)

## 1.6.2 - 2026-04-18

### Fixed

- Compile bundled grammars with `-fno-strict-aliasing` to prevent undefined behavior (#100)

### Changed

- Update dependencies across lockfiles
- Regenerate READMEs for 1.6.1 version bump (#101)

## 1.6.1 - 2026-04-17

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

## 1.6.0 - 2026-04-14

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

## 1.5.0 - 2026-04-08

### Added

- 57 new permissively-licensed grammars — 305 languages total
  - abl, c3, cel, cfml, chuck, cst, dhall, elvish, gap, gdshader, glimmer, gnuplot, gotmpl, gowork, gpg, hjson, hocon, hoon, htmldjango, jai, javadoc, json5, kcl, mlir, nasm, norg_meta, ocamllex, openscad, phpdoc, poe_filter, prql, rasi, razor, rbs, roc, rtf, slang, smalltalk, sml, snakemake, souffle, sourcepawn, sql_bigquery, stan, superhtml, sway, systemverilog, tact, tera, typespec, typoscript, vhs, vrl, wgsl_bevy, x86asm, ziggy, ziggy_schema
- CI license validation job in `ci-validate.yaml` — blocks PRs that introduce non-permissive (GPL/AGPL/LGPL/MPL) grammars

### Fixed

- `less` grammar: regenerated parser from ABI 11 to ABI 14 (was incompatible with tree-sitter 0.26)
- `corn` smoke fixture: replaced invalid `"x"` snippet with valid corn syntax

## 1.4.1 - 2026-03-31

### Fixed

- Include `language_definitions.json` in the published crate so `build.rs` can find extension mappings, ambiguity data, and C symbol overrides when installed from crates.io

### Changed

- Updated dependencies across all language ecosystems

## 1.4.0 - 2026-03-29

### Fixed

- Expose `detect_language` in Python public API (#85)
- PHP extension name corrected to `ts-pack-php` (hyphens)

### Changed

- All language snippet READMEs and documentation corrected
- Removed automated grammar updates workflow

## 1.3.3 - 2026-03-27

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

## 1.3.2 - 2026-03-26

### Fixed

- Dynamic parser loading for languages with `c_symbol` overrides (`csharp`, `vb`, `embeddedtemplate`, `nushell`) — build was naming libraries with the raw name but runtime loader expected the `c_symbol` name (#80)
- Go E2E generator: unused `tspack` import in non-process test files
- Elixir: add missing `extract/2` and `validate_extraction/1` NIF declarations
- PHP E2E generator: use double-quoted strings for source code so `\n` is interpreted correctly
- Nim grammar: switch from abandoned `paranim/tree-sitter-nim` (ABI v11) to `aMOPel/tree-sitter-nim` (MIT, ABI v14)

### Added

- Smoke test fixtures for all `c_symbol` override languages (csharp, vb, embeddedtemplate, nushell)
- Dynamic-linking CI step in `ci-all-grammars.yaml` to catch `c_symbol` naming mismatches

## 1.3.1 - 2026-03-26

### Fixed

- Ruby binding: `process()`, `extract()`, `validate_extraction()` now return native Ruby Hash instead of raw JSON string
- WASM binding: output keys now use camelCase (matching Node.js binding convention), input config accepts both camelCase and snake_case
- Go E2E generator: use typed `*ProcessResult` struct fields instead of invalid `json.Unmarshal` on non-string return
- Elixir CI: stage NIF with both hyphenated and underscored filenames to satisfy Rustler force-build check and `load_from` loader

## 1.3.0 - 2026-03-26

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

## 1.2.1 - 2026-03-25

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

## 1.2.0 - 2026-03-25

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

## 1.1.4 - 2026-03-24

### Added

- New language: `al` (AL / Business Central) — 198 languages total
- Grammar license linter (`scripts/lint_grammar_licenses.py`, `task lint:licenses`) verifies all grammars use permissive licenses
- Permissive license policy documented in CONTRIBUTING.md, docs, and README

### Fixed

- Replace `nim` grammar (alaviss, MPL-2.0 copyleft) with paranim/tree-sitter-nim (MIT)
- Replace `prolog` grammar (codeberg foxy, AGPL-3.0 copyleft) with Rukiza/tree-sitter-prolog (ISC)
- Docs: align mkdocs config with kreuzberg branding; mermaid diagrams now render (fixes [#81](https://github.com/kreuzberg-dev/tree-sitter-language-pack/issues/81))

## 1.1.3 - 2026-03-24

### Fixed

- Dynamic loader: resolve `c_symbol` overrides for csharp, embeddedtemplate, and vb so `get_language()` works for dynamically loaded grammars (fixes [#80](https://github.com/kreuzberg-dev/tree-sitter-language-pack/issues/80))
- E2E generator: enable all ProcessConfig features (structure, imports, exports, comments, docstrings, symbols, diagnostics) for intel tests so diagnostics assertions pass

### Added

- 23 new smoke test fixtures for languages missing coverage: asciidoc, awk, batch, caddy, cedar, cedarschema, csharp, devicetree, diff, dot, embeddedtemplate, idris, jinja2, jq, lean, pkl, postscript, prolog, rescript, ssh_config, textproto, tlaplus, vb, wit, zsh
- CI workflow (`ci-all-grammars.yaml`) that tests all 197 grammars end-to-end, preventing regressions like #80
- `rust:e2e:all-grammars` task for running the full grammar suite locally

## 1.1.2 - 2026-03-23

### Fixed

- Elixir NIF: fix Rustler crate name mismatch (`ts_pack_elixir` → `ts-pack-elixir`) causing compilation failure
- Rust crate publish: embed query file contents at build time instead of using `include_str!` with relative paths that break in the cargo package tarball

## 1.1.1 - 2026-03-23

### Fixed

- WASM build: ahash uses compile-time-rng instead of runtime-rng (avoids getrandom on wasm32)
- Docker/static build: add `c_symbol` override for grammars with non-standard C symbol names (csharp, vb, embeddedtemplate)
- Unused imports when `dynamic-loading` feature disabled (WASM builds)
- Python sdist: `.pyi` and `py.typed` now included in both wheel and sdist
- C# build: add missing `ExtensionAmbiguityResult` model class
- Set `generate: true` for csharp, vb, embeddedtemplate grammars

### Changed

- Switch from `std::HashMap`/`HashSet` to `ahash::AHashMap`/`AHashSet` for faster hashing in registry

## 1.1.0 - 2026-03-23

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

## 1.0.0 - 2026-03-21

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

### 0.12.0

#### Added

- tree-sitter-cobol grammar support

#### Fixed

- MSVC build compatibility for cobol grammar
- Alpine Linux (musl) wheel platform tag support (PEP 656)
- Wheel file discovery in CI test action

### 0.11.0

#### Added

- tree-sitter-bsl (1C:Enterprise) grammar support

#### Changed

- Updated all dependencies and relocked

### 0.10.0

#### Added

- tree-sitter 0.25 support

#### Changed

- Dropped Python 3.9 support
- Adopted prek pre-commit workflow
- CI: cancel superseded workflow runs

### 0.9.1

#### Added

- WASM (wast & wat) grammar support
- F# and F# signature grammar support

### 0.9.0

#### Added

- tree-sitter-nim grammar support
- tree-sitter-ini grammar support
- Swift grammar update (trailing comma support)

### 0.8.0

#### Fixed

- sdist build issues resolved

### 0.7.4

#### Added

- GraphQL grammar support
- Kotlin grammar support (SAM conversions)
- Netlinx grammar support

### 0.7.3

#### Changed

- Swift grammar update (macros + copyable)

### 0.7.2

#### Added

- Apex grammar support

#### Fixed

- MSYS2 GCC build issues

### 0.7.1

#### Added

- OCaml and OCaml Interface grammar support
- Markdown inline parser support

#### Fixed

- Pinned elm and rust grammar versions
- Pinned tree-sitter-tcl to known-good revision

### 0.6.1

#### Added

- ARM64 Linux CI builds

#### Fixed

- Build issue resolved

### 0.6.0

#### Fixed

- Windows DLL loading compatibility issues

### 0.5.0

#### Fixed

- Windows compatibility and encoding issues for non-English locales

### 0.4.0

#### Added

- PyCapsule-based language loading
- Protocol Buffers (proto) grammar support
- SPARQL grammar support

### 0.3.0

#### Changed

- Updated generation setup and build matrix
- Removed magik and swift grammars (temporarily)

### 0.2.0

#### Changed

- Version bump with dependency updates

### 0.1.2

#### Fixed

- Added MANIFEST.in for sdist packaging

### 0.1.1

#### Fixed

- Missing parsers in package data

### 0.1.0

#### Added

- Initial release with 100+ tree-sitter language grammars
- Python package with pre-compiled parsers
- Multi-platform wheel builds (Linux, macOS, Windows)
