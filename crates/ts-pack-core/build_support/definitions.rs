use serde::Deserialize;
use std::collections::BTreeMap;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Deserialize)]
pub(crate) struct LanguageDefinition {
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
    pub(crate) extensions: Vec<String>,
    #[serde(default)]
    pub(crate) ambiguous: BTreeMap<String, Vec<String>>,
    /// Override for the C symbol name when it differs from the language name.
    /// E.g. language "csharp" exports `tree_sitter_c_sharp()`.
    #[serde(default)]
    pub(crate) c_symbol: Option<String>,
}

pub(crate) fn find_project_root() -> PathBuf {
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

/// Reject grammar keys that are not plain identifiers.
///
/// Every key becomes a filesystem path segment (`parsers/<key>`), a `cc` output
/// archive name, and — on the dynamic path — an argument in a raw compiler
/// [`std::process::Command`]. A key beginning with `-` would be parsed by the
/// compiler as a flag, and separators or shell metacharacters would escape the
/// parsers tree. Enforce the same alphabet [`selected_languages`] already
/// requires of `TSLP_LANGUAGES`. ~keep
pub(crate) fn validate_definition_keys(definitions: &BTreeMap<String, LanguageDefinition>) {
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

pub(crate) fn selected_languages(definitions: &BTreeMap<String, LanguageDefinition>) -> Vec<String> {
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
/// A full 372-grammar wasm build is impractical: the parser.c sources total ~1.7 GB,
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

pub(crate) fn load_definitions(project_root: &Path) -> BTreeMap<String, LanguageDefinition> {
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

    if definitions_path.exists() {
        println!("cargo:rerun-if-changed={}", definitions_path.display());
        let definitions_json = fs::read_to_string(&definitions_path)
            .unwrap_or_else(|e| panic!("Failed to read {}: {e}", definitions_path.display()));
        serde_json::from_str(&definitions_json).expect("Failed to parse language_definitions.json")
    } else {
        BTreeMap::new()
    }
}
