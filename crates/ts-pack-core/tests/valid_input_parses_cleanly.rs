//! Valid input that older grammar pins (#216, #217, #220) parsed with ERROR nodes.
#![allow(clippy::unwrap_used, clippy::expect_used)] // ~keep: a failed setup step in a test must abort loudly

use tree_sitter_language_pack::get_parser;

fn assert_parses_cleanly(language: &str, source: &str) -> String {
    let mut parser = get_parser(language).unwrap_or_else(|e| {
        panic!("{language} grammar required; build with TSLP_LANGUAGES=bash,c,css,dockerfile,groovy,javascript,sql,zig: {e}")
    });
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

#[test]
fn zig_destructuring_declarations() {
    let source = concat!(
        "fn f() void {\n",
        "    const a, const b = .{ 1, 2 };\n",
        "    var c: u8, var d: u8 = .{ 3, 4 };\n",
        "    _ = .{ a, b, c, d };\n",
        "}\n",
    );
    let sexp = assert_parses_cleanly("zig", source);
    assert!(
        sexp.contains("(variable_declaration (identifier) (identifier) (anonymous_struct_initializer"),
        "{sexp}"
    );
    assert!(
        sexp.contains("(variable_declaration (identifier) type: (builtin_type) (identifier) type: (builtin_type)"),
        "{sexp}"
    );
}

#[test]
fn bash_accepts_a_variable_substring_offset() {
    assert_parses_cleanly("bash", "c=\"${s:$i:1}\"\n");
}

#[test]
fn dockerfile_accepts_eof_and_crlf_continuations() {
    assert_parses_cleanly("dockerfile", "FROM alpine\nRUN echo done");
    assert_parses_cleanly("dockerfile", "FROM alpine\r\nRUN apk add \\\r\n    curl\r\n");
}

#[test]
fn sql_accepts_grant_and_sqlite_constraints() {
    for source in [
        "GRANT SELECT ON users TO web_user;",
        "CREATE TABLE users (id INTEGER PRIMARY KEY AUTOINCREMENT);",
        "CREATE TABLE schedule (day_of_week INTEGER CHECK (day_of_week BETWEEN 0 AND 6));",
    ] {
        assert_parses_cleanly("sql", source);
    }
}

#[test]
fn groovy_accepts_left_shift() {
    assert_parses_cleanly("groovy", "x = 1 << 2\n");
}

#[test]
fn orchard_css_and_javascript_accept_common_syntax() {
    assert_parses_cleanly("css", "@media (min-width: 10px) { .card:hover { color: red; } }\n");
    assert_parses_cleanly("javascript", "export const lookup = (item) => item?.value ?? 0;\n");
}
