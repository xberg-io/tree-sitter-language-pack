---
title: Architecture
description: "How tree-sitter-language-pack is structured: Rust core, thin binding layer, and the download system."
---

Tree-sitter-language-pack follows a layered architecture: a single Rust core library handles all parsing logic, and thin binding layers expose that API natively in each target language. No business logic lives in the bindings — they are pure translation layers.

---

## High-Level Diagram

```mermaid
graph TD
    subgraph Bindings["Language Bindings"]
        PY["Python<br/>(PyO3 / maturin)"]
        NODE["Node.js<br/>(NAPI-RS)"]
        RB["Ruby<br/>(Magnus)"]
        EL["Elixir<br/>(Rustler NIF)"]
        PHP["PHP<br/>(ext-php-rs)"]
        WASM["WebAssembly<br/>(wasm-bindgen)"]
        FFI["C FFI<br/>(cbindgen)"]
    end

    subgraph FFIConsumers["FFI Consumers"]
        GO["Go<br/>(cgo)"]
        JAVA["Java<br/>(Panama FFM)"]
        CS["C# / .NET<br/>(P/Invoke)"]
    end

    subgraph Core["Rust Core (ts-pack-core)"]
        DL["Download Manager"]
        CACHE["Parser Cache"]
        PROC["Code Intelligence Engine"]
        CHUNK["Chunker"]
        TS["tree-sitter runtime"]
    end

    subgraph Parsers["Parser Binaries (remote)"]
        MANIFEST["parsers.json manifest"]
        BIN["Platform-specific .so / .dll / .dylib"]
    end

    PY --> Core
    NODE --> Core
    RB --> Core
    EL --> Core
    PHP --> Core
    WASM --> Core
    FFI --> Core
    GO --> FFI
    JAVA --> FFI
    CS --> FFI

    DL -->|"HTTPS download"| MANIFEST
    DL -->|"fetch binary"| BIN
    CACHE -->|"dlopen"| BIN
    Core --> TS
```

---

## Rust Core

All logic lives in a single crate: `crates/ts-pack-core`.

| Component                    | Responsibility                                                                                                              |
| ---------------------------- | --------------------------------------------------------------------------------------------------------------------------- |
| **Download Manager**         | Resolves the remote manifest, fetches platform-specific parser binaries, stores them in the local cache.                    |
| **Parser Cache**             | Maps language names to loaded `tree_sitter::Language` values. Once loaded, a parser is reused without re-reading from disk. |
| **Code Intelligence Engine** | Runs tree-sitter queries against a parsed tree to extract structure, imports, exports, symbols, comments, and docstrings.   |
| **Chunker**                  | Walks the syntax tree and splits source code at natural boundaries, respecting a configurable token budget.                 |

The core has no language-specific code. It calls tree-sitter through its stable C ABI using dynamically loaded parser binaries.

---

## Binding Layer

Each binding is a thin crate that:

1. Calls Rust core functions.
2. Converts Rust types to the target language's native types (`String` → `str`, `Vec<T>` → list/array, `Result<T, E>` → exception/error).
3. Exposes an idiomatic API matching the target language's conventions.

Binding crates contain no parsing logic, no query definitions, and no chunking code.

| Location                   | Framework        | Distribution         |
| -------------------------- | ---------------- | -------------------- |
| `crates/ts-pack-core-py`   | PyO3 + maturin   | PyPI wheels          |
| `crates/ts-pack-core-node` | NAPI-RS          | npm (multi-platform) |
| `packages/ruby`            | Magnus           | RubyGems native gem  |
| `packages/elixir`          | Rustler NIF      | Hex.pm               |
| `crates/ts-pack-core-php`  | ext-php-rs       | Packagist            |
| `crates/ts-pack-core-wasm` | wasm-bindgen     | npm (Wasm)           |
| `crates/ts-pack-core-ffi`  | cbindgen (C FFI) | GitHub releases      |
| `packages/go`              | cgo              | Go modules           |
| `packages/java`            | Panama FFM       | Maven Central        |
| `packages/csharp`          | P/Invoke         | NuGet                |

---

## Parser Binaries

Tree-sitter parsers are **not** compiled into the package. Instead:

1. A `parsers.json` manifest (on GitHub releases) lists one bundle per target platform plus per-language metadata for all 306 grammars.
2. On first use, the matching platform bundle downloads and extracts to the local cache directory.
3. The runtime opens the relevant grammar binary via `dlopen` / `LoadLibrary` and resolves the `tree_sitter_<language>` symbol.

This keeps installation fast and download sizes minimal. See [Download Model](download-model.md) for the full detail.

---

## Repository Layout

```text
tree-sitter-language-pack/
├── crates/
│   ├── ts-pack-core/       # Rust core library
│   ├── ts-pack-cli/        # CLI binary
│   ├── ts-pack-core-py/    # Python (PyO3) binding
│   ├── ts-pack-core-node/  # Node.js (NAPI-RS) binding
│   ├── ts-pack-core-php/   # PHP (ext-php-rs) extension
│   ├── ts-pack-core-wasm/  # WebAssembly (wasm-bindgen) binding
│   └── ts-pack-core-ffi/   # C FFI for Go / Java / C#
├── packages/
│   ├── python/             # Python package wrapper
│   ├── typescript/         # TypeScript / Node.js package
│   ├── ruby/               # Ruby gem (Magnus NIF)
│   ├── elixir/             # Elixir package (Rustler NIF)
│   ├── php/                # PHP Composer package
│   ├── go/                 # Go module (cgo wrapper)
│   ├── java/               # Java package (Panama FFM)
│   ├── csharp/             # C# / .NET package (P/Invoke)
│   └── wasm/               # WebAssembly npm package
├── sources/
│   └── language_definitions.json  # Grammar source registry
└── scripts/
    └── generate_readme.py  # README sync tooling
```

Documentation snippets under `docs/snippets/` are discovered, validated, and
audited by `alef snippets` (see `task docs:snippets:check`). The legacy
`tools/snippet-runner` Rust crate was retired in favour of this shared
capability.

---

## Design Principles

- **Single source of truth** — All parsing and intelligence logic lives in `ts-pack-core`. Binding crates are pure glue.
- **On-demand downloads** — Parsers do not ship in the package. The pack fetches and caches them per-platform on first use.
- **ABI stability** — The C FFI layer (`ts-pack-core-ffi`) follows strict semantic versioning. Go, Java, and C# bindings depend on a stable C ABI, not Rust internals.
- **Zero duplication** — Query definitions, chunking strategies, and intelligence extraction are each written once in Rust and reused across all 11 language surfaces.
