//! Integration tests for the `ts-pack mcp` subcommand.

/// Verify that `ts-pack mcp --help` exits cleanly and mentions the expected flags.
///
/// This test requires the binary to be built with `--features mcp`.  We skip
/// it when the feature flag is absent by checking the help output for the
/// subcommand name; if the binary was built without the feature the test is
/// simply not compiled (it lives behind `#[cfg(feature = "mcp")]`).
#[cfg(feature = "mcp")]
#[test]
fn test_mcp_help_output() {
    let output = std::process::Command::new(env!("CARGO_BIN_EXE_ts-pack"))
        .args(["mcp", "--help"])
        .output()
        .expect("failed to run ts-pack mcp --help");

    assert!(output.status.success(), "mcp --help should exit with status 0");

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("MCP") || stdout.contains("Model Context Protocol"),
        "help text should mention MCP"
    );
    assert!(stdout.contains("--transport"), "help text should list --transport flag");
    assert!(stdout.contains("--port"), "help text should list --port flag");
    assert!(stdout.contains("--config"), "help text should list --config flag");
}

/// Verify that `ts-pack mcp` is absent when the binary is built without the
/// feature (the `Commands` enum won't have the variant).
#[cfg(not(feature = "mcp"))]
#[test]
fn test_mcp_subcommand_absent_without_feature() {
    let output = std::process::Command::new("cargo")
        .args([
            "run",
            "--no-default-features",
            "--bin",
            "ts-pack",
            "--",
            "mcp",
            "--help",
        ])
        .output()
        .expect("failed to run ts-pack");

    assert!(
        !output.status.success(),
        "ts-pack mcp should be unknown without the mcp feature"
    );
}
