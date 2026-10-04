use std::env;
use std::fs;
use std::path::PathBuf;

/// Find the wasi-sysroot include path for wasm32 cross-compilation.
/// Checks WASI_SYSROOT env, then common Homebrew/system paths.
fn find_wasi_sysroot() -> Option<PathBuf> {
    if let Ok(sysroot) = env::var("WASI_SYSROOT") {
        let p = PathBuf::from(sysroot);
        if p.exists() {
            return Some(p);
        }
    }

    let candidates = [
        "/opt/homebrew/share/wasi-sysroot",
        "/usr/local/share/wasi-sysroot",
        "/opt/wasi-sdk/share/wasi-sysroot",
    ];
    for candidate in &candidates {
        let p = PathBuf::from(candidate);
        if p.exists() {
            return Some(p);
        }
    }

    if let Ok(entries) = fs::read_dir("/opt/homebrew/Cellar/wasi-libc") {
        for entry in entries.flatten() {
            let sysroot = entry.path().join("share/wasi-sysroot");
            if sysroot.exists() {
                return Some(sysroot);
            }
        }
    }

    None
}

/// Optimize cc::Build for wasm32 targets to reduce memory usage on CI runners.
/// Disables debug info to reduce object file sizes.
pub(crate) fn apply_wasm32_optimizations(build: &mut cc::Build) {
    if env::var("CARGO_CFG_TARGET_ARCH").unwrap_or_default() == "wasm32" {
        build.cargo_warnings(false);
        build.debug(false);
        build.opt_level(2);
    }
}

/// Default upper bound (bytes) for a grammar's `parser.c` when compiling to wasm32.
///
/// A handful of grammars ship pathologically large *generated* `parser.c` files (e.g. `abl` at
/// ~130 MB). Compiling one of those to wasm32 needs 18-25 GB＋ of clang RAM at *any* optimization
/// level (the cost is in parsing/IR-building the giant single-function source, not optimization),
/// which OOMs standard ≤16 GB CI runners — `CARGO_BUILD_JOBS=1` cannot help because a single file
/// already exceeds the budget. 40 MB keeps every common language (including the ~40 MB `sql`
/// grammar) while excluding only the unbuildable outliers.
const DEFAULT_WASM_MAX_PARSER_BYTES: u64 = 40 * 1024 * 1024;

/// Resolve the wasm32 `parser.c` size limit. Returns `None` (gate disabled) only when
/// `TSLP_WASM_MAX_PARSER_BYTES=0`. Any unparsable value falls back to the default.
pub(crate) fn wasm_parser_size_limit() -> Option<u64> {
    match env::var("TSLP_WASM_MAX_PARSER_BYTES") {
        Ok(raw) => match raw.trim().parse::<u64>() {
            Ok(0) => None,
            Ok(limit) => Some(limit),
            Err(_) => Some(DEFAULT_WASM_MAX_PARSER_BYTES),
        },
        Err(_) => Some(DEFAULT_WASM_MAX_PARSER_BYTES),
    }
}

/// Grammars whose external scanners cannot be compiled/linked for
/// `wasm32-unknown-unknown` and are skipped on that target by default.
///
/// Each depends on host libc/libc++ facilities that wasi-libc does not provide
/// freestanding:
/// - `gitcommit`, `perl` — `<wctype.h>` (`iswcntrl`/`iswspace`) needs locale
///   tables absent on wasm32; `perl` also calls `fprintf(stderr, …)` in a DEBUG
///   macro (no stderr on wasm32).
/// - `mojo`, `nim` — C++ `<cwctype>` (`std::iswspace`) fails to link against
///   wasi-libc's C++ wctype/locale layer.
/// - `norg` — C++ `<regex>` + `<locale>` + `<iostream>`, none available on wasm32.
/// - `tmux` — its ~30 MB generated `parser.c` is under the size gate but still
///   overruns the wasm32 clang backend during codegen (fails to compile), so it
///   is skipped explicitly rather than by the byte-size heuristic.
///
/// These are niche grammars; dropping them from the wasm bundle degrades
/// gracefully (absent from `STATIC_LANGUAGES`, no dangling FFI symbol). Override
/// the set with `TSLP_WASM_SKIP_GRAMMARS` (comma-separated; empty disables the
/// skip). Mirrors the `TSLP_WASM_MAX_PARSER_BYTES` size-gate pattern.
const DEFAULT_WASM_SKIP_GRAMMARS: [&str; 6] = ["gitcommit", "mojo", "nim", "norg", "perl", "tmux"];

