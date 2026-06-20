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
    // Build the binary with the mcp feature before running.
    let build = std::process::Command::new("cargo")
        .args(["build", "--bin", "ts-pack", "--features", "mcp"])
        .status()
        .expect("failed to run cargo build");
    assert!(build.success(), "cargo build with mcp feature should succeed");

    // Locate the binary relative to the test output directory.
    let binary = env!("CARGO_TARGET_TMPDIR")
        .split("target")
        .next()
        .map(|prefix| format!("{prefix}target/debug/ts-pack"))
        .unwrap_or_else(|| "target/debug/ts-pack".to_string());

    let output = std::process::Command::new(&binary)
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
    // This test only runs when compiled without the mcp feature.
    // If we reach here, the binary should not expose the `mcp` subcommand.
    let output = std::process::Command::new("cargo")
        .args(["run", "--bin", "ts-pack", "--", "mcp", "--help"])
        .output()
        .expect("failed to run ts-pack");

    // clap exits with status 2 for unknown subcommands
    assert!(
        !output.status.success(),
        "ts-pack mcp should be unknown without the mcp feature"
    );
}
