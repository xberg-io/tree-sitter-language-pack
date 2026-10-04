#![allow(clippy::unwrap_used, clippy::expect_used)] // ~keep: a failed setup step in a test must abort loudly
use tree_sitter_language_pack::{LanguageRegistry, get_parser};

const SAMPLE: &str = "class Person {\n  name string\n  age int\n}\n\nenum Level {\n  Junior\n  Senior\n}\n\nfunction Greet(p: Person) -> string {\n  p.name\n}\n";

fn require_baml() {
    // ~keep This tree builds with ZERO grammars unless TSLP_LANGUAGES is set; without this guard
    // ~keep a build lacking baml would fail on the missing grammar instead of the behaviour under test.
    assert!(
        LanguageRegistry::new()
            .available_languages()
            .iter()
            .any(|l| l == "baml"),
        "`baml` is not built into this binary, so this test proves nothing; set TSLP_LANGUAGES=baml"
    );
}

#[test]
fn should_parse_baml_declarations_without_errors() {
    require_baml();
    let tree = get_parser("baml")
        .expect("baml parser")
        .parse(SAMPLE)
        .expect("parse returned a tree");
    let root = tree.root_node();
    assert_eq!(root.kind(), "source_file");
    assert!(!root.has_error(), "baml sample parsed with errors: {}", root.to_sexp());
    let kinds: Vec<String> = (0..root.child_count())
        .filter_map(|i| root.child(i as u32))
        .map(|n| n.kind())
        .collect();
    assert_eq!(kinds, ["class_declaration", "enum_declaration", "function_declaration"]);
}
