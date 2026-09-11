"""Exercise the input guards on the PHP source builder used by the publish workflow."""

import subprocess
import unittest
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
BUILDER = ROOT / "scripts/ci/build-php-source.sh"


class PhpSourceBuilderTests(unittest.TestCase):
    def test_rejects_invalid_inputs_before_downloading(self) -> None:
        for series, prefix in (("9.0", "/unused"), ("8.3", "relative")):
            result = subprocess.run(
                ["bash", str(BUILDER), series, prefix],
                capture_output=True,
                text=True,
                check=False,
            )
            assert result.returncode != 0, f"{series} {prefix} should have been rejected"
            assert "Unsupported PHP series" in result.stderr or "prefix must be absolute" in result.stderr


if __name__ == "__main__":
    unittest.main()
