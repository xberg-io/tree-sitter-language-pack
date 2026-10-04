// Build scripts run before the crate compiles and cannot route through `tracing`; stderr is
// their diagnostic channel (stdout is reserved for `cargo:` directives, exempt natively). ~keep
#![allow(clippy::print_stderr)]
// A build script is not the library: it links no host process, so an unrecoverable
// condition here should abort the build loudly rather than be threaded through a
// `Result` no caller can act on. The crate-wide `unwrap_used`/`expect_used` deny
// exists to stop a panic taking down a PHP or Python host, which build.rs cannot do. ~keep
#![allow(clippy::unwrap_used, clippy::expect_used)]

#[path = "build_support/codegen.rs"]
mod codegen;
#[path = "build_support/definitions.rs"]
mod definitions;
#[path = "build_support/dynamic.rs"]
mod dynamic;
#[path = "build_support/patches.rs"]
mod patches;
#[path = "build_support/shim.rs"]
mod shim;
#[path = "build_support/sources.rs"]
mod sources;
#[path = "build_support/static_link.rs"]
mod static_link;
#[path = "build_support/wasm.rs"]
mod wasm;

use codegen::{generate_extensions_lookup, generate_queries_registry, generate_registry};
use definitions::{
    LanguageDefinition, find_project_root, load_definitions, selected_languages, validate_definition_keys,
};
use dynamic::compile_parser_dynamic;
use patches::{apply_msvc_compat_patches, patch_grammar_sources};
use shim::{compile_utf8proc_archive, ctype_shim_assets};
use sources::ensure_parser_sources;
use static_link::compile_parser_static;
use std::collections::BTreeMap;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use wasm::{wasm_parser_size_limit, wasm_skip_grammars};

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

#[derive(Default)]
struct CompileOutcome {
    static_compiled: Vec<String>,
    dynamic_compiled: Vec<String>,
    failed: Vec<String>,
    skipped_wasm: Vec<String>,
}

struct LanguageCompiler<'a> {
    definitions: &'a BTreeMap<String, LanguageDefinition>,
    parsers_dir: &'a Path,
    libs_dir: &'a Path,
    link_mode: &'a str,
    wasm_size_limit: Option<u64>,
    wasm_skip: Vec<String>,
}

impl LanguageCompiler<'_> {
    fn build_static(&self, name: &str, parser_dir: &Path, outcome: &mut CompileOutcome) -> bool {
        let ok = compile_parser_static(name, parser_dir);
        if ok {
            outcome.static_compiled.push(name.to_string());
        }
        ok
    }

    fn build_dynamic(&self, name: &str, parser_dir: &Path, outcome: &mut CompileOutcome) -> bool {
        let c_sym = self.definitions.get(name).and_then(|d| d.c_symbol.as_deref());
        let ok = compile_parser_dynamic(name, c_sym, parser_dir, self.libs_dir);
        if ok {
            outcome.dynamic_compiled.push(name.to_string());
        }
        ok
    }

    fn skipped_for_wasm(&self, name: &str, parser_c: &Path) -> bool {
        if self.wasm_skip.iter().any(|g| g == name) {
            println!(
                "cargo:warning=wasm32: skipping grammar '{}' — its external scanner is not wasm32-compatible (wctype/locale or C++ stdlib). Override with TSLP_WASM_SKIP_GRAMMARS.",
                name,
            );
            return true;
        }

        if let Some(limit) = self.wasm_size_limit
            && let Ok(size) = fs::metadata(parser_c).map(|m| m.len())
            && size > limit
        {
            println!(
                "cargo:warning=wasm32: skipping grammar '{}' — parser.c is {} MB (limit {} MB); too large to compile to wasm32 within runner memory. Override with TSLP_WASM_MAX_PARSER_BYTES (0 disables the gate).",
                name,
                size / (1024 * 1024),
                limit / (1024 * 1024),
            );
            return true;
        }
        false
    }

    fn build_for_link_mode(&self, name: &str, parser_dir: &Path, outcome: &mut CompileOutcome) -> bool {
        match self.link_mode {
            "static" => self.build_static(name, parser_dir, outcome),
            "dynamic" => self.build_dynamic(name, parser_dir, outcome),
            "both" => {
                let ok_s = self.build_static(name, parser_dir, outcome);
                let ok_d = self.build_dynamic(name, parser_dir, outcome);
                ok_s || ok_d
            }
            _ => {
                println!(
                    "cargo:warning=Unknown TSLP_LINK_MODE '{}', defaulting to dynamic",
                    self.link_mode
                );
                self.build_dynamic(name, parser_dir, outcome)
            }
        }
    }

    fn compile_language(&self, name: &str, outcome: &mut CompileOutcome) {
        let parser_dir = self.parsers_dir.join(name);
        let parser_c = parser_dir.join("src/parser.c");
        if !parser_c.exists() {
            println!("cargo:warning=Parser sources not found for '{}', skipping", name);
            outcome.failed.push(name.to_string());
            return;
        }

        if self.skipped_for_wasm(name, &parser_c) {
            outcome.skipped_wasm.push(name.to_string());
            return;
        }

        let out_dir = env::var("OUT_DIR").unwrap_or_default();
        if !parser_dir.starts_with(&out_dir) {
            emit_rerun_if_changed(&parser_dir);
        }

        if !self.build_for_link_mode(name, &parser_dir, outcome) {
            outcome.failed.push(name.to_string());
        }
    }
}

