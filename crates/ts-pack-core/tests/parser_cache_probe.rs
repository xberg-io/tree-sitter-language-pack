#![allow(clippy::unwrap_used, clippy::expect_used)]
#[path = "../build_support/parser_cache.rs"]
mod parser_cache;

use parser_cache::{cached_parsers_root, parsers_root_populated};
use std::path::Path;

fn parser(root: &Path, language: &str) {
    let dir = root.join(language).join("src");
    std::fs::create_dir_all(&dir).unwrap();
    std::fs::write(dir.join("parser.c"), "/* fixture */").unwrap();
}

#[test]
fn selected_languages_reuse_nested_release_bundle() {
    let temp = tempfile::tempdir().unwrap();
    let root = temp.path().join("parsers");
    parser(&root, "python");
    parser(&root, "rust");
    let selected = vec!["python".into(), "rust".into()];
    assert_eq!(cached_parsers_root(temp.path(), &selected), Some(root));
}

#[test]
fn flat_cache_layout_remains_supported() {
    let temp = tempfile::tempdir().unwrap();
    parser(temp.path(), "rust");
    assert_eq!(
        cached_parsers_root(temp.path(), &["rust".into()]),
        Some(temp.path().to_path_buf())
    );
}

#[test]
fn partial_or_directory_only_cache_is_not_a_hit() {
    let temp = tempfile::tempdir().unwrap();
    let root = temp.path().join("parsers");
    parser(&root, "python");
    std::fs::create_dir_all(root.join("rust/src/parser.c")).unwrap();
    assert!(!parsers_root_populated(&root, &["python".into(), "rust".into()]));
    assert_eq!(
        cached_parsers_root(temp.path(), &["python".into(), "rust".into()]),
        None
    );
}

#[test]
fn unselected_probe_uses_python_in_either_layout() {
    let temp = tempfile::tempdir().unwrap();
    assert_eq!(cached_parsers_root(temp.path(), &[]), None);
    parser(temp.path(), "python");
    assert_eq!(cached_parsers_root(temp.path(), &[]), Some(temp.path().to_path_buf()));
    let nested = temp.path().join("parsers");
    parser(&nested, "python");
    assert_eq!(cached_parsers_root(temp.path(), &[]), Some(nested));
}

#[test]
fn unrelated_nested_directory_does_not_hide_valid_flat_cache() {
    let temp = tempfile::tempdir().unwrap();
    std::fs::create_dir_all(temp.path().join("parsers")).unwrap();
    parser(temp.path(), "rust");
    assert_eq!(
        cached_parsers_root(temp.path(), &["rust".into()]),
        Some(temp.path().to_path_buf())
    );
}
