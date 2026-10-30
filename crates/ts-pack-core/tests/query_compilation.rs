//! Compiles every bundled query (`highlights.scm`, `injections.scm`, `locals.scm`,
//! `tags.scm`, `indents.scm`, `folds.scm`) for every language actually built into
//! this binary, so a grammar bump that silently invalidates a query — vendored or
//! a hand-written `query-overlays/` overlay — fails a test instead of shipping.
//!
//! Requires `TSLP_LANGUAGES` (or the `download` feature with a populated cache) at
//! build time: this crate links zero grammars by default, and a naive sweep over an
//! empty language set would pass vacuously. Run with, for example:
//! `TSLP_LANGUAGES=python,rust,dart,java,kotlin,php,scala,cpp,swift,csharp,ruby cargo test \
//!  -p tree-sitter-language-pack --test query_compilation`

// ~keep Integration tests are separate crates and do not inherit the library's crate-root
// ~keep allow, so the skip notice below needs its own opt-in to the workspace print_stderr deny.
#![allow(clippy::print_stdout, clippy::print_stderr, clippy::dbg_macro)]

use tree_sitter_language_pack::{LanguageRegistry, QueryKind, get_query};

/// Languages with a hand-written `query-overlays/{lang}/tags.scm` that overrides the
/// vendored file (see `crates/ts-pack-core/build.rs::effective_query_path`). A grammar
/// bump cannot regenerate these — they must be hand-fixed, so their compile failures
/// are asserted separately from the bulk sweep. ~keep
const OVERLAY_TAGS_LANGUAGES: [&str; 9] = [
    "dart", "java", "kotlin", "php", "scala", "cpp", "swift", "csharp", "ruby",
];

/// Queries that fail to compile because of a defect in the query file UPSTREAM ships, not in
/// this repo's selection or overlays. Each was traced to the upstream commit that introduced it,
/// and in each case we are already pinned at upstream HEAD with no better candidate to select.
///
/// This list is enforced in BOTH directions: an entry that starts compiling fails the test just
/// as loudly as an unlisted failure. A known-failures list nobody is forced to revisit is how a
/// gate rots into a green light that checks nothing. ~keep
const KNOWN_UPSTREAM_BROKEN: [(&str, QueryKind, &str); 3] = [
    (
        "flatbuffers",
        QueryKind::Highlights,
        "upstream typo: grammar.js defines `enum_val_decl`, the query says `enumval_decl`",
    ),
    (
        "solidity",
        QueryKind::Highlights,
        "upstream query text is malformed s-expression syntax; vendored bytes are identical to upstream and no patch of ours touches it",
    ),
    (
        "vhdl",
        QueryKind::Highlights,
        "upstream query expects an `integer` child inside `integer_decimal`, which `token(...)` collapses to a childless token",
    ),
];

fn known_upstream_breakage(language: &str, kind: QueryKind) -> Option<&'static str> {
    KNOWN_UPSTREAM_BROKEN
        .iter()
        .find(|(lang, broken_kind, _)| *lang == language && *broken_kind == kind)
        .map(|(_, _, reason)| *reason)
}

const ALL_QUERY_KINDS: [QueryKind; 6] = [
    QueryKind::Highlights,
    QueryKind::Injections,
    QueryKind::Locals,
    QueryKind::Tags,
    QueryKind::Indents,
    QueryKind::Folds,
];

/// One accumulated failure: which language, which query kind, and the tree-sitter error.
struct QueryFailure {
    language: String,
    kind: QueryKind,
    error: String,
}

impl std::fmt::Display for QueryFailure {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} / {:?}: {}", self.language, self.kind, self.error)
    }
}

/// Compiles every bundled query kind for `language`, returning the count of queries that
/// actually compiled (kind bundled and `Query::new` succeeded) and appending any compile
/// failure to `failures`. A bundled-but-absent kind (`Ok(None)`) is not a failure.
fn compile_all_kinds_for(language: &str, failures: &mut Vec<QueryFailure>, repaired: &mut Vec<String>) -> usize {
    let mut compiled = 0usize;
    for kind in ALL_QUERY_KINDS {
        let known = known_upstream_breakage(language, kind);
        match (get_query(language, kind), known) {
            // ~keep Upstream fixed it, or a pin bump pulled the fix in. Say so and demand the
            // ~keep entry be deleted, rather than letting the list quietly outlive the defect.
            (Ok(_), Some(reason)) => repaired.push(format!("{language} / {kind:?} — listed as: {reason}")),
            (Ok(Some(_)), None) => compiled += 1,
            (Ok(None), None) => {}
            (Err(_), Some(_)) => {}
            (Err(error), None) => failures.push(QueryFailure {
                language: language.to_string(),
                kind,
                error: error.to_string(),
            }),
        }
    }
    compiled
}

