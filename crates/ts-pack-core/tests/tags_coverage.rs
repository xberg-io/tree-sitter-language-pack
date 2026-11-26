//! Gate for `sources/tags_coverage.json`: every language needs a working `tags.scm` unless that
//! file lists it under `not_applicable` with a reason, so a newly added grammar is a tags gap by
//! default.
//!
//! Each language that needs tags has a fixture directory `tests/fixtures/tags/<lang>/` holding
//! exactly one source file and `expected.tags`: one `<role>\t<name>` line per tag, in source
//! order, where the role is the `definition.<kind>` or `reference.<kind>` capture and the name is
//! the text of the `@name` capture (`\`, tab and newline escaped as `\\`, `\t`, `\n`). Tags
//! are resolved as tree-sitter-tags resolves them; see `tag_lines`.
//!
//! The structural tests read only the repository. The per-language test needs the grammars at
//! build time, for example: `TSLP_LANGUAGES=all cargo test -p tree-sitter-language-pack \
//!  --test tags_coverage`. `TSLP_TAGS_BLESS=1` rewrites each built language's `expected.tags`
//! from the query output; review the diff before committing it.

// ~keep Integration tests do not inherit the library's crate-root allow: the notices need
// ~keep print_stderr, and a failed setup step in a test must abort loudly.
#![allow(clippy::print_stderr, clippy::expect_used)]

use std::collections::BTreeMap;
use std::fmt::Write as _;
use std::path::{Path, PathBuf};

use tree_sitter::{Parser, QueryCursor, StreamingIterator};
use tree_sitter_language_pack::{LanguageRegistry, QueryKind, get_query};

const EXPECTED_FILE: &str = "expected.tags";

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn fixtures_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/tags")
}

fn read_json(path: &Path) -> serde_json::Value {
    let text = std::fs::read_to_string(path).unwrap_or_else(|e| panic!("read {}: {e}", path.display()));
    serde_json::from_str(&text).unwrap_or_else(|e| panic!("parse {}: {e}", path.display()))
}

/// Every language name in `sources/language_definitions.json`.
fn all_languages() -> Vec<String> {
    let definitions = read_json(&repo_root().join("sources/language_definitions.json"));
    let object = definitions
        .as_object()
        .expect("language_definitions.json must be an object");
    object.keys().cloned().collect()
}

/// The `not_applicable` map of `sources/tags_coverage.json`: language to reason.
fn not_applicable() -> BTreeMap<String, String> {
    let coverage = read_json(&repo_root().join("sources/tags_coverage.json"));
    let entries = coverage
        .get("not_applicable")
        .and_then(serde_json::Value::as_object)
        .expect("tags_coverage.json must have a `not_applicable` object");
    entries
        .iter()
        .map(|(language, reason)| {
            let reason = reason
                .as_str()
                .unwrap_or_else(|| panic!("not_applicable reason for {language} must be a string"));
            (language.clone(), reason.to_string())
        })
        .collect()
}

/// A fixture directory's source file, or why the directory is malformed. `expected.tags` may be
/// absent only while blessing, which writes it.
fn fixture_source(dir: &Path, require_expected: bool) -> Result<PathBuf, String> {
    let entries = std::fs::read_dir(dir).map_err(|e| format!("read {}: {e}", dir.display()))?;
    let mut sources = Vec::new();
    let mut has_expected = false;
    for entry in entries {
        let path = entry.map_err(|e| format!("read {}: {e}", dir.display()))?.path();
        if !path.is_file() {
            return Err(format!("{} is not a regular file", path.display()));
        }
        if path.file_name().is_some_and(|name| name == EXPECTED_FILE) {
            has_expected = true;
        } else {
            sources.push(path);
        }
    }
    if require_expected && !has_expected {
        return Err(format!("{} has no {EXPECTED_FILE}", dir.display()));
    }
    match <[PathBuf; 1]>::try_from(sources) {
        Ok([source]) => Ok(source),
        Err(sources) => Err(format!(
            "{} must hold exactly one source file besides {EXPECTED_FILE}, found {}",
            dir.display(),
            sources.len()
        )),
    }
}

fn escape(name: &str) -> String {
    name.replace('\\', "\\\\").replace('\t', "\\t").replace('\n', "\\n")
}

/// The bundled tags query for `language`, or why there is none.
fn tags_query(language: &str) -> Result<std::sync::Arc<tree_sitter::Query>, String> {
    match get_query(language, QueryKind::Tags) {
        Ok(Some(query)) => Ok(query),
        Ok(None) => Err("no tags.scm is bundled (vendored or query-overlays/<lang>/tags.scm)".to_string()),
        Err(e) => Err(format!("tags.scm does not compile: {e}")),
    }
}

