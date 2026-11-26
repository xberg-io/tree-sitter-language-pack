//! Gate for `sources/tags_coverage.json`: every language needs a working `tags.scm` unless that
//! file lists it under `not_applicable` (tags do not apply) or `grammar_blocked` (tags apply, but
//! the vendored grammar cannot produce correct names) with a reason, so a newly added grammar is a
//! tags gap by default. Both lists are enforced in both directions: a listed language must ship
//! no tags query and no fixture, and one that starts bundling a tags query fails until its entry
//! is revisited.
//!
//! Each language that needs tags has a fixture directory `tests/fixtures/tags/<lang>/` holding
//! exactly one source file and `expected.tags`: one `<role>\t<name>` line per tag, in source
//! order, where the role is the `definition.<kind>` or `reference.<kind>` capture and the name is
//! the text of the `@name` capture (`\`, tab and newline escaped as `\\`, `\t`, `\n`). Tags
//! are resolved as tree-sitter-tags resolves them; see `extract_tags`.
//!
//! Besides matching `expected.tags`, each language's tags must pass these checks, every one
//! reporting all of its violations at once:
//!
//! - `roles`: every role, emitted or declared as a capture in the query, is in `ALLOWED_ROLES`.
//! - `names`: a name equals its trim, holds no newline, carriage return or tab, and is not
//!   wrapped in matching quotes (`"`, `'`, `` ` ``) or in brackets (`()`, `[]`, `{}`, `<>`)
//!   around an identifier character (operator names such as `<>` or `[]` stay legal, and so do the
//!   JVM's own method names `<init>` and `<clinit>`).
//! - `definitions`: at least one `definition.*` tag.
//! - `dead_patterns`: every query pattern carrying a role capture wins at least one tag on the
//!   fixture, unless `unexercised_patterns` in `sources/tags_coverage.json` lists it with a
//!   reason; a listed pattern that fires or no longer exists fails too. A pattern is identified by
//!   its normalized source text (see `normalize_pattern`), so moving it keeps its identity and
//!   editing it needs the opt-out revisited.
//! - `duplicates`: two `definition.*` tags with the same role and name fail when they start at
//!   the same position, when their definition nodes (the nodes the role captures) are children of
//!   the same parent node (overloads, multi-clause functions, a prototype beside its definition),
//!   or when one definition node overlaps the other (a name tagged both by a wrapper and by the
//!   declaration it wraps). The same name defined under different parents, such as a method
//!   `area` in two classes, passes. References are never duplicates.
//!
//! The structural tests read only the repository. The per-language test needs the grammars at
//! build time, for example: `TSLP_LANGUAGES=all cargo test -p tree-sitter-language-pack \
//!  --test tags_coverage`. `TSLP_TAGS_BLESS=1` rewrites each built language's `expected.tags`
//! from the query output; review the diff before committing it. `TSLP_TAGS_REPORT=<path>` writes
//! every built language's violations as JSON, grouped by check.

// ~keep Integration tests do not inherit the library's crate-root allow: the notices need
// ~keep print_stderr, and a failed setup step in a test must abort loudly.
#![allow(clippy::print_stderr, clippy::expect_used)]

use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Write as _;
use std::ops::Range;
use std::path::{Path, PathBuf};

use tree_sitter::{CaptureQuantifier, Parser, Query, QueryCursor, StreamingIterator};
use tree_sitter_language_pack::{LanguageRegistry, QueryKind, get_query, get_tags_query};

const EXPECTED_FILE: &str = "expected.tags";

/// Every role a tag may carry. Mapping other kinds onto these is documented in
/// `docs-site/src/content/docs/contributing.md`.
const ALLOWED_ROLES: &[&str] = &[
    "definition.class",
    "definition.interface",
    "definition.function",
    "definition.method",
    "definition.constructor",
    "definition.module",
    "definition.type",
    "definition.enum",
    "definition.constant",
    "definition.variable",
    "definition.field",
    "definition.macro",
    "definition.section",
    "definition.label",
    "reference.call",
    "reference.class",
    "reference.type",
    "reference.interface",
    "reference.implementation",
    "reference.module",
    "reference.macro",
    "reference.label",
    "reference.variable",
    "reference.constant",
];

const CHECK_ROLES: &str = "roles";
const CHECK_NAMES: &str = "names";
const CHECK_DEFINITIONS: &str = "definitions";
const CHECK_DEAD_PATTERNS: &str = "dead_patterns";
const CHECK_DUPLICATES: &str = "duplicates";
/// Fixture problems: missing or malformed files, parse errors, broken patterns, and output that
/// differs from `expected.tags`.
const CHECK_FIXTURE: &str = "fixture";

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

