use super::*;

// ~keep A missing grammar reports `SKIPPED` on stderr; see `intel::test_support`.
fn parse(source: &str, lang: &str) -> Option<tree_sitter::Tree> {
    crate::intel::test_support::parse_or_skip(source, lang)
}

/// Extract, or skip when the grammar is missing.
///
/// ~keep A present grammar that yields no data tree is a defect, not a skip:
/// ~keep it fails here instead of leaving the caller's `else { return }` silent.
fn extract(source: &str, lang: &str) -> Option<DataNode> {
    let tree = parse(source, lang)?;
    let extracted = extract_data(&tree.root_node(), source, lang);
    assert!(
        extracted.is_some(),
        "'{lang}' is loaded but produced no data tree for {source:?}"
    );
    extracted
}

#[test]
fn should_not_panic_when_a_quoted_token_is_unterminated() {
    assert_eq!(strip_quotes("\""), "\"", "a lone quote has nothing to strip");
    assert_eq!(strip_quotes("'"), "'");
    assert_eq!(strip_quotes(""), "");
    assert_eq!(strip_quotes("\"\""), "");
    assert_eq!(strip_quotes("\"a\""), "a");
    assert_eq!(strip_quotes("bare"), "bare");
}

#[test]
fn should_extract_a_truncated_po_file_without_panicking() {
    let source = "msgid \"a\"\nmsgstr \"\n";
    let Some(tree) = parse(source, "po") else {
        return;
    };
    let extracted = extract_data(&tree.root_node(), source, "po");
    assert!(
        extracted.is_some(),
        "a truncated translation file must still yield a data tree"
    );
}

#[test]
fn test_json_flat_object() {
    let source = r#"{"host": "localhost", "port": 8080}"#;
    let Some(root) = extract(source, "json") else { return };
    assert_eq!(root.kind, DataNodeKind::KeyValue);
    assert_eq!(root.children.len(), 2);
    let host = root.children.iter().find(|c| c.key.as_deref() == Some("host"));
    assert!(host.is_some(), "should find 'host' key");
    assert_eq!(host.unwrap().value.as_deref(), Some(r#""localhost""#));
}

#[test]
fn test_json_nested_object() {
    let source = r#"{"server": {"host": "x", "port": 8080}}"#;
    let Some(root) = extract(source, "json") else { return };
    let server = root.children.iter().find(|c| c.key.as_deref() == Some("server"));
    assert!(server.is_some());
    let server = server.unwrap();
    assert!(!server.children.is_empty(), "server should have children");
}

#[test]
fn test_json_array() {
    let source = r#"[1, 2, 3]"#;
    let Some(root) = extract(source, "json") else { return };
    assert_eq!(root.kind, DataNodeKind::Sequence);
    assert_eq!(root.children.len(), 3);
    assert_eq!(root.children[0].key.as_deref(), Some("0"));
}

#[test]
fn test_toml_flat() {
    let source = "host = \"localhost\"\nport = 8080\n";
    let Some(root) = extract(source, "toml") else { return };
    assert!(
        root.children.iter().any(|c| c.key.as_deref() == Some("host")),
        "should find 'host'"
    );
    assert!(
        root.children.iter().any(|c| c.key.as_deref() == Some("port")),
        "should find 'port'"
    );
}

#[test]
fn test_toml_table() {
    let source = "[server]\nhost = \"localhost\"\n";
    let Some(root) = extract(source, "toml") else { return };
    let server = root.children.iter().find(|c| c.key.as_deref() == Some("server"));
    assert!(server.is_some(), "should find [server] table");
}

#[test]
fn test_properties_flat() {
    let source = "host=localhost\nport=8080\n";
    let Some(root) = extract(source, "properties") else {
        return;
    };
    assert!(root.children.iter().any(|c| c.key.as_deref() == Some("host")));
    assert!(root.children.iter().any(|c| c.key.as_deref() == Some("port")));
}

#[test]
fn test_yaml_flat() {
    let source = "host: localhost\nport: 8080\n";
    let Some(root) = extract(source, "yaml") else { return };
    assert!(root.children.iter().any(|c| c.key.as_deref() == Some("host")));
}

#[test]
fn test_yaml_nested() {
    let source = "server:\n  host: localhost\n  port: 8080\n";
    let Some(root) = extract(source, "yaml") else { return };
    let server = root.children.iter().find(|c| c.key.as_deref() == Some("server"));
    assert!(server.is_some(), "should find nested server key");
}

#[test]
fn test_csv_rows() {
    let source = "a,b,c\n1,2,3\n";
    let Some(root) = extract(source, "csv") else { return };
    assert_eq!(root.kind, DataNodeKind::Sequence);
    assert!(!root.children.is_empty(), "should have rows");
    let row0 = &root.children[0];
    assert_eq!(row0.kind, DataNodeKind::Sequence);
    assert_eq!(row0.children.len(), 3);
    assert_eq!(row0.children[0].key.as_deref(), Some("0"));
}

#[test]
fn test_ini_flat() {
    let source = "host=localhost\nport=8080\n";
    let Some(root) = extract(source, "ini") else { return };
    assert!(!root.children.is_empty(), "should have settings");
}

#[test]
fn test_unsupported_language_returns_none() {
    let source = "x = 1";
    let Some(tree) = parse(source, "python") else {
        return;
    };
    let result = extract_data(&tree.root_node(), source, "python");
    assert!(result.is_none(), "python should return None for data extraction");
}