/// The tags `language`'s bundled query yields over `source`, as `<role>\t<name>` lines.
///
/// Mirrors tree-sitter-tags: each `@name` node carries at most one tag, taken from the earliest
/// pattern that matched it; a match with `@ignore` and no role claims its node without a tag; a
/// match with no role (`@doc`, `@local` helpers) yields nothing; tags come out ordered by
/// position. A role without `@name` is a broken pattern and fails.
fn tag_lines(language: &str, grammar: &tree_sitter::Language, source: &str) -> Result<Vec<String>, String> {
    let query = tags_query(language)?;
    let mut parser = Parser::new();
    parser
        .set_language(grammar)
        .map_err(|e| format!("grammar does not load: {e}"))?;
    let tree = parser.parse(source, None).ok_or("the fixture did not parse")?;
    if tree.root_node().has_error() {
        return Err(format!(
            "the fixture needs error recovery: {}",
            tree.root_node().to_sexp()
        ));
    }

    // ~keep Keyed like tree-sitter-tags' queue, by (name end, name start); the value is the
    // ~keep claiming pattern and its line, None for an @ignore claim.
    let mut claims: BTreeMap<(usize, usize), (usize, Option<String>)> = BTreeMap::new();
    let mut cursor = QueryCursor::new();
    let mut matches = cursor.matches(&query, tree.root_node(), source.as_bytes());
    while let Some(m) = matches.next() {
        let mut role = None;
        let mut name = None;
        let mut ignored = None;
        for capture in m.captures() {
            let capture_name = query.capture_names()[capture.index as usize];
            if capture_name == "name" {
                name = Some(capture.node.byte_range());
            } else if capture_name == "ignore" {
                ignored = Some(capture.node.byte_range());
            } else if capture_name.starts_with("definition.") || capture_name.starts_with("reference.") {
                role = Some(capture_name);
            }
        }
        let (range, line) = match (role, name, ignored) {
            (Some(role), Some(range), _) if range.is_empty() => {
                return Err(format!(
                    "pattern {} matched {role} with an empty @name",
                    m.pattern_index
                ));
            }
            (Some(role), Some(range), _) => {
                let line = format!("{role}\t{}", escape(&source[range.clone()]));
                (range, Some(line))
            }
            (Some(role), None, _) => {
                return Err(format!(
                    "pattern {} matched {role} without a @name capture",
                    m.pattern_index
                ));
            }
            (None, _, Some(range)) => (range, None),
            (None, _, None) => continue,
        };
        let claim = claims
            .entry((range.end, range.start))
            .or_insert((m.pattern_index, line.clone()));
        if m.pattern_index < claim.0 {
            *claim = (m.pattern_index, line);
        }
    }
    Ok(claims.into_values().filter_map(|(_, line)| line).collect())
}

/// Line-by-line comparison that names each differing position.
fn describe_mismatch(expected: &[String], actual: &[String]) -> String {
    let mut out = String::new();
    for index in 0..expected.len().max(actual.len()) {
        let want = expected.get(index).map_or("<none>", String::as_str);
        let got = actual.get(index).map_or("<none>", String::as_str);
        if want != got {
            let _ = writeln!(out, "    line {}: expected {want:?}, got {got:?}", index + 1);
        }
    }
    let _ = write!(out, "    full output:\n      {}", actual.join("\n      "));
    out
}

/// Checks one required language against its fixture; `Ok(false)` when the fixture is missing,
/// a gap `every_language_requiring_tags_has_a_fixture` reports.
fn check_required(language: &str, grammar: &tree_sitter::Language, bless: bool) -> Result<bool, String> {
    let dir = fixtures_root().join(language);
    if !dir.is_dir() {
        return Ok(false);
    }
    let source_path = fixture_source(&dir, !bless)?;
    let source = std::fs::read_to_string(&source_path).map_err(|e| format!("read {}: {e}", source_path.display()))?;
    let actual = tag_lines(language, grammar, &source)?;
    if actual.is_empty() {
        return Err(format!("the query yields no tags for {}", source_path.display()));
    }
    let expected_path = dir.join(EXPECTED_FILE);
    if bless {
        let mut text = actual.join("\n");
        text.push('\n');
        std::fs::write(&expected_path, text).map_err(|e| format!("write {}: {e}", expected_path.display()))?;
        return Ok(true);
    }
    let expected_text =
        std::fs::read_to_string(&expected_path).map_err(|e| format!("read {}: {e}", expected_path.display()))?;
    let expected: Vec<String> = expected_text.lines().map(str::to_string).collect();
    if expected != actual {
        return Err(format!(
            "tags differ from {}:\n{}",
            expected_path.display(),
            describe_mismatch(&expected, &actual)
        ));
    }
    Ok(true)
}