fn coverage() -> serde_json::Value {
    read_json(&repo_root().join("sources/tags_coverage.json"))
}

/// Every language name in `sources/language_definitions.json`.
fn all_languages() -> Vec<String> {
    let definitions = read_json(&repo_root().join("sources/language_definitions.json"));
    let object = definitions
        .as_object()
        .expect("language_definitions.json must be an object");
    object.keys().cloned().collect()
}

const NOT_APPLICABLE: &str = "not_applicable";
const GRAMMAR_BLOCKED: &str = "grammar_blocked";
const UNEXERCISED_PATTERNS: &str = "unexercised_patterns";

/// A `{language: reason}` section of `sources/tags_coverage.json`.
fn coverage_section(section: &str) -> BTreeMap<String, String> {
    let coverage = coverage();
    let entries = coverage
        .get(section)
        .and_then(serde_json::Value::as_object)
        .unwrap_or_else(|| panic!("tags_coverage.json must have a `{section}` object"));
    entries
        .iter()
        .map(|(language, reason)| {
            let reason = reason
                .as_str()
                .unwrap_or_else(|| panic!("{section} reason for {language} must be a string"));
            (language.clone(), reason.to_string())
        })
        .collect()
}

/// Languages where tags do not apply: language to reason.
fn not_applicable() -> BTreeMap<String, String> {
    coverage_section(NOT_APPLICABLE)
}

/// Languages where tags apply but the vendored grammar cannot produce correct names: language to
/// reason.
fn grammar_blocked() -> BTreeMap<String, String> {
    coverage_section(GRAMMAR_BLOCKED)
}

/// A pattern deliberately left unexercised by its language's fixture.
#[derive(Debug, Clone, PartialEq, Eq)]
struct PatternOptOut {
    pattern: String,
    reason: String,
}