#[test]
fn should_compile_every_bundled_query_for_every_language_built_into_this_binary() {
    let registry = LanguageRegistry::new();
    let languages = registry.available_languages();

    // ~keep The single most important assertion here: a build with zero grammars linked
    // ~keep (no TSLP_LANGUAGES, no download cache) makes every loop below a no-op, and a
    // ~keep no-op sweep is indistinguishable from a passing one unless it fails loudly.
    assert!(
        !languages.is_empty(),
        "no languages built into this binary — set TSLP_LANGUAGES (e.g. \
         TSLP_LANGUAGES=python,rust,dart,java,kotlin,php,scala,cpp,swift,csharp,ruby) and rebuild \
         before running this test, otherwise it vacuously passes"
    );

    let mut failures: Vec<QueryFailure> = Vec::new();
    let mut repaired: Vec<String> = Vec::new();
    let mut total_compiled = 0usize;
    for language in &languages {
        total_compiled += compile_all_kinds_for(language, &mut failures, &mut repaired);
    }

    assert!(
        repaired.is_empty(),
        "{} quer{} listed in KNOWN_UPSTREAM_BROKEN now compile(s). Upstream fixed this (or a pin \
         bump pulled the fix in) — delete the entr{} so the list keeps meaning what it says:\n{}",
        repaired.len(),
        if repaired.len() == 1 { "y" } else { "ies" },
        if repaired.len() == 1 { "y" } else { "ies" },
        repaired.join("\n")
    );

    assert!(
        total_compiled > 0,
        "{} languages were built ({languages:?}) but zero bundled queries compiled — \
         either no .scm files are bundled for any of them, or every one failed to compile \
         (see failures below)\nfailures:\n{}",
        languages.len(),
        failures.iter().map(ToString::to_string).collect::<Vec<_>>().join("\n")
    );

    assert!(
        failures.is_empty(),
        "{} bundled quer{} failed to compile:\n{}",
        failures.len(),
        if failures.len() == 1 { "y" } else { "ies" },
        failures.iter().map(ToString::to_string).collect::<Vec<_>>().join("\n")
    );
}

#[test]
fn should_compile_the_hand_written_tags_overlay_for_every_overlay_language_built_into_this_binary() {
    let registry = LanguageRegistry::new();

    let mut present_overlay_languages: Vec<&str> = Vec::new();
    let mut failures: Vec<QueryFailure> = Vec::new();

    for &language in &OVERLAY_TAGS_LANGUAGES {
        if registry.get_language(language).is_err() {
            // ~keep Not built in this run; matches the skip-per-entry convention used by
            // ~keep intel::tests::has_grammar for partial `TSLP_LANGUAGES` builds.
            continue;
        }
        present_overlay_languages.push(language);

        match get_query(language, QueryKind::Tags) {
            Ok(Some(_)) => {}
            Ok(None) => failures.push(QueryFailure {
                language: language.to_string(),
                kind: QueryKind::Tags,
                error: "expected a hand-written query-overlays/<lang>/tags.scm but none is bundled".to_string(),
            }),
            Err(error) => failures.push(QueryFailure {
                language: language.to_string(),
                kind: QueryKind::Tags,
                error: error.to_string(),
            }),
        }
    }

    if present_overlay_languages.is_empty() {
        eprintln!(
            "SKIPPED should_compile_the_hand_written_tags_overlay_for_every_overlay_language_built_into_this_binary: \
             none of {OVERLAY_TAGS_LANGUAGES:?} are built into this binary — set TSLP_LANGUAGES to include at least \
             one of them to exercise the hand-written overlays"
        );
        return;
    }

    assert!(
        failures.is_empty(),
        "{} of {} built overlay language(s) {present_overlay_languages:?} failed to compile their hand-written \
         tags.scm overlay:\n{}",
        failures.len(),
        present_overlay_languages.len(),
        failures.iter().map(ToString::to_string).collect::<Vec<_>>().join("\n")
    );
}

#[test]
fn should_capture_abl_directive_keywords_without_their_names_values_or_bodies() {
    use tree_sitter::{Parser, QueryCursor, StreamingIterator};

    let Ok(language) = LanguageRegistry::new().get_language("abl") else {
        eprintln!("SKIPPED ABL directive captures: build with TSLP_LANGUAGES=abl");
        return;
    };
    let query = get_query("abl", QueryKind::Highlights)
        .expect("ABL highlights must compile")
        .expect("ABL must bundle highlights");
    let fixtures: [(&str, &[&str]); 11] = [
        ("&gLoBaL-DeFiNe FLAG 1\n", &["&gLoBaL-DeFiNe"]),
        ("&ScOpEd-DeFiNe FLAG 1\n", &["&ScOpEd-DeFiNe"]),
        ("&UnDeFiNe FLAG\n", &["&UnDeFiNe"]),
        ("&MeSsAgE hello\n", &["&MeSsAgE"]),
        ("&AnAlYzE-SuSpEnD _UIB-CODE-BLOCK\n", &["&AnAlYzE-SuSpEnD"]),
        ("&AnAlYzE-ReSuMe metadata\n", &["&AnAlYzE-ReSuMe"]),
        ("&iF TRUE\n", &["&iF"]),
        ("&tHeN\n", &["&tHeN"]),
        ("&eLsEiF FALSE\n", &["&eLsEiF"]),
        ("&eLsE\n", &["&eLsE"]),
        ("&eNdIf\n", &["&eNdIf"]),
    ];
    let mut parser = Parser::new();
    parser.set_language(&language).expect("ABL must load");
    for (source, expected) in fixtures {
        let tree = parser.parse(source, None).expect("fixture must parse");
        assert!(!tree.root_node().has_error(), "fixture needs recovery: {source}");
        let mut cursor = QueryCursor::new();
        let mut matches = cursor.matches(&query, tree.root_node(), source.as_bytes());
        let mut captured = Vec::new();
        while let Some(found) = matches.next() {
            for capture in found.captures() {
                if query.capture_names()[capture.index as usize] == "keyword.directive" {
                    captured.push(&source[capture.node.byte_range()]);
                }
            }
        }
        assert_eq!(captured, expected, "directive spans for {source}");
    }
}
