use crate::shim::{ctype_shim_assets, ctype_shim_excluded};
use std::env;
use std::path::{Path, PathBuf};
use std::process::Command;

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

struct SharedLibPlan<'a> {
    name: &'a str,
    includes: Vec<PathBuf>,
    c_sources: Vec<PathBuf>,
    scanner_cc: PathBuf,
    has_scanner_cc: bool,
    shim: bool,
    shim_header: PathBuf,
    output_dir: &'a Path,
    output_path: PathBuf,
    os: String,
}

fn plan_shared_lib<'a>(
    name: &'a str,
    c_symbol: Option<&str>,
    parser_dir: &Path,
    output_dir: &'a Path,
) -> Option<SharedLibPlan<'a>> {
    let src_dir = parser_dir.join("src");
    let parser_c = src_dir.join("parser.c");

    if !parser_c.exists() {
        println!(
            "cargo:warning=Skipping language '{}': parser.c not found at {}",
            name,
            parser_c.display()
        );
        return None;
    }

    let mut c_sources = vec![parser_c];
    let scanner_c = src_dir.join("scanner.c");
    let has_scanner_c = scanner_c.exists();
    if has_scanner_c {
        c_sources.push(scanner_c);
    }

    let scanner_cc = src_dir.join("scanner.cc");
    let has_scanner_cc = scanner_cc.exists();
    let has_scanner = has_scanner_c || has_scanner_cc;
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
    let (shim_header, utf8proc_include, utf8proc_source) = ctype_shim_assets();
    if shim {
        includes.push(utf8proc_include);
        c_sources.push(utf8proc_source);
    }

    let sym = c_symbol.unwrap_or(name);
    let lib_name = format!("tree_sitter_{sym}");
    let os = target_os();
    let (prefix, ext) = shared_lib_components(&os);
    let output_path = output_dir.join(format!("{prefix}{lib_name}.{ext}"));

    Some(SharedLibPlan {
        name,
        includes,
        c_sources,
        scanner_cc,
        has_scanner_cc,
        shim,
        shim_header,
        output_dir,
        output_path,
        os,
    })
}

fn add_msvc_args(cmd: &mut Command, plan: &SharedLibPlan) {
    // ~keep Use per-file MSVC language flags; global `/TC`/`/TP` breaks parser.c C99 initializers.
    cmd.arg("/std:c11");
    cmd.arg("/utf-8");
    cmd.arg("/O2");
    cmd.arg("/wd4244");
    cmd.arg("/wd4566");
    cmd.arg("/wd4819");
    if plan.shim {
        // ~keep utf8proc.c is compiled straight into this shared library, and utf8proc.h
        // decorates every utf8proc_* declaration __declspec(dllimport) unless this is
        // defined — so MSVC rejects the definitions in the same TU with C2491 ("definition
        // of dllimport function not allowed") and every scanner grammar fails. Both
        // static-link paths (apply_ctype_shim, compile_utf8proc_archive) already define it;
        // this third, raw-Command path did not, so no Windows parser binary ever built.
        cmd.arg("/DUTF8PROC_STATIC");
        cmd.arg(format!("/FI{}", plan.shim_header.display()));
    }
    for inc in &plan.includes {
        cmd.arg(format!("/I{}", inc.display()));
    }
    for src in &plan.c_sources {
        cmd.arg(format!("/Tc{}", src.display()));
    }
    if plan.has_scanner_cc {
        cmd.arg(format!("/Tp{}", plan.scanner_cc.display()));
    }
    cmd.arg("/LD");
    cmd.arg(format!("/Fe:{}", plan.output_path.display()));
}

