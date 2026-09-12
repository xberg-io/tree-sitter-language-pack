// Build scripts run before the crate compiles and cannot route through `tracing`; stderr is
// their diagnostic channel (stdout is reserved for `cargo:` directives, exempt natively). ~keep
#![allow(clippy::print_stderr)]
// A build script is not the library: it links no host process, so an unrecoverable
// condition here should abort the build loudly rather than be threaded through a
// `Result` no caller can act on. The crate-wide `unwrap_used`/`expect_used` deny
// exists to stop a panic taking down a PHP or Python host, which build.rs cannot do. ~keep
#![allow(clippy::unwrap_used, clippy::expect_used)]

use serde::Deserialize;
use sha2::{Digest, Sha256};
use std::collections::BTreeMap;
use std::env;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::time::Duration;

/// Workspace-relative path of the helper that clones upstream grammar sources.
const CLONE_VENDORS_SCRIPT: &str = "scripts/clone_vendors.py";

/// Env var naming an explicit interpreter for [`CLONE_VENDORS_SCRIPT`].
const CLONE_VENDORS_INTERPRETER_ENV: &str = "TSLP_PYTHON";

/// Interpreter probe list used when [`CLONE_VENDORS_INTERPRETER_ENV`] is unset.
const DEFAULT_CLONE_VENDORS_RUNNERS: &[&[&str]] =
    &[&["uv", "run", "--no-sync"], &["uv", "run"], &["python3"], &["python"]];

#[derive(Debug, Deserialize)]
struct LanguageDefinition {
    #[allow(dead_code)]
    #[serde(default)]
    repo: Option<String>,
    #[allow(dead_code)]
    rev: Option<String>,
    #[allow(dead_code)]
    branch: Option<String>,
    #[allow(dead_code)]
    directory: Option<String>,
    #[allow(dead_code)]
    generate: Option<bool>,
    #[allow(dead_code)]
    abi_version: Option<u32>,
    #[serde(default)]
    extensions: Vec<String>,
    #[serde(default)]
    ambiguous: BTreeMap<String, Vec<String>>,
    /// Override for the C symbol name when it differs from the language name.
    /// E.g. language "csharp" exports `tree_sitter_c_sharp()`.
    #[serde(default)]
    c_symbol: Option<String>,
}

fn find_project_root() -> PathBuf {
    if let Ok(root) = env::var("PROJECT_ROOT") {
        return PathBuf::from(root);
    }

    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());

    // ~keep `parsers/` is gitignored and only materializes once `ensure_parser_sources`
    // runs later in this same build script invocation, so it cannot be used to detect the
    // workspace root on a fresh checkout (find_project_root runs first). `patches/` is
    // committed and is the same directory `patch_grammar_sources` needs immediately after,
    // so it is a stable, always-present workspace-root marker.
    let mut dir = manifest_dir.as_path();
    loop {
        if dir.join("sources/language_definitions.json").exists() && dir.join("patches").is_dir() {
            return dir.to_path_buf();
        }
        match dir.parent() {
            Some(parent) => dir = parent,
            None => break,
        }
    }

    // ~keep Published crates include crate-local language_definitions.json but not workspace sources/parsers.
    manifest_dir
}

/// Locate the vendored deterministic wide-ctype shim assets.
///
/// ~keep Scanner character classification must be identical on every platform.
/// Returns `(force_include_header, utf8proc_include_dir, utf8proc_source)`; the
/// header is force-included ahead of each scanner TU and utf8proc is linked in
/// to back its wrappers.
fn ctype_shim_assets() -> (PathBuf, PathBuf, PathBuf) {
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let header = manifest_dir.join("src/ctype_shim.h");
    let utf8proc_include = manifest_dir.join("third_party/utf8proc");
    let utf8proc_source = utf8proc_include.join("utf8proc.c");
    (header, utf8proc_include, utf8proc_source)
}

/// Force-include the deterministic wide-ctype shim into a [`cc::Build`] and put
/// the vendored utf8proc header on its include path.
///
/// ~keep Grammars excluded from the deterministic wide-ctype shim. `norg`'s C++
/// scanner pulls in heavy libstdc++ headers (<cwctype>, <locale>, <regex>) that
/// re-declare the wide-ctype classifiers AFTER this shim's function-like redirect
/// macros are in scope; under musl (Alpine/Docker) those redeclarations expand
/// through the macros and fail to compile (`ts_pack_iswspace redeclared as a
/// different kind of entity` + `expected primary-expression before ')'`). norg
/// calls the classifiers unqualified on essentially ASCII structural characters,
/// so falling back to libc for this one grammar costs no meaningful cross-platform
/// determinism while keeping it buildable. Applied on every path so norg tokenizes
/// consistently (always libc) regardless of static/dynamic linkage.
const CTYPE_SHIM_EXCLUDED_LANGUAGES: &[&str] = &["norg"];

fn ctype_shim_excluded(name: &str) -> bool {
    CTYPE_SHIM_EXCLUDED_LANGUAGES.contains(&name)
}

/// ~keep Used by the static-link path (the dynamic path drives the compiler via
/// a raw `Command`). The shim only *references* utf8proc; the implementation is
/// linked once via [`compile_utf8proc_archive`] to avoid duplicate symbols when
/// many grammars are statically linked into one binary.
fn apply_ctype_shim(build: &mut cc::Build, header: &Path, utf8proc_include: &Path) {
    build.include(utf8proc_include);
    // ~keep Must match compile_utf8proc_archive's UTF8PROC_STATIC define: this shim's TU only
    // *declares* utf8proc_* (dllimport on MSVC without it), while the archive *defines* them: a
    // mismatch produces unresolved __imp_utf8proc_* references at link time.
    build.define("UTF8PROC_STATIC", None);
    if build.get_compiler().is_like_msvc() {
        build.flag(format!("/FI{}", header.display()));
    } else {
        build.flag("-include");
        build.flag(header.to_str().expect("ctype shim path is valid UTF-8"));
    }
}

/// Extra compiler flags applied to the vendored grammar C/C++ on the static-link
/// path, and to nothing else.
///
/// ~keep This exists because `CFLAGS_<triple>` cannot express "instrument only the
/// grammars". cc-rs keys that variable on the triple alone, and on a Linux x86_64
/// runner the host triple equals the target triple — so it also reaches the C in
/// every *build-dependency* (zstd-sys, ring), whose objects are linked into this
/// crate's build-script binary. Cargo deliberately withholds RUSTFLAGS from host
/// units when `--target` is passed, so that link never gets `-lasan`/`-lubsan` and
/// fails with undefined `__asan_*`/`__ubsan_*` (CI Sanitize run 32550410081). The
/// dynamic-link path is not covered: it drives the compiler through a raw `Command`
/// and would additionally need `-shared-libasan` on each shared object.
fn grammar_extra_flags() -> Vec<String> {
    env::var("TSLP_GRAMMAR_CFLAGS")
        .unwrap_or_default()
        .split_whitespace()
        .map(str::to_string)
        .collect()
}

fn apply_grammar_extra_flags(build: &mut cc::Build) {
    for flag in grammar_extra_flags() {
        build.flag(flag);
    }
}

/// Compile the vendored utf8proc exactly once into a static archive linked into
/// the final binary, backing the wide-ctype shim's wrappers.
///
/// ~keep Compiled once (not per grammar) so its symbols are not multiply defined
/// across the statically linked scanner archives.
fn compile_utf8proc_archive(utf8proc_source: &Path) {
    let mut build = cc::Build::new();
    build
        .file(utf8proc_source)
        .flag_if_supported("-fvisibility=hidden")
        .warnings(false);
    // ~keep Without this, MSVC sees every utf8proc_* symbol declared dllimport (utf8proc.h's
    // default under _WIN32) and then defined in this same TU: error C2491. Static archives export
    // via the archive itself, not __declspec(dllexport/dllimport).
    build.define("UTF8PROC_STATIC", None);
    build.std("c11");
    apply_wasm32_sysroot(&mut build);
    apply_wasm32_optimizations(&mut build);
    build.compile("ts_pack_utf8proc");
}

/// Reject grammar keys that are not plain identifiers.
///
/// Every key becomes a filesystem path segment (`parsers/<key>`), a `cc` output
/// archive name, and — on the dynamic path — an argument in a raw compiler
/// [`std::process::Command`]. A key beginning with `-` would be parsed by the
/// compiler as a flag, and separators or shell metacharacters would escape the
/// parsers tree. Enforce the same alphabet [`selected_languages`] already
/// requires of `TSLP_LANGUAGES`. ~keep
fn validate_definition_keys(definitions: &BTreeMap<String, LanguageDefinition>) {
    let invalid: Vec<&str> = definitions
        .keys()
        .filter(|name| {
            name.is_empty()
                || !name
                    .chars()
                    .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '_')
        })
        .map(String::as_str)
        .collect();
    assert!(
        invalid.is_empty(),
        "Invalid grammar name(s) in language_definitions.json: {}. Grammar names become filesystem paths and compiler arguments, so they must match ^[a-z0-9_]+$.",
        invalid.join(", "),
    );
}

