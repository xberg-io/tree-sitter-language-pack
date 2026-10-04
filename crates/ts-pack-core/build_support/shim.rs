use crate::wasm::{apply_wasm32_optimizations, apply_wasm32_sysroot};
use std::env;
use std::path::{Path, PathBuf};

/// Locate the vendored deterministic wide-ctype shim assets.
///
/// ~keep Scanner character classification must be identical on every platform.
/// Returns `(force_include_header, utf8proc_include_dir, utf8proc_source)`; the
/// header is force-included ahead of each scanner TU and utf8proc is linked in
/// to back its wrappers.
pub(crate) fn ctype_shim_assets() -> (PathBuf, PathBuf, PathBuf) {
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

pub(crate) fn ctype_shim_excluded(name: &str) -> bool {
    CTYPE_SHIM_EXCLUDED_LANGUAGES.contains(&name)
}

/// ~keep Used by the static-link path (the dynamic path drives the compiler via
/// a raw `Command`). The shim only *references* utf8proc; the implementation is
/// linked once via [`compile_utf8proc_archive`] to avoid duplicate symbols when
/// many grammars are statically linked into one binary.
pub(crate) fn apply_ctype_shim(build: &mut cc::Build, header: &Path, utf8proc_include: &Path) {
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

pub(crate) fn apply_grammar_extra_flags(build: &mut cc::Build) {
    for flag in grammar_extra_flags() {
        build.flag(flag);
    }
}

/// Compile the vendored utf8proc exactly once into a static archive linked into
/// the final binary, backing the wide-ctype shim's wrappers.
///
/// ~keep Compiled once (not per grammar) so its symbols are not multiply defined
/// across the statically linked scanner archives.
pub(crate) fn compile_utf8proc_archive(utf8proc_source: &Path) {
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
