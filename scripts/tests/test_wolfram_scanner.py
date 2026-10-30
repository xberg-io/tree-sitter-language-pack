"""Exercise the comment scanner without compiling the language pack."""

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
};
"""


class WolframCommentTests(unittest.TestCase):
    """Check token boundaries, nested delimiters and incomplete comments."""

    @classmethod
    def setUpClass(cls) -> None:
        """Compile the production scanner against the lexer methods it uses."""
        cls.temporary = tempfile.TemporaryDirectory(prefix="wolfram-scanner-")
        cls.addClassCleanup(cls.temporary.cleanup)
        directory = Path(cls.temporary.name)
        header = directory / "tree_sitter/parser.h"
        header.parent.mkdir()
        header.write_text(LEXER_HEADER)
        cls.executable = directory / "scanner"
        subprocess.run(
            [
                "cc",
                "-std=c11",
                "-Wall",
                "-Wextra",
                "-Wno-unused-parameter",
                "-Werror",
                "-I",
                str(directory),
                str(Path(__file__).with_name("wolfram_scanner_harness.c")),
                "-o",
                str(cls.executable),
            ],
            check=True,
            capture_output=True,
            text=True,
        )

    def test_should_preserve_comment_acceptance_and_consumption(self) -> None:
        """Successful tokens stop after their matching closing delimiter."""
        cases = [
            ("", (0, 0, 0, 99)),
            ("x", (0, 0, 0, 99)),
            ("(x", (0, 1, 0, 99)),
            ("(*", (0, 2, 0, 99)),
            ("(* incomplete (*)", (0, 17, 0, 99)),
            ("(**)tail", (1, 4, 0, 0)),
            (" \n\t(* text *)x", (1, 13, 3, 0)),
            ("(* outer (* inner *) end *)x", (1, 27, 0, 0)),
            ("(* a(**)b *)x", (1, 12, 0, 0)),
            ("(* (text) ***)x", (1, 14, 0, 0)),
            ("(* *x) text *)x", (1, 14, 0, 0)),
        ]
        for source, expected in cases:
            with self.subTest(source=source):
                result = subprocess.run([str(self.executable), source], check=True, capture_output=True, text=True)
                assert tuple(map(int, result.stdout.split())) == expected


if __name__ == "__main__":
    unittest.main()
