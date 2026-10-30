"""Characterize Reason token boundaries and state transitions across scanner refactors."""

import json
import os
from pathlib import Path
import subprocess
import tempfile
import unittest

LEXER_HEADER = """#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>
typedef struct TSLexer TSLexer;
struct TSLexer {
  int32_t lookahead;
  uint16_t result_symbol;
  void (*advance)(TSLexer *, bool);
  void (*mark_end)(TSLexer *);
  bool (*eof)(const TSLexer *);
};
"""


class ReasonScannerTests(unittest.TestCase):
    """Keep lexer consumption, marked ranges and scanner state stable."""

    @classmethod
    def setUpClass(cls) -> None:
        """Compile the real scanner with only the lexer callbacks it uses."""
        cls.temporary = tempfile.TemporaryDirectory(prefix="reason-scanner-")
        cls.addClassCleanup(cls.temporary.cleanup)
        directory = Path(cls.temporary.name)
        header = directory / "tree_sitter/parser.h"
        header.parent.mkdir()
        header.write_text(LEXER_HEADER)
        cls.executable = directory / "scanner"
        command = ["cc", "-std=c11", "-Wall", "-Wextra", "-Wno-unused-parameter", "-Werror"]
        if source := os.environ.get("REASON_SCANNER_SOURCE"):
            # ~keep The original comparison source contains an unused newline temporary.
            command += ["-Wno-unused-variable", f"-DSCANNER_SOURCE={json.dumps(str(Path(source).resolve()))}"]
        subprocess.run(
            [
                *command,
                "-I",
                str(directory),
                str(Path(__file__).with_name("reason_scanner_harness.c")),
                "-o",
                str(cls.executable),
            ],
            check=True,
            capture_output=True,
            text=True,
        )

    def scan(self, source: str, mask: int, flags: int = 0, parens: int = 0, repeats: int = 1) -> list[tuple[int, ...]]:
        result = subprocess.run(
            [str(self.executable), source, str(mask), str(flags), str(parens), str(repeats)],
            check=True,
            capture_output=True,
            text=True,
            timeout=5,
        )
        return [tuple(map(int, line.split())) for line in result.stdout.splitlines()]

    def test_token_boundaries_and_state_transitions(self) -> None:
        """A consumed lookahead must not change the marked token or scanner flags."""
        cases = [
            ("  x", 0, 0, 0, (0, 3, 2, 0, 99, 0, 0, 0, 0)),
            ("  x", 0, 1, 0, (0, 1, 1, 0, 99, 0, 1, 0, 0)),
            ('"x', 8, 0, 0, (1, 1, 0, 1, 3, 0, 1, 0, 0)),
            ('"x', 8, 1, 0, (1, 1, 0, 1, 3, 0, 0, 0, 0)),
            ("`x", 16, 0, 0, (1, 1, 0, 1, 4, 0, 0, 1, 0)),
            ("`x", 16, 2, 0, (1, 1, 0, 1, 4, 0, 0, 0, 0)),
            ("(x", 64, 0, 0, (1, 1, 0, 1, 6, 1, 0, 0, 0)),
            (")x", 128, 0, 1, (1, 1, 0, 1, 7, 0, 0, 0, 0)),
            ("//x\nx", 2, 0, 0, (1, 3, 0, 3, 1, 0, 0, 0, 0)),
            ("/*x*/x", 2, 0, 0, (1, 5, 0, 5, 1, 0, 0, 0, 0)),
            ("/x", 2, 0, 0, (0, 1, 0, 0, 1, 0, 0, 0, 0)),
            ("/x", 2, 1, 0, (0, 1, 0, 0, 99, 0, 1, 0, 0)),
            ("abc`", 32, 2, 0, (1, 3, 0, 3, 5, 0, 0, 0, 0)),
            ("`", 32, 2, 0, (0, 0, 0, 0, 5, 0, 0, 0, 0)),
            ("abc", 32, 2, 0, (0, 3, 0, 3, 5, 0, 0, 1, 0)),
            ("a$foo", 32, 2, 0, (1, 2, 0, 1, 5, 0, 0, 1, 0)),
            ("a${x", 32, 2, 0, (1, 2, 0, 1, 5, 0, 0, 1, 0)),
            ("a\\q", 32, 2, 0, (1, 1, 0, 1, 5, 0, 0, 1, 0)),
            ("ab$1c`", 32, 2, 0, (1, 5, 0, 5, 5, 0, 0, 0, 0)),
            ("\nx", 1, 0, 0, (1, 1, 1, 1, 0, 0, 0, 0, 0)),
            ("\nabc", 1, 0, 0, (1, 2, 1, 1, 0, 0, 0, 0, 0)),
            ("\nanx", 1, 0, 0, (1, 3, 1, 1, 0, 0, 0, 0, 0)),
            ("\nand x", 1, 0, 0, (0, 4, 1, 1, 0, 0, 0, 0, 0)),
            ("\n->x", 1, 0, 0, (0, 3, 1, 1, 0, 0, 0, 0, 0)),
            ("\n-x", 1, 0, 0, (1, 2, 1, 1, 0, 0, 0, 0, 0)),
            ("\n|x", 1, 0, 0, (0, 2, 1, 1, 0, 0, 0, 0, 0)),
            ("\n?x", 1, 0, 0, (0, 2, 1, 1, 0, 0, 0, 0, 0)),
            ("\n:x", 1, 0, 0, (0, 2, 1, 1, 0, 0, 0, 0, 0)),
            ("\n}x", 1, 0, 0, (0, 2, 1, 1, 0, 0, 0, 0, 0)),
            ("\n/*ok*/x", 5, 0, 0, (1, 7, 1, 7, 2, 0, 0, 0, 0)),
            ("\n/*ok*/|x", 7, 0, 0, (1, 7, 1, 7, 1, 0, 0, 0, 0)),
        ]
        for source, mask, flags, parens, expected in cases:
            with self.subTest(source=source, mask=mask, flags=flags, parens=parens):
                assert self.scan(source, mask, flags, parens) == [expected]

    def test_eof_newline_is_reported_only_once(self) -> None:
        """Repeated EOF scans must not produce an unbounded sequence of newlines."""
        assert self.scan("", 1, repeats=2) == [
            (1, 0, 0, 0, 0, 0, 0, 0, 1),
            (0, 0, 0, 0, 0, 0, 0, 0, 1),
        ]


if __name__ == "__main__":
    unittest.main()
