#!/usr/bin/env bash
# Grammar parse test for ts-pack CLI — verifies static parsers work for a subset of languages.
# Usage: TS_PACK_BIN=./target/release/ts-pack TS_PACK_LANGUAGES="python,rust,go" bash test_apps/cli/test_all_grammars.sh
set -euo pipefail

BIN="${TS_PACK_BIN:-ts-pack}"
LANGUAGES="${TS_PACK_LANGUAGES:-python,rust,javascript,typescript,go,html,css,json}"

echo "=== grammar parse test ==="
echo "Binary: $BIN"
echo "Languages: $LANGUAGES"

# Minimal source snippets for each supported language.
declare -A SOURCES
SOURCES[python]='x = 1'
SOURCES[rust]='fn main() {}'
SOURCES[javascript]='const x = 1;'
SOURCES[typescript]='const x: number = 1;'
SOURCES[go]='package main'
SOURCES[html]='<html></html>'
SOURCES[css]='body { color: red; }'
SOURCES[json]='{"key": "value"}'
SOURCES[mojo]='fn main(): None: pass'
SOURCES[nim]='echo "hi"'
SOURCES[norg]='* Heading'
SOURCES[wolfram]='x = 1'

PASS=0
FAIL=0

IFS=',' read -ra LANG_LIST <<< "$LANGUAGES"
for lang in "${LANG_LIST[@]}"; do
  if [[ -v SOURCES[$lang] ]]; then
    src="${SOURCES[$lang]}"
  else
    src="// $lang"
  fi
  if echo "$src" | "$BIN" parse - --language "$lang" > /dev/null 2>&1; then
    echo "  PASS: $lang"
    ((PASS++))
  else
    echo "  FAIL: $lang" >&2
    ((FAIL++))
  fi
done

echo "=== results: $PASS passed, $FAIL failed ==="
if [[ "$FAIL" -gt 0 ]]; then
  exit 1
fi
