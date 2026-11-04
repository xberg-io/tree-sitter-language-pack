//! Valid input that older bash, c and dockerfile pins parsed with ERROR nodes (#216).
#![allow(clippy::unwrap_used, clippy::expect_used)] // ~keep: a failed setup step in a test must abort loudly

use tree_sitter_language_pack::get_parser;

fn assert_parses_cleanly(language: &str, source: &str) -> String {
    let mut parser = get_parser(language)
        .unwrap_or_else(|e| panic!("{language} grammar required; build with TSLP_LANGUAGES=bash,c,dockerfile: {e}"));
    let tree = parser.parse(source).unwrap();
    let root = tree.root_node();
    let sexp = root.to_sexp();
    assert!(!root.has_error(), "{language} parsed {source:?} with errors: {sexp}");
    assert_eq!(root.end_byte(), source.len());
    sexp
}

#[test]
fn bash_for_loop_without_in_list_or_separator_before_do() {
    let sexp = assert_parses_cleanly("bash", "for arg do echo \"$arg\"; done\n");
    assert!(
        sexp.starts_with("(program (for_statement variable: (variable_name)"),
        "{sexp}"
    );
}

#[test]
fn bash_read_write_redirect() {
    let sexp = assert_parses_cleanly("bash", "exec 3<>/dev/tcp/localhost/80\n");
    assert!(sexp.contains("(file_redirect descriptor: (file_descriptor)"), "{sexp}");
}

#[test]
fn c_has_include_with_system_header_argument() {
    let sexp = assert_parses_cleanly("c", "#if __has_include(<stdio.h>)\n#endif\n");
    assert!(sexp.contains("(system_lib_string)"), "{sexp}");
}

#[test]
fn dockerfile_legacy_env_value_with_spaces() {
    let sexp = assert_parses_cleanly("dockerfile", "ENV LEGACY some value here\n");
    assert!(sexp.contains("(env_instruction (env_pair"), "{sexp}");
}