/// The `unexercised_patterns` section: language to opt-outs. Malformed entries are returned as
/// problems rather than panicking so the structural test lists them all.
fn unexercised_patterns() -> (BTreeMap<String, Vec<PatternOptOut>>, Vec<String>) {
    let coverage = coverage();
    let mut problems = Vec::new();
    let Some(section) = coverage
        .get(UNEXERCISED_PATTERNS)
        .and_then(serde_json::Value::as_object)
    else {
        return (
            BTreeMap::new(),
            vec![format!(
                "tags_coverage.json must have an `{UNEXERCISED_PATTERNS}` object"
            )],
        );
    };
    let mut result = BTreeMap::new();
    for (language, entries) in section {
        let Some(entries) = entries.as_array() else {
            problems.push(format!("{language}: {UNEXERCISED_PATTERNS} must be a list"));
            continue;
        };
        let mut opt_outs = Vec::new();
        for entry in entries {
            let field = |key: &str| entry.get(key).and_then(serde_json::Value::as_str).map(str::trim);
            match (field("pattern"), field("reason")) {
                (Some(pattern), Some(reason)) if !pattern.is_empty() && !reason.is_empty() => {
                    opt_outs.push(PatternOptOut {
                        pattern: pattern.to_string(),
                        reason: reason.to_string(),
                    });
                }
                _ => problems.push(format!(
                    "{language}: each {UNEXERCISED_PATTERNS} entry needs a non-empty `pattern` and `reason`: {entry}"
                )),
            }
        }
        result.insert(language.clone(), opt_outs);
    }
    (result, problems)
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

/// The bundled tags query for `language` and its source text, or why there is none.
fn tags_query(language: &str) -> Result<(std::sync::Arc<Query>, &'static str), String> {
    match (get_query(language, QueryKind::Tags), get_tags_query(language)) {
        (Ok(Some(query)), Some(source)) => Ok((query, source)),
        (Ok(_), _) => Err("no tags.scm is bundled (vendored or query-overlays/<lang>/tags.scm)".to_string()),
        (Err(e), _) => Err(format!("tags.scm does not compile: {e}")),
    }
}

/// One emitted tag.
#[derive(Debug, Clone, PartialEq, Eq)]
struct Tag {
    role: String,
    name: String,
    /// Byte range of the `@name` node.
    name_range: Range<usize>,
    /// Zero-based row of the `@name` node.
    row: usize,
    /// Byte range of the node the role captures (the definition or reference node).
    node_range: Range<usize>,
    /// Tree-unique id of that node's parent, if it has one.
    parent: Option<usize>,
    /// Index of the pattern that emitted the tag.
    pattern: usize,
}

impl Tag {
    fn line(&self) -> String {
        format!("{}\t{}", self.role, escape(&self.name))
    }

    fn describe(&self) -> String {
        format!("{} {:?} at line {}", self.role, self.name, self.row + 1)
    }
}

/// The tags a query yields over `source`.
///
/// Mirrors tree-sitter-tags: each `@name` node carries at most one tag, taken from the earliest
/// pattern that matched it; a match with `@ignore` and no role claims its node without a tag; a
/// match with no role (`@doc`, `@local` helpers) yields nothing; tags come out ordered by
/// position. A role without `@name`, or with an empty one, is a broken pattern and fails.
fn extract_tags(query: &Query, grammar: &tree_sitter::Language, source: &str) -> Result<Vec<Tag>, String> {
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
    // ~keep claiming pattern and its tag, None for an @ignore claim.
    let mut claims: BTreeMap<(usize, usize), (usize, Option<Tag>)> = BTreeMap::new();
    let mut cursor = QueryCursor::new();
    let mut matches = cursor.matches(query, tree.root_node(), source.as_bytes());
    while let Some(m) = matches.next() {
        let mut role = None;
        let mut name = None;
        let mut ignored = None;
        for capture in m.captures() {
            let capture_name = query.capture_names()[capture.index as usize];
            if capture_name == "name" {
                name = Some(capture.node);
            } else if capture_name == "ignore" {
                ignored = Some(capture.node.byte_range());
            } else if is_role(capture_name) {
                role = Some((capture_name, capture.node));
            }
        }
        let pattern = m.pattern_index;
        let (range, tag) = match (role, name, ignored) {
            (Some((role, _)), Some(name), _) if name.byte_range().is_empty() => {
                return Err(format!("pattern {pattern} matched {role} with an empty @name"));
            }
            (Some((role, node)), Some(name), _) => {
                let tag = Tag {
                    role: role.to_string(),
                    name: source[name.byte_range()].to_string(),
                    name_range: name.byte_range(),
                    row: name.start_position().row,
                    node_range: node.byte_range(),
                    parent: node.parent().map(|parent| parent.id()),
                    pattern,
                };
                (name.byte_range(), Some(tag))
            }
            (Some((role, _)), None, _) => {
                return Err(format!("pattern {pattern} matched {role} without a @name capture"));
            }
            (None, _, Some(range)) => (range, None),
            (None, _, None) => continue,
        };
        let claim = claims.entry((range.end, range.start)).or_insert((pattern, tag.clone()));
        if pattern < claim.0 {
            *claim = (pattern, tag);
        }
    }
    Ok(claims.into_values().filter_map(|(_, tag)| tag).collect())
}

fn is_role(capture_name: &str) -> bool {
    capture_name.starts_with("definition.") || capture_name.starts_with("reference.")
}

/// `roles`: every emitted role and every role capture the query declares is allowed.
fn check_roles(tags: &[Tag], declared: &[&str]) -> Vec<String> {
    let mut problems = Vec::new();
    for role in declared.iter().filter(|role| is_role(role)) {
        if !ALLOWED_ROLES.contains(role) {
            problems.push(format!("the query declares @{role}, which is not an allowed role"));
        }
    }
    for tag in tags {
        if !ALLOWED_ROLES.contains(&tag.role.as_str()) {
            problems.push(format!("{}: role is not allowed", tag.describe()));
        }
    }
    problems
}

const QUOTES: &[char] = &['"', '\'', '`'];
const BRACKETS: &[(char, char)] = &[('(', ')'), ('[', ']'), ('{', '}'), ('<', '>')];
/// Names whose brackets are part of the identifier at every use site: the JVM names constructors
/// and static initializers this way in bytecode, smali and javap output.
const BRACKETED_IDENTIFIERS: &[&str] = &["<init>", "<clinit>"];

/// Why `name` is not a clean identifier, if it is not.
fn name_problem(name: &str) -> Option<&'static str> {
    if name.trim() != name {
        return Some("has leading or trailing whitespace");
    }
    if name.contains(['\n', '\r', '\t']) {
        return Some("contains a newline, carriage return or tab");
    }
    let (Some(first), Some(last)) = (name.chars().next(), name.chars().last()) else {
        return Some("is empty");
    };
    if name.chars().count() < 2 {
        return None;
    }
    if first == last && QUOTES.contains(&first) {
        return Some("is wrapped in quotes; capture the string content instead");
    }
    let inner = &name[first.len_utf8()..name.len() - last.len_utf8()];
    let wraps_identifier = inner.chars().any(|c| c.is_alphanumeric() || c == '_');
    if wraps_identifier && BRACKETS.contains(&(first, last)) && !BRACKETED_IDENTIFIERS.contains(&name) {
        return Some("is wrapped in brackets; capture the identifier inside instead");
    }
    None
}

/// `names`: every tag name is a clean identifier.
fn check_names(tags: &[Tag]) -> Vec<String> {
    tags.iter()
        .filter_map(|tag| name_problem(&tag.name).map(|problem| format!("{}: name {problem}", tag.describe())))
        .collect()
}