/// ~keep Mixed C/C++ shared libs compile scanner.cc to an object first, then link everything.
/// Returns `None` after reporting the failure when the object could not be built.
fn compile_cpp_scanner_object(plan: &SharedLibPlan) -> Option<PathBuf> {
    let name = plan.name;
    let scanner_obj = plan.output_dir.join(format!("{name}_scanner.o"));
    let cpp_compiler = cc::Build::new().cpp(true).get_compiler();
    let mut cpp_cmd = cpp_compiler.to_command();
    cpp_cmd.arg("-c");
    cpp_cmd.arg("-fPIC");
    cpp_cmd.arg("-O2");
    cpp_cmd.arg("-fno-strict-aliasing");
    for inc in &plan.includes {
        cpp_cmd.arg(format!("-I{}", inc.display()));
    }
    if plan.shim {
        // ~keep The shim header declares utf8proc_*; this object links into the same
        // shared library that defines them, so it must agree on the decoration.
        cpp_cmd.arg("-DUTF8PROC_STATIC");
        cpp_cmd.arg("-include");
        cpp_cmd.arg(&plan.shim_header);
    }
    cpp_cmd.arg(&plan.scanner_cc);
    cpp_cmd.arg("-o");
    cpp_cmd.arg(&scanner_obj);
    match cpp_cmd.status() {
        Ok(s) if s.success() => Some(scanner_obj),
        Ok(s) => {
            println!(
                "cargo:warning=Failed to compile C++ scanner for '{}': exit code {:?}",
                name,
                s.code()
            );
            None
        }
        Err(e) => {
            println!("cargo:warning=Failed to run C++ compiler for '{}': {}", name, e);
            None
        }
    }
}

/// Returns `false` when the C++ scanner object could not be built.
fn add_unix_args(cmd: &mut Command, plan: &SharedLibPlan) -> bool {
    cmd.arg("-std=c11");
    cmd.arg("-O2");
    cmd.arg("-fno-strict-aliasing");
    cmd.arg("-fPIC");
    for inc in &plan.includes {
        cmd.arg(format!("-I{}", inc.display()));
    }
    if plan.shim {
        // ~keep Harmless on non-MSVC (utf8proc.h only decorates under _WIN32), but keeps the
        // three compile paths defining the same macro set. See the MSVC branch above.
        cmd.arg("-DUTF8PROC_STATIC");
        cmd.arg("-include");
        cmd.arg(&plan.shim_header);
    }
    for src in &plan.c_sources {
        cmd.arg(src);
    }
    if plan.has_scanner_cc {
        let Some(scanner_obj) = compile_cpp_scanner_object(plan) else {
            return false;
        };
        cmd.arg(&scanner_obj);
    }

    let apple = plan.os == "macos" || plan.os == "ios";
    cmd.arg(if apple { "-dynamiclib" } else { "-shared" });
    // ~keep C++ scanners require the C++ standard library for std:: and ABI symbols.
    if plan.has_scanner_cc {
        cmd.arg(if apple { "-lc++" } else { "-lstdc++" });
    }
    cmd.arg("-o");
    cmd.arg(&plan.output_path);
    true
}

fn run_shared_lib_command(mut cmd: Command, plan: &SharedLibPlan) -> bool {
    let name = plan.name;
    match cmd.status() {
        Ok(s) if s.success() => {
            // ~keep Some compilers return success without a dylib after resource exhaustion; verify output exists.
            if plan.output_path.exists() {
                true
            } else {
                println!(
                    "cargo:warning=Failed to compile shared library for '{}': compiler succeeded but output file was not created at {}",
                    name,
                    plan.output_path.display()
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

/// Compile a single language parser into a shared library (.so/.dylib/.dll).
///
/// When a language has a `c_symbol` override (e.g. "csharp" → "c_sharp"),
/// the library is named using the c_symbol so the runtime loader can find it.
pub(crate) fn compile_parser_dynamic(name: &str, c_symbol: Option<&str>, parser_dir: &Path, output_dir: &Path) -> bool {
    let Some(plan) = plan_shared_lib(name, c_symbol, parser_dir, output_dir) else {
        return false;
    };

    let compiler = cc::Build::new().get_compiler();
    let is_msvc = compiler.is_like_msvc();
    let mut cmd = compiler.to_command();

    if is_msvc {
        add_msvc_args(&mut cmd, &plan);
    } else if !add_unix_args(&mut cmd, &plan) {
        return false;
    }

    run_shared_lib_command(cmd, &plan)
}