fn selected_languages(definitions: &BTreeMap<String, LanguageDefinition>) -> Vec<String> {
    // ~keep TSLP_LANGUAGES selects static grammars; unset means runtime download via `download`.
    if let Ok(langs) = env::var("TSLP_LANGUAGES") {
        let trimmed = langs.trim();
        // ~keep `all` or `*` compiles every grammar for CI/WASM paths without runtime downloads.
        if trimmed.eq_ignore_ascii_case("all") || trimmed == "*" {
            return definitions.keys().cloned().collect();
        }
        // ~keep A duplicate name here (e.g. a CI shard selection that concatenates two
        // ~keep overlapping lists) would make `generate_registry` emit two `extern "C"`
        // ~keep blocks for the same symbol and fail the build with E0428. `seen` is only
        // ~keep ever probed with `insert`, never iterated, so de-duplicating this way
        // ~keep stays deterministic and preserves first-occurrence order.
        let mut seen = std::collections::HashSet::new();
        let selected: Vec<String> = trimmed
            .split(',')
            .map(|s| s.trim().to_string())
            .filter(|name| seen.insert(name.clone()))
            .collect();
        for name in &selected {
            if !name.chars().all(|c| c.is_ascii_alphanumeric() || c == '_') {
                panic!(
                    "Invalid language name in TSLP_LANGUAGES: '{}'. Only alphanumeric and underscore characters are allowed.",
                    name
                );
            }
            if !definitions.contains_key(name) {
                eprintln!(
                    "Language '{}' from TSLP_LANGUAGES not found in language_definitions.json",
                    name
                );
            }
        }
        return selected;
    }

    // ~keep wasm32 lacks dynamic loading/downloads; default to a curated static subset to avoid clang OOM.
    if env::var("CARGO_CFG_TARGET_ARCH").unwrap_or_default() == "wasm32" {
        return WASM_DEFAULT_LANGUAGES
            .iter()
            .filter(|name| definitions.contains_key(**name))
            .map(|name| (*name).to_string())
            .collect();
    }

    Vec::new()
}

/// Curated wasm32 default language subset (~50 common web + mainstream languages).
///
/// A full 371-grammar wasm build is impractical: the parser.c sources total ~1.7 GB,
/// the resulting bundle is far too large for browsers, and the largest grammars OOM
/// the wasm32 clang backend (see [`DEFAULT_WASM_MAX_PARSER_BYTES`]). This allowlist is
/// the default wasm surface; override it with `TSLP_LANGUAGES` (comma-separated, or
/// `all`) for a custom or full wasm build.
const WASM_DEFAULT_LANGUAGES: &[&str] = &[
    "javascript",
    "typescript",
    "tsx",
    "html",
    "css",
    "scss",
    "json",
    "yaml",
    "toml",
    "xml",
    "markdown",
    "python",
    "rust",
    "go",
    "java",
    "c",
    "cpp",
    "csharp",
    "ruby",
    "php",
    "bash",
    "lua",
    "kotlin",
    "swift",
    "scala",
    "dart",
    "elixir",
    "haskell",
    "r",
    "sql",
    "graphql",
    "dockerfile",
    "make",
    "cmake",
    "hcl",
    "proto",
    "vue",
    "svelte",
    "regex",
    "ini",
    "perl",
    "objc",
    "julia",
    "ocaml",
    "erlang",
    "clojure",
    "groovy",
    "powershell",
    "nix",
];

/// Get the target OS, using CARGO_CFG_TARGET_OS for cross-compilation correctness.
fn target_os() -> String {
    env::var("CARGO_CFG_TARGET_OS").unwrap_or_else(|_| {
        if cfg!(target_os = "macos") {
            "macos".to_string()
        } else if cfg!(target_os = "windows") {
            "windows".to_string()
        } else {
            "linux".to_string()
        }
    })
}

/// Get shared library filename components for the target OS.
fn shared_lib_components(target_os: &str) -> (&'static str, &'static str) {
    match target_os {
        "macos" | "ios" => ("lib", "dylib"),
        "windows" => ("", "dll"),
        _ => ("lib", "so"),
    }
}

