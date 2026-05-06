#!/usr/bin/env bash
# Smoke test for ts-pack CLI — verifies basic commands work end-to-end.
# Usage: TS_PACK_BIN=./target/release/ts-pack bash test_apps/cli/smoke_test.sh
set -euo pipefail

BIN="${TS_PACK_BIN:-ts-pack}"

echo "=== ts-pack CLI smoke test ==="
echo "Binary: $BIN"

# Version and help
"$BIN" --version
"$BIN" --help > /dev/null

# List languages (static registry, no download needed)
LANG_COUNT=$("$BIN" list | wc -l | tr -d ' ')
echo "Language count: $LANG_COUNT"
if [[ "$LANG_COUNT" -lt 100 ]]; then
  echo "ERROR: expected at least 100 languages, got $LANG_COUNT" >&2
  exit 1
fi

# List with filter
"$BIN" list --filter python | grep -q "python"

# Info for a known language
"$BIN" info python > /dev/null

# Parse inline Python (static mode, no download)
echo "x = 1" | "$BIN" parse - --language python > /dev/null

echo "=== smoke test passed ==="
