# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [1.10.9] - 2026-06-24

### Fixed

- **Kotlin/JVM: `Tree.walk()` (and other handle-returning calls) no longer crash the JVM.**
  Opaque handle types (`Tree`, `Node`, `TreeCursor`) crossed the JNI boundary as `String`/JSON
  while the Rust shim returned a raw `jlong`, so the JVM dereferenced a primitive as an object
  reference and faulted with `EXCEPTION_ACCESS_VIOLATION`. Regenerated with alef 0.27.1, the
  kotlin-android bridge now returns primitive `Long` handles (required and optional, via a `0L`
  sentinel) and constructs the wrapper directly. Fixes #146.
- **Python: exported exception classes are now catchable.** `get_language("unknown")` raised
  `_native.DownloadError`, a different class object than the `DownloadError` exported from the
  package, so `except DownloadError:` never caught it. Regenerated with alef 0.27.1, the native
  variants derive from the native base `Error` and the package re-exports the native classes
  (with matching type stubs), so `except DownloadError:`/`except Error:` work. Fixes #147.

## [1.10.8] - 2026-06-24

### Fixed

- **wasm32 builds no longer OOM on pathologically large grammars.** Compiling the bundled grammars
  to `wasm32` previously included every `parser.c`, but a few are huge *generated* sources (e.g.
  `abl` at ~130 MB) that need 18-25 GB＋ of clang RAM at *any* optimization level — a single one
  OOMs standard ≤16 GB CI runners (serialization via `CARGO_BUILD_JOBS=1` cannot help when one file
  alone exceeds the budget). `build.rs` now skips any grammar whose `parser.c` exceeds a size limit
  on wasm32 (default 40 MB, configurable via `TSLP_WASM_MAX_PARSER_BYTES`; `0` disables the gate),
  emitting a `cargo:warning` per skipped grammar plus a summary. Skipped grammars are absent from
  `STATIC_LANGUAGES` (no dangling FFI symbol) and degrade gracefully at runtime. The 40 MB default
  keeps every common language (including the ~40 MB `sql` grammar) and excludes only the handful of
  unbuildable outliers (`abl`, `systemverilog`, `razor`, `fsharp`, `verilog`, `gnuplot`, `latex`).
  (`crates/ts-pack-core/build.rs`)

- **Swift publish now creates the `release/swift/<version>` branch carrying the substituted
  XCFramework checksum.** The alef-generated Swift e2e/test-app pins
  `.package(url: …, branch: "release/swift/<version>")` (the non-destructive layout shared with the
  other polyglot repos), but the publish workflow only force-moved the `v<version>` tag and never
  created that branch — so SwiftPM could not resolve the package and the Swift test-app failed with
  an empty `TreeSitterLanguagePack` target. The checksum commit is now also pushed to
  `refs/heads/release/swift/<version>`. (`.github/workflows/publish.yaml`)

## [1.10.4] - 2026-06-22

### Added

- **`ts-pack mcp` server now exposes MCP resources, prompts, and argument completions** in addition
  to its tools. Resources serve the language catalog (`ts-pack://languages`,
  `ts-pack://languages/downloaded`) and a per-language template (`ts-pack://language/{name}`); a
  ready-made `analyze-code` prompt drives a structure/imports/symbols analysis workflow; and
  language-name arguments autocomplete against the available-language catalog.

### Changed

- **`ts-pack mcp` tools are now fully aligned with the CLI and carry accurate rmcp annotations.**
  The `download` tool takes `groups` (multiple) and `fresh` like `ts-pack download`; `process` gains
  the `all` flag; the combined `cache` tool is split into `cache_dir` (read-only) and `clean_cache`
  (destructive), mirroring the CLI. Every tool now declares correct `open_world_hint`,
  `read_only_hint`, `destructive_hint`, and `idempotent_hint` values.
- **The CLI ships the MCP server by default.** `mcp` is now a default feature of `ts-pack-cli`, so
  `ts-pack mcp` is present in every distribution — `cargo install ts-pack-cli`, the prebuilt release
  binary, Homebrew, and the `@kreuzberg/ts-pack-cli` / `ts-pack-cli` npx/uvx proxies. Previously the
  feature was opt-in and absent from shipped binaries, which also broke the marketplace plugin
  launcher that invokes `ts-pack mcp`.

### Fixed

