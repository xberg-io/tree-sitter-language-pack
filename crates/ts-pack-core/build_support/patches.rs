use crate::definitions::LanguageDefinition;
use std::collections::BTreeMap;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};

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
pub(crate) fn patch_grammar_sources(
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
pub(crate) fn apply_msvc_compat_patches(parsers_dir: &Path) {
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