#[test]
fn tags_coverage_names_real_languages_with_reasons_and_no_tags() {
    let languages = all_languages();
    let not_applicable = not_applicable();
    let mut problems = Vec::new();

    for (language, reason) in &not_applicable {
        if !languages.contains(language) {
            problems.push(format!(
                "{language}: not_applicable names no language in language_definitions.json"
            ));
        }
        if reason.trim().is_empty() {
            problems.push(format!("{language}: not_applicable needs a reason"));
        }
        if fixtures_root().join(language).exists() {
            problems.push(format!("{language}: not_applicable but has a tags fixture directory"));
        }
        if repo_root()
            .join("query-overlays")
            .join(language)
            .join("tags.scm")
            .exists()
        {
            problems.push(format!(
                "{language}: not_applicable but has query-overlays/{language}/tags.scm"
            ));
        }
    }

    let entries = std::fs::read_dir(fixtures_root()).expect("tests/fixtures/tags must exist");
    for entry in entries {
        let path = entry.expect("fixture directory entry").path();
        let name = path
            .file_name()
            .map(|n| n.to_string_lossy().into_owned())
            .unwrap_or_default();
        if !path.is_dir() {
            problems.push(format!(
                "{name}: tests/fixtures/tags holds only per-language directories"
            ));
        } else if !languages.contains(&name) {
            problems.push(format!(
                "{name}: fixture directory names no language in language_definitions.json"
            ));
        } else if let Err(problem) = fixture_source(&path, true) {
            problems.push(format!("{name}: {problem}"));
        }
    }

    problems.sort();
    assert!(
        problems.is_empty(),
        "{} tags coverage problem(s):\n{}",
        problems.len(),
        problems.join("\n")
    );
}

#[test]
fn every_language_requiring_tags_has_a_fixture() {
    let not_applicable = not_applicable();
    let mut gaps: Vec<String> = all_languages()
        .into_iter()
        .filter(|language| !not_applicable.contains_key(language) && !fixtures_root().join(language).is_dir())
        .collect();
    gaps.sort();
    assert!(
        gaps.is_empty(),
        "{} language(s) need a tags.scm and a fixture under tests/fixtures/tags/<lang>/, or a \
         not_applicable entry with a reason in sources/tags_coverage.json:\n{}",
        gaps.len(),
        gaps.join("\n")
    );
}

#[test]
fn every_built_language_matches_its_tags_classification() {
    let registry = LanguageRegistry::new();
    let languages = all_languages();
    // ~keep available_languages() also lists aliases of built grammars; their targets are checked.
    let built: Vec<String> = registry
        .available_languages()
        .into_iter()
        .filter(|language| languages.contains(language))
        .collect();
    let not_applicable = not_applicable();
    let bless = std::env::var_os("TSLP_TAGS_BLESS").is_some();

    let mut failures = Vec::new();
    let mut checked = 0usize;
    let mut deferred = Vec::new();
    for language in &built {
        if not_applicable.contains_key(language) {
            match get_query(language, QueryKind::Tags) {
                Ok(None) => checked += 1,
                _ => failures.push(format!(
                    "{language}: listed not_applicable in sources/tags_coverage.json but bundles a tags.scm; \
                     remove the entry and add a fixture"
                )),
            }
            continue;
        }
        let grammar = registry
            .get_language(language)
            .unwrap_or_else(|e| panic!("{language} is listed as built but does not load: {e}"));
        match check_required(language, &grammar, bless) {
            Ok(true) => checked += 1,
            Ok(false) => deferred.push(language.as_str()),
            Err(problem) => failures.push(format!("{language}: {problem}")),
        }
    }

    eprintln!(
        "tags coverage: checked {checked} of {} built language(s); {} without a fixture yet \
         (reported by every_language_requiring_tags_has_a_fixture)",
        built.len(),
        deferred.len()
    );
    if built.is_empty() {
        eprintln!("SKIPPED tags classification: build with TSLP_LANGUAGES (for example TSLP_LANGUAGES=all)");
    }
    if option_env!("TSLP_LANGUAGES").is_some_and(|value| !value.is_empty()) {
        assert!(
            checked > 0,
            "TSLP_LANGUAGES was set at build time but no built language was checked; built: {built:?}"
        );
    }
    failures.sort();
    assert!(
        failures.is_empty(),
        "{} language(s) fail their tags classification:\n{}",
        failures.len(),
        failures.join("\n")
    );
}