/// `definitions`: at least one definition tag.
fn check_definitions(tags: &[Tag]) -> Vec<String> {
    if tags.iter().any(|tag| tag.role.starts_with("definition.")) {
        Vec::new()
    } else {
        vec!["the fixture yields no definition.* tag".to_string()]
    }
}

/// Why two definitions of the same role and name are one entity tagged twice, if they are.
fn duplicate_reason(earlier: &Tag, later: &Tag) -> Option<&'static str> {
    if earlier.role != later.role || earlier.name != later.name || !earlier.role.starts_with("definition.") {
        return None;
    }
    if earlier.name_range.start == later.name_range.start {
        return Some("starts at the same position");
    }
    if earlier.parent.is_some() && earlier.parent == later.parent {
        return Some("has a definition node under the same parent (overload, extra clause or forward declaration)");
    }
    let overlaps = earlier.node_range.start < later.node_range.end && later.node_range.start < earlier.node_range.end;
    if overlaps {
        return Some("has an overlapping definition node (tagged by both a wrapper and the declaration)");
    }
    None
}

/// `duplicates`: no entity is defined twice; each later tag names the first tag it repeats.
fn check_duplicates(tags: &[Tag]) -> Vec<String> {
    let mut problems = Vec::new();
    for (index, later) in tags.iter().enumerate() {
        let repeated = tags[..index]
            .iter()
            .find_map(|earlier| duplicate_reason(earlier, later).map(|reason| (earlier, reason)));
        if let Some((earlier, reason)) = repeated {
            problems.push(format!(
                "{} repeats line {}: {reason}",
                later.describe(),
                earlier.row + 1
            ));
        }
    }
    problems
}

/// A query pattern as the dead-pattern check sees it.
#[derive(Debug, Clone, PartialEq, Eq)]
struct PatternInfo {
    /// Normalized source text, suffixed ` #<n>` for the n-th identical copy.
    identity: String,
    /// Whether the pattern carries a `definition.*` or `reference.*` capture.
    has_role: bool,
    /// How many emitted tags the pattern won.
    tags: usize,
}

/// A pattern's source text with comments dropped and whitespace runs collapsed to one space, so
/// reformatting or moving a pattern keeps its identity. String literals are kept verbatim.
fn normalize_pattern(text: &str) -> String {
    let mut out = String::new();
    let mut chars = text.chars().peekable();
    let mut pending_space = false;
    while let Some(c) = chars.next() {
        if c == ';' {
            while chars.next_if(|&next| next != '\n').is_some() {}
            pending_space = true;
            continue;
        }
        if c.is_whitespace() {
            pending_space = true;
            continue;
        }
        if pending_space && !out.is_empty() {
            out.push(' ');
        }
        pending_space = false;
        out.push(c);
        if c == '"' {
            while let Some(inner) = chars.next() {
                out.push(inner);
                if inner == '\\' {
                    out.extend(chars.next());
                } else if inner == '"' {
                    break;
                }
            }
        }
    }
    out
}

/// Each pattern of `query` with the number of `tags` it won.
fn pattern_infos(query: &Query, source: &str, tags: &[Tag]) -> Vec<PatternInfo> {
    let mut seen: BTreeMap<String, usize> = BTreeMap::new();
    (0..query.pattern_count())
        .map(|index| {
            let text = &source[query.start_byte_for_pattern(index)..query.end_byte_for_pattern(index)];
            let normalized = normalize_pattern(text);
            let copy = seen.entry(normalized.clone()).or_insert(0);
            *copy += 1;
            let identity = if *copy == 1 {
                normalized
            } else {
                format!("{normalized} #{copy}")
            };
            let has_role = query
                .capture_quantifiers(index)
                .iter()
                .zip(query.capture_names())
                .any(|(quantifier, name)| *quantifier != CaptureQuantifier::Zero && is_role(name));
            PatternInfo {
                identity,
                has_role,
                tags: tags.iter().filter(|tag| tag.pattern == index).count(),
            }
        })
        .collect()
}

