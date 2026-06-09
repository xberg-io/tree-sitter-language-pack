---
description: "How to build tree-sitter-language-pack from source, configure feature flags, and compile parsers statically."
---

# Building from source

This guide covers building the Rust core from source — useful if you need static linking, a custom parser subset, or want to contribute to the library itself.

## Prerequisites

- Rust toolchain (see `rust-toolchain.toml` in the repository root for the pinned version)
- Python 3 (for the vendor scripts)
- A C compiler (`gcc` or `clang`) — required by `build.rs` to compile parser grammars
- The [Task](https://taskfile.dev) runner

Clone the repository:

```bash
git clone https://github.com/kreuzberg-dev/tree-sitter-language-pack
cd tree-sitter-language-pack
```

## Workspace layout

The Cargo workspace contains the following crates:

| Crate                      | Purpose                                       |
| -------------------------- | --------------------------------------------- |
| `crates/ts-pack-core`      | Core Rust library (parsers, download, config) |
| `crates/ts-pack-cli`       | `ts-pack` CLI binary                          |
| `crates/ts-pack-core-py`   | Python bindings (PyO3/maturin)                |
| `crates/ts-pack-core-node` | Node.js bindings (NAPI-RS)                    |
| `crates/ts-pack-core-php`  | PHP extension (ext-php-rs)                    |
| `crates/ts-pack-core-wasm` | WebAssembly bindings (wasm-bindgen)           |
| `crates/ts-pack-core-ffi`  | C-compatible FFI library (cbindgen)           |

Language-specific packages live under `packages/`: `python/`, `typescript/`, `ruby/`, `elixir/`, `php/`, `go/`, `java/`, `csharp/`, and `wasm/`.

## Cargo features

The core library (`tree-sitter-language-pack`) has four features:

| Feature           | Default | What it enables                                                  |
| ----------------- | ------- | ---------------------------------------------------------------- |
| `dynamic-loading` | Yes     | Load parser `.so`/`.dylib`/`.dll` files at runtime               |
| `download`        | Yes     | Download parsers from GitHub releases; implies `dynamic-loading` |
| `serde`           | No      | `Serialize`/`Deserialize` on public types                        |
| `config`          | No      | Read `language-pack.toml` config files; implies `serde`          |

To use the library without the download machinery (for example in a Wasm target or with statically compiled parsers):

```toml
[dependencies]
tree-sitter-language-pack = { version = "...", default-features = false }
```

## Build-time environment variables

`build.rs` reads these variables at compile time, not at runtime.

### `TSLP_LANGUAGES`

Comma-separated list of languages to compile statically into the binary. When set, `build.rs` compiles those parser grammars from source and links them in.

```bash
TSLP_LANGUAGES=python,rust,javascript cargo build
```

When not set (the default), no parsers get compiled statically. The library downloads them at runtime using the `download` feature.

Names must be alphanumeric or underscore and must exist in `sources/language_definitions.json`. Unknown names produce a build warning.

### `TSLP_LINK_MODE`

Controls how statically-selected parsers link. Requires `TSLP_LANGUAGES`.

| Value               | Effect                                                                 |
| ------------------- | ---------------------------------------------------------------------- |
| `dynamic` (default) | Compile parsers into `.so`/`.dylib`/`.dll` files, load them at runtime |
| `static`            | Link parsers directly into the binary                                  |
| `both`              | Produce both static and dynamic variants                               |

`wasm32` targets always use `static`, regardless of this setting.

### `TSLP_LINK_MODE=static` example

To produce a single self-contained binary:

```bash
TSLP_LANGUAGES=python,rust,javascript TSLP_LINK_MODE=static cargo build --release
```

### `PROJECT_ROOT`

Override the directory `build.rs` searches for `sources/language_definitions.json`. `build.rs` walks up the directory tree to find it automatically; this variable is useful for unusual build setups.

### `WASI_SYSROOT`

Path to the WASI sysroot when cross-compiling for `wasm32-wasi`. Used by `build.rs` when targeting that architecture.

## How build.rs works

`build.rs` (in `crates/ts-pack-core/`) runs every time environment variables or source files change. It does these steps:

1. **Reads `sources/language_definitions.json`** — 306 language entries, each specifying the grammar repository, revision, file extensions, and optional C symbol overrides.

2. **Compiles selected parsers** — when `TSLP_LANGUAGES` has a value, it invokes the system C compiler on each `parsers/<language>/src/parser.c`. The output format (static archive or shared library) follows from `TSLP_LINK_MODE`.

3. **Generates Rust source files** written to `OUT_DIR`:
   - `registry_generated.rs` — the language registry (name → parser function)
   - `extensions_generated.rs` — file extension to language name mapping
   - `ambiguities_generated.rs` — ambiguous extension lookup table
   - Query files for highlights, injections, and locals

The build embeds these generated files via `include!()` macros in `src/registry.rs` and `src/extensions.rs`.

## Vendor the grammar sources

Before building with `TSLP_LANGUAGES`, you need the parser C sources locally:

```bash
task clone
```

This runs `scripts/clone_vendors.py`, which checks out the correct revision for each grammar into `parsers/`. The script is idempotent — already-cloned grammars do not re-clone.

To clone a specific language:

```bash
python3 scripts/clone_vendors.py --languages python,rust
```

## Building the CLI

```bash
cargo build -p ts-pack-cli
# or
cargo build --release -p ts-pack-cli
```

The release profile uses thin LTO, a single codegen unit, `opt-level = 3`, and strips debug symbols.

## Running tests

```bash
# All Rust tests
cargo test -p tree-sitter-language-pack

# Run a single test
cargo test -p tree-sitter-language-pack detect_language_from_extension

# Criterion benchmarks
cargo bench -p tree-sitter-language-pack
```

Run `task --list` to see all available task commands including per-binding test commands.