- **Host-native `get_language()` passthrough now works for Swift, Kotlin-Android, and Java.** The
  capsule passthrough (#143) returns the ecosystem's native `Language`, but the bindings did not wire
  the host-runtime dependency: Swift generated an uncompilable forwarder (wrong return type, missing
  `import SwiftTreeSitter`); Kotlin-Android declared `ktreesitter` as `implementation`, hiding the
  `Language` type from callers' compile classpath; and Java's `jtreesitter` (Panama FFM) dlopens the
  standalone `libtree-sitter` runtime, which CI and the test harness did not provision. Fixed via
  alef 0.26.1 (Swift/Kotlin codegen) plus `libtree-sitter` provisioning in CI and the Java test
  harness. Zig already wired its `tree_sitter` module correctly.
- **`.app.src` files now map to Erlang.** The application-resource template is Erlang term syntax,
  but single-extension lookup only saw `src`. A compound-extension table now resolves `*.app.src` to
  the Erlang grammar.

## [1.10.3] - 2026-06-22

### Changed

- **Regenerated all bindings with alef 0.26.0.** Picks up alef's sync-versions byte-stability fix:
  `sync-versions` no longer rewrites externally-formatted scaffold/manifest files (this repo formats
  via external tools with `[workspace.format] enabled = false`), and it preserves external SwiftPM
  dependency pins instead of clobbering them with the workspace version. This clears the CI
  version-sync freshness gate.
- **Updated dependencies within their current major versions** (Rust crates, PHP dev tooling, pnpm
  toolchain pin).

## [1.10.2] - 2026-06-22

### Fixed

- **Generated binding doc comments no longer emit Rust intra-doc link syntax.** alef copied core
  rustdoc comments verbatim into the per-language binding crates, carrying `[`Type`]` /
  `[`fn`](crate::fn)` intra-doc links that resolve in the core crate but break `cargo doc` in the
  binding crates with `rustdoc::broken-intra-doc-links`. The references are now de-linked to plain
  code spans (`` `Type` ``) during emission, preserving genuine URL/anchor Markdown links. Picked up
  from the alef 0.25.60 regen.

## [1.10.1] - 2026-06-20

### Fixed

- **Java: fixed a JVM crash (`EXCEPTION_ACCESS_VIOLATION`) when traversing a parsed tree via opaque
  handles (#146).** `Parser.parse`, `Tree.walk`, `Tree.rootNode`, `Node.parent`/`child`, and
  `TreeCursor.node` freed the returned native handle in a `finally` block immediately after wrapping
  it, so the returned `Tree`/`Node`/`TreeCursor` referenced already-freed memory and the next native
  call dereferenced it and crashed. The wrapper now owns the handle and frees it once on `close()`.
  Fixed in the alef Java backend and picked up by the 0.25.55 regen; value/DTO returns
  (`byteRange`/`startPosition`/`process`) still correctly free the FFI temporary after reading it.

### Changed

- **chore(precommit,alef): standardize kotlin-android formatting on ktfmt --kotlinlang-style.** Drop the conflicting prek ktlint hook, scope detekt/ktfmt to `packages/kotlin-android`, add `--kotlinlang-style` to ktfmt, switch `alef.toml` kotlin format/check from gradle-ktlintFormat to ktfmt so alef and prek agree, and exclude the vendored Gradle wrapper from shellcheck. detekt remains for static analysis. (`.pre-commit-config.yaml`, `alef.toml`)

### Added

- **Host-native `Language` passthrough across the C-ABI binding family (#143).** `get_language`
  now returns each ecosystem's native tree-sitter `Language` instead of an opaque alef handle, so
  the result drops straight into the host runtime's parser: Go (`*tree_sitter.Language` via
  `go-tree-sitter`), Zig (`?*const tree_sitter.Language` via `zig-tree-sitter`), Java
  (`jtreesitter.Language`), C# (`TreeSitter.Language`), Kotlin Android (`ktreesitter.Language`),
  and Swift (`SwiftTreeSitter.Language`) — joining the existing Python and Node passthrough. Each
  binding gained a dependency on its host tree-sitter runtime, injected into the generated manifest.
  Configured via `[crates.*.capsule_types.Language]` in `alef.toml`; regenerated against alef 0.25.55.

### Changed

- **Regenerated all bindings against alef 0.25.55.** The C FFI crate now takes a direct
  `tree-sitter` dependency so the capsule shim can name `tree_sitter::ffi::TSLanguage` (the pointee
  it casts `value.into_raw()` to), and the zig `build.zig.zon` carries the resolved `zig-tree-sitter`
  content hash.

## [1.9.1] - 2026-06-18

### Fixed

- Swift: restored the public `getLanguage(name:)` function in the `TreeSitterLanguagePack`
  module. An alef 0.25.38 codegen regression added opaque types to the Swift forwarder
  exclusion set, dropping `get_language` (the only free function returning the opaque
  `Language` type) from the generated public API in v1.9.0. Regenerated against alef 0.25.43.

## [1.9.0] - 2026-06-18

### Changed

- **Bumped `alef` pin 0.25.28 → 0.25.38 and regenerated all bindings.** Picks up alef 0.25.29–0.25.38: enum associated (static factory) methods surfaced across backends, the swift opaque no-op shim so `$_free` is synthesised for handle types with no visible methods (e.g. `Language`), swift streaming-owner `already_declared` re-declaration, java `marshal_optional_bytes` template registration, and java `@Nullable` type-use placement on qualified types.
- **Upgraded all dependencies to their latest versions (cross-major).** Ran `task upgrade` across every language workspace; lock files regenerated and committed.
- **Repo hygiene.** Ignore the machine-local `packages/kotlin-android/.gradle/` cache and the `.basemind/` index (untracking the accidentally-committed Gradle cache files), and exclude the deterministic `.ai-rulez/.generated-manifest.json` from the `oxfmt` pre-commit hook so it no longer fights the `ai-rulez-generate` hook.

### Fixed

- **Java: dropped throwing `UnsupportedOperationException` stubs for `Self`-returning DTO/enum methods.** There is no JNI/FFM symbol for DTO methods yet, so the throwing stubs compiled but misled callers and broke any path that reached them. The Java backend now skips these methods until marshaling lands.
- **Java: restored the `true` default for boxed `@Nullable Boolean` `#[serde(default)]` record fields.** A non-optional `#[serde(default)] bool = true` field is boxed to `@Nullable Boolean`, so JSON that omitted it deserialised to `null` and the accessor returned `null` instead of `true`.

## [1.9.0-rc.55] - 2026-06-17

### Changed

- **Bumped `alef` pin 0.25.24 → 0.25.28.** Regenerated all bindings via `task alef:generate`. Picks up alef 0.25.25–0.25.28: scaffold `excluded_default_features` for dart/swift wrappers, publish/vendor retry on crates.io registry-index propagation lag, e2e/codegen wasm `[crates.e2e.env]` block, e2e/codegen php PIE invocation syntax for v1.4.5, backends/swift `RustBridgeC` import + Vec skip on already-declared types, docs heading demotion, e2e/codegen typescript `SsrfPolicy.denyPrivate=false` for WASM e2e, backends/ffi shared extractor → FFI same-name fn dedup, and **backends/dart FRB primitive bridge return-value type cast restoration** (`.map(|v| v as i64)` regression that blocked rc.55 regen).

### Fixed

- **Elixir Hex install OTP 27.2 TLS `key_usage_mismatch` against `builds.hex.pm`.** Switched test-elixir jobs in `ci.yaml` and `ci-e2e.yaml` to `kreuzberg-dev/actions/setup-elixir@v1` wrapper which routes through `cdn.hex.pm` to bypass OTP 27.2 TLS cert-chain rejection against `builds.hex.pm`.
- **`ci.yaml` test-* jobs 404 race on `parsers.json`.** Mirrored `ci-e2e.yaml`'s `build-e2e-bundles` job into `ci.yaml` and added `TREE_SITTER_LANGUAGE_PACK_MANIFEST_URL` manifest wiring to all test-* jobs (test-python, test-node, test-wasm, test-go, test-java, test-csharp, test-ruby, test-php, test-elixir, test-c-ffi). Pre-publish, the workspace version has no GitHub Release yet, so the runtime's network fetch of `parsers.json` would 404; bundling parsers locally and exporting the manifest URL avoids the race.

## [1.9.0-rc.54] - 2026-06-17

### Changed

- **Bumped `alef` pin 0.25.20 → 0.25.24.** Regenerated all bindings via `task alef:generate`. Picks up alef 0.25.21–0.25.24: dart e2e setEnv robustness, swift `already_declared` opaque-handle class triples, java PMD/palantir-java-format compliance, C FFI e2e download_ffi.sh derives FFI_PKG_NAME from `lib_name` (was hardcoded), kotlin-android per-file ktfmt invocation, plus the rc.53 → rc.54 structural fixes below.

### Fixed

- **Java loader RID alignment (rc.53 regression).** `NativeLib.resolveNativesRid()` previously emitted `osx-aarch64`/`linux-aarch64` (a JNA/LWJGL-style convention) while the published JAR's `natives/<rid>/…` directory is named via `go_java_platform()` (`macos-arm64`, `linux-aarch64`, `windows-x86_64`). Result: every macOS-arm64 client failed with `UnsatisfiedLinkError` because `natives/osx-aarch64/libts_pack_core_ffi.dylib` does not exist (it's at `natives/macos-arm64/`). Loader template now matches `go_java_platform()` naming.
- **Elixir download NIFs unregistered in precompiled binary (rc.53 regression).** The rustler NIF crate `Cargo.toml` had no `[features]` table — only a `[lints.rust] check-cfg` reference to `download`. Default precompiled CI builds therefore stripped the download/cache/init/configure NIFs from the cdylib, producing `:nif_not_loaded` errors on every `TreeSitterLanguagePack.Native.download/*`, `cache_dir/0`, `clean_cache/0`, `init/0`, `configure/1`, `downloaded_languages/0` call (10/450 errors at rc.53). Template now emits canonical `[features] default = ["config", "download", "serde"]` block forwarding to the core crate, mirroring the magnus fix from alef 0.25.19.
- **Node vitest first-load timeouts.** `smoke_devicetree` and `smoke_ocamllex` exceeded the default 30 s test timeout on first load. Raised `testTimeout` to 60 s, `hookTimeout` to 120 s.
- **C FFI E2E 404 race in `ci-e2e.yaml`.** `e2e/c/download_ffi.sh` pinned the FFI tarball URL to the current workspace version; on main pushes before the matching tag was created, the curl 404'd because the GitHub Release for that version didn't exist yet. Script now honours `ALEF_FFI_LOCAL_DIR` env override to skip the network fetch and consume pre-staged headers/libs. `ci-e2e.yaml/test-c-ffi` is now `needs: build-ffi` and stages the locally-built artifact via the override.

## [1.9.0-rc.53] - 2026-06-16

### Changed

- **Bumped `alef` pin 0.25.18 → 0.25.20.** Regenerated all bindings via `task alef:generate`. Picks up:
  - **0.25.19** — magnus binding `Cargo.toml` `[features]` block (fixes rc.52 Ruby gem build under `-D warnings`), elixir NIF `Cargo.toml` `[lints.rust]` ordering (fixes CI `Check version sync`), ruby Rakefile yard-coverage hook, FFI opaque-pointer call-site `.clone()` for service-API codegen, `binding_excluded` field fallthrough that preserves bespoke core `Default::default()` semantics, csharp e2e csproj arm64 RID branching.
  - **0.25.20** — dart loader absolutize defensive improvement, zig opaque method error decoding (`_first_error` → `_error_with_message`), `language_pages.rs` modularization under 1000-LOC cap.

### Fixed

- **Ruby gem publish (rc.52 regression).** All four `Build Ruby gem` matrix jobs failed at rc.52 — the magnus binding's `Cargo.toml` lacked the `[features]` table forwarding `download` to the core crate, so 18× `#[cfg(feature = "download")]` arms triggered `error: unexpected cfg condition value: download` under `-D warnings`. The skipped `Publish Ruby gems` step meant rc.52 never reached `rubygems.org` (test-apps:ruby failed with `Could not find gem 'tree_sitter_language_pack ~> 1.9.0.pre.rc.52'`). Picked up via alef 0.25.19.
- **CI `Check version sync` red on `main`.** The elixir NIF `Cargo.toml` emitted `[lints.rust]` before `[dependencies]`; consumers' `prek run --all-files` runs cargo-sort which reorders the block to the file end, producing a perpetual diff. The CI version-sync step does NOT run cargo-sort, so it reported "Versions are out of sync" on every release tag. Picked up via alef 0.25.19.
- **Dart publish pipeline native staging (rc.52 regression).** The `assemble-dart-package` job in `.github/workflows/publish.yaml` used `download-artifact@v8` with `merge-multiple: true`, flattening every `dart-native-<rid>` artifact's contents directly under `dart-natives/`. The subsequent RID inference (`basename "$(dirname "$f")"`) then resolved to the literal string `dart-natives` for every file, causing all four native libraries to be skipped with `Warning: unrecognized rid 'dart-natives'`. The published rc.52 pub.dev tarball contained no `lib/src/native/<rid>/` directory; the FRB loader fell through to the default relative-path dlopen which macOS hardened-runtime rejected with "relative path not allowed in hardened program". Fix: drop `merge-multiple: true` so each artifact extracts to its own `dart-natives/dart-native-<rid>/` directory, and derive the RID by stripping the `dart-native-` prefix from the artifact directory name.

## [1.9.0-rc.52] - 2026-06-16

### Changed

- **Bumped `alef` pin 0.25.15 → 0.25.18.** Regenerated all bindings via `task alef:generate` + `task alef:sync`. Picks up:
  - **0.25.16** — drop cfg propagation on enum `From`-impl match arms, API reference docs improvements.
  - **0.25.17** — dart `unreachable_patterns` allow at crate root, zig test sequencing to avoid `clean_cache` race, node smoke timeout for `vb`.
  - **0.25.18** — swift cfg-gated extern blocks for `DownloadManager` (`#[cfg(feature = "download")]`), dart absolutize env + `Platform.script` paths in hardened runtime loader, node slow-grammar list extended (`earthfile`, `perl`), pyo3 + napi emit binding-side wrapper structs for `[workspace.opaque_types]` entries without `capsule_types` override, napi binding `Cargo.toml` `[features]` block (`default = ["download"]`), cleanup detection tightened from loose `by alef` substring to specific `auto-generated by alef`, cbindgen `autogen_warning` updated so generated C headers are correctly identified as cbindgen-owned.

### Fixed

- **Python `get_language` / `get_parser` API consistency (#141).** Both helpers now return the binding's own native types (`Language`, `Parser`) instead of the standalone `tree_sitter` package's `Language`, matching every other binding (Java, Go, Swift, Ruby, C#, etc.). Dropped `pip_dependencies = ["tree-sitter>=0.23"]` from the Python package. **Breaking change** for callers that relied on `tree_sitter.Parser(get_language(name))` — use `get_parser(name)` directly, or import `tree_sitter` as a separate dependency.
- **Node `getLanguage` / `getParser` API consistency.** Same shape change as Python — `getLanguage` now returns the native `Language` class from `@kreuzberg/tree-sitter-language-pack` rather than passing through to the upstream `tree-sitter` npm package. Dropped the `tree-sitter` devDependency from the e2e harness.
- **Swift `DownloadManager` build error.** swift-bridge proc-macro previously failed with `no type named 'DownloadManager' in module 'RustBridge'` (and 9 related missing-member errors) because cfg-gated extern blocks were emitted unconditionally. Now wrapped in `#[cfg(feature = "download")]` so disabled-feature compile correctly elides the bridge surface.
- **Dart hardened-runtime dlopen failures.** All 9 dart e2e tests previously failed at `setUpAll` with "relative path not allowed in hardened program". Loader now absolutizes env-var and `Platform.script`-derived paths before constructing search roots.
- **Node smoke timeouts on `earthfile` and `perl`.** Tree-sitter grammars with heavy scanner.c logic now receive the 90000ms slow-grammar timeout (previously only `vb` was covered).

### Removed

- **Python: standalone `tree-sitter` package dependency.** No longer required by `tree-sitter-language-pack`. Install it separately if you need the upstream API.
- **Node: `e2e/node/tests/capsule_passthrough.test.ts` and `tree-sitter` devDependency.** Obsolete now that node `getLanguage` returns the native type.

## [1.9.0-rc.51] - 2026-06-15

### Changed

- **Bumped `alef` pin 0.25.14 → 0.25.15.** Regenerated all bindings via `task alef:sync` + `task alef:generate`. Picks up the swift cfg-postprocess revert:
  - **0.25.15 — revert(swift): drop cfg-union postprocessing passes (c89926d5e, 4313b6e1d).** The wrapper-type and function cfg-union propagation passes introduced in 0.25.12/0.25.13 caused downstream binding regressions; reverted in favour of the 0.25.14 default-features approach in the swift binding Cargo.toml.
- **Bumped tslp `1.9.0-rc.50` → `1.9.0-rc.51`** propagated via `task alef:sync`.

## [1.9.0-rc.50] - 2026-06-15

### Changed

- **Bumped `alef` pin 0.25.11 → 0.25.14.** Regenerated all bindings via `task alef:sync` + `task alef:generate`. Picks up the accumulated fixes:
  - **0.25.12 — (java): bind `${classifier}` property to maven-jar-plugin config so native JARs are emitted with the correct classifier.** Resolves rc.49 Maven Central regression where `tree-sitter-language-pack-java-1.9.0-rc.49.jar` was published without classifier (missing `/natives/{rid}/libts_pack_core_ffi.dylib`), causing `UnsatisfiedLinkError` at JNI init.
  - **0.25.12 — (dart): copy `.framework` directories recursively in publish workflow assemble step.** The `find` predicate matched only `*.so`/`*.dylib`/`*.dll` files and skipped macOS `.framework/` bundles; pub.dev rc.49 was missing `tree_sitter_language_pack_dart.framework/`.
  - **0.25.12 — (php): always stage PIE extension as `.so` on Unix.** PIE 1.4.5 probes for `.so` on all Unix platforms including macOS; the previous OS-branching produced rc.49 prebuilt archives missing the `.so` (PIE extracted source but found no binary).
  - **0.25.12 — Swift wrapper-type cfg union postprocess.** Wrapper structs whose fields reference cfg-gated upstream types now inherit the union cfg gate, fixing swift-bridge-build `Type must be declared with 'type X'` panics.
  - **0.25.13 — Swift function cfg union postprocess.** Free helper functions taking cfg-gated wrapper struct references now inherit the union cfg gate, complementing the wrapper-type fix.
  - **0.25.14 — Swift binding Cargo.toml lists every forwarded cfg-feature in `default = [...]`.** Prevents `error[E0425]: cannot find type 'DownloadManager' in this scope` on regen consumers when the wrapper struct is cfg-gated but free helper functions referencing it are not — the binding's default profile now matches what its core dep already pulls in via `features = [..., "download"]`.
- **Bumped tslp `1.9.0-rc.49` → `1.9.0-rc.50`** propagated via `task alef:sync`.

## [1.9.0-rc.49] - 2026-06-15

### Changed

- **Bumped `alef` pin 0.25.9 → 0.25.11.** Regenerated all bindings via `task alef:sync` + `task alef:generate`. Picks up the accumulated 0.25.10 and 0.25.11 fix series:
  - **0.25.10 — `publish prepare` canonicalize bug.** `publish prepare` was canonicalizing only `manifest_dir.join("Cargo.toml")`, not `manifest_dir` itself. With `current_dir(manifest_dir)` set on the cargo subprocess, the `--manifest-path ./packages/elixir/.../Cargo.toml` argument resolved relative to the new cwd — effectively doubling the path — and cargo bailed with `manifest path '...' does not exist` followed by the misleading "publish core first" hint. Every source-build binding in rc.48 hit this (Python sdist, Ruby gem, Elixir NIF, PHP PIE). 0.25.10 canonicalizes `manifest_dir` itself at the top of the regenerate branch and `manifest_abs` in `rewrite_binding_path_deps`.
  - **0.25.10 — kotlin-android `copyHostJni` always reads workspace target.** Drop the configuration-time `if (workspaceTarget.exists())` selector that evaluated before `cargo build` finished, eliminating the `UnsatisfiedLinkError` cascade at static-init time on every JNI-loading test class.
  - **0.25.10 — R extendr enum path resolution.** `gen_from_binding_to_core`/`gen_from_core_to_binding` now use `resolve_type_path` against a `build_type_path_lookup(api)` map instead of `core_enum_path_remapped`, fixing E0433 `cannot find ImageOutputFormat in crate 'kreuzberg'` for enums defined outside the crate root.
  - **0.25.10 — Swift `From<core>` arms cfg-gate variants.** `emit_enum_wrapper` now prepends `#[cfg(...)]` before each rendered arm so the match remains valid when the binding crate's feature set drops upstream variants (iOS / Android cross-targets).
  - **0.25.10 — C# e2e csproj emits `<GenerateAssemblyInfo>false</GenerateAssemblyInfo>`.** Closes the CS0579 duplicate-attribute path for consumer e2e directories that carry a hand-checked-in `Properties/AssemblyInfo.cs`.
  - **0.25.10 — Visitor result routes bare strings to `Custom` when multiple string-payload variants exist.** Fixes silent fallback to the default variant when an enum has both `Custom(String)` and `Error(String)`.
  - **0.25.10 — FFI visitor context emits enum-typed fields as `i32` discriminant.** Closes the `ArrayIndexOutOfBoundsException` cascade in `VisitorBridge.decodeContext` caused by reading the low 4 bytes of `tag_name` pointer when the C struct omitted the enum field.
  - **0.25.10 — Dart cfg-extraction whitespace bug and check-cfg allow-list.** `extract_feature_names_from_cfg` now normalizes whitespace + handles the `any(test, feature = "X")` sibling form; check-cfg allow-list populates from every `EnumDef.variants[*].cfg` instead of falling back to the single-entry `cfg(frb_expand)` form.
  - **0.25.10 — Release task: `task set-version` handles prerelease versions when updating `ALEF_REV`** + **Ruby Rakefile template documents `GEMSPEC` constant** for YARD coverage.
  - **0.25.11 — README generation supports named non-language targets.** `[crates.readme.targets.<name>]` renders additional template-backed README outputs alongside per-language READMEs.
  - **0.25.11 — Option B cfg forwarding for Dart and Swift binding crates.** Each cfg feature name referenced by any IR type/field/variant/function is now emitted as `{name} = ["{core_dep_key}/{name}"]` in `[features]`, making the feature resolvable at the binding level and eliminating `unexpected_cfgs` without an allow-list. Shared collection logic extracted to `src/codegen/cfg.rs`. WASM backend now delegates to it. Swift `emit_enum_wrapper` emits a `_ => unreachable!()` catch-all whenever any variant in the primary list carries a `#[cfg(...)]` gate.
  - **0.25.11 — Generated Homebrew test apps trust third-party taps before installing formulae.** `run_tests.sh` now calls `brew trust "$TAP" || true` before `brew bundle install`.
  - **0.25.11 — Dart test_app run no longer invokes `download_libs`.** Natives ship inside the pub.dev package; the FRB loader resolves them from `lib/src/native/<rid>/`. Drops the structural HTTP 404 against `releases/download/v.../tree-sitter-language-pack-dart-...` that masked publish-pipeline failures.
  - **0.25.11 — C e2e Makefile: always re-download `ts_pack.h`.** Drops the `HEADER_PATH`/`LIB_PATH` short-circuit that elided the header dependency whenever a stale prior-rc header was on disk. Per-version marker `ffi/.alef-ffi-version` keeps unchanged trees from paying network cost. Resolves the rc.48 C test_app `unknown type name 'TS_PACKDataNode'` cascade.
  - **0.25.11 — C# scaffold: SDK-generated AssemblyInfo with explicit version stamps and full RID list.** Enables `<GenerateAssemblyInfo>` (drops `false` suppression), stamps `<AssemblyVersion>`/`<FileVersion>` as 4-component numeric (`MAJOR.MINOR.PATCH.0`) via new `to_dotnet_assembly_version` helper, preserves full SemVer on `<InformationalVersion>`, replaces conditional singular `<RuntimeIdentifier>` with a plural list of all six published RIDs, and pins `<PlatformTarget>AnyCPU</PlatformTarget>`. Resolves the rc.48 C# `Version=0.0.0.0` + `targets a different processor` cascade.
  - **0.25.11 — Zig `_error_with_message` dispatches to per-error-set message-prefix matchers.** Replaces the unconditional `_first_error(E)` fallback that masked the real cause of every typed FFI failure; emits a `if (E == ErrName) return _from_ffi_msg_ErrName(msg_opt);` chain per declared error-set.
  - **0.25.11 — Rustler codegen clippy violations** (type complexity, collapsible if, struct update, useless conversions) + **Rustler trait-bridge parameter cloning skips no-op clones on reference types** + **JNI clippy lints + Swift cargo.rs api param** + **pyo3 async lifetime/result-handling cleanup**.
- **Bumped tslp `1.9.0-rc.48` → `1.9.0-rc.49`** propagated via `task alef:sync`.

## [1.9.0-rc.48] - 2026-06-15

### Changed

- **Bumped `alef` pin 0.25.6 → 0.25.9.** Regenerated all bindings via `alef sync-versions --skip-swift-checksum` + `task alef:generate`. Picks up the accumulated 0.25.6/0.25.7/0.25.8/0.25.9 fixes:
  - **0.25.6 — Java codegen.** `NativeLib` downcall fallback chain now emits in the single-line shape that palantir-java-format produces, so the regenerated `NativeLib.java` no longer triggers a post-regen formatter rewrite (the diff that broke rc.47's `Validate Lint & Format` job).
  - **0.25.6 — publish vendor `cargo --locked` paradox.** `alef publish prepare` was running `cargo update -p <crate>` with `--locked` passed via `CARGO_BUILD_LOCKED`, which made cargo refuse to update the lockfile. The Python sdist + all 4 Ruby gem + all 4 Elixir NIF + all 18 PHP PIE jobs on rc.47 failed with "cannot update the lock file ... because --locked was passed". Fix: `vendor.rs` now `.env_remove("CARGO_BUILD_LOCKED")` before both cargo invocations and drops the `--locked` flag from `cargo metadata`. Regression test `scrub_lock_succeeds_for_non_workspace_binding_crate_with_incomplete_seed` covers the path.
  - **0.25.8 — Dart mirror enum cfg-strip.** `emit_mirror_enum` no longer propagates `variant.cfg` into the generated mirror enum body. The mirror is a DTO/wire type that `flutter_rust_bridge_codegen` references unconditionally from `frb_generated.rs` — gating a variant out via `#[cfg]` left the unconditional reference dangling with `E0599 no variant named 'Heif' found for enum 'ImageOutputFormat'` when the binding crate didn't declare the upstream feature. The catch-all `_ => unreachable!()` arm in the `From<CoreType>` impl (introduced earlier) handles runtime safety.
  - **0.25.9 — Dart check-cfg allow-list + mirror dead-code cleanup + publish current_dir.** Resolves the v0.25.8 build regression where the 0.25.8 cfg-strip patch orphaned the `emit_variant_cfg_open`/`emit_variant_cfg_close` helpers, tripping `-D warnings` on all 4 alef publish jobs (3× Build CLI + crates.io). Also widens the dart check-cfg allow-list and tightens `publish` current-dir handling.
  - **alef-side accumulated fixes (released as 0.25.9 alongside the above).** Direct-deps replacement for the no-op `[patch.crates-io]` block alef 0.25.8 emitted in the Elixir NIF `Cargo.toml` (cargo refused with "patch points to the same source"). Direct deps with `=` constraints + matching `package.metadata.cargo-machete.ignored` entries pin `alloc-no-stdlib`/`alloc-stdlib`/`brotli-decompressor` transitively. YARD doc-coverage hook fixed via a documented `GEMSPEC` constant in the generated `packages/ruby/Rakefile` and a matching docstring in the in-tree stale `packages/ruby/ext/ts_pack_core_rb/src/Rakefile` (the latter file is hand-maintained and not regenerated; cleanup deferred).

## [1.9.0-rc.46] - 2026-06-14

### Changed

- **Bumped `alef` pin 0.25.1 → 0.25.2.** Picks up two source-build publish-prepare fixes:
  - `publish prepare` now strips workspace-member `[[package]]` entries from the seeded `Cargo.lock` before per-member `cargo update -p`. Without this strip the path-source seed entry collides with the rewritten registry-source dep and `cargo metadata --locked` validation fails.
  - `publish prepare` disambiguates the per-member `cargo update -p` spec by using the full `registry+https://github.com/rust-lang/crates.io-index#NAME@VERSION` package id when the member version is known. Both fixes are required to unblock Ruby gem + Elixir NIF + PHP extension matrix builds on rc.46 (rc.45 failed Ruby macos-x86_64 / linux-aarch64 + Elixir linux-aarch64 / macos-x86_64 on this exact path).

## [1.9.0-rc.45] - 2026-06-14

### Changed

- **Cross-major dependency upgrade** via `task upgrade`. Rust + Python + Node + Java + Elixir + PHP + Ruby dep trees rebased to their latest semver-compatible heads; lockfiles re-resolved (`Cargo.lock`, `e2e/rust/Cargo.lock`, `composer.lock`, `pnpm-lock.yaml`, `mix.lock`, `uv.lock`, `packages/php/composer.lock`). `sources/language_definitions.json` regenerated.
- **Bumped `alef` pin 0.25.0 → 0.25.1.** Picks up the `assertions.rs:227` C-e2e codegen hardening (panic-on-missing-`fields_c_types` rather than the silent PascalCase fallback that produced `TS_PACKData` instead of `TS_PACKDataNode` in rc.43).
- **All cargo invocations across `.github/workflows/` and `.task/` now pass `--locked`.** Sweep applied in a separate commit (`130627437`) ahead of this regen to keep the manifest-normalisation fix isolated; this rc carries it forward. Same motivation as the actions-side v1.8.68 sweep: a broken upstream release (recent `brotli-decompressor 5.0.1`) can no longer silently override the committed lockfile during CI.

## [1.9.0-rc.44] - 2026-06-14

### Fixed

- **`publish-release`: normalise parser library names across platforms in the
  `parsers.json` manifest generator.** Linux/macOS produce `libtree_sitter_<lang>.{so,dylib}`,
  Windows produces `tree_sitter_<lang>.dll` (no `lib` prefix). The
  `Generate parsers.json manifest` step compared the stripped basenames as-is, so
  the intersection of grammar names across platforms was empty whenever the
  Windows archives were present — the manifest then reported every grammar as
  missing on Windows and refused to upload. The generator now strips the `lib`
  prefix and `tree_sitter_` / `tree-sitter-` prefix uniformly and reverses the
  four `c_symbol` overrides (`c_sharp`/`embedded_template`/`nu`/`vb_dotnet`)
  so the per-platform sets agree on language identifiers.
- **`alef.toml`: declare `process_result.data → DataNode` in
  `[crates.e2e.fields_c_types]`.** alef's C e2e generator falls back to
  `<field>.to_pascal_case()` when a field path is not declared, which
  produced `TS_PACKData` instead of the actual cbindgen-emitted `TS_PACKDataNode`.
  Adding the explicit mapping makes the regenerated `e2e/c/test_data_extraction.c`
  reference `TS_PACKDataNode*` and the `ts_pack_data_node_*` accessor family
  consistently. A deeper alef-side hardening of the fallback (loud error or
  IR-driven type lookup) is accumulated locally in `../alef` and pending an
  alef release.
- **`build.rs`: allow `clippy::type_complexity` on the MSVC patches table.**
  The `&[(&str, &str, &[(&str, &str)])]` shape introduced for crystal/sml
  MSVC compat tripped `clippy -D warnings` in CI; the table reads cleanly as
  a literal and isn't worth a type alias.
- **`rust-max-lines` pre-commit cap: exclude
  `crates/ts-pack-core/src/intel/data_extraction.rs`.** New 1322-line module
  added for hierarchical data extraction; remediation backlog entry.

## [1.9.0-rc.43] - 2026-06-14

### Fixed

- **`publish-release`: read `.tar.zst` parser archives via `zstandard`.** Python's
  `tarfile.open(path, 'r:*')` only supports `.gz`, `.bz2`, `.xz`, and uncompressed
  `.tar` — not `.tar.zst`. The `Generate parsers.json manifest` step was failing
  with `tarfile.ReadError: not a gzip file / not a bzip2 file / not an lzma file /
  invalid header`, blocking parsers.json + parsers-*.tar.zst upload to the release
  and breaking every downstream consumer at runtime with
  `Failed to fetch manifest from .../parsers.json: http status: 404`. The step
  now `pip install --user zstandard` then opens each archive via
  `ZstdDecompressor().stream_reader()` and `tarfile.open(fileobj=..., mode='r|')`.

## [1.9.0-rc.42] - 2026-06-14

### Changed

- **Regenerated against released alef 0.25.0.** Picks up the new `Extension` trait surface (per-extension TOML config + `transform_emitted_files` hook), Swift target-specific core dependency overrides, the zig `_first_error` → contextual error fix, and the Dart hardened-runtime framework load fix. Restores `crates/ts-pack-core-ffi/{src/lib.rs,build.rs,cbindgen.toml}` which a transient pre-release regen against an in-progress alef had erroneously dropped.

## [1.9.0-rc.41] - 2026-06-13

### Fixed

- **JNI codegen: `&[&str]` core params no longer fail E0308.** The JNI function/method shims emitted `&names` for `Vec<String>` slots, which coerces to `&[String]` but not `&[&str]`. Core fns declared as `&[&str]` (e.g. `download(&[&str])`) failed to compile with `expected reference &[&str], found reference &Vec<String>`. Alef now consults the IR `vec_inner_is_ref` flag to materialise a `Vec<&str>` and borrow it (`&names.iter().map(|s| s.as_str()).collect::<Vec<_>>()`) when the core function expects `&[&str]`, matching the existing Dart codegen behaviour. Folded into the alef 0.24.17 release.

## [1.9.0-rc.40] - 2026-06-13

### Fixed

- **Removed stray `test_apps/kotlin_android/file:/tmp/` directory that broke every Windows publish job.** A prior regen wrote a runtime download cache (`.download.lock`, `manifest.json`) into a literal directory named `file:` because a `cache_dir` value of the form `file:/tmp/…` was interpreted as a relative path rather than a URI. The `:` is illegal in Windows paths, so every `actions/checkout` step on Windows failed with `invalid path 'test_apps/kotlin_android/file:/tmp/.download.lock'` — collapsing 21 Windows builds in the rc.39 publish run and leaving npm / PyPI / NuGet / Maven stuck at rc.38. The bad files are removed and `.gitignore` now blocks the pattern (`test_apps/*/file:/`) alongside the existing `test_documents/file:/` guard.

## [1.9.0-rc.39] - 2026-06-12

### Added

- **Hierarchical data extraction for 17 data-format languages (#136).** Set `data_extraction = true` on `ProcessConfig` to extract a nested `DataNode` tree preserving the original document's hierarchy. Covers JSON, HJSON, JSON5, TOML, properties, Cue, HCL, HOCON, KDL, YAML, INI, EditorConfig, PO, Nginx, Caddy (key-value pairs); XML and DTD (element shape); and CSV/PSV (sequence shape). See [docs/guides/intelligence.md#data-extraction](https://docs.tree-sitter-language-pack.kreuzberg.dev/guides/intelligence/#data-extraction).

- **JNI is a first-class test-apps target for Kotlin Android host-JVM.** The kotlin_android test app's host-JVM gradle tests now satisfy `Language::Jni` in `alef test-apps run`, enabling CI/CD verification without Android emulator. Requires alef 0.24.14+.

### Changed

- **Alef pin bumped 0.24.10 → 0.24.14.** Pulls in the JNI run-default split (host-JVM gradle runner replaces the `Ffi | Jni` no-op), JNI return marshalling for raw `String` / `Option<String>` returns (no more JSON-encoded `"\"python\""` surfacing in Kotlin), Kotlin test emitter `loadLibrary` respecting `[crates.ffi] prefix` (resolves `ts_pack_jni` instead of the literal crate name), and the Kotlin assertion emitter switching list-`contains` checks to a case-insensitive `toString().lowercase().contains(...)` shape that mirrors the Java emitter.

## [1.9.0-rc.32] - 2026-06-11

### Fixed

- **`release-finalize` job guards `Finalize release` on `prepare` success.** The job ran with `if: always()` and unconditionally invoked `finalize-release@v1`, which errors with `INPUT_TAG is required` whenever `prepare`'s `tag` output is empty (cancelled or failed `prepare`). Result: a cancelled rc.31 surfaced as a confusing `Finalize release: failure` on top of the actual upstream cancellation. Now `if: needs.prepare.result == 'success'`. rc.31 publish run 27214336783.
- **PHP `test_apps/install.sh` verifies extension load via `extension_loaded()` rather than parsing `php -m` output.** When the PIE-installed extension was already loaded through the global `php.ini`, an explicit `php -d extension=...` invocation caused PHP to emit `Module already loaded` to stderr; the harness's combined-output capture treated the warning as fatal and the install step exited non-zero before the actual smoke test ran. Switched to `php -r 'exit(extension_loaded("...") ? 0 : 1);'` so the check is decoupled from PHP's logging and tolerant of double-loading.
- **`ts-pack-core-ffi` regen emits zero rustdoc warnings.** Previously the regen produced 26 broken-intra-doc-link warnings on every build because emitted `///` comments contained bare and backtick-wrapped intra-doc-link forms (`[download()]`, `` [`Error::LanguageNotFound`] ``, etc.) referencing core-crate items not in scope from the FFI wrapper. Pulled in via the alef 0.24.2 bump.
- **`ts-pack-core-node` regen emits zero rustdoc warnings.** The previous regen left `Vec<u8>` and `Array<number>` bare in `JsBytes` doc comments, which rustdoc parsed as unclosed HTML tags. Pulled in via the alef 0.24.2 bump.
- **`test_apps/zig/build.zig.zon` URLs now match publish-zig asset naming.** Previous releases emitted URLs with Go-style platform labels (`linux-aarch64`, `macos-arm64`, …) while published assets used Rust target triples (`aarch64-unknown-linux-gnu`, `aarch64-apple-darwin`, …), so `zig fetch` 404'd. The alef 0.24.2 bump switches both sides to Rust triples; tslp's `alef.toml` `[crates.e2e.registry.packages.zig.platform_hashes]` keys updated to match. Reverts the simple-arch direction taken in rc.31.

### Changed

- **Alef pin bumped 0.23.68 → 0.24.2.** Pulls in the FFI/NAPI rustdoc-warning fixes and Zig URL alignment above, plus a Kotlin Android host JNI artifact for JVM test_apps (`buildHostJni` / `copyHostJni` Gradle tasks guarded by `alef.skipHostJni`), Go scaffold `module_major` parameterization that lets non-kreuzberg consumers configure their `packages/go/v{N}` layout, and a broad sweep of trait-bridge adapter fixes across Kotlin Android, C#, Java, Node, R, Swift, Dart, Elixir, and Go.
- **`crates/ts-pack-core/build.rs` added to the `rust-max-lines` exclude list (1081 LOC > 1000-line ceiling).** Joins the existing remediation backlog of large files awaiting split.

## [1.9.0-rc.31] - 2026-06-09

### Fixed

- **Alef pin bumped 0.23.58 → 0.23.65.** Pulls in two test_apps-driven fixes from alef 0.23.65:
  - **kotlin-android**: Foojay toolchain resolver plugin bumped v0.7.0 → v0.10.0 in both `settings.gradle.kts` emitters. v0.7.0 referenced `JvmVendorSpec.IBM_SEMERU`, which Gradle 9.0+ removed (renamed to `IBM`); Gradle 9.5.1 hosts failed at project-evaluation with `Class org.gradle.jvm.toolchain.JvmVendorSpec does not have member field 'IBM_SEMERU'`. v0.10.0 is Gradle 9.x-safe.
  - **zig**: published tarballs now use simple-arch platform labels (`linux-x86_64`, `linux-aarch64`, `macos-arm64`, `macos-x86_64`, `windows-x86_64`) matching `build.zig.zon` URL templates. Previously `RustTarget::platform_for(Language::Zig)` returned the rust triple, so `alef publish package --lang zig --target …` emitted `…-aarch64-apple-darwin.tar.gz` but the e2e codegen's URL templates and per-platform `[crates.e2e.registry.packages.zig.platform_hashes]` user config used the simple-arch convention. Consumers' `zig fetch` then 404'd.

## [1.9.0-rc.30] - 2026-06-09

### Fixed

- **`Stage Go FFI libraries` step uses `git add -f`.** Root `.gitignore` globally ignores `*.so`/`*.dylib`/`*.dll`/`*.lib`, so the plain `git add` silently refused to stage the downloaded FFI artifacts under `packages/go/.lib/`. `xargs` propagated the (silent) failure as exit 123, failing the step before the `packages/go/v<version>` subtree tag could be pushed. Added `-f` so the published Go module deliberately ships pre-built FFI artifacts past the global ignore. Fixes rc.29 publish run 27192809836 Stage Go FFI failure.
- **`upload-release-assets@v1` receives the publisher-app token as an action input on all 4 cross-repo-write call sites.** The shared action sets `GH_TOKEN` inside its own composite step from `inputs.token` (default `github.token`), so a step-level `env: GH_TOKEN: …` on the calling job had no effect — uploads ran with the read-only default `GITHUB_TOKEN` and hit `HTTP 403: Resource not accessible by integration`. Now passes `token: ${{ steps.app-token.outputs.token }}` on the Go FFI, Elixir NIF, Swift bundle, and Zig upload sites. The PHP PIE upload site (line 2473) keeps the default token because its job declares `permissions: contents: write`. Fixes rc.29 publish run 27192809836 Upload Go FFI 403.
- **Pulls in `kreuzberg-dev/actions` v1.8.49 retry-on-SSL upload fix.** `publish-github-release/scripts/upload_artifacts.py` now retries 5× with exponential backoff on `URLError` / `ssl.SSLError` / `ConnectionError` / `TimeoutError` / HTTP 5xx. rc.29 parser-sources bundle upload hit a transient `ssl.SSLEOFError` mid-upload on a 30 MB asset and cascaded to ~15 dependent failures (skipping `publish-crates`, which broke every PHP/Ruby/Elixir/Python-sdist `cargo generate-lockfile` against the unpublished workspace member); the retry absorbs the SSL race.

## [1.9.0-rc.29] - 2026-06-09

### Changed

- **Alef pin bumped 0.23.48 → 0.23.58.** Pulls in the PHP MINIT module-startup mutex (0.23.50 — `crates/ts-pack-core-php/src/lib.rs` now wires `__ext_php_rs_module_startup` into the extension builder so class registrations actually reach PHP), the NAPI TS overload/optional-param signature cleanup (0.23.55), the NAPI arrow-type return type strip fix (0.23.54), the napi service-wrapper lowerCamelCase fix (0.23.57), per-item version annotations in the IR + docs generator (0.23.58), the NAPI enum variant JSDoc `*/` escape (0.23.58 — `crates/ts-pack-core-node/index.d.ts` no longer prematurely closes the JSDoc block around `DocstringFormat::JSDoc` / `::JavaDoc` variant docs), and a sweep of Java/Kotlin/Zig/Dart formatting normalization across all generated binding files.

- **Release automation migrated to the `kreuzberg-dev-publisher` GitHub App.** All 15 release-write jobs in `.github/workflows/publish.yaml` (parser-sources upload, parser-binaries upload, Go FFI upload, C FFI upload, Elixir NIF upload + draft create, Hex checksums fetch, pubdev workflow dispatch, Swift manifest commit + tag force-push, Zig upload, CLI upload, homebrew formula render + tap push, homebrew bottle build + DSL merge + tap push, Go subtree commit + tag push, finalize-release) now mint a short-lived installation token via `actions/create-github-app-token@v2` keyed off the org secrets `BOT_APP_ID` / `BOT_APP_PRIVATE_KEY`. Bot identity: `kreuzberg-dev-publisher[bot]` (user id 291994444). Eliminates the `HOMEBREW_TOKEN` PAT for cross-repo tap pushes and lets tag pushes trigger downstream workflows (`GITHUB_TOKEN`-driven pushes don't). Branch protection on `main` requires `kreuzberg-dev-publisher[bot]` in the bypass list.

### Fixed

- **`Stage Go FFI libraries` step in `.github/workflows/publish.yaml` now resolves the artifact path correctly.** The step `cd`s into `packages/go/` before walking the downloaded artifact tree, so the `find` invocation needs `../../tmp/go-ffi-all` (two levels up to the repo root). Commit `1de6c8dca` introduced `../../../tmp/go-ffi-all` (three levels up), pointing one directory above the workspace root → `find: '…/tmp/go-ffi-all': No such file or directory` → exit 1 → `packages/go/v1.9.0-rc.28` subtree tag never pushed. Manually staged + tagged rc.28; the next publish run picks up the fix.

## [1.9.0-rc.28] - 2026-06-08

### Fixed

- **Homebrew `libts-pack` bottle now ships with all 306 grammars statically compiled.** The `build-c-ffi` step in `.github/workflows/publish.yaml` was invoking `alef publish build --lang ffi` without `TSLP_LANGUAGES`, so `crates/ts-pack-core/build.rs` defaulted to zero statically-compiled grammars and the resulting FFI tarball (downloaded verbatim by the libts-pack formula) had an empty language registry. The bottle's `ts_pack_available_languages()` returned an empty string, breaking `test_apps/homebrew/ffi_smoke.c`. Step now sets `TSLP_LANGUAGES` to the full language list (via the same `python3 -c "import json"` extraction used by the CLI build) and `TSLP_LINK_MODE=static`.

- **C# NuGet `TreeSitterLanguagePack` package now bundles an FFI dylib with all 306 grammars statically compiled.** Same root cause as the libts-pack fix above — the `build-csharp-native` step invoked `build-csharp-natives@v1` without setting `TSLP_LANGUAGES`. The composite action's native cargo build inherits the calling step's env, so adding `TSLP_LANGUAGES`/`TSLP_LINK_MODE=static`/`PROJECT_ROOT` on the step propagates into cargo. Fixes the `Language 'comment' not found` failure surfaced by `test_apps/csharp` against rc.27.

- **`CommentKind::Block` and `CommentKind::Doc` rustdoc no longer contains literal `*/` inside backticks.** The `*/` sequence inside `` `/* ... */` `` code spans was landing verbatim in NAPI-RS-emitted JSDoc, prematurely closing the `/** ... */` block and triggering oxlint `TS(1164): Computed property names are not allowed in enums`. Reworded the rustdoc to avoid the `*/` terminator. (Alef 0.23.47 added an `escape_jsdoc_block_close` sanitization helper but it does not reach the napi enum variant doc path — tracked as alef 0.23.48+ follow-up.)

### Changed

- **Alef pin bumped 0.23.34 → 0.23.48.** Pulls in the Zig null-check primitive-return fix (0.23.47), PHP module entry explicit-name fix (0.23.47), JSDoc `*/` sanitization helper (0.23.47), kotlin-android foojay-resolver plugin emission (0.23.47), Zig publish package name using Zig platform mapping (0.23.48), Zig null-guard returning canonical `error.Serialization` (0.23.47), FFI Finalize owner-pointer preservation (0.23.46), and the c download_ffi.sh asset name + zig cache clear + php pie always-install fixes (0.23.43–45).

## [1.9.0-rc.27] - 2026-06-08

### Fixed

- **Publish smoke install: scope `--no-binary` to `tree-sitter-language-pack`.** `pip install --no-binary :all: --no-build-isolation` forced source builds for transitive deps too and pip then failed `BackendUnavailable: Cannot import 'hatchling.build'` because the smoke venv only pre-installed maturin + setuptools + wheel. The smoke step now scopes `--no-binary` to just our package; transitives use their published wheels.

- **Exclude `php8.5 / macos-arm64` from the PIE matrix.** `shivammathur/setup-php@2.37.1` cannot install PHP 8.5 on macOS arm64 — the brew arm64 bottle is not yet published, and the macOS arm64 runner images ship no pre-installed PHP. All other PHP 8.5 variants build cleanly. Re-enable when upstream catches up.

- **Retry transient HTTP errors when downloading parser sources in `crates/ts-pack-core/build.rs`.** `fetch_bytes` now retries up to 6 times with exponential backoff (2s → 64s) on any ureq error, covering both network blips and the GitHub release CDN's intermittent 504s. Without retries, a single 504 mid-`cargo publish` verify-build would blow up the publish workflow (as happened on rc.26's `Publish Rust crates` job during the 2026-06-08 GH CDN incident).

- **Local-clone fallback in `crates/ts-pack-core/build.rs`.** When the workspace `parsers/` tree is empty (gitignored on a fresh clone) and the GH release tarball for `parser-sources-{version}.tar.zst` isn't published yet (rc builds during the publish workflow window), the build no longer panics on a 404. The new resolution order is: workspace populated → OUT_DIR cache → `scripts/clone_vendors.py` (if present, dev workspace) → GH release tarball. The local-clone path tries `uv run --no-sync`, `uv run`, `python3`, and `python` in turn so it works across dev environments. Existing `TSLP_OFFLINE` and `TSLP_SOURCE_BUNDLE_URL` overrides are unchanged.

### Added

- **`get_tags_query(language: &str) -> Option<&'static str>`** — new public accessor in `crates/ts-pack-core` mirroring `get_highlights_query` / `get_injections_query` / `get_locals_query`. Returns `Some` for the 15 languages with vendored `tags.scm` (rust, kotlin, csharp, swift, gleam, gap, al, enforce, gdshader, roc, cfml, ql, tact, sourcepawn, mojo) and `None` otherwise. Propagated to the PyO3, NAPI, and FFI bindings via the alef codegen cascade.

- **`gherkin` grammar.** Pre-compiled `tree-sitter-gherkin` parser for `.feature` files. Source: `SamyAB/tree-sitter-gherkin` pinned at `43873ee8de16476635b48d52c46f5b6407cb5c09`.

### Fixed

- **Bump alef pin `0.23.30 → 0.23.34` and regen all bindings, e2e, test_apps.** Pulls in 7 rolling alef fixes triaged from rc.25 test_app failures: php `#[php_class]` constants escape PHP-reserved variant names (`CLASS_`/`INTERFACE_`/…) avoiding `Fatal error: A class constant must not be called 'class'`; java javadoc `escape_javadoc_line` rewrites nested `*/` inside `{@code …}` to `*&#47;` so the surrounding `/** … */` block isn't closed prematurely (was breaking `mvn compile` on `CommentKind.java` + `DocstringFormat.java`); swift `ZSwiftPluginHelpers.swift` imports `RustBridge` (not `RustBridgeC`) so `RustString` resolves; zig test_apps_run sed pattern `s/}, */}\n/g` correctly splits `build.zig.zon` dep blocks so `zig fetch --save` populates `.hash` fields; dart flutter_rust_bridge external library loader uses `Abi.current()` instead of `Platform.version` string parsing for reliable arch detection; java e2e pom.xml antrun copy-native-lib step falls back from `ffi/lib/` (pre-built FFI tarball) to `target/release/` (local Cargo build); php install.sh appends `extension=<name>` to the loaded php.ini after PIE 1.4.5+ install (PIE's `--skip-enable-extension` default no longer auto-enables). (`alef.toml`, 482-file regen across `packages/`, `e2e/`, `crates/`, `test_apps/`)

### Fixed

- **Omit field-level javadoc in multiline Java record declarations for PMD compliance.** PMD 7.x does not recognize javadoc preceding annotations as belonging to record components (DanglingJavadoc rule). Field-level documentation is omitted from multiline record declarations since records are self-documenting value types and class-level record javadoc provides sufficient context. (Alef upstream: `src/backends/java/gen_bindings/types.rs`)

- **Suppress `missing_docs` lint in generated swift-bridge bindings.** The swift-bridge crate
  (`packages/swift/rust/src/lib.rs`) is entirely generated code with 1:1 wrapper mirrors of
  `tree_sitter_language_pack` types. Rustdoc coverage on these wrappers is not meaningful —
  the file now emits `#![allow(missing_docs)]` at the crate root, matching the pyo3 and wasm
  backends. (Alef upstream: `src/backends/swift/gen_rust_crate/mod.rs`)

- **Bump alef pin `0.20.10 → 0.20.12` and regen all bindings, e2e, docs, test_apps, README.** Pulls in upstream alef fixes since the rc.16 regen: `v0.20.11` ruby `Dir.chdir(ext/<name>/native)` wrap on `RbSys::ExtensionTask.new` so `Cargo::Metadata` lookup finds the workspace-excluded crate; go `embed`-import-named-not-blank + extra-blank-line cleanup; R extendr unit-enum constructor wrappers. `v0.20.12` R extendr numeric-double handling + fixture-extracted backend name; PHP e2e static-method teardown; ruby restore `config.ext_dir = "native"` in extconf.rb so build-time mkmf path matches the new `ExtensionTask` resolution. Subsequent main fixes consumed via local hand-edit pending alef 0.20.13: rustler `RustlerPrecompiled` `base_url:` template pre-wrap so `mix format` is idempotent (`packages/elixir/lib/tree_sitter_language_pack/native.ex` — wrapped manually until the alef `300d0b85b` rustler-template fix ships). (`alef.toml`, 435+ regen files across `packages/`, `e2e/`, `crates/ts-pack-core-*`, `docs/reference/`, `test_apps/`, `README.md`)
- **Drop `alef fmt` from `Check version sync` step in both CI workflows.** The rc.16 hotfix wired `alef fmt` between `alef sync-versions` and the diff check to absorb `index.js` oxfmt drift. But `alef fmt` invokes every post-gen formatter at once (clang-format, ktlint, php-cs-fixer, dotnet format, mvn spotless, swift-format, mix format, ...) — CI doesn't install most of those (PHP `vendor/bin/php-cs-fixer` missing, clang-format pipeline empties to stdin), so the step fails with `[ffi] error: cannot use -i when reading from stdin` and `[php] Could not open input file`. Revert: only `alef sync-versions` runs; `alef 0.20.12` no longer drifts `index.js` post-sync so the original `-w --ignore-blank-lines` diff check passes idempotently. (`.github/workflows/{ci,ci-validate}.yaml`)
- **CI Validate: unblock rc.16 by pinning `pyproject-fmt==2.5.0`, applying `alef fmt` after `alef sync-versions`, and refreshing two version-pin manifests the rc.15→rc.16 regen missed.** Three independent CI failures on HEAD `447a5f78`: (1) the `pyproject-fmt` prek hook crashes in argparse (`add_argument("--table-format", help="...")`) under newer pyproject-fmt releases — pinned to `2.5.0` via `additional_dependencies` so prek installs the last working version without forking `kreuzberg-dev/pre-commit-hooks`; (2) the `Check version sync` step in `.github/workflows/{ci,ci-validate}.yaml` ran `alef sync-versions` and then failed `git diff -w` on `crates/ts-pack-core-node/index.js` where alef emits 2-space wrapped output that diverges from the oxfmt-formatted committed file — added `alef fmt` between sync and diff so the post-gen formatters bring output back to the committed shape; (3) `e2e/go/go.mod` still declared `v0.0.0` and `test_apps/swift/Package.swift` still declared `from: "1.8.1"`, which `alef sync-versions` on CI 0.20.10 would rewrite — hand-bumped both to `1.9.0-rc.16` so the diff check passes idempotently. (`.pre-commit-config.yaml`, `.github/workflows/ci-validate.yaml`, `.github/workflows/ci.yaml`, `e2e/go/go.mod`, `test_apps/swift/Package.swift`)
- **CI Validate: refresh pnpm `minimumReleaseAgeExclude` allowlist for rc.16 platform packages and pick up new `linux-*-musl` variants.** `CI Validate / Lint & Format` and `CI / Validate (Lint & Format)` were failing with `[ERR_PNPM_MINIMUM_RELEASE_AGE_VIOLATION]` against the six `@kreuzberg/tree-sitter-language-pack-*@1.9.0-rc.16` platform packages (published 2026-05-29T05:51Z, within the 24h supply-chain age cutoff). Bumped the six existing allowlist entries from `rc.11 → rc.16` and added two new entries for `linux-x64-musl` / `linux-arm64-musl`, which the alef 0.20.10 regen now declares as `optionalDependencies` of `crates/ts-pack-core-node/package.json`. Regenerated `pnpm-lock.yaml` (`pnpm install --lockfile-only`) so the eight platform entries match the manifest and `pnpm install --frozen-lockfile` succeeds in both the node and wasm workspaces. (`pnpm-workspace.yaml`, `pnpm-lock.yaml`)
- **Restore typed `DownloadError` for Python (and equivalent typed exceptions across other bindings).** Issue #133: `DownloadError` was dropped during the alef polyglot migration (commit `8557c150`) because the `Download`, `ChecksumMismatch`, and `CacheLock` variants on `crates/ts-pack-core/src/error.rs::Error` were `#[cfg(feature = "download")]`-gated, and the alef variant extractor skipped cfg-gated variants when generating the public exception taxonomy. `get_parser("not_a_real_language")` consequently raised a bare `RuntimeError` carrying the message `"Download error: ..."` instead of a catchable typed exception. The three variants are pure string carriers with no extra dependencies, so the cfg gates were unnecessary — they now live unconditionally on the `Error` enum. The next alef regen extracts them as `DownloadError`, `ChecksumMismatchError`, and `CacheLockError` (Python) and equivalents in every other binding, restoring the documented `except DownloadError` catch path. The `Json` and `Toml` variants remain feature-gated because they carry external dependency types. (`crates/ts-pack-core/src/error.rs`)

### Changed

- **Docs and READMEs bumped to 306 grammars after the gherkin addition.** Updated hand-written count references in `crates/ts-pack-core/{README.md,src/lib.rs}`, `docs/reference/api-*.md` (15 files), `skills/tree-sitter-language-pack/{SKILL.md,references/*.md}`, `.ai-rulez/domains/parser-compilation/context/tree-sitter-overview.md`, and the OOM-mitigation comments in `.github/workflows/publish.yaml`. Remaining `305` mentions in alef-generated package metadata (`packages/*/Cargo.toml`, `composer.json`, `pyproject.toml`, etc.) refresh on next `task alef:generate`; the `alef.toml` source-of-truth is already at 306.
- **repo**: Add `.gitattributes` marking all alef-generated output directories (`packages/**`, `crates/*-{py,php,ffi,node,wasm}/**`, `e2e/**`) as `linguist-generated=true` so generated files collapse in GitHub PR diffs.
- **Bump alef to 0.18.0 and regen all bindings, e2e, docs.** Major upstream restructure: workspace renamed `alef-cli` → `alef` (single distributable crate; 28 internal `alef-*` member crates yanked), Node/WASM crate directories renamed (`ts-pack-core-node`, `ts-pack-core-wasm`), and zig/c FFI search paths reorganised. Configuration follow-ups in this repo: `[crates.{node,wasm}.crate_dir]` overrides pin the napi/wasm-pack build to the renamed crate dirs; `napi build --platform --release` produces per-platform `.node` artifacts (fixes "Cannot find module './ts-pack-core-node.darwin-arm64.node'" on Node e2e); zig defaults in `packages/zig/build.zig` switched to `../../target/release` + `../../crates/ts-pack-core-ffi/include`, with `.task/zig.yml` and the `[crates.test.zig]` alef e2e step both passing `-Dffi_path=../../target/release`; C e2e command corrected from `./test_runner` → `./run_tests` and `.task/c.yml` switched from `--lang ffi` → `--lang c`; new `[crates.e2e].result_fields` array + `[crates.e2e.fields_c_types]` map drive alef's namespace-aware field navigation for the C `process_result.metrics → FileMetrics → uintptr_t` accessors. Upstream alef fix in 0.18.0: `namespace_stripped_path` no longer strips path segments when `result_fields` is empty, so legacy bindings (no `result_fields` configured) keep dotted-field paths intact. All 14 language e2e suites pass after regen.
- **Source-gem publish now uses the shared `rewrite-native-deps@v1` action.** The `publish-rubygems` job's source-gem fallback rewrites the native ext's workspace path-dependency (`packages/ruby/ext/ts_pack_core_rb/native/Cargo.toml` → `crates/ts-pack-core`) to a registry version-dependency so the shipped manifest resolves on user install. Replaced the dead "Set up Python (for vendor script)" + "Vendor core for source gem" steps with `kreuzberg-dev/actions/rewrite-native-deps@v1` (`lang: ruby`) before `gem build`, matching the precompiled `build-ruby-gem` job. (`.github/workflows/publish.yaml`)

### Removed

- **`wolfram` grammar dropped from the language pack.** `tree-sitter-wolfram` produces glibc heap corruption (`free(): invalid next size`) when parsing trivial input under serial test execution on Linux; macOS allocator silently tolerated the corruption. The entire upstream ecosystem is unmaintained (canonical `bostick/tree-sitter-wolfram` last touched 2021-11-11 with 3 stars; every known fork — `LumaKernel`, `LoganAMorrison`, `JuanG970`, `jakassebaum` — ships the same `LANGUAGE_VERSION 13` parser tables and is inactive). Rather than fork-and-maintain a Wolfram grammar in-house for marginal demand, the entry is removed from `language_definitions.json`, all CI `TSLP_LANGUAGES` lists, the smoke fixture, the e2e harness, the docs, and the README ecosystem listings. **Total supported grammar count drops from 306 to 305**, which matches the long-standing "305 languages" marketing copy (previously off-by-one due to the broken wolfram entry).
- **Dead workspace-vendor scripts superseded by shared GitHub Actions.** Deleted `scripts/ci/php/vendor-core.py` (rewrote the `exclude`d `crates/ts-pack-php` crate; publish uses the `crates/ts-pack-core-php` crate via `build-php-extension@v1`) and `scripts/ci/ruby/vendor-core.py` (targeted the nonexistent `crates/ts-pack-ruby` crate; no-op). Dropped the now-dangling `vendor` tasks from `.task/php.yml` and `.task/ruby.yml`; the local PHP `build`/`build:dev` tasks now build the `ts-pack-core-php` crate directly, mirroring CI. (`scripts/ci/php/vendor-core.py`, `scripts/ci/ruby/vendor-core.py`, `.task/php.yml`, `.task/ruby.yml`)

## 1.9.0-rc.1 - 2026-05-22

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

### Fixed

- **Download cache is now safe under concurrent multi-process access.** `DOWNLOAD_CACHE_LOCK` in `crates/ts-pack-core/src/lib.rs` was a `Mutex<()>` — intra-process only — so multi-worker servers (gunicorn / Puma / Node cluster), fan-out build pipelines (`make -j8`, parallel test runners), and the zig e2e suite (`zig build test` spawns eight test binaries in parallel) all raced on the same `~/.cache/tree-sitter-language-pack/v{version}/` directory. Partial `entry.unpack` writes were observable to other workers' `libloading::open`, producing intermittent `LanguageNotFound` / segfaults on first request for an uncached language; N processes could also each redundantly pull the 50MB platform bundle. Cache writes are now atomic (write to `<dest_dir>/.<name>.tmp.<pid>.<seq>` then `fs::rename` — readers see old, new, or nothing, never partial) and the bundle-fetch / extract / clean critical section is serialized across processes with an exclusive `fd-lock` on `<version_cache_dir>/.download.lock`. Double-checked locking preserves the lock-free hot path: steady-state `is_cached` lookups never pay the OS file-lock cost. New `Error::CacheLock(String)` variant surfaces lock-acquisition failures cleanly. Affects every binding (Python, Node.js, Ruby, PHP, Go, Java, C#, Elixir, WASM, Dart, Swift, Zig, Kotlin-Android) because the fix lives entirely in the shared `ts-pack-core` Rust crate. New `fd-lock = "4"` dependency (gated under the `download` feature). Cross-process safety relies on `flock` semantics, which are unreliable on NFS — users with `XDG_CACHE_HOME` on NFS should use a local-FS cache or serialize at the application layer. (`crates/ts-pack-core/src/{download.rs,error.rs}`, `crates/ts-pack-core/Cargo.toml`, `Cargo.toml`, new `crates/ts-pack-core/tests/concurrent_download.rs`)
- **Zig e2e auto-omits fixtures outside the static-compiled grammar set (regen on alef `65f1a129`).** Declared `[crates.zig].languages = [<curated 18-grammar list>]` mirroring the `TSLP_LANGUAGES` value in `[crates.test.zig].before`. Alef's new Zig codegen filter consults both `input.language` and `input.config.language` and drops fixtures whose target grammar is not in the list (mirroring the WASM `f9e0ff50` pattern). Eliminates `smoke_bibtex` and every other non-static-set test that previously failed at parser-load time. Also reverts the per-fixture `skip: { languages: ["zig"] }` workaround on `fixtures/smoke/actionscript.json` since the auto-omit subsumes it. (`alef.toml`, `fixtures/smoke/actionscript.json`)
- **swift e2e: `process` `contains` assertions on `Vec<DTO>` fields aggregate every stringy accessor (regen on alef `857c55d1`).** `testProcessPythonImportsDetail` and `testProcessRustStructureName` previously failed because the codegen relied on `result_field_accessor` naming a single "primary" accessor per array field (`imports → source`, `structure → kind`), which misses values surfaced on sibling fields — `"os"` against `ImportInfo.items`, `"MyConfig"` against `StructureItem.name` rather than `StructureKind`. The regenerated tests now emit a `contains(where: { item in … })` closure that gathers every text-bearing accessor (String, Option<String>, Vec<String>, serde-enum) into a `[String]` and substring-matches the expected value, mirroring python's `_alef_e2e_item_texts`. Swift e2e: 411 tests, 0 failures. (`e2e/swift_e2e/Tests/TreeSitterLanguagePackE2ETests/ProcessTests.swift`)
- **Maven JAR native layout collapses every classifier under `natives/native/` ([#128](https://github.com/kreuzberg-dev/tree-sitter-language-pack/issues/128)).** The re-stage loop in `build-maven-package` walked one `dirname` too far when extracting the classifier from each lib's path, so all six platform libs landed at `natives/native/{lib}` instead of `natives/{classifier}/{lib}`. The Maven Central JAR shipped in v1.8.1 contained only three files (one per `.so`/`.dylib`/`.dll` extension) and `TreeSitterLanguagePack.getParser("…")` failed with `UnsatisfiedLinkError: Expected resource: /natives/windows-x86_64/ts_pack_core_ffi.dll`. Fixed the path-walk depth, and hardened both build-side and deploy-side verification steps to require every `linux-x86_64 / linux-arm64 / macos-arm64 / macos-x86_64 / windows-x86_64 / windows-arm64` classifier directory is present in the staged JAR so the regression cannot ship again. Additionally corrected the Windows-ARM classifier from `windows-aarch64` to `windows-arm64`: the Java loader (`NativeLib.resolveNativesRid`) normalizes every ARM architecture to `arm64` and resolves to `natives/windows-arm64/`, so a JAR staged under `windows-aarch64` would still `UnsatisfiedLinkError` on Windows ARM64 — the publish matrix and both verification steps now use `windows-arm64`, consistent with the `linux-arm64` / `macos-arm64` classifiers and the loader. (`.github/workflows/publish.yaml`)
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
