use tree_sitter_language_pack::{Parser, get_parser};

const PY_SRC: &str = "def foo(): pass";
const INVALID_PY_SRC: &str = "def (((";

#[test]
fn parses_source_after_manual_set_language() {
    let mut parser = Parser::new();
    parser.set_language("python").expect("set_language succeeded");
    let tree = parser.parse(PY_SRC).expect("parse returned Some");
    assert_eq!(tree.root_node().kind(), "module");
}

#[test]
fn returns_none_when_no_language_set() {
    let mut parser = Parser::new();
    let result = parser.parse("x = 1");
    assert!(result.is_none());
}

#[test]
fn get_parser_convenience_produces_ready_parser() {
    let mut parser = get_parser("python").expect("get_parser succeeded");
    let tree = parser.parse(PY_SRC).expect("parse returned Some");
    assert_eq!(tree.root_node().kind(), "module");
}

#[test]
fn tree_root_node_kind_is_module_for_python() {
    let tree = get_parser("python")
        .unwrap()
        .parse(PY_SRC)
        .expect("parse returned Some");
    assert_eq!(tree.root_node().kind(), "module");
}

#[test]
fn tree_clone_is_cheap_and_shares_content() {
    let tree = get_parser("python")
        .unwrap()
        .parse(PY_SRC)
        .expect("parse returned Some");
    let cloned = tree.clone();
    assert_eq!(tree.root_node().kind(), cloned.root_node().kind());
}

#[test]
fn cursor_walk_goto_first_child_succeeds_on_nonempty_tree() {
    let tree = get_parser("python")
        .unwrap()
        .parse(PY_SRC)
        .expect("parse returned Some");
    let mut cursor = tree.walk();
    let moved = cursor.goto_first_child();
    assert!(moved);
}

#[test]
fn cursor_goto_next_sibling_and_goto_parent_round_trip() {
    let tree = get_parser("python")
        .unwrap()
        .parse(PY_SRC)
        .expect("parse returned Some");
    let mut cursor = tree.walk();
    cursor.goto_first_child();
    let went_up = cursor.goto_parent();
    assert!(went_up);
    assert_eq!(cursor.node().kind(), "module");
}

#[test]
fn cursor_field_name_is_some_on_named_field_child() {
    let tree = get_parser("python")
        .unwrap()
        .parse(PY_SRC)
        .expect("parse returned Some");
    let mut cursor = tree.walk();
    cursor.goto_first_child();
    cursor.goto_first_child();

    let mut found_field = false;
    loop {
        if cursor.field_name().is_some() {
            found_field = true;
            break;
        }
        if !cursor.goto_next_sibling() {
            break;
        }
    }
    assert!(found_field);
}

#[test]
fn node_kind_id_is_nonzero_for_named_node() {
    let tree = get_parser("python")
        .unwrap()
        .parse(PY_SRC)
        .expect("parse returned Some");
    let root = tree.root_node();
    assert!(root.kind_id() > 0);
}

#[test]
fn node_start_and_end_bytes_span_full_source() {
    let tree = get_parser("python")
        .unwrap()
        .parse(PY_SRC)
        .expect("parse returned Some");
    let root = tree.root_node();
    assert_eq!(root.start_byte(), 0);
    assert_eq!(root.end_byte(), PY_SRC.len());
}

#[test]
fn node_byte_range_matches_start_and_end_bytes() {
    let tree = get_parser("python")
        .unwrap()
        .parse(PY_SRC)
        .expect("parse returned Some");
    let root = tree.root_node();
    let range = root.byte_range();
    assert_eq!(range.start, root.start_byte());
    assert_eq!(range.end, root.end_byte());
}

#[test]
fn node_start_position_is_row_zero_col_zero_for_root() {
    let tree = get_parser("python")
        .unwrap()
        .parse(PY_SRC)
        .expect("parse returned Some");
    let root = tree.root_node();
    let pos = root.start_position();
    assert_eq!(pos.row, 0);
    assert_eq!(pos.column, 0);
}

#[test]
fn node_end_position_row_is_zero_for_single_line() {
    let tree = get_parser("python")
        .unwrap()
        .parse(PY_SRC)
        .expect("parse returned Some");
    let root = tree.root_node();
    let pos = root.end_position();
    assert_eq!(pos.row, 0);
}

#[test]
fn root_node_is_named() {
    let tree = get_parser("python")
        .unwrap()
        .parse(PY_SRC)
        .expect("parse returned Some");
    assert!(tree.root_node().is_named());
}

#[test]
fn root_node_of_valid_source_is_not_error() {
    let tree = get_parser("python")
        .unwrap()
        .parse(PY_SRC)
        .expect("parse returned Some");
    assert!(!tree.root_node().is_error());
}

#[test]
fn root_node_of_valid_source_is_not_missing() {
    let tree = get_parser("python")
        .unwrap()
        .parse(PY_SRC)
        .expect("parse returned Some");
    assert!(!tree.root_node().is_missing());
}

#[test]
fn root_node_of_valid_source_is_not_extra() {
    let tree = get_parser("python")
        .unwrap()
        .parse(PY_SRC)
        .expect("parse returned Some");
    assert!(!tree.root_node().is_extra());
}

#[test]
fn root_node_of_valid_source_has_no_error() {
    let tree = get_parser("python")
        .unwrap()
        .parse(PY_SRC)
        .expect("parse returned Some");
    assert!(!tree.root_node().has_error());
}

#[test]
fn root_node_of_invalid_source_has_error() {
    let tree = get_parser("python")
        .unwrap()
        .parse(INVALID_PY_SRC)
        .expect("parse returned Some even for invalid input");
    assert!(tree.root_node().has_error());
}

