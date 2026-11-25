"""Verify launcher cwd and argv through an actual exec of a cached binary."""

import json
import os
from pathlib import Path
import re
import subprocess
import sys
import tempfile
import unittest


class LauncherDirectoryTests(unittest.TestCase):
    """Run the launcher against a stand-in binary cached in the plugin root."""

    def test_should_preserve_project_directory_and_relative_configuration(self) -> None:
        """The launched process must see the caller's config and exact arguments."""
        root = Path(__file__).resolve().parents[2]
        source = (root / "plugin/scripts/mcp-launch.sh").read_text()
        mode = re.search(r'LAUNCHER="\$\{([A-Z_]+):-auto\}"', source)
        pin = re.search(r'(?m)^PINNED_VERSION="([^"]+)"', source)
        name = re.search(r'(?m)^BINARY_NAME="([^"]+)"', source)
        assert mode is not None and pin is not None and name is not None
        with tempfile.TemporaryDirectory(prefix="mcp-launch-cwd-") as temporary:
            directory = Path(temporary)
            plugin = directory / "plugin"
            bin_dir = plugin / "bin"
            bin_dir.mkdir(parents=True)
            executable = bin_dir / name[1]
            executable.write_text(
                f"#!{sys.executable}\n"
                "import json, os, sys\n"
                f'if "--version" in sys.argv: print("{pin[1]}"); sys.exit(0)\n'
                'print(json.dumps({"cwd": os.getcwd(), "config": open("model.toml").read(), "args": sys.argv[1:]}))\n'
            )
            executable.chmod(0o755)
            project = directory / "caller project"
            project.mkdir()
            (project / "model.toml").write_text("provider-model-route")
            environment = os.environ | {mode[1]: "auto", "CLAUDE_PLUGIN_ROOT": str(plugin)}
            arguments = ["mcp", "relative file.txt", "", "--stdio"]
            result = subprocess.run(
                ["bash", str(root / "plugin/scripts/mcp-launch.sh"), *arguments],
                cwd=project,
                env=environment,
                capture_output=True,
                text=True,
                timeout=60,
                check=False,
            )
            assert result.returncode == 0, result.stderr
            payload = json.loads(result.stdout)
            assert Path(payload["cwd"]).resolve() == project.resolve()
            assert payload["config"] == "provider-model-route"
            assert payload["args"] == arguments

    def test_should_run_the_exact_pinned_npm_package_when_no_binary_is_installed(self) -> None:
        """With no cached or PATH binary, npx runs the pinned package and keeps the caller's cwd."""
        root = Path(__file__).resolve().parents[2]
        script = root / "plugin/scripts/mcp-launch.sh"
        source = script.read_text()
        mode = re.search(r'LAUNCHER="\$\{([A-Z_]+):-auto\}"', source)
        pin = re.search(r'(?m)^PINNED_VERSION="([^"]+)"', source)
        assert mode is not None and pin is not None
        with tempfile.TemporaryDirectory(prefix="mcp-launch-npx-") as temporary:
            directory = Path(temporary)
            stubs = directory / "stubs"
            stubs.mkdir()
            for name in ("node", "npx"):
                stub = stubs / name
                stub.write_text(
                    f"#!{sys.executable}\nimport json, os, sys\n"
                    'print(json.dumps({"cwd": os.getcwd(), "args": sys.argv[1:]}))\n'
                )
                stub.chmod(0o755)
            project = directory / "caller project"
            project.mkdir()
            environment = {
                "PATH": f"{stubs}:/usr/bin:/bin",
                mode[1]: "auto",
                "CLAUDE_PLUGIN_ROOT": str(directory / "empty-plugin"),
            }
            result = subprocess.run(
                ["bash", str(script), "mcp", "--transport", "stdio"],
                cwd=project,
                env=environment,
                capture_output=True,
                text=True,
                timeout=60,
                check=False,
            )
            assert result.returncode == 0, result.stderr
            payload = json.loads(result.stdout)
            assert payload["args"][0] == "--yes"
            assert f"@xberg-io/ts-pack-cli@{pin[1]}" in payload["args"]
            assert payload["args"][-3:] == ["mcp", "--transport", "stdio"]
            assert Path(payload["cwd"]).resolve() == project.resolve()


if __name__ == "__main__":
    unittest.main()
