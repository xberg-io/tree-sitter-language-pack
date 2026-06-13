#[test]
fn kdl_process_via_download() {
    let cfg = tree_sitter_language_pack::ProcessConfig::new("kdl");
    let r = tree_sitter_language_pack::process("host \"localhost\"\nport 8080\n", &cfg);
    println!("result: {:?}", r);
    assert!(r.is_ok(), "kdl process failed: {:?}", r);
}