#[test]
fn root_node_has_at_least_one_child() {
    let tree = get_parser("python")
        .unwrap()
        .parse(PY_SRC)
        .expect("parse returned Some");
    assert!(tree.root_node().child_count() > 0);
}

#[test]
fn child_at_index_zero_is_some() {
    let tree = get_parser("python")
        .unwrap()
        .parse(PY_SRC)
        .expect("parse returned Some");
    let child = tree.root_node().child(0);
    assert!(child.is_some());
}

#[test]
fn named_child_count_is_nonzero_for_function_definition() {
    let tree = get_parser("python")
        .unwrap()
        .parse(PY_SRC)
        .expect("parse returned Some");
    let func_def = tree.root_node().named_child(0).expect("function_definition exists");
    assert_eq!(func_def.kind(), "function_definition");
    assert!(func_def.named_child_count() > 0);
}

#[test]
fn named_child_at_zero_is_some_for_function_definition() {
    let tree = get_parser("python")
        .unwrap()
        .parse(PY_SRC)
        .expect("parse returned Some");
    let func_def = tree.root_node().named_child(0).expect("function_definition exists");
    let first_named = func_def.named_child(0);
    assert!(first_named.is_some());
}

#[test]
fn parent_of_child_is_the_original_node() {
    let tree = get_parser("python")
        .unwrap()
        .parse(PY_SRC)
        .expect("parse returned Some");
    let root = tree.root_node();
    let child = root.child(0).expect("root has a child");
    let parent = child.parent().expect("child has a parent");
    assert_eq!(parent.kind(), root.kind());
}

#[test]
fn child_by_field_name_finds_function_name_in_def_foo() {
    let tree = get_parser("python")
        .unwrap()
        .parse(PY_SRC)
        .expect("parse returned Some");
    let func_def = tree.root_node().named_child(0).expect("function_definition");
    let name_node = func_def
        .child_by_field_name("name")
        .expect("'name' field exists on function_definition");
    assert_eq!(name_node.kind(), "identifier");
}

#[test]
fn to_sexp_is_nonempty_for_module_node() {
    let tree = get_parser("python")
        .unwrap()
        .parse(PY_SRC)
        .expect("parse returned Some");
    let sexp = tree.root_node().to_sexp();
    assert!(!sexp.is_empty());
}

#[test]
fn node_walk_returns_cursor_at_same_node() {
    let tree = get_parser("python")
        .unwrap()
        .parse(PY_SRC)
        .expect("parse returned Some");
    let root = tree.root_node();
    let cursor = root.walk();
    assert_eq!(cursor.node().kind(), root.kind());
}

#[test]
fn debug_reports_configuration_of_a_fresh_parser() {
    let rendered = format!("{:?}", Parser::new());
    assert_eq!(
        rendered,
        "Parser { language_name: None, max_source_bytes: None, timeout_ms: None, .. }"
    );
}

#[test]
fn debug_reports_limits_once_they_are_set() {
    let mut parser = Parser::new();
    parser.set_max_source_bytes(Some(4096));
    parser.set_parse_timeout_ms(Some(250));
    let rendered = format!("{parser:?}");
    assert!(
        rendered.contains("max_source_bytes: Some(4096)"),
        "max_source_bytes missing from {rendered}"
    );
    assert!(
        rendered.contains("timeout_ms: Some(250)"),
        "timeout_ms missing from {rendered}"
    );
}

#[test]
fn debug_of_a_get_parser_result_is_formattable() {
    let result = get_parser("definitely-not-a-language");
    assert!(format!("{result:?}").starts_with("Err("));
}

#[test]
fn incremental_parse_reuses_an_edited_tree_without_changing_existing_nodes() {
    use tree_sitter_language_pack::{InputEdit, Point};

    let before = "x = 'é'\ny = 1\n";
    let after = "x = '🙂'\ny = 1\n";
    let mut parser = get_parser("python").expect("Python parser");
    let original = parser.parse(before).expect("original tree");
    let original_root = original.root_node();
    let edited = original
        .edited(InputEdit {
            start_byte: 5,
            old_end_byte: 7,
            new_end_byte: 9,
            start_position: Point::new(0, 5),
            old_end_position: Point::new(0, 7),
            new_end_position: Point::new(0, 9),
        })
        .expect("valid edit");

    assert_eq!(original_root.end_byte(), before.len());
    assert_eq!(edited.root_node().end_byte(), after.len());

    let incremental = parser.parse_incremental(after, &edited).expect("incremental parse");
    let fresh = parser.parse(after).expect("fresh parse");
    assert!(!incremental.root_node().has_error());
    assert_eq!(incremental.root_node().to_sexp(), fresh.root_node().to_sexp());
    assert_eq!(incremental.root_node().end_byte(), after.len());
}

#[test]
fn edited_tree_rejects_reversed_byte_offsets() {
    use tree_sitter_language_pack::{Error, InputEdit, Point};

    let tree = get_parser("python").unwrap().parse("x = 1\n").unwrap();
    let invalid = InputEdit {
        start_byte: 4,
        old_end_byte: 3,
        new_end_byte: 5,
        start_position: Point::new(0, 4),
        old_end_position: Point::new(0, 4),
        new_end_position: Point::new(0, 5),
    };

    assert!(matches!(tree.edited(invalid), Err(Error::InvalidRange(_))));
}

#[test]
fn incremental_parse_honors_the_parsers_language_and_size_limit() {
    let source = "x = 1\n";
    let mut python = get_parser("python").unwrap();
    let tree = python.parse(source).unwrap();
    assert!(Parser::new().parse_incremental(source, &tree).is_none());

    python.set_max_source_bytes(Some(1));
    assert!(python.parse_incremental(source, &tree).is_none());
}