/// Compile a single language parser into a shared library (.so/.dylib/.dll).
///
/// When a language has a `c_symbol` override (e.g. "csharp" → "c_sharp"),
/// the library is named using the c_symbol so the runtime loader can find it.
fn compile_parser_dynamic(name: &str, c_symbol: Option<&str>, parser_dir: &Path, output_dir: &Path) -> bool {
    let src_dir = parser_dir.join("src");
    let parser_c = src_dir.join("parser.c");

    if !parser_c.exists() {
        println!(
            "cargo:warning=Skipping language '{}': parser.c not found at {}",
            name,
            parser_c.display()
        );
        return false;
    }

    let mut c_sources = vec![parser_c];
    let scanner_c = src_dir.join("scanner.c");
    let has_scanner_c = scanner_c.exists();
    if has_scanner_c {
        c_sources.push(scanner_c);
    }

    let scanner_cc = src_dir.join("scanner.cc");
    let has_scanner = has_scanner_c || scanner_cc.exists();
    // ~keep Force-include the ctype shim (and link utf8proc) only for scanners not
    // carved out via `ctype_shim_excluded` (see its doc comment for why norg is).
    let shim = has_scanner && !ctype_shim_excluded(name);

    let mut includes = vec![src_dir.clone()];
    let common_dir = parser_dir.join("common");
    if common_dir.exists() {
        includes.push(common_dir);
    }

    // ~keep Only scanners call the locale-divergent libc wide-ctype; back them with
    // the deterministic utf8proc shim (force-included below) and link utf8proc in.
    let (ctype_shim_header, utf8proc_include, utf8proc_source) = ctype_shim_assets();
    if shim {
        includes.push(utf8proc_include);
        c_sources.push(utf8proc_source);
    }

    let sym = c_symbol.unwrap_or(name);
    let lib_name = format!("tree_sitter_{sym}");
    let os = target_os();
    let (prefix, ext) = shared_lib_components(&os);
    let output_path = output_dir.join(format!("{prefix}{lib_name}.{ext}"));

    let compiler = cc::Build::new().get_compiler();
    let is_msvc = compiler.is_like_msvc();

    let mut cmd = compiler.to_command();

    if is_msvc {
        // ~keep Use per-file MSVC language flags; global `/TC`/`/TP` breaks parser.c C99 initializers.
        cmd.arg("/std:c11");
        cmd.arg("/utf-8");
        cmd.arg("/O2");
        cmd.arg("/wd4244");
        cmd.arg("/wd4566");
        cmd.arg("/wd4819");
        if shim {
            // ~keep utf8proc.c is compiled straight into this shared library, and utf8proc.h
            // decorates every utf8proc_* declaration __declspec(dllimport) unless this is
            // defined — so MSVC rejects the definitions in the same TU with C2491 ("definition
            // of dllimport function not allowed") and every scanner grammar fails. Both
            // static-link paths (apply_ctype_shim, compile_utf8proc_archive) already define it;
            // this third, raw-Command path did not, so no Windows parser binary ever built.
            cmd.arg("/DUTF8PROC_STATIC");
            cmd.arg(format!("/FI{}", ctype_shim_header.display()));
        }
        for inc in &includes {
            cmd.arg(format!("/I{}", inc.display()));
        }
        for src in &c_sources {
            cmd.arg(format!("/Tc{}", src.display()));
        }
        if scanner_cc.exists() {
            cmd.arg(format!("/Tp{}", scanner_cc.display()));
        }
        cmd.arg("/LD");
        cmd.arg(format!("/Fe:{}", output_path.display()));
    } else {
        cmd.arg("-std=c11");
        cmd.arg("-O2");
        cmd.arg("-fno-strict-aliasing");
        cmd.arg("-fPIC");
        for inc in &includes {
            cmd.arg(format!("-I{}", inc.display()));
        }
        if shim {
            // ~keep Harmless on non-MSVC (utf8proc.h only decorates under _WIN32), but keeps the
            // three compile paths defining the same macro set. See the MSVC branch above.
            cmd.arg("-DUTF8PROC_STATIC");
            cmd.arg("-include");
            cmd.arg(&ctype_shim_header);
        }
        for src in &c_sources {
            cmd.arg(src);
        }
        if scanner_cc.exists() {
            // ~keep Mixed C/C++ shared libs compile scanner.cc to an object first, then link everything.
            let scanner_obj = output_dir.join(format!("{name}_scanner.o"));
            let cpp_compiler = cc::Build::new().cpp(true).get_compiler();
            let mut cpp_cmd = cpp_compiler.to_command();
            cpp_cmd.arg("-c");
            cpp_cmd.arg("-fPIC");
            cpp_cmd.arg("-O2");
            cpp_cmd.arg("-fno-strict-aliasing");
            for inc in &includes {
                cpp_cmd.arg(format!("-I{}", inc.display()));
            }
            if shim {
                // ~keep The shim header declares utf8proc_*; this object links into the same
                // shared library that defines them, so it must agree on the decoration.
                cpp_cmd.arg("-DUTF8PROC_STATIC");
                cpp_cmd.arg("-include");
                cpp_cmd.arg(&ctype_shim_header);
            }
            cpp_cmd.arg(&scanner_cc);
            cpp_cmd.arg("-o");
            cpp_cmd.arg(&scanner_obj);
            let cpp_status = cpp_cmd.status();
            match cpp_status {
                Ok(s) if s.success() => {
                    cmd.arg(&scanner_obj);
                }
                Ok(s) => {
                    println!(
                        "cargo:warning=Failed to compile C++ scanner for '{}': exit code {:?}",
                        name,
                        s.code()
                    );
                    return false;
                }
                Err(e) => {
                    println!("cargo:warning=Failed to run C++ compiler for '{}': {}", name, e);
                    return false;
                }
            }
        }

        if os == "macos" || os == "ios" {
            cmd.arg("-dynamiclib");
        } else {
            cmd.arg("-shared");
        }
        // ~keep C++ scanners require the C++ standard library for std:: and ABI symbols.
        if scanner_cc.exists() {
            if os == "macos" || os == "ios" {
                cmd.arg("-lc++");
            } else {
                cmd.arg("-lstdc++");
            }
        }
        cmd.arg("-o");
        cmd.arg(&output_path);
    }

    let status = cmd.status();
    match status {
        Ok(s) if s.success() => {
            // ~keep Some compilers return success without a dylib after resource exhaustion; verify output exists.
            if output_path.exists() {
                true
            } else {
                println!(
                    "cargo:warning=Failed to compile shared library for '{}': compiler succeeded but output file was not created at {}",
                    name,
                    output_path.display()
                );
                false
            }
        }
        Ok(s) => {
            println!(
                "cargo:warning=Failed to compile shared library for '{}': exit code {:?}",
                name,
                s.code()
            );
            false
        }
        Err(e) => {
            println!("cargo:warning=Failed to run compiler for '{}': {}", name, e);
            false
        }
    }
}

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
fn apply_wasm32_optimizations(build: &mut cc::Build) {
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
fn wasm_parser_size_limit() -> Option<u64> {
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
fn wasm_skip_grammars() -> Vec<String> {
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
fn apply_wasm32_sysroot(build: &mut cc::Build) {
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

/// Prefix collision-prone scanner helper symbols (`scan`, `serialize`,
/// `deserialize`, `scan_comment`) with the language name so multiple statically
/// linked grammars don't clash on identically-named globals.
///
/// Skipped for scanners that build their tree-sitter API through the
/// `tree_sitter_external_scanner(<symbol>)` macro: there `scan`/`serialize`/
/// `deserialize` appear as bare macro *arguments*, so these `-D` defines would
/// rewrite them and mangle the ABI entry points (e.g.
/// `tree_sitter_moonbit_external_scanner_scan`), leaving them undefined at link.
/// Such scanners keep their own helpers `static`, so nothing collides. ~keep
fn apply_scanner_symbol_prefix(build: &mut cc::Build, name: &str, scanner_path: &Path) {
    let uses_ext_macro = fs::read_to_string(scanner_path)
        .map(|src| src.contains("tree_sitter_external_scanner("))
        .unwrap_or(false);
    if uses_ext_macro {
        return;
    }
    build
        .define("scan", &*format!("tree_sitter_{name}_ext_scan"))
        .define("deserialize", &*format!("tree_sitter_{name}_ext_deserialize"))
        .define("serialize", &*format!("tree_sitter_{name}_ext_serialize"))
        .define("scan_comment", &*format!("tree_sitter_{name}_ext_scan_comment"));
}

/// Compile a parser statically and link it into the main binary.
///
/// Compiles parser.c and scanner.c/cc separately to avoid symbol collisions
/// when statically linking multiple grammars. Scanner functions (`scan`,
/// `deserialize`, `serialize`, `scan_comment`) are prefixed with the language
/// name via C preprocessor defines (see [`apply_scanner_symbol_prefix`]).
fn compile_parser_static(name: &str, parser_dir: &Path) -> bool {
    let src_dir = parser_dir.join("src");
    let parser_c = src_dir.join("parser.c");
    let common_dir = parser_dir.join("common");

    let mut build = cc::Build::new();
    build
        .include(&src_dir)
        .file(&parser_c)
        .define("TREE_SITTER_HIDE_SYMBOLS", None)
        .flag_if_supported("-fvisibility=hidden")
        .flag_if_supported("-fno-strict-aliasing")
        .warnings(false);
    build.std("c11");
    apply_wasm32_sysroot(&mut build);
    apply_wasm32_optimizations(&mut build);
    apply_grammar_extra_flags(&mut build);
    if common_dir.exists() {
        build.include(&common_dir);
    }

    if let Err(e) = build.try_compile(&format!("tree_sitter_{name}_parser")) {
        println!("cargo:warning=Failed to compile parser for '{}': {}", name, e);
        return false;
    }

    // ~keep Prefix scanner symbols to avoid collisions when multiple grammars are statically linked.
    let scanner_c = src_dir.join("scanner.c");
    if scanner_c.exists() {
        let mut scanner_build = cc::Build::new();
        scanner_build
            .include(&src_dir)
            .file(&scanner_c)
            .define("TREE_SITTER_HIDE_SYMBOLS", None)
            .flag_if_supported("-fvisibility=hidden")
            .flag_if_supported("-fno-strict-aliasing")
            .warnings(false);
        apply_scanner_symbol_prefix(&mut scanner_build, name, &scanner_c);
        scanner_build.std("c11");
        apply_wasm32_sysroot(&mut scanner_build);
        apply_wasm32_optimizations(&mut scanner_build);
        apply_grammar_extra_flags(&mut scanner_build);
        if common_dir.exists() {
            scanner_build.include(&common_dir);
        }
        if !ctype_shim_excluded(name) {
            let (shim_header, utf8proc_include, _) = ctype_shim_assets();
            apply_ctype_shim(&mut scanner_build, &shim_header, &utf8proc_include);
        }
        if let Err(e) = scanner_build.try_compile(&format!("tree_sitter_{name}_scanner")) {
            println!("cargo:warning=Failed to compile C scanner for '{}': {}", name, e);
            return false;
        }
    }

    // ~keep C++ scanners also need separate compilation with the same symbol prefixing.
    let scanner_cc = src_dir.join("scanner.cc");
    if scanner_cc.exists() {
        let mut cpp_build = cc::Build::new();
        cpp_build
            .include(&src_dir)
            .file(&scanner_cc)
            .define("TREE_SITTER_HIDE_SYMBOLS", None)
            .flag_if_supported("-fvisibility=hidden")
            .flag_if_supported("-fno-strict-aliasing")
            .warnings(false)
            .cpp(true);
        apply_scanner_symbol_prefix(&mut cpp_build, name, &scanner_cc);
        apply_wasm32_sysroot(&mut cpp_build);
        apply_wasm32_optimizations(&mut cpp_build);
        apply_grammar_extra_flags(&mut cpp_build);
        if common_dir.exists() {
            cpp_build.include(&common_dir);
        }
        if !ctype_shim_excluded(name) {
            let (shim_header, utf8proc_include, _) = ctype_shim_assets();
            apply_ctype_shim(&mut cpp_build, &shim_header, &utf8proc_include);
        }
        if let Err(e) = cpp_build.try_compile(&format!("tree_sitter_{name}_scanner_cpp")) {
            println!("cargo:warning=Failed to compile C++ scanner for '{}': {}", name, e);
            return false;
        }
    }

    true
}

fn generate_registry(
    static_langs: &[String],
    dynamic_langs: &[String],
    definitions: &BTreeMap<String, LanguageDefinition>,
    libs_dir: &Path,
    out_dir: &Path,
) {
    let path = out_dir.join("registry_generated.rs");
    let mut f = fs::File::create(&path).expect("Failed to create registry_generated.rs");

    writeln!(f, "// Auto-generated by build.rs — do not edit").unwrap();
    writeln!(
        f,
        "// This file is include!()'d into registry.rs which imports tree_sitter::Language"
    )
    .unwrap();
    writeln!(f).unwrap();

    let c_sym = |name: &str| -> String {
        definitions
            .get(name)
            .and_then(|d| d.c_symbol.as_deref())
            .unwrap_or(name)
            .to_string()
    };

    if !static_langs.is_empty() {
        for name in static_langs {
            let sym = c_sym(name);
            writeln!(f, "unsafe extern \"C\" {{").unwrap();
            writeln!(f, "    fn tree_sitter_{sym}() -> *const tree_sitter::ffi::TSLanguage;").unwrap();
            writeln!(f, "}}").unwrap();
        }
        writeln!(f).unwrap();
    }

    writeln!(
        f,
        "#[allow(unused, clippy::type_complexity)]\npub(crate) static STATIC_LANGUAGES: &[(&str, fn() -> Language)] = &["
    )
    .unwrap();
    for name in static_langs {
        let sym = c_sym(name);
        writeln!(
            f,
            "    (\"{name}\", || unsafe {{ Language::from_raw(tree_sitter_{sym}()) }}),",
        )
        .unwrap();
    }
    writeln!(f, "];").unwrap();
    writeln!(f).unwrap();

    writeln!(
        f,
        "#[allow(unused)]\npub(crate) static DYNAMIC_LANGUAGE_NAMES: &[&str] = &["
    )
    .unwrap();
    for name in dynamic_langs {
        writeln!(f, "    \"{name}\",").unwrap();
    }
    writeln!(f, "];").unwrap();
    writeln!(f).unwrap();

    // ~keep `libs_dir` is this build host's `OUT_DIR/libs`, so baking it in makes a
    // ~keep `dynamic-loading` build non-relocatable: move or copy the artifact and the
    // ~keep static points at a directory that no longer exists. It cannot be fixed here —
    // ~keep a `static &str` is a compile-time constant and nothing in build.rs knows where
    // ~keep the artifact will end up — so it is emitted as a documented last-resort
    // ~keep fallback and the runtime search order is `registry.rs`'s responsibility.
    writeln!(f, "/// Directory this build wrote dynamically loadable parsers to.").unwrap();
    writeln!(f, "///").unwrap();
    writeln!(
        f,
        "/// An absolute path on the build host, and therefore only valid where the"
    )
    .unwrap();
    writeln!(
        f,
        "/// crate was built. A relocated artifact must point the registry elsewhere:"
    )
    .unwrap();
    writeln!(f, "/// `LanguageRegistry::with_libs_dir` replaces this directory, and").unwrap();
    writeln!(
        f,
        "/// `LanguageRegistry::add_extra_libs_dir` adds another one to search — the"
    )
    .unwrap();
    writeln!(
        f,
        "/// download cache registers itself that way. Empty unless the build actually"
    )
    .unwrap();
    writeln!(
        f,
        "/// produced dynamic libraries (`TSLP_LINK_MODE` + `TSLP_LANGUAGES`)."
    )
    .unwrap();
    writeln!(f, "#[allow(unused)]").unwrap();
    writeln!(
        f,
        "pub(crate) static LIBS_DIR: &str = {:?};",
        libs_dir.display().to_string()
    )
    .unwrap();
    writeln!(f).unwrap();

    // ~keep Download/runtime paths need C symbols for all definitions, not just compiled languages.
    writeln!(
        f,
        "#[allow(unused)]\npub(crate) static C_SYMBOL_OVERRIDES: &[(&str, &str)] = &["
    )
    .unwrap();
    for (name, def) in definitions.iter() {
        if let Some(c_sym) = &def.c_symbol {
            writeln!(f, "    (\"{name}\", \"{c_sym}\"),").unwrap();
        }
    }
    writeln!(f, "];").unwrap();
    writeln!(f).unwrap();

    // ~keep This is the canonical known-language list, independent of static or dynamic compilation.
    // ~keep Download callers use it to probe a language before triggering on-demand download.
    writeln!(f, "#[allow(unused)]\npub(crate) static KNOWN_LANGUAGES: &[&str] = &[").unwrap();
    for (name, _) in definitions.iter() {
        writeln!(f, "    \"{name}\",").unwrap();
    }
    writeln!(f, "];").unwrap();
}

/// Emit rerun-if-changed for specific source files in a parser directory.
fn emit_rerun_if_changed(parser_dir: &Path) {
    let src_dir = parser_dir.join("src");
    for file in &["parser.c", "scanner.c", "scanner.cc"] {
        let path = src_dir.join(file);
        if path.exists() {
            println!("cargo:rerun-if-changed={}", path.display());
        }
    }
}

/// Generate extension-to-language lookup code from language_definitions.json.
///
/// Reads extensions from the JSON definitions and generates a match expression
/// that maps file extensions to language names. Validates strictly:
/// - No duplicate extensions across languages
/// - Extensions must be ASCII, lowercase, non-empty
/// - Extensions must not contain dots, whitespace, or special characters
fn generate_extensions_lookup(definitions: &BTreeMap<String, LanguageDefinition>, out_dir: &Path) {
    let mut ext_to_lang: BTreeMap<String, String> = BTreeMap::new();

    for (lang_name, def) in definitions {
        for ext in &def.extensions {
            if ext.is_empty() {
                panic!("Empty extension for language '{lang_name}' in language_definitions.json");
            }
            if !ext.is_ascii() {
                panic!("Non-ASCII extension '{ext}' for language '{lang_name}' in language_definitions.json");
            }
            if ext != &ext.to_ascii_lowercase() {
                panic!("Extension '{ext}' for language '{lang_name}' must be lowercase in language_definitions.json");
            }
            if ext.contains('.') {
                panic!(
                    "Extension '{ext}' for language '{lang_name}' must not contain dots (use 'py' not '.py') in language_definitions.json"
                );
            }
            if !ext.chars().all(|c| c.is_ascii_alphanumeric() || c == '_') {
                panic!(
                    "Extension '{ext}' for language '{lang_name}' contains invalid characters in language_definitions.json. Only alphanumeric and underscore allowed."
                );
            }
            if let Some(existing) = ext_to_lang.get(ext) {
                panic!(
                    "Duplicate extension '{ext}' in language_definitions.json: mapped to both '{existing}' and '{lang_name}'"
                );
            }
            ext_to_lang.insert(ext.clone(), lang_name.clone());
        }
    }

    let mut lang_to_exts: BTreeMap<&str, Vec<&str>> = BTreeMap::new();
    for (ext, lang) in &ext_to_lang {
        lang_to_exts.entry(lang.as_str()).or_default().push(ext.as_str());
    }

    let path = out_dir.join("extensions_generated.rs");
    let mut f = fs::File::create(&path).expect("Failed to create extensions_generated.rs");

    writeln!(
        f,
        "// Auto-generated by build.rs from sources/language_definitions.json — do not edit"
    )
    .unwrap();
    writeln!(f, "{{").unwrap();
    writeln!(f, "    let mut buf = [0u8; 32];").unwrap();
    writeln!(f, "    let ext_lower = if ext.len() <= buf.len() && ext.is_ascii() {{").unwrap();
    writeln!(f, "        for (i, b) in ext.bytes().enumerate() {{").unwrap();
    writeln!(f, "            buf[i] = b.to_ascii_lowercase();").unwrap();
    writeln!(f, "        }}").unwrap();
    writeln!(f, "        match std::str::from_utf8(&buf[..ext.len()]) {{").unwrap();
    writeln!(f, "            Ok(s) => s,").unwrap();
    writeln!(f, "            Err(_) => return None,").unwrap();
    writeln!(f, "        }}").unwrap();
    writeln!(f, "    }} else {{").unwrap();
    writeln!(f, "        return None;").unwrap();
    writeln!(f, "    }};").unwrap();
    writeln!(f).unwrap();
    writeln!(f, "    match ext_lower {{").unwrap();

    for (lang, exts) in &lang_to_exts {
        let patterns: Vec<String> = exts.iter().map(|e| format!("\"{e}\"")).collect();
        writeln!(f, "        {} => Some(\"{lang}\"),", patterns.join(" | ")).unwrap();
    }

    writeln!(f, "        _ => None,").unwrap();
    writeln!(f, "    }}").unwrap();
    writeln!(f, "}}").unwrap();

    let mut ambiguities: BTreeMap<String, (String, Vec<String>)> = BTreeMap::new();
    for (lang_name, def) in definitions {
        for (ext, alternatives) in &def.ambiguous {
            if !def.extensions.contains(ext) {
                panic!("Ambiguous extension '{ext}' for language '{lang_name}' is not in its extensions list");
            }
            for alt in alternatives {
                if !definitions.contains_key(alt) {
                    panic!(
                        "Ambiguous alternative '{alt}' for extension '{ext}' (language '{lang_name}') does not exist in language_definitions.json"
                    );
                }
            }
            ambiguities.insert(ext.clone(), (lang_name.clone(), alternatives.clone()));
        }
    }

    let ambig_path = out_dir.join("ambiguities_generated.rs");
    let mut af = fs::File::create(&ambig_path).expect("Failed to create ambiguities_generated.rs");

    writeln!(
        af,
        "// Auto-generated by build.rs from sources/language_definitions.json — do not edit"
    )
    .unwrap();
    writeln!(af, "match ext_lower {{").unwrap();
    for (ext, (assigned, alternatives)) in &ambiguities {
        let alt_strs: Vec<String> = alternatives.iter().map(|a| format!("\"{a}\"")).collect();
        writeln!(
            af,
            "    \"{ext}\" => Some((\"{assigned}\", &[{}])),",
            alt_strs.join(", ")
        )
        .unwrap();
    }
    writeln!(af, "    _ => None,").unwrap();
    writeln!(af, "}}").unwrap();
}

/// Generate bundled highlight/injection/locals/tags/indents/folds query functions from
/// parsers/{lang}/queries/*.scm.
///
/// Scans the parsers directory for query files and generates a Rust source file with six
/// match functions: `get_highlights_query_impl`, `get_injections_query_impl`,
/// `get_locals_query_impl`, `get_tags_query_impl`, `get_indents_query_impl`, and
/// `get_folds_query_impl`. Only languages that actually have the relevant .scm file on disk
/// at build time are included.
///
/// Query overlay files in `query-overlays/{lang}/{file}` (relative to the project root) take
/// precedence over the vendored `parsers/{lang}/queries/{file}` files. This allows adding or
/// replacing queries for upstream grammars without modifying the gitignored `parsers/` tree.
fn generate_queries_registry(definitions: &BTreeMap<String, LanguageDefinition>, parsers_dir: &Path, out_dir: &Path) {
    let path = out_dir.join("queries_generated.rs");
    let mut f = fs::File::create(&path).expect("Failed to create queries_generated.rs");

    writeln!(f, "// Auto-generated by build.rs — do not edit").unwrap();
    writeln!(f).unwrap();

    // ~keep Resolve project root for `query-overlays`, which is a sibling of `crates/`.
    let overlays_dir = {
        let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
        // ~keep Walk up to the directory containing `query-overlays` or `sources/language_definitions.json`.
        let mut candidate = manifest_dir.as_path();
        loop {
            if candidate.join("query-overlays").exists() || candidate.join("sources/language_definitions.json").exists()
            {
                break;
            }
            match candidate.parent() {
                Some(p) => candidate = p,
                None => {
                    candidate = &manifest_dir;
                    break;
                }
            }
        }
        candidate.join("query-overlays")
    };

    // ~keep Query overlay files intentionally win over vendored query files.
    let effective_query_path = |lang: &str, file: &str| -> Option<PathBuf> {
        let overlay = overlays_dir.join(lang).join(file);
        if overlay.exists() {
            return Some(overlay);
        }
        let vendored = parsers_dir.join(lang).join("queries").join(file);
        if vendored.exists() {
            return Some(vendored);
        }
        None
    };

    let mut highlights: Vec<String> = Vec::new();
    let mut injections: Vec<String> = Vec::new();
    let mut locals: Vec<String> = Vec::new();
    let mut tags: Vec<String> = Vec::new();
    let mut indents: Vec<String> = Vec::new();
    let mut folds: Vec<String> = Vec::new();

    for lang in definitions.keys() {
        if let Some(p) = effective_query_path(lang, "highlights.scm") {
            highlights.push(lang.clone());
            println!("cargo:rerun-if-changed={}", p.display());
        }
        if let Some(p) = effective_query_path(lang, "injections.scm") {
            injections.push(lang.clone());
            println!("cargo:rerun-if-changed={}", p.display());
        }
        if let Some(p) = effective_query_path(lang, "locals.scm") {
            locals.push(lang.clone());
            println!("cargo:rerun-if-changed={}", p.display());
        }
        if let Some(p) = effective_query_path(lang, "tags.scm") {
            tags.push(lang.clone());
            println!("cargo:rerun-if-changed={}", p.display());
        }
        if let Some(p) = effective_query_path(lang, "indents.scm") {
            indents.push(lang.clone());
            println!("cargo:rerun-if-changed={}", p.display());
        }
        if let Some(p) = effective_query_path(lang, "folds.scm") {
            folds.push(lang.clone());
            println!("cargo:rerun-if-changed={}", p.display());
        }
    }

    // ~keep Rerun on overlay directory changes so newly added overlay files rebuild immediately.
    if overlays_dir.exists() {
        println!("cargo:rerun-if-changed={}", overlays_dir.display());
    }

    // ~keep Embed query contents instead of include_str! because packaged crates break parser-relative paths.
    let gen_query_fn = |f: &mut fs::File, name: &str, langs: &[String], query_file: &str| {
        writeln!(f, "pub(crate) fn {name}(lang: &str) -> Option<&'static str> {{").unwrap();
        if langs.is_empty() {
            writeln!(f, "    let _ = lang;").unwrap();
            writeln!(f, "    None").unwrap();
        } else {
            writeln!(f, "    match lang {{").unwrap();
            for lang in langs {
                let query_path = effective_query_path(lang, query_file).unwrap_or_else(|| {
                    panic!("query file {query_file} for {lang} disappeared between scan and codegen")
                });
                let contents = fs::read_to_string(&query_path)
                    .unwrap_or_else(|e| panic!("Failed to read {}: {e}", query_path.display()));
                let escaped = contents
                    .replace('\\', "\\\\")
                    .replace('"', "\\\"")
                    .replace('\r', "")
                    .replace('\n', "\\n");
                writeln!(f, "        \"{lang}\" => Some(\"{escaped}\"),",).unwrap();
            }
            writeln!(f, "        _ => None,").unwrap();
            writeln!(f, "    }}").unwrap();
        }
        writeln!(f, "}}").unwrap();
        writeln!(f).unwrap();
    };

    gen_query_fn(&mut f, "get_highlights_query_impl", &highlights, "highlights.scm");
    gen_query_fn(&mut f, "get_injections_query_impl", &injections, "injections.scm");
    gen_query_fn(&mut f, "get_locals_query_impl", &locals, "locals.scm");
    gen_query_fn(&mut f, "get_tags_query_impl", &tags, "tags.scm");
    gen_query_fn(&mut f, "get_indents_query_impl", &indents, "indents.scm");
    gen_query_fn(&mut f, "get_folds_query_impl", &folds, "folds.scm");
}

/// Probe whether the parsers tree at `root` looks populated for the requested
/// selection. Uses the same heuristic in both the workspace and OUT_DIR cache
/// locations.
fn parsers_root_populated(root: &Path, selected: &[String]) -> bool {
    match selected.first() {
        Some(first) => root.join(first).join("src/parser.c").exists(),
        None => root.join("python").join("src/parser.c").exists(),
    }
}

/// Try to populate the workspace `parsers/` tree by invoking
/// `scripts/clone_vendors.py` from `project_root`. Returns true if the script
/// ran successfully AND the tree is populated afterwards. Returns false if no
/// script is present, no runner is available, or the script failed.
///
/// This is the local-development fallback: a fresh workspace clone has
/// `parsers/` gitignored, and a rc whose release hasn't been published yet has
/// no GH tarball — so neither the workspace nor the remote path works. Cloning
/// upstream grammars via the script bridges the gap.
fn try_clone_vendors_locally(project_root: &Path, parsers_dir: &Path, selected: &[String]) -> bool {
    let clone_script = project_root.join(CLONE_VENDORS_SCRIPT);
    if !clone_script.exists() {
        return false;
    }

    println!("cargo:rerun-if-env-changed={CLONE_VENDORS_INTERPRETER_ENV}");
    println!("cargo:warning=parsers/ tree is empty; running {CLONE_VENDORS_SCRIPT} to populate from upstream grammars");

    let runners = clone_vendors_runners();
    let mut spawned_any = false;

    for cmd_args in &runners {
        let mut cmd = std::process::Command::new(&cmd_args[0]);
        cmd.args(&cmd_args[1..]);
        cmd.current_dir(project_root);
        match cmd.status() {
            Ok(status) if status.success() => {
                if parsers_root_populated(parsers_dir, selected) {
                    return true;
                }
                println!(
                    "cargo:warning={} succeeded but parsers/ is still not populated",
                    cmd_args.join(" ")
                );
                return false;
            }
            Ok(status) => {
                spawned_any = true;
                println!("cargo:warning={} exited with {:?}", cmd_args.join(" "), status.code());
            }
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
                println!("cargo:warning=interpreter '{}' not found on PATH", cmd_args[0]);
            }
            Err(e) => {
                spawned_any = true;
                println!("cargo:warning=could not run {}: {e}", cmd_args.join(" "));
            }
        }
    }

    if !spawned_any {
        report_no_interpreter(&runners);
    }
    false
}

/// Build the ordered list of argv vectors used to run [`CLONE_VENDORS_SCRIPT`].
///
/// A non-empty [`CLONE_VENDORS_INTERPRETER_ENV`] replaces the probe list entirely
/// so hermetic builds (nix, containers, vendored toolchains) do not depend on
/// `uv` or `python3` happening to be on `PATH`.
fn clone_vendors_runners() -> Vec<Vec<String>> {
    let script = CLONE_VENDORS_SCRIPT.to_string();

    if let Ok(interpreter) = env::var(CLONE_VENDORS_INTERPRETER_ENV)
        && !interpreter.trim().is_empty()
    {
        return vec![vec![interpreter.trim().to_string(), script]];
    }

    DEFAULT_CLONE_VENDORS_RUNNERS
        .iter()
        .map(|base| base.iter().map(|a| (*a).to_string()).chain([script.clone()]).collect())
        .collect()
}

/// Emit an actionable diagnostic when no candidate interpreter could be spawned.
fn report_no_interpreter(runners: &[Vec<String>]) {
    let mut tried: Vec<&str> = Vec::new();
    for runner in runners {
        let name = runner[0].as_str();
        if !tried.contains(&name) {
            tried.push(name);
        }
    }
    println!(
        "cargo:warning=No usable Python interpreter was found to run {CLONE_VENDORS_SCRIPT} (tried: {}). \
         Install `uv` or `python3`, or set {CLONE_VENDORS_INTERPRETER_ENV} to an interpreter path. \
         Falling back to the released parser-source bundle; set TSLP_OFFLINE=1 to skip parser sources entirely.",
        tried.join(", ")
    );
}

/// Directory holding committed grammar source patches, relative to the project
/// root in a workspace build and to the crate root in a published crate.
const GRAMMAR_PATCHES_DIR: &str = "patches";

/// Extension of a unified diff under [`GRAMMAR_PATCHES_DIR`].
const GRAMMAR_PATCH_EXTENSION: &str = "patch";

/// Escape hatch that disables the grammar patch layer. Local debugging only.
const SKIP_GRAMMAR_PATCHES_ENV: &str = "TSLP_SKIP_GRAMMAR_PATCHES";

/// One hunk of a unified diff: the 1-based pre-image start line plus the body,
/// each line tagged with its leading marker (`b' '`, `b'+'` or `b'-'`).
struct PatchHunk {
    old_start: usize,
    body: Vec<(u8, String)>,
}

/// Apply every committed patch under `patches/` to the resolved grammar sources.
///
/// ~keep This is the single choke point that covers all of [`ensure_parser_sources`]'s
/// resolution paths (workspace tree, OUT_DIR cache, `clone_vendors.py`, release
/// tarball). Patching any one of them individually — the vendor clone in
/// particular — produces a fix that never reaches a published package.
fn patch_grammar_sources(
    project_root: &Path,
    parsers_dir: &Path,
    definitions: &BTreeMap<String, LanguageDefinition>,
    selected: &[String],
) {
    println!("cargo:rerun-if-env-changed={SKIP_GRAMMAR_PATCHES_ENV}");
    // ~keep An empty selection compiles no grammar sources at all (runtime download mode),
    // so there is nothing to patch and no reason to require the patch tree.
    if selected.is_empty() {
        return;
    }
    if env::var(SKIP_GRAMMAR_PATCHES_ENV).is_ok_and(|v| !v.is_empty() && v != "0") {
        println!(
            "cargo:warning={SKIP_GRAMMAR_PATCHES_ENV} is set — grammar memory-safety patches are NOT applied. \
             The resulting binaries contain known heap buffer overflows in external scanners. \
             Never set this for a build whose artifacts are published or distributed."
        );
        return;
    }

    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    // ~keep Third candidate is the extracted release bundle's root (sibling of the unpacked
    // `parsers/`), so the tarball path stays self-contained once the bundle ships `patches/`.
    let mut candidates = vec![project_root.join(GRAMMAR_PATCHES_DIR)];
    if let Some(bundle_root) = parsers_dir.parent() {
        candidates.push(bundle_root.join(GRAMMAR_PATCHES_DIR));
    }
    candidates.push(manifest_dir.join(GRAMMAR_PATCHES_DIR));

    let Some(patches_root) = candidates.iter().find(|path| path.is_dir()) else {
        let searched: Vec<String> = candidates.iter().map(|path| path.display().to_string()).collect();
        panic!(
            "Grammar patch directory not found (searched {}), but {} grammar(s) are being compiled from source. \
             These patches fix heap buffer overflows in external scanners, so a build that cannot find them is refused. \
             Set {SKIP_GRAMMAR_PATCHES_ENV}=1 for local debugging only.",
            searched.join(", "),
            selected.len(),
        )
    };
    apply_grammar_patches(parsers_dir, patches_root, definitions, selected);
}

/// Walk `patches/<language>/` and apply each language's patches in sorted order.
fn apply_grammar_patches(
    parsers_dir: &Path,
    patches_root: &Path,
    definitions: &BTreeMap<String, LanguageDefinition>,
    selected: &[String],
) {
    println!("cargo:rerun-if-changed={}", patches_root.display());
    let entries =
        fs::read_dir(patches_root).unwrap_or_else(|e| panic!("Failed to read {}: {e}", patches_root.display()));
    let mut language_dirs: Vec<PathBuf> = entries
        .filter_map(|entry| entry.ok().map(|entry| entry.path()))
        .filter(|path| path.is_dir())
        .collect();
    language_dirs.sort();

    for dir in &language_dirs {
        let language = dir
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or_else(|| panic!("Patch directory name is not valid UTF-8: {}", dir.display()))
            .to_string();
        assert!(
            definitions.contains_key(&language),
            "patches/{language}/ does not name a language in language_definitions.json. \
             A misnamed patch directory is a patch that never applies and never reports, \
             so this is a hard error: rename the directory to the language key or delete it."
        );
        println!("cargo:rerun-if-changed={}", dir.display());
        if !selected.contains(&language) {
            continue;
        }
        let grammar_root = parsers_dir.join(&language);
        // ~keep A selected grammar with no sources is already reported and failed by main();
        // patching would only mask that error with a less useful one.
        if !grammar_root.join("src/parser.c").exists() {
            continue;
        }
        apply_language_patches(&grammar_root, dir, &language);
    }
}

/// Apply every `*.patch` in `patch_dir` to `grammar_root`, panicking on failure.
fn apply_language_patches(grammar_root: &Path, patch_dir: &Path, language: &str) {
    let entries = fs::read_dir(patch_dir).unwrap_or_else(|e| panic!("Failed to read {}: {e}", patch_dir.display()));
    let mut patch_files: Vec<PathBuf> = entries
        .filter_map(|entry| entry.ok().map(|entry| entry.path()))
        .filter(|path| path.extension().and_then(|ext| ext.to_str()) == Some(GRAMMAR_PATCH_EXTENSION))
        .collect();
    patch_files.sort();

    for patch_path in &patch_files {
        println!("cargo:rerun-if-changed={}", patch_path.display());
        if let Err(e) = apply_grammar_patch(grammar_root, patch_path) {
            panic!(
                "Failed to apply {} to grammar '{language}' at {}: {e}. \
                 This patch fixes a heap buffer overflow, so it is never skipped. \
                 If the grammar's pinned rev moved, regenerate the patch (see patches/README.md); \
                 if the tree was patched by an older version of this patch, delete {} and re-run the vendor clone.",
                patch_path.display(),
                grammar_root.display(),
                grammar_root.display(),
            );
        }
    }
}

/// Apply a single-file unified diff to `grammar_root`, treating a clean reverse
/// apply as "already applied".
///
/// ~keep The reverse-apply probe is the idempotency signal every committed patch
/// was verified against; it is what makes re-running a build a no-op.
fn apply_grammar_patch(grammar_root: &Path, patch_path: &Path) -> Result<(), String> {
    let patch_text = fs::read_to_string(patch_path).map_err(|e| format!("read patch file: {e}"))?;
    let target = unified_diff_target(&patch_text)?;
    let hunks = parse_unified_diff(&patch_text)?;
    let file_path = grammar_root.join(&target);
    let raw = fs::read_to_string(&file_path).map_err(|e| format!("read patch target {}: {e}", file_path.display()))?;

    // ~keep A Windows checkout with core.autocrlf rewrites the vendored sources; the
    // committed diffs are LF, so normalize for matching and restore on write.
    let uses_crlf = raw.contains("\r\n");
    let normalized = if uses_crlf { raw.replace("\r\n", "\n") } else { raw };
    let ends_with_newline = normalized.ends_with('\n');
    let lines: Vec<String> = normalized.lines().map(|line| line.to_string()).collect();

    let patched = match apply_hunks(&lines, &hunks, false) {
        Ok(patched) => patched,
        Err(forward_error) => {
            if apply_hunks(&lines, &hunks, true).is_ok() {
                return Ok(());
            }
            return Err(format!(
                "{forward_error} in {}; it does not reverse-apply either, so the file is neither pristine nor already patched",
                file_path.display()
            ));
        }
    };

    let mut output = patched.join("\n");
    if ends_with_newline {
        output.push('\n');
    }
    if uses_crlf {
        output = output.replace('\n', "\r\n");
    }
    fs::write(&file_path, output).map_err(|e| format!("write {}: {e}", file_path.display()))
}

/// Extract the grammar-root-relative target path from a unified diff's `+++` header.
fn unified_diff_target(patch_text: &str) -> Result<String, String> {
    let targets: Vec<&str> = patch_text
        .lines()
        .filter_map(|line| line.strip_prefix("+++ "))
        .collect();
    if targets.len() != 1 {
        return Err(format!(
            "expected exactly one '+++' target header (single-file patches only), found {}",
            targets.len()
        ));
    }
    let header = targets[0];
    let path = header.split('\t').next().unwrap_or(header).trim();
    // ~keep -p1: patch paths are grammar-root-relative behind an `a/`/`b/` prefix.
    let stripped = path.split_once('/').map(|(_, tail)| tail).unwrap_or(path);
    if stripped.is_empty() || stripped.starts_with('/') || stripped.split('/').any(|part| part == "..") {
        return Err(format!("refusing patch target outside the grammar root: {path:?}"));
    }
    Ok(stripped.to_string())
}

/// Parse the hunks of a unified diff, using the `@@` line counts to delimit each
/// hunk body so source lines can never be mistaken for diff headers.
fn parse_unified_diff(patch_text: &str) -> Result<Vec<PatchHunk>, String> {
    let mut hunks: Vec<PatchHunk> = Vec::new();
    let mut lines = patch_text.lines();
    while let Some(line) = lines.next() {
        let Some(rest) = line.strip_prefix("@@ -") else {
            continue;
        };
        let (old_start, old_count, new_count) = parse_hunk_header(rest)?;
        let mut hunk = PatchHunk {
            old_start,
            body: Vec::new(),
        };
        let mut seen_old = 0usize;
        let mut seen_new = 0usize;
        while seen_old < old_count || seen_new < new_count {
            let Some(body_line) = lines.next() else {
                return Err(format!("hunk at line {old_start} is truncated"));
            };
            if body_line.starts_with('\\') {
                continue;
            }
            let bytes = body_line.as_bytes();
            let marker = if bytes.is_empty() { b' ' } else { bytes[0] };
            let content = if bytes.is_empty() { "" } else { &body_line[1..] };
            match marker {
                b' ' => {
                    seen_old += 1;
                    seen_new += 1;
                }
                b'-' => seen_old += 1,
                b'+' => seen_new += 1,
                _ => return Err(format!("unexpected line in hunk body: {body_line:?}")),
            }
            hunk.body.push((marker, content.to_string()));
        }
        hunks.push(hunk);
    }
    if hunks.is_empty() {
        return Err("patch contains no '@@' hunk headers".to_string());
    }
    Ok(hunks)
}

/// Parse `<old_start>,<old_count> +<new_start>,<new_count> @@ …` (the text after `@@ -`).
fn parse_hunk_header(rest: &str) -> Result<(usize, usize, usize), String> {
    let mut parts = rest.split(' ');
    let old = parts.next().unwrap_or_default();
    let new = parts.next().unwrap_or_default().strip_prefix('+').unwrap_or_default();
    let parse_range = |spec: &str| -> Option<(usize, usize)> {
        match spec.split_once(',') {
            Some((start, count)) => Some((start.parse().ok()?, count.parse().ok()?)),
            None => Some((spec.parse().ok()?, 1)),
        }
    };
    let (old_start, old_count) = parse_range(old).ok_or_else(|| format!("malformed hunk pre-image range '-{old}'"))?;
    let (_, new_count) = parse_range(new).ok_or_else(|| format!("malformed hunk post-image range '+{new}'"))?;
    Ok((old_start, old_count, new_count))
}

/// Apply `hunks` to `lines`, forward or reversed. Context and removed lines must
/// match exactly; only the hunk's position may shift, as `git apply` allows.
fn apply_hunks(lines: &[String], hunks: &[PatchHunk], reverse: bool) -> Result<Vec<String>, String> {
    let (removed, added) = if reverse { (b'+', b'-') } else { (b'-', b'+') };
    let mut result: Vec<String> = lines.to_vec();
    let mut offset: isize = 0;

    for hunk in hunks {
        let old: Vec<&str> = hunk
            .body
            .iter()
            .filter(|(marker, _)| *marker == b' ' || *marker == removed)
            .map(|(_, content)| content.as_str())
            .collect();
        let new: Vec<String> = hunk
            .body
            .iter()
            .filter(|(marker, _)| *marker == b' ' || *marker == added)
            .map(|(_, content)| content.clone())
            .collect();

        let guess = hunk.old_start.saturating_sub(1) as isize + offset;
        let position = find_hunk_position(&result, &old, guess)
            .ok_or_else(|| format!("hunk at line {} does not match", hunk.old_start))?;
        let end = position + old.len();

        let mut spliced: Vec<String> = Vec::with_capacity(result.len() + new.len());
        spliced.extend_from_slice(&result[..position]);
        spliced.extend(new.iter().cloned());
        spliced.extend_from_slice(&result[end..]);
        result = spliced;

        offset += (position as isize - guess) + (new.len() as isize - old.len() as isize);
    }
    Ok(result)
}

/// Locate `old` in `lines`, preferring `guess` and searching outward from it.
fn find_hunk_position(lines: &[String], old: &[&str], guess: isize) -> Option<usize> {
    let matches_at = |start: usize| -> bool {
        if start + old.len() > lines.len() {
            return false;
        }
        old.iter()
            .enumerate()
            .all(|(index, want)| lines[start + index] == *want)
    };

    let clamped = guess.max(0) as usize;
    if matches_at(clamped) {
        return Some(clamped);
    }
    for distance in 1..=lines.len() {
        if clamped >= distance && matches_at(clamped - distance) {
            return Some(clamped - distance);
        }
        if matches_at(clamped + distance) {
            return Some(clamped + distance);
        }
    }
    None
}

/// In-place fixups for vendored grammar sources that don't compile under MSVC.
///
/// MSVC rejects two C99/GCC-isms tree-sitter scanners commonly use:
///   * Variable-length arrays (crystal scanner.c:1307).
///   * `__attribute__((unused))` parameter annotations (sml scanner.c).
///
/// The substitutions below are equivalent on every compiler — the array bound
/// `MAX_HEREDOC_WORD_SIZE + 4` is the same compile-time constant the runtime
/// already caps `max_word_size` at (lines 1303-1305), and dropping the unused-
/// attribute keeps the parameter signatures intact.
///
/// A sentinel comment is written on the first run so subsequent build.rs
/// invocations short-circuit without re-reading or re-writing the source.
fn apply_msvc_compat_patches(parsers_dir: &Path) {
    #[allow(clippy::type_complexity)]
    let patches: &[(&str, &str, &[(&str, &str)])] = &[
        (
            "crystal",
            "src/scanner.c",
            &[(
                "uint8_t word[max_word_size + 4];",
                "uint8_t word[MAX_HEREDOC_WORD_SIZE + 4]; /* TSLP_MSVC_PATCH: VLA → fixed bound */",
            )],
        ),
        (
            "sml",
            "src/scanner.c",
            &[(
                "__attribute__ ((unused))",
                "/* TSLP_MSVC_PATCH: dropped __attribute__((unused)) for MSVC */",
            )],
        ),
    ];

    for (lang, rel_path, subs) in patches {
        // ~keep A grammar that was never vendored (language subset build) is not a
        // missing patch; main() reports and fails any grammar it actually needs.
        if !parsers_dir.join(lang).is_dir() {
            continue;
        }
        apply_msvc_compat_patch(&parsers_dir.join(lang).join(rel_path), lang, subs);
    }
}

/// Apply one MSVC substitution set to `path`, reporting every outcome that is
/// not "patched" or "already patched".
fn apply_msvc_compat_patch(path: &Path, lang: &str, subs: &[(&str, &str)]) {
    let original = match fs::read_to_string(path) {
        Ok(text) => text,
        Err(e) => {
            println!(
                "cargo:warning=MSVC compat patch for '{lang}': cannot read {} ({e}); MSVC builds of this grammar will fail to compile",
                path.display()
            );
            return;
        }
    };
    if original.contains("TSLP_MSVC_PATCH") {
        return;
    }
    let mut patched = original.clone();
    for (from, to) in subs {
        patched = patched.replace(from, to);
    }
    // ~keep An absent target is ambiguous — upstream may have fixed the incompatibility, or the
    // pin moved and the shim is now stale — and it only affects MSVC, so it is reported rather
    // than hard-failing every non-MSVC build; the defect being fixed here is the silence.
    if patched == original {
        println!(
            "cargo:warning=MSVC compat patch for '{lang}': no target substring found in {}. Either upstream fixed the incompatibility (drop the entry from apply_msvc_compat_patches) or the pinned rev moved and MSVC builds of this grammar will fail.",
            path.display()
        );
        return;
    }
    if let Err(e) = fs::write(path, patched) {
        println!(
            "cargo:warning=Failed to apply MSVC compat patch to {} ({lang}): {e}",
            path.display()
        );
    }
}

/// Resolve grammar sources for the build. Order of preference:
/// 1. Workspace `parsers/` tree is already populated → use it as-is.
/// 2. `TSLP_OFFLINE=1` → return the empty workspace dir; downstream code falls
///    into the "missing parser, skipping" warning path.
/// 3. OUT_DIR cache already populated from a prior run → reuse it.
/// 4. Workspace tree exists (`scripts/clone_vendors.py` is present) → invoke
///    it to clone upstream grammars locally. Bridges the gap for fresh
///    workspace clones and unpublished rc builds where the remote release
///    tarball doesn't exist yet.
/// 5. Otherwise (sdist install or step 4 failed) → download the
///    `parser-sources-{version}.tar.zst` release asset and unpack it under
///    OUT_DIR. `TSLP_SOURCE_BUNDLE_URL` overrides the URL (also accepts
///    `file://` for local-bundle smoke testing).
///
/// Returns the directory to use as the parsers root.
fn ensure_parser_sources(parsers_dir: &Path, selected: &[String], out_dir: &Path) -> PathBuf {
    if parsers_root_populated(parsers_dir, selected) {
        return parsers_dir.to_path_buf();
    }
    if env::var("TSLP_OFFLINE").is_ok_and(|v| !v.is_empty() && v != "0") {
        println!("cargo:warning=TSLP_OFFLINE set; refusing to download parser sources");
        return parsers_dir.to_path_buf();
    }

    let cache_dir = out_dir.join("_parsers");
    // ~keep Same two-probe strategy for the OUT_DIR parser-source cache.
    let cache_populated = match selected.first() {
        Some(first) => cache_dir.join(first).join("src/parser.c").exists(),
        None => cache_dir.join("parsers").join("python").join("src/parser.c").exists(),
    };
    if cache_populated {
        let inner = cache_dir.join("parsers");
        return if inner.is_dir() { inner } else { cache_dir };
    }

    // ~keep Dev checkouts clone upstream grammars before falling back to release tarballs.
    // ~keep This covers fresh workspaces and unpublished rc builds.
    let project_root = parsers_dir.parent().unwrap_or(parsers_dir).to_path_buf();
    println!("cargo:rerun-if-env-changed=TSLP_OFFLINE");
    println!("cargo:rerun-if-env-changed=TSLP_SOURCE_BUNDLE_URL");
    if try_clone_vendors_locally(&project_root, parsers_dir, selected) {
        return parsers_dir.to_path_buf();
    }

    let version = env::var("CARGO_PKG_VERSION").unwrap_or_else(|_| "unknown".to_string());
    let default_url = format!(
        "https://github.com/xberg-io/tree-sitter-language-pack/releases/download/v{version}/parser-sources-{version}.tar.zst"
    );
    let url = env::var("TSLP_SOURCE_BUNDLE_URL").unwrap_or(default_url);
    let sha_url = format!("{url}.sha256");

    println!("cargo:warning=Downloading parser sources from {url}");

    let body = fetch_bytes(&url).unwrap_or_else(|e| {
        panic!(
            "Failed to download parser sources from {url}: {e}. Set TSLP_OFFLINE=1 to skip, or ensure network access and that v{version} is published with a parser-sources-{version}.tar.zst asset."
        )
    });
    // ~keep The bytes fetched above become C that is compiled into this binary, so a
    // missing or malformed checksum sidecar is a hard error rather than a skipped
    // check. TSLP_OFFLINE=1 is the existing escape hatch for "do not download at all".
    let sha_text = fetch_text(&sha_url).unwrap_or_else(|e| {
        panic!(
            "Failed to download the SHA-256 sidecar {sha_url}: {e}. The parser-source bundle is compiled into this crate, so it is refused without an integrity check. Set TSLP_OFFLINE=1 to skip downloading parser sources entirely, or point TSLP_SOURCE_BUNDLE_URL at a bundle that has a .sha256 sidecar next to it."
        )
    });
    let expected = sha_text.split_whitespace().next().unwrap_or_default().to_lowercase();
    assert!(
        expected.len() == 64 && expected.bytes().all(|b| b.is_ascii_hexdigit()),
        "Malformed SHA-256 sidecar at {sha_url}: expected a 64-character hex digest, got {expected:?}. Refusing to compile unverified parser sources; set TSLP_OFFLINE=1 to skip the download entirely."
    );
    let digest = Sha256::digest(&body);
    let actual: String = digest.iter().map(|b| format!("{b:02x}")).collect();
    assert_eq!(
        expected, actual,
        "SHA-256 mismatch for {url}: expected {expected}, got {actual}"
    );

    fs::create_dir_all(&cache_dir).expect("create OUT_DIR/_parsers");
    let cursor = std::io::Cursor::new(body);
    let decoder = zstd::stream::read::Decoder::with_buffer(cursor).expect("zstd decoder");
    let mut archive = tar::Archive::new(decoder);
    archive.unpack(&cache_dir).expect("extract parser-sources tarball");

    // ~keep Release tarballs contain top-level `parsers/` and `sources/`, nested under OUT_DIR extraction.
    let inner = cache_dir.join("parsers");
    if inner.is_dir() { inner } else { cache_dir }
}

fn fetch_bytes(url: &str) -> Result<Vec<u8>, String> {
    if let Some(path) = url.strip_prefix("file://") {
        return fs::read(path).map_err(|e| format!("read {path}: {e}"));
    }

    // ~keep Retry transport errors, including intermittent GitHub release CDN 504s during publish verification.
    let max_attempts = 6u32;
    let mut last_err = String::new();
    for attempt in 1..=max_attempts {
        let call_result = ureq::get(url)
            .config()
            .timeout_global(Some(Duration::from_secs(300)))
            .build()
            .call();

        match call_result {
            Ok(response) => {
                let mut reader = response.into_body().into_reader();
                let mut buf = Vec::new();
                match std::io::Read::read_to_end(&mut reader, &mut buf) {
                    Ok(_) => return Ok(buf),
                    Err(e) => last_err = format!("read body for {url}: {e}"),
                }
            }
            Err(e) => last_err = format!("GET {url}: {e}"),
        }

        if attempt < max_attempts {
            let backoff = Duration::from_secs(2u64.saturating_pow(attempt));
            println!(
                "cargo:warning={last_err}; retrying in {}s (attempt {}/{})",
                backoff.as_secs(),
                attempt + 1,
                max_attempts
            );
            std::thread::sleep(backoff);
        }
    }
    Err(last_err)
}

fn fetch_text(url: &str) -> Result<String, String> {
    let bytes = fetch_bytes(url)?;
    String::from_utf8(bytes).map_err(|e| format!("decode utf8: {e}"))
}

fn main() {
    println!("cargo:rerun-if-env-changed=TSLP_LANGUAGES");
    println!("cargo:rerun-if-env-changed=PROJECT_ROOT");
    println!("cargo:rerun-if-env-changed=TSLP_LINK_MODE");
    println!("cargo:rerun-if-env-changed=TSLP_WASM_MAX_PARSER_BYTES");
    println!("cargo:rerun-if-env-changed=TSLP_GRAMMAR_CFLAGS");

    let (shim_header, _, utf8proc_source) = ctype_shim_assets();
    println!("cargo:rerun-if-changed={}", shim_header.display());
    println!("cargo:rerun-if-changed={}", utf8proc_source.display());

    let project_root = find_project_root();

    // ~keep Definitions live in workspace sources during development and crate-local JSON after publish.
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let crate_local = manifest_dir.join("language_definitions.json");
    let workspace_path = project_root.join("sources/language_definitions.json");
    // ~keep Prefer workspace definitions and fall back to crate-local copy for crates.io packages.
    let definitions_path = if workspace_path.exists() {
        // ~keep Keep the crate-local copy in sync so `cargo publish` includes it.
        if crate_local.parent().is_some_and(|p| p.exists()) {
            let _ = fs::copy(&workspace_path, &crate_local);
        }
        workspace_path
    } else {
        crate_local
    };

    let definitions: BTreeMap<String, LanguageDefinition> = if definitions_path.exists() {
        println!("cargo:rerun-if-changed={}", definitions_path.display());
        let definitions_json = fs::read_to_string(&definitions_path)
            .unwrap_or_else(|e| panic!("Failed to read {}: {e}", definitions_path.display()));
        serde_json::from_str(&definitions_json).expect("Failed to parse language_definitions.json")
    } else {
        BTreeMap::new()
    };
    validate_definition_keys(&definitions);
    let workspace_parsers_dir = project_root.join("parsers");

    let selected = selected_languages(&definitions);

    // ~keep Force static mode for wasm32 targets because shared libraries are unsupported.
    let target_arch = env::var("CARGO_CFG_TARGET_ARCH").unwrap_or_default();
    let link_mode = if target_arch == "wasm32" {
        "static".to_string()
    } else {
        env::var("TSLP_LINK_MODE").unwrap_or_else(|_| "dynamic".to_string())
    };

    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let libs_dir = out_dir.join("libs");
    fs::create_dir_all(&libs_dir).expect("Failed to create libs directory");

    // ~keep sdist installs lack local parser sources, so fetch release artifacts instead.
    let parsers_dir = ensure_parser_sources(&workspace_parsers_dir, &selected, &out_dir);
    // ~keep Ordered before the MSVC shim: the committed diffs are exact-context and were
    // generated against pristine upstream, while the shim is an order-insensitive substitution.
    patch_grammar_sources(&project_root, &parsers_dir, &definitions, &selected);
    apply_msvc_compat_patches(&parsers_dir);

    let mut static_compiled = Vec::new();
    let mut dynamic_compiled = Vec::new();
    let mut failed = Vec::new();
    let mut skipped_wasm: Vec<String> = Vec::new();

    // ~keep On wasm32, skip oversized parser.c files before compile to avoid OOM and dangling FFI symbols.
    let wasm_size_limit = if target_arch == "wasm32" {
        wasm_parser_size_limit()
    } else {
        None
    };

    // ~keep On wasm32, skip scanners incompatible with wasi-libc to avoid dangling FFI symbols.
    let wasm_skip = if target_arch == "wasm32" {
        wasm_skip_grammars()
    } else {
        Vec::new()
    };

    for name in &selected {
        let parser_dir = parsers_dir.join(name);
        let parser_c = parser_dir.join("src/parser.c");
        if !parser_c.exists() {
            println!("cargo:warning=Parser sources not found for '{}', skipping", name);
            failed.push(name.clone());
            continue;
        }

        if wasm_skip.iter().any(|g| g == name) {
            println!(
                "cargo:warning=wasm32: skipping grammar '{}' — its external scanner is not wasm32-compatible (wctype/locale or C++ stdlib). Override with TSLP_WASM_SKIP_GRAMMARS.",
                name,
            );
            skipped_wasm.push(name.clone());
            continue;
        }

        if let Some(limit) = wasm_size_limit
            && let Ok(size) = fs::metadata(&parser_c).map(|m| m.len())
            && size > limit
        {
            println!(
                "cargo:warning=wasm32: skipping grammar '{}' — parser.c is {} MB (limit {} MB); too large to compile to wasm32 within runner memory. Override with TSLP_WASM_MAX_PARSER_BYTES (0 disables the gate).",
                name,
                size / (1024 * 1024),
                limit / (1024 * 1024),
            );
            skipped_wasm.push(name.clone());
            continue;
        }

        let out_dir = env::var("OUT_DIR").unwrap_or_default();
        if !parser_dir.starts_with(&out_dir) {
            emit_rerun_if_changed(&parser_dir);
        }

        let ok = match link_mode.as_str() {
            "static" => {
                let ok = compile_parser_static(name, &parser_dir);
                if ok {
                    static_compiled.push(name.clone());
                }
                ok
            }
            "dynamic" => {
                let c_sym = definitions.get(name.as_str()).and_then(|d| d.c_symbol.as_deref());
                let ok = compile_parser_dynamic(name, c_sym, &parser_dir, &libs_dir);
                if ok {
                    dynamic_compiled.push(name.clone());
                }
                ok
            }
            "both" => {
                let ok_s = compile_parser_static(name, &parser_dir);
                if ok_s {
                    static_compiled.push(name.clone());
                }
                let c_sym = definitions.get(name.as_str()).and_then(|d| d.c_symbol.as_deref());
                let ok_d = compile_parser_dynamic(name, c_sym, &parser_dir, &libs_dir);
                if ok_d {
                    dynamic_compiled.push(name.clone());
                }
                ok_s || ok_d
            }
            _ => {
                println!(
                    "cargo:warning=Unknown TSLP_LINK_MODE '{}', defaulting to dynamic",
                    link_mode
                );
                let c_sym = definitions.get(name.as_str()).and_then(|d| d.c_symbol.as_deref());
                let ok = compile_parser_dynamic(name, c_sym, &parser_dir, &libs_dir);
                if ok {
                    dynamic_compiled.push(name.clone());
                }
                ok
            }
        };
        if !ok {
            failed.push(name.clone());
        }
    }

    if !skipped_wasm.is_empty() {
        println!(
            "cargo:warning=wasm32: skipped {} oversized grammar(s) (excluded from the wasm build): {}. Set TSLP_WASM_MAX_PARSER_BYTES=0 to force-compile them on a high-memory builder.",
            skipped_wasm.len(),
            skipped_wasm.join(", "),
        );
    }

    if !failed.is_empty() {
        // ~keep Persist build failures so CI tooling can inspect them after the panic.
        let _ = fs::create_dir_all(&out_dir);
        let failed_path = out_dir.join("failed_languages.txt");
        if let Err(e) = fs::write(&failed_path, failed.join("\n") + "\n") {
            eprintln!(
                "WARNING: Failed to write {} for CI inspection: {}",
                failed_path.display(),
                e
            );
        }
        let allow_failures = env::var("TSLP_ALLOW_FAILED_GRAMMARS").ok().is_some_and(|v| v == "1");
        let message = format!(
            "FAILED to compile {} grammar(s): {}. Published artifacts must not advertise grammars that fail to build — fix the grammar pin or remove the entry from sources/language_definitions.json. Set TSLP_ALLOW_FAILED_GRAMMARS=1 for local debugging only.",
            failed.len(),
            failed.join(", "),
        );
        if allow_failures {
            println!("cargo:warning={message}");
        } else {
            panic!("{message}");
        }
    }

    // ~keep Statically linked scanners reference the utf8proc-backed wide-ctype
    // shim; compile utf8proc once here so its symbols resolve at final link
    // without being multiply defined across the per-grammar scanner archives.
    if (link_mode == "static" || link_mode == "both") && !static_compiled.is_empty() {
        let (_, _, utf8proc_source) = ctype_shim_assets();
        compile_utf8proc_archive(&utf8proc_source);
    }

    generate_registry(&static_compiled, &dynamic_compiled, &definitions, &libs_dir, &out_dir);
    generate_extensions_lookup(&definitions, &out_dir);
    generate_queries_registry(&definitions, &parsers_dir, &out_dir);
}
