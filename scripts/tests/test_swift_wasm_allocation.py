"""Exercise the pinned Swift scanner's allocation branch through its durable source patch."""

from pathlib import Path
import json
import signal
import subprocess
import tempfile
import unittest


ROOT = Path(__file__).resolve().parents[2]
SOURCE = """void *tree_sitter_swift_external_scanner_create() {
    void *payload = calloc(1, sizeof(struct ScannerState));
    if (!payload) {
        fprintf(stderr, "tree-sitter-swift: out of memory allocating scanner state\\n");
        exit(1);
    }
    return payload;
}
"""


class SwiftAllocationTests(unittest.TestCase):
    def compile_and_run(self, *, wasm: bool, fail: bool) -> subprocess.CompletedProcess:
        with tempfile.TemporaryDirectory(prefix="swift-allocation-") as temporary:
            directory = Path(temporary).resolve()
            source = directory / "src/scanner.c"
            source.parent.mkdir()
            source.write_text(SOURCE)
            patches = list((ROOT / "patches/swift").glob("*.patch"))
            for patch in patches:
                subprocess.run(
                    ["git", "apply", "--unsafe-paths", f"--directory={directory}", str(patch)],
                    cwd=ROOT,
                    check=True,
                    capture_output=True,
                    text=True,
                )
            body = source.read_text()
            header = "#include <stdio.h>\n#include <stdlib.h>\nstruct ScannerState { unsigned value; };\n"
            footer = (
                "void *fixture_calloc(size_t count, size_t size) {\n"
                "  static struct ScannerState state;\n"
                "  if (count != 1 || size != sizeof(state)) abort();\n"
                f"  return {'NULL' if fail else '&state'};\n"
                "}\nint main(void) { return tree_sitter_swift_external_scanner_create() == NULL; }\n"
            )
            source.write_text(header + body + footer)
            executable = directory / "allocation"
            command = ["cc", "-std=c11", "-Dcalloc=fixture_calloc", str(source), "-o", str(executable)]
            if wasm:
                command.insert(1, "-D__wasm__")
            subprocess.run(command, check=True, capture_output=True, text=True)
            return subprocess.run([str(executable)], check=False, capture_output=True, text=True)

    def test_patch_targets_the_reviewed_upstream_revision(self) -> None:
        definitions = json.loads((ROOT / "sources/language_definitions.json").read_text())
        assert definitions["swift"]["rev"] == "9e3b87f66f1fd2f3b931376fd3723ab788e337c3"
        patches = list((ROOT / "patches/swift").glob("*.patch"))
        assert len(patches) == 1
        text = patches[0].read_text()
        assert text.count("+++ b/src/scanner.c") == 1
        assert "__builtin_trap" in text
        assert "-        fprintf" not in text
        assert "-        exit" not in text

    def test_native_allocation_failure_keeps_the_diagnostic_and_exit_status(self) -> None:
        result = self.compile_and_run(wasm=False, fail=True)
        assert result.returncode == 1
        assert result.stderr == "tree-sitter-swift: out of memory allocating scanner state\n"
        assert result.stdout == ""

    def test_wasm_allocation_failure_traps_without_a_stdio_or_exit_call(self) -> None:
        result = self.compile_and_run(wasm=True, fail=True)
        assert result.returncode in (-signal.SIGILL, -signal.SIGTRAP)
        assert result.stderr == ""
        assert result.stdout == ""

    def test_successful_allocation_is_unchanged_on_both_branches(self) -> None:
        for wasm in (False, True):
            with self.subTest(wasm=wasm):
                result = self.compile_and_run(wasm=wasm, fail=False)
                assert result.returncode == 0
                assert result.stderr == ""
                assert result.stdout == ""


if __name__ == "__main__":
    unittest.main()
