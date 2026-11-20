#!/usr/bin/env python3
"""Exit 0 only when every grammar in sources/language_definitions.json has a parser in parsers/.

An interrupted scripts/clone_vendors.py leaves a partial parsers/ tree; a non-empty directory
check cannot tell that from a finished clone.
"""

import json
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]


def main() -> int:
    definitions = json.loads((ROOT / "sources" / "language_definitions.json").read_text())
    missing = [name for name in definitions if not (ROOT / "parsers" / name / "src" / "parser.c").is_file()]
    if missing:
        print(f"parsers/ is incomplete: {len(missing)} of {len(definitions)} grammars missing", file=sys.stderr)
        return 1
    return 0


if __name__ == "__main__":
    sys.exit(main())
