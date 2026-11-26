//! Runs each hand-written `query-overlays/<lang>/tags.scm` over a fixture in `tests/fixtures/tags/`
//! and asserts the exact tags it yields, in match order, so a pattern that compiles but matches
//! nothing (or the wrong node) fails instead of shipping.
//!
//! Requires the languages at build time, for example:
//! `TSLP_LANGUAGES=sql,bash,scss,hcl,terraform,starlark cargo test -p tree-sitter-language-pack \
//!  --test tags_overlays`

// ~keep Integration tests do not inherit the library's crate-root allow: the skip notice needs
// ~keep print_stderr, and a failed setup step in a test must abort loudly.
#![allow(clippy::print_stderr, clippy::expect_used)]

use tree_sitter::{Parser, QueryCursor, StreamingIterator};
use tree_sitter_language_pack::{LanguageRegistry, QueryKind, get_query};

/// Collects one `(role, text of @name)` per match, where the role is the `definition.<kind>` or
/// `reference.<kind>` capture of that match. Returns `None` when `language` is not built in.
fn tags(language: &str, fixture: &str) -> Option<Vec<(String, String)>> {
    let Ok(grammar) = LanguageRegistry::new().get_language(language) else {
        eprintln!("SKIPPED {language} tags: build with TSLP_LANGUAGES including {language}");
        return None;
    };
    let path = format!("{}/tests/fixtures/tags/{fixture}", env!("CARGO_MANIFEST_DIR"));
    let source = std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("read {path}: {e}"));
    let query = get_query(language, QueryKind::Tags)
        .unwrap_or_else(|e| panic!("{language} tags must compile: {e}"))
        .unwrap_or_else(|| panic!("{language} must bundle tags.scm"));

    let mut parser = Parser::new();
    parser.set_language(&grammar).expect("grammar must load");
    let tree = parser.parse(&source, None).expect("fixture must parse");
    assert!(
        !tree.root_node().has_error(),
        "{fixture} needs error recovery under {language}: {}",
        tree.root_node().to_sexp()
    );

    let mut cursor = QueryCursor::new();
    let mut matches = cursor.matches(&query, tree.root_node(), source.as_bytes());
    let mut found = Vec::new();
    while let Some(m) = matches.next() {
        let mut role = None;
        let mut name = None;
        for capture in m.captures() {
            let capture_name = query.capture_names()[capture.index as usize];
            if capture_name == "name" {
                name = Some(source[capture.node.byte_range()].to_string());
            } else if capture_name.starts_with("definition.") || capture_name.starts_with("reference.") {
                role = Some(capture_name.to_string());
            }
        }
        let role = role.unwrap_or_else(|| panic!("{language} match without a definition/reference capture"));
        let name = name.unwrap_or_else(|| panic!("{language} match without a @name capture"));
        found.push((role, name));
    }
    Some(found)
}

fn expected(pairs: &[(&str, &str)]) -> Vec<(String, String)> {
    pairs
        .iter()
        .map(|(role, name)| ((*role).to_string(), (*name).to_string()))
        .collect()
}

#[test]
fn should_tag_sql_create_statements_by_their_unqualified_names() {
    let Some(found) = tags("sql", "schema.sql") else { return };
    assert_eq!(
        found,
        expected(&[
            ("definition.module", "billing"),
            ("definition.struct", "users"),
            ("definition.type", "active_users"),
            ("definition.type", "user_counts"),
            ("definition.type", "mood"),
            ("definition.function", "add_one"),
            ("definition.variable", "users_email_idx"),
            ("definition.variable", "order_seq"),
            ("definition.function", "audit_users"),
        ])
    );
}

#[test]
fn should_tag_bash_functions_and_only_top_level_variables() {
    let Some(found) = tags("bash", "deploy.sh") else { return };
    assert_eq!(
        found,
        expected(&[
            ("definition.variable", "PREFIX"),
            ("definition.variable", "PATH_EXTRA"),
            ("definition.variable", "VERSION"),
            ("definition.variable", "A"),
            ("definition.variable", "B"),
            ("definition.function", "build"),
            ("definition.function", "deploy"),
        ])
    );
}

#[test]
fn should_tag_scss_top_level_variables_mixins_functions_and_placeholders() {
    let Some(found) = tags("scss", "theme.scss") else {
        return;
    };
    assert_eq!(
        found,
        expected(&[
            ("definition.variable", "$primary"),
            ("definition.macro", "button"),
            ("definition.function", "double"),
            ("definition.class", "%message-shared"),
            ("definition.class", "%notice"),
        ])
    );
}

const TERRAFORM_MAIN_TAGS: [(&str, &str); 9] = [
    ("definition.module", "terraform"),
    ("definition.module", "aws"),
    ("definition.class", "logs"),
    ("definition.class", "ubuntu"),
    ("definition.variable", "env"),
    ("definition.constant", "bucket_arn"),
    ("definition.module", "vpc"),
    ("definition.variable", "name_prefix"),
    ("definition.variable", "tags"),
];

#[test]
fn should_tag_terraform_top_level_blocks_and_locals() {
    let Some(found) = tags("terraform", "main.tf") else {
        return;
    };
    assert_eq!(found, expected(&TERRAFORM_MAIN_TAGS));
}

#[test]
fn should_tag_terraform_source_identically_under_the_hcl_grammar() {
    let Some(found) = tags("hcl", "main.tf") else { return };
    assert_eq!(found, expected(&TERRAFORM_MAIN_TAGS));
}

#[test]
fn should_tag_terragrunt_hcl_includes_dependencies_locals_and_inputs() {
    let Some(found) = tags("hcl", "terragrunt.hcl") else {
        return;
    };
    assert_eq!(
        found,
        expected(&[
            ("definition.module", "include"),
            ("definition.module", "root"),
            ("definition.module", "vpc"),
            ("definition.variable", "region"),
            ("definition.variable", "inputs"),
        ])
    );
}

#[test]
fn should_tag_starlark_top_level_defs_assignments_targets_and_loads() {
    let Some(found) = tags("starlark", "BUILD.bzl") else {
        return;
    };
    assert_eq!(
        found,
        expected(&[
            ("reference.module", "//tools:defs.bzl"),
            ("definition.variable", "DEPS"),
            ("definition.function", "my_macro"),
            ("definition.class", "lib"),
            ("definition.variable", "target"),
            ("definition.class", "gen"),
        ])
    );
}