/// `dead_patterns`: every role-bearing pattern wins a tag unless opted out, and every opt-out
/// names a role-bearing pattern that wins none.
fn check_dead_patterns(patterns: &[PatternInfo], opt_outs: &[PatternOptOut]) -> Vec<String> {
    let opted: BTreeSet<&str> = opt_outs.iter().map(|opt_out| opt_out.pattern.as_str()).collect();
    let mut problems = Vec::new();
    for pattern in patterns.iter().filter(|pattern| pattern.has_role) {
        let listed = opted.contains(pattern.identity.as_str());
        if pattern.tags == 0 && !listed {
            problems.push(format!(
                "pattern yields no tag on the fixture; extend the fixture, delete the pattern or list it in \
                 {UNEXERCISED_PATTERNS}: {}",
                pattern.identity
            ));
        } else if pattern.tags > 0 && listed {
            problems.push(format!(
                "pattern listed in {UNEXERCISED_PATTERNS} now yields {} tag(s); remove the entry: {}",
                pattern.tags, pattern.identity
            ));
        }
    }
    for opt_out in opt_outs {
        let exists = patterns
            .iter()
            .any(|pattern| pattern.has_role && pattern.identity == opt_out.pattern);
        if !exists {
            problems.push(format!(
                "{UNEXERCISED_PATTERNS} lists a pattern the query does not contain (or that carries no role); \
                 remove or update it: {}",
                opt_out.pattern
            ));
        }
    }
    let mut counts: BTreeMap<&str, usize> = BTreeMap::new();
    for opt_out in opt_outs {
        *counts.entry(opt_out.pattern.as_str()).or_default() += 1;
    }
    for (pattern, count) in counts.into_iter().filter(|(_, count)| *count > 1) {
        problems.push(format!(
            "{UNEXERCISED_PATTERNS} lists a pattern {count} times: {pattern}"
        ));
    }
    problems
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

/// One language's violations, check name to messages.
type Violations = BTreeMap<&'static str, Vec<String>>;

fn add(violations: &mut Violations, check: &'static str, problems: Vec<String>) {
    if !problems.is_empty() {
        violations.entry(check).or_default().extend(problems);
    }
}

/// Every check over one language's tags, independent of the fixture comparison.
fn tag_violations(tags: &[Tag], declared: &[&str], patterns: &[PatternInfo], opt_outs: &[PatternOptOut]) -> Violations {
    let mut violations = Violations::new();
    add(&mut violations, CHECK_ROLES, check_roles(tags, declared));
    add(&mut violations, CHECK_NAMES, check_names(tags));
    add(&mut violations, CHECK_DEFINITIONS, check_definitions(tags));
    add(
        &mut violations,
        CHECK_DEAD_PATTERNS,
        check_dead_patterns(patterns, opt_outs),
    );
    add(&mut violations, CHECK_DUPLICATES, check_duplicates(tags));
    violations
}

/// Checks one required language against its fixture; `None` when the fixture is missing, a gap
/// `every_language_requiring_tags_has_a_fixture` reports.
fn check_required(
    language: &str,
    grammar: &tree_sitter::Language,
    opt_outs: &[PatternOptOut],
    bless: bool,
) -> Option<Violations> {
    let dir = fixtures_root().join(language);
    if !dir.is_dir() {
        return None;
    }
    let fixture_error = |problem: String| Some(Violations::from([(CHECK_FIXTURE, vec![problem])]));
    let source_path = match fixture_source(&dir, !bless) {
        Ok(path) => path,
        Err(problem) => return fixture_error(problem),
    };
    let source = match std::fs::read_to_string(&source_path) {
        Ok(source) => source,
        Err(e) => return fixture_error(format!("read {}: {e}", source_path.display())),
    };
    let (query, query_source) = match tags_query(language) {
        Ok(query) => query,
        Err(problem) => return fixture_error(problem),
    };
    let tags = match extract_tags(&query, grammar, &source) {
        Ok(tags) => tags,
        Err(problem) => return fixture_error(problem),
    };
    let patterns = pattern_infos(&query, query_source, &tags);
    let mut violations = tag_violations(&tags, query.capture_names(), &patterns, opt_outs);
    let actual: Vec<String> = tags.iter().map(Tag::line).collect();
    add(&mut violations, CHECK_FIXTURE, compare_expected(&dir, &actual, bless));
    Some(violations)
}

/// Compares (or, when blessing, writes) `expected.tags`.
fn compare_expected(dir: &Path, actual: &[String], bless: bool) -> Vec<String> {
    let expected_path = dir.join(EXPECTED_FILE);
    if bless {
        let mut text = actual.join("\n");
        text.push('\n');
        return match std::fs::write(&expected_path, text) {
            Ok(()) => Vec::new(),
            Err(e) => vec![format!("write {}: {e}", expected_path.display())],
        };
    }
    let expected_text = match std::fs::read_to_string(&expected_path) {
        Ok(text) => text,
        Err(e) => return vec![format!("read {}: {e}", expected_path.display())],
    };
    let expected: Vec<String> = expected_text.lines().map(str::to_string).collect();
    if expected == actual {
        Vec::new()
    } else {
        vec![format!(
            "tags differ from {}:\n{}",
            expected_path.display(),
            describe_mismatch(&expected, actual)
        )]
    }
}

/// Writes `{"counts": {check: {languages, violations}}, "languages": {lang: {check: [..]}}}`.
fn write_report(path: &Path, report: &BTreeMap<String, Violations>) {
    let mut counts: BTreeMap<&str, (usize, usize)> = BTreeMap::new();
    for violations in report.values() {
        for (check, problems) in violations {
            let entry = counts.entry(check).or_default();
            entry.0 += 1;
            entry.1 += problems.len();
        }
    }
    let counts: serde_json::Map<String, serde_json::Value> = counts
        .into_iter()
        .map(|(check, (languages, violations))| {
            (
                check.to_string(),
                serde_json::json!({"languages": languages, "violations": violations}),
            )
        })
        .collect();
    let document = serde_json::json!({"counts": counts, "languages": report});
    let text = serde_json::to_string_pretty(&document).expect("serialize the tags report");
    std::fs::write(path, text + "\n").unwrap_or_else(|e| panic!("write {}: {e}", path.display()));
}

#[test]
fn tags_coverage_names_real_languages_with_reasons_and_no_tags() {
    let languages = all_languages();
    let not_applicable = not_applicable();
    let grammar_blocked = grammar_blocked();
    let mut problems = Vec::new();

    for (section, entries) in [(NOT_APPLICABLE, &not_applicable), (GRAMMAR_BLOCKED, &grammar_blocked)] {
        for (language, reason) in entries {
            if !languages.contains(language) {
                problems.push(format!(
                    "{language}: {section} names no language in language_definitions.json"
                ));
            }
            if reason.trim().is_empty() {
                problems.push(format!("{language}: {section} needs a reason"));
            }
            if fixtures_root().join(language).exists() {
                problems.push(format!("{language}: {section} but has a tags fixture directory"));
            }
            if repo_root()
                .join("query-overlays")
                .join(language)
                .join("tags.scm")
                .exists()
            {
                problems.push(format!(
                    "{language}: {section} but has query-overlays/{language}/tags.scm"
                ));
            }
        }
    }
    for language in grammar_blocked.keys().filter(|l| not_applicable.contains_key(*l)) {
        problems.push(format!(
            "{language}: listed both {NOT_APPLICABLE} and {GRAMMAR_BLOCKED}; tags either apply or do not"
        ));
    }

    let (opt_outs, opt_out_problems) = unexercised_patterns();
    problems.extend(opt_out_problems);
    for language in opt_outs.keys() {
        if !fixtures_root().join(language).is_dir() {
            problems.push(format!(
                "{language}: {UNEXERCISED_PATTERNS} lists a language with no tags fixture"
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
    let grammar_blocked = grammar_blocked();
    let mut gaps: Vec<String> = all_languages()
        .into_iter()
        .filter(|language| {
            !not_applicable.contains_key(language)
                && !grammar_blocked.contains_key(language)
                && !fixtures_root().join(language).is_dir()
        })
        .collect();
    gaps.sort();
    assert!(
        gaps.is_empty(),
        "{} language(s) need a tags.scm and a fixture under tests/fixtures/tags/<lang>/, or a \
         not_applicable or grammar_blocked entry with a reason in sources/tags_coverage.json:\n{}",
        gaps.len(),
        gaps.join("\n")
    );
}

/// A listed language (`not_applicable` or `grammar_blocked`) must bundle no tags query.
fn listed_language_problem(language: &str, section: &str) -> Option<String> {
    match get_query(language, QueryKind::Tags) {
        Ok(None) => None,
        _ if section == NOT_APPLICABLE => Some(format!(
            "listed {NOT_APPLICABLE} in sources/tags_coverage.json but bundles a tags.scm; \
             remove the entry and add a fixture"
        )),
        _ => Some(format!(
            "listed {GRAMMAR_BLOCKED} in sources/tags_coverage.json but bundles a tags.scm; check whether \
             the grammar now yields correct names, then remove the entry and add a fixture"
        )),
    }
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
    let grammar_blocked = grammar_blocked();
    let (opt_outs, _) = unexercised_patterns();
    let bless = std::env::var_os("TSLP_TAGS_BLESS").is_some();

    let mut report: BTreeMap<String, Violations> = BTreeMap::new();
    let mut checked = 0usize;
    let mut deferred = Vec::new();
    for language in &built {
        let section = if not_applicable.contains_key(language) {
            Some(NOT_APPLICABLE)
        } else if grammar_blocked.contains_key(language) {
            Some(GRAMMAR_BLOCKED)
        } else {
            None
        };
        if let Some(section) = section {
            checked += 1;
            if let Some(problem) = listed_language_problem(language, section) {
                report.insert(language.clone(), Violations::from([(CHECK_FIXTURE, vec![problem])]));
            }
            continue;
        }
        let grammar = registry
            .get_language(language)
            .unwrap_or_else(|e| panic!("{language} is listed as built but does not load: {e}"));
        let language_opt_outs = opt_outs.get(language).map_or(&[][..], Vec::as_slice);
        match check_required(language, &grammar, language_opt_outs, bless) {
            None => deferred.push(language.as_str()),
            Some(violations) => {
                checked += 1;
                if !violations.is_empty() {
                    report.insert(language.clone(), violations);
                }
            }
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
    if let Some(path) = std::env::var_os("TSLP_TAGS_REPORT") {
        write_report(Path::new(&path), &report);
        eprintln!(
            "tags coverage: wrote the violation report to {}",
            Path::new(&path).display()
        );
    }
    if option_env!("TSLP_LANGUAGES").is_some_and(|value| !value.is_empty()) {
        assert!(
            checked > 0,
            "TSLP_LANGUAGES was set at build time but no built language was checked; built: {built:?}"
        );
    }
    let mut failures = Vec::new();
    for (language, violations) in &report {
        for (check, problems) in violations {
            for problem in problems {
                failures.push(format!("{language} [{check}]: {problem}"));
            }
        }
    }
    assert!(
        failures.is_empty(),
        "{} language(s) fail their tags checks ({} violation(s)):\n{}",
        report.len(),
        failures.len(),
        failures.join("\n")
    );
}

/// Negative controls: each check fires on a synthetic tag list built to violate it, and stays
/// quiet on the closest legitimate shape, without loading any grammar.
mod checks {
    use super::*;

    fn tag(role: &str, name: &str, name_start: usize, node: Range<usize>, parent: Option<usize>) -> Tag {
        Tag {
            role: role.to_string(),
            name: name.to_string(),
            name_range: name_start..name_start + name.len(),
            row: name_start,
            node_range: node,
            parent,
            pattern: 0,
        }
    }

    fn definition(name: &str, name_start: usize) -> Tag {
        tag(
            "definition.function",
            name,
            name_start,
            name_start..name_start + 10,
            Some(1),
        )
    }

    fn pattern(identity: &str, has_role: bool, tags: usize) -> PatternInfo {
        PatternInfo {
            identity: identity.to_string(),
            has_role,
            tags,
        }
    }

    fn opt_out(pattern: &str) -> PatternOptOut {
        PatternOptOut {
            pattern: pattern.to_string(),
            reason: "kept for a form the fixture cannot show".to_string(),
        }
    }

    #[test]
    fn roles_reject_unlisted_emitted_and_declared_roles() {
        let tags = [definition("f", 0), tag("reference.send", "g", 20, 20..30, Some(1))];
        let problems = check_roles(
            &tags,
            &["name", "definition.function", "definition.struct", "reference.send"],
        );
        assert_eq!(problems.len(), 3, "{problems:#?}");
        assert!(problems[0].contains("@definition.struct"));
        assert!(problems[1].contains("@reference.send"));
        assert!(problems[2].contains("reference.send \"g\""));
    }

    #[test]
    fn roles_accept_every_allowed_role() {
        let tags: Vec<Tag> = ALLOWED_ROLES
            .iter()
            .enumerate()
            .map(|(index, role)| tag(role, "x", index * 10, index * 10..index * 10 + 5, None))
            .collect();
        assert_eq!(check_roles(&tags, ALLOWED_ROLES), Vec::<String>::new());
    }

    #[test]
    fn names_reject_whitespace_quotes_and_brackets() {
        let bad = [
            " lead",
            "trail ",
            "two\nlines",
            "a\tb",
            "\"quoted\"",
            "'single'",
            "`tick`",
            "(paren)",
            "[list]",
            "{brace}",
            "<angle>",
        ];
        let tags: Vec<Tag> = bad
            .iter()
            .enumerate()
            .map(|(i, name)| definition(name, i * 20))
            .collect();
        let problems = check_names(&tags);
        assert_eq!(problems.len(), bad.len(), "{problems:#?}");
    }

    #[test]
    fn names_accept_identifiers_sigils_and_operators() {
        let good = [
            "foo", "$var", "@f", "%reg", "<>", "[]", "()", "<=>", "x", "\"", "it's", "Foo.bar", "<init>", "<clinit>",
        ];
        let tags: Vec<Tag> = good
            .iter()
            .enumerate()
            .map(|(i, name)| definition(name, i * 20))
            .collect();
        assert_eq!(check_names(&tags), Vec::<String>::new());
    }

    #[test]
    fn definitions_require_at_least_one_definition() {
        let references = [tag("reference.call", "f", 0, 0..5, None)];
        assert_eq!(check_definitions(&references).len(), 1);
        assert_eq!(check_definitions(&[]).len(), 1);
        assert!(check_definitions(&[definition("f", 0)]).is_empty());
    }

    #[test]
    fn duplicates_reject_same_start_same_parent_and_overlap() {
        let same_start = [definition("f", 0), definition("f", 0)];
        assert_eq!(check_duplicates(&same_start).len(), 1);

        let same_parent = [
            tag("definition.function", "f", 0, 0..10, Some(7)),
            tag("definition.function", "f", 20, 20..30, Some(7)),
            tag("definition.function", "f", 40, 40..50, Some(7)),
        ];
        let problems = check_duplicates(&same_parent);
        assert_eq!(problems.len(), 2, "{problems:#?}");
        assert!(problems.iter().all(|p| p.contains("repeats line 1")), "{problems:#?}");

        let nested = [
            tag("definition.function", "f", 6, 0..40, Some(1)),
            tag("definition.function", "f", 20, 12..40, Some(2)),
        ];
        assert_eq!(check_duplicates(&nested).len(), 1);
    }

    #[test]
    fn duplicates_accept_other_scopes_roles_and_references() {
        let tags = [
            tag("definition.method", "area", 0, 0..10, Some(1)),
            tag("definition.method", "area", 20, 20..30, Some(2)),
            tag("definition.class", "f", 40, 40..50, Some(3)),
            tag("definition.function", "f", 60, 60..70, Some(3)),
            tag("reference.call", "g", 80, 80..85, Some(3)),
            tag("reference.call", "g", 90, 90..95, Some(3)),
            tag("definition.variable", "x", 100, 100..105, None),
            tag("definition.variable", "x", 110, 110..115, None),
        ];
        assert_eq!(check_duplicates(&tags), Vec::<String>::new());
    }

    #[test]
    fn dead_patterns_require_every_role_pattern_to_fire() {
        let patterns = [
            pattern("(a) @definition.function", true, 0),
            pattern("(b) @ignore", false, 0),
        ];
        let problems = check_dead_patterns(&patterns, &[]);
        assert_eq!(problems.len(), 1, "{problems:#?}");
        assert!(problems[0].contains("(a) @definition.function"));
    }

    #[test]
    fn dead_patterns_honor_opt_outs_in_both_directions() {
        let patterns = [
            pattern("(a) @definition.function", true, 0),
            pattern("(b) @reference.call", true, 2),
        ];
        assert!(check_dead_patterns(&patterns, &[opt_out("(a) @definition.function")]).is_empty());

        let fired = check_dead_patterns(
            &patterns,
            &[opt_out("(a) @definition.function"), opt_out("(b) @reference.call")],
        );
        assert_eq!(fired.len(), 1, "{fired:#?}");
        assert!(fired[0].contains("now yields 2 tag(s)"));

        let stale = check_dead_patterns(
            &patterns,
            &[opt_out("(a) @definition.function"), opt_out("(gone) @definition.class")],
        );
        assert_eq!(stale.len(), 1, "{stale:#?}");
        assert!(stale[0].contains("does not contain"));

        let repeated = check_dead_patterns(
            &patterns,
            &[opt_out("(a) @definition.function"), opt_out("(a) @definition.function")],
        );
        assert_eq!(repeated.len(), 1, "{repeated:#?}");
    }

    #[test]
    fn normalized_patterns_ignore_layout_and_comments_but_keep_strings() {
        let pattern = "(function_definition ; a comment\n    name: (identifier) @name\n  (#eq? @name \"a  ; b\")) @definition.function\n; trailing\n";
        assert_eq!(
            normalize_pattern(pattern),
            "(function_definition name: (identifier) @name (#eq? @name \"a  ; b\")) @definition.function"
        );
        assert_eq!(normalize_pattern("(a \"x\\\"y\")"), "(a \"x\\\"y\")");
    }

    #[test]
    fn tag_violations_group_by_check() {
        let tags = [tag("definition.struct", " S", 0, 0..10, Some(1))];
        let patterns = [
            pattern("(s) @definition.struct", true, 1),
            pattern("(t) @reference.type", true, 0),
        ];
        let violations = tag_violations(&tags, &["definition.struct", "reference.type", "name"], &patterns, &[]);
        let checks: Vec<&str> = violations.keys().copied().collect();
        assert_eq!(checks, [CHECK_DEAD_PATTERNS, CHECK_NAMES, CHECK_ROLES]);

        let references = [tag("reference.call", "f", 0, 0..5, None)];
        let violations = tag_violations(&references, &["reference.call", "name"], &[], &[]);
        assert_eq!(violations.keys().copied().collect::<Vec<_>>(), [CHECK_DEFINITIONS]);
    }
}
