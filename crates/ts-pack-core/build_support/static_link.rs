use crate::shim::{apply_ctype_shim, apply_grammar_extra_flags, ctype_shim_assets, ctype_shim_excluded};
use crate::wasm::{apply_wasm32_optimizations, apply_wasm32_sysroot};
use std::fs;
use std::path::Path;

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

struct ScannerKind {
    file: &'static str,
    cpp: bool,
    archive_suffix: &'static str,
    failure_label: &'static str,
}

const C_SCANNER: ScannerKind = ScannerKind {
    file: "scanner.c",
    cpp: false,
    archive_suffix: "scanner",
    failure_label: "C scanner",
};

const CPP_SCANNER: ScannerKind = ScannerKind {
    file: "scanner.cc",
    cpp: true,
    archive_suffix: "scanner_cpp",
    failure_label: "C++ scanner",
};

fn grammar_build(src_dir: &Path, source: &Path) -> cc::Build {
    let mut build = cc::Build::new();
    build
        .include(src_dir)
        .file(source)
        .define("TREE_SITTER_HIDE_SYMBOLS", None)
        .flag_if_supported("-fvisibility=hidden")
        .flag_if_supported("-fno-strict-aliasing")
        .warnings(false);
    build
}

fn compile_parser_object(name: &str, src_dir: &Path, common_dir: &Path) -> bool {
    let parser_c = src_dir.join("parser.c");
    let mut build = grammar_build(src_dir, &parser_c);
    build.std("c11");
    apply_wasm32_sysroot(&mut build);
    apply_wasm32_optimizations(&mut build);
    apply_grammar_extra_flags(&mut build);
    if common_dir.exists() {
        build.include(common_dir);
    }

    if let Err(e) = build.try_compile(&format!("tree_sitter_{name}_parser")) {
        println!("cargo:warning=Failed to compile parser for '{}': {}", name, e);
        return false;
    }
    true
}

fn compile_scanner(kind: &ScannerKind, name: &str, src_dir: &Path, common_dir: &Path) -> bool {
    let scanner = src_dir.join(kind.file);
    if !scanner.exists() {
        return true;
    }

    let mut build = grammar_build(src_dir, &scanner);
    if kind.cpp {
        build.cpp(true);
    }
    apply_scanner_symbol_prefix(&mut build, name, &scanner);
    if !kind.cpp {
        build.std("c11");
    }
    apply_wasm32_sysroot(&mut build);
    apply_wasm32_optimizations(&mut build);
    apply_grammar_extra_flags(&mut build);
    if common_dir.exists() {
        build.include(common_dir);
    }
    if !ctype_shim_excluded(name) {
        let (shim_header, utf8proc_include, _) = ctype_shim_assets();
        apply_ctype_shim(&mut build, &shim_header, &utf8proc_include);
    }
    if let Err(e) = build.try_compile(&format!("tree_sitter_{name}_{}", kind.archive_suffix)) {
        println!(
            "cargo:warning=Failed to compile {} for '{}': {}",
            kind.failure_label, name, e
        );
        return false;
    }
    true
}

/// Compile a parser statically and link it into the main binary.
///
/// Compiles parser.c and scanner.c/cc separately to avoid symbol collisions
/// when statically linking multiple grammars. Scanner functions (`scan`,
/// `deserialize`, `serialize`, `scan_comment`) are prefixed with the language
/// name via C preprocessor defines (see [`apply_scanner_symbol_prefix`]).
pub(crate) fn compile_parser_static(name: &str, parser_dir: &Path) -> bool {
    let src_dir = parser_dir.join("src");
    let common_dir = parser_dir.join("common");

    // ~keep Prefix scanner symbols to avoid collisions when multiple grammars are statically linked.
    // ~keep C++ scanners also need separate compilation with the same symbol prefixing.
    compile_parser_object(name, &src_dir, &common_dir)
        && compile_scanner(&C_SCANNER, name, &src_dir, &common_dir)
        && compile_scanner(&CPP_SCANNER, name, &src_dir, &common_dir)
}
