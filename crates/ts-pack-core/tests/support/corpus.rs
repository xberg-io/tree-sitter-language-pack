//! Loader for real source files: this repo's fixtures and sources plus the sibling shared
//! corpus. Std-only; `#[path]`-included by the targets that sweep real files.

use std::path::{Path, PathBuf};

/// A real source file found on disk.
pub struct CorpusFile {
    pub path: PathBuf,
    pub lang: &'static str,
    pub source: String,
}

fn lang_for(path: &Path) -> Option<&'static str> {
    Some(match path.extension()?.to_str()? {
        "rs" => "rust",
        "py" => "python",
        "js" | "mjs" => "javascript",
        "ts" | "tsx" => "typescript",
        "go" => "go",
        "c" | "h" => "c",
        "java" => "java",
        "json" => "json",
        "yaml" | "yml" => "yaml",
        "toml" => "toml",
        "md" => "markdown",
        "csv" => "csv",
        _ => return None,
    })
}

fn collect(dir: &Path, depth: usize, out: &mut Vec<PathBuf>) {
    let Ok(entries) = std::fs::read_dir(dir) else { return };
    let mut entries: Vec<_> = entries.flatten().map(|e| e.path()).collect();
    entries.sort();
    for path in entries {
        if path.is_dir() {
            if depth < 6 {
                collect(&path, depth + 1, out);
            }
        } else if lang_for(&path).is_some() {
            out.push(path);
        }
    }
}

/// Real files from this repo's fixtures and sources, plus the sibling shared corpus
/// (`../xberg/test_documents/code`) when it is checked out next to this repo. Anything
/// missing is skipped; files over 2 MiB are left out to keep the sweep fast.
pub fn corpus() -> Vec<CorpusFile> {
    let manifest = Path::new(env!("CARGO_MANIFEST_DIR"));
    let roots = [
        manifest.join("../../fixtures/bench"),
        manifest.join("tests/fixtures"),
        manifest.join("src"),
        manifest.join("../../../xberg/test_documents/code"),
    ];
    let mut paths = Vec::new();
    for root in &roots {
        collect(root, 0, &mut paths);
    }
    paths
        .into_iter()
        .filter_map(|path| {
            let lang = lang_for(&path)?;
            let source = std::fs::read_to_string(&path).ok()?;
            (source.len() <= 2 << 20 && !source.is_empty()).then_some(CorpusFile { path, lang, source })
        })
        .collect()
}