/// Resolve the wasm32 grammar skip-list. Returns the default set unless
/// `TSLP_WASM_SKIP_GRAMMARS` is set, in which case its comma-separated entries
/// replace the default (an empty/whitespace value disables the skip entirely).
pub(crate) fn wasm_skip_grammars() -> Vec<String> {
    match env::var("TSLP_WASM_SKIP_GRAMMARS") {
        Ok(raw) => raw
            .split(',')
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect(),
        Err(_) => DEFAULT_WASM_SKIP_GRAMMARS.iter().map(|s| s.to_string()).collect(),
    }
}

/// Target-triple subdirectories under a wasi-sysroot's `include/`, newest layout first.
///
/// ~keep wasi-libc renamed `wasm32-wasi` to `wasm32-wasip1` (wasi-sdk 22+; current Homebrew
/// `wasi-libc` ships only the `p1`/`p2`/`p3` spellings). Probing a single hardcoded name meant a
/// sysroot that existed but used the newer layout added no include flag at all, and every grammar
/// then failed with `'stdlib.h' file not found`. `p2`/`p3` target the component model and are
/// deliberately not probed. Toolchains whose clang already knows its own sysroot — the wasi-sdk
/// used in CI — compile fine with no match here, which is why this stayed invisible.
const WASI_INCLUDE_SUBDIRS: [&str; 2] = ["include/wasm32-wasi", "include/wasm32-wasip1"];

/// Apply wasi-sysroot includes to a cc::Build for wasm32 targets.
///
/// Use `-isystem` to add the wasi include dir which has stdlib.h etc.
/// Avoid `--sysroot` which pulls in wasi/api.h through stdio.h and fails
/// for wasm32-unknown-unknown targets.
pub(crate) fn apply_wasm32_sysroot(build: &mut cc::Build) {
    if env::var("CARGO_CFG_TARGET_ARCH").unwrap_or_default() != "wasm32" {
        return;
    }

    // ~keep tree-sitter 0.27 links its own wasm libc, but that libc is a documented *subset*:
    // `src/wasm-stdlib/imports.txt` enumerates what a scanner may import, and `assert` is not on
    // it ("Wasm language modules are compiled without a C standard library"). A scanner that keeps
    // its `assert()` calls therefore emits an unresolved `__assert_fail`, which rust-lld turns
    // into an `env` module import rather than a link error -- the module then loads nowhere, and
    // every wasm test dies with `Cannot find module 'env'`. Compiling the asserts out is the only
    // resolution available on this target; native builds keep them.
    build.define("NDEBUG", None);

    let Some(sysroot) = find_wasi_sysroot() else {
        println!(
            "cargo:warning=wasm32 target detected but no wasi-sysroot found. \
             Install wasi-libc (brew install wasi-libc) or set WASI_SYSROOT env var."
        );
        return;
    };

    let wasi_include = WASI_INCLUDE_SUBDIRS
        .iter()
        .map(|d| sysroot.join(d))
        .find(|p| p.exists());

    match wasi_include {
        Some(include) => {
            // ~keep Define __wasi__ only for parser C compilation so wasi/api.h guards pass.
            build.define("__wasi__", None);
            build.flag(format!("-isystem{}", include.display()));
        }
        None => println!(
            "cargo:warning=wasi-sysroot at {} has none of {:?}; compiling parsers without its \
             headers. A clang that does not supply its own sysroot will fail on <stdlib.h>.",
            sysroot.display(),
            WASI_INCLUDE_SUBDIRS
        ),
    }
}