fn report_failed_languages(failed: &[String], out_dir: &Path) {
    if failed.is_empty() {
        return;
    }
    // ~keep Persist build failures so CI tooling can inspect them after the panic.
    let _ = fs::create_dir_all(out_dir);
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

fn emit_rerun_directives() {
    println!("cargo:rerun-if-env-changed=TSLP_LANGUAGES");
    println!("cargo:rerun-if-env-changed=PROJECT_ROOT");
    println!("cargo:rerun-if-env-changed=TSLP_LINK_MODE");
    println!("cargo:rerun-if-env-changed=TSLP_WASM_MAX_PARSER_BYTES");
    println!("cargo:rerun-if-env-changed=TSLP_GRAMMAR_CFLAGS");

    let (shim_header, _, utf8proc_source) = ctype_shim_assets();
    println!("cargo:rerun-if-changed={}", shim_header.display());
    println!("cargo:rerun-if-changed={}", utf8proc_source.display());
}

fn resolve_link_mode(target_arch: &str) -> String {
    // ~keep Force static mode for wasm32 targets because shared libraries are unsupported.
    if target_arch == "wasm32" {
        "static".to_string()
    } else {
        env::var("TSLP_LINK_MODE").unwrap_or_else(|_| "dynamic".to_string())
    }
}

fn compile_languages(
    selected: &[String],
    definitions: &BTreeMap<String, LanguageDefinition>,
    parsers_dir: &Path,
    libs_dir: &Path,
    link_mode: &str,
    target_arch: &str,
) -> CompileOutcome {
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

    let compiler = LanguageCompiler {
        definitions,
        parsers_dir,
        libs_dir,
        link_mode,
        wasm_size_limit,
        wasm_skip,
    };
    let mut outcome = CompileOutcome::default();
    for name in selected {
        compiler.compile_language(name, &mut outcome);
    }

    if !outcome.skipped_wasm.is_empty() {
        println!(
            "cargo:warning=wasm32: skipped {} oversized grammar(s) (excluded from the wasm build): {}. Set TSLP_WASM_MAX_PARSER_BYTES=0 to force-compile them on a high-memory builder.",
            outcome.skipped_wasm.len(),
            outcome.skipped_wasm.join(", "),
        );
    }
    outcome
}

fn main() {
    emit_rerun_directives();

    let project_root = find_project_root();

    let definitions = load_definitions(&project_root);
    validate_definition_keys(&definitions);
    let workspace_parsers_dir = project_root.join("parsers");

    let selected = selected_languages(&definitions);

    let target_arch = env::var("CARGO_CFG_TARGET_ARCH").unwrap_or_default();
    let link_mode = resolve_link_mode(&target_arch);

    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let libs_dir = out_dir.join("libs");
    fs::create_dir_all(&libs_dir).expect("Failed to create libs directory");

    // ~keep sdist installs lack local parser sources, so fetch release artifacts instead.
    let parsers_dir = ensure_parser_sources(&workspace_parsers_dir, &selected, &out_dir);
    // ~keep Ordered before the MSVC shim: the committed diffs are exact-context and were
    // generated against pristine upstream, while the shim is an order-insensitive substitution.
    patch_grammar_sources(&project_root, &parsers_dir, &definitions, &selected);
    apply_msvc_compat_patches(&parsers_dir);

    let outcome = compile_languages(
        &selected,
        &definitions,
        &parsers_dir,
        &libs_dir,
        &link_mode,
        &target_arch,
    );

    report_failed_languages(&outcome.failed, &out_dir);

    // ~keep Statically linked scanners reference the utf8proc-backed wide-ctype
    // shim; compile utf8proc once here so its symbols resolve at final link
    // without being multiply defined across the per-grammar scanner archives.
    if (link_mode == "static" || link_mode == "both") && !outcome.static_compiled.is_empty() {
        let (_, _, utf8proc_source) = ctype_shim_assets();
        compile_utf8proc_archive(&utf8proc_source);
    }

    generate_registry(
        &outcome.static_compiled,
        &outcome.dynamic_compiled,
        &definitions,
        &libs_dir,
        &out_dir,
    );
    generate_extensions_lookup(&definitions, &out_dir);
    generate_queries_registry(&definitions, &parsers_dir, &out_dir);
}
