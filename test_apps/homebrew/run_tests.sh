#!/usr/bin/env bash
# Tests the Homebrew CLI formula and FFI formula.
set -euo pipefail

VERSION="${TSLP_BREW_VERSION:-1.9.0-rc.11}"
TAP="${TSLP_BREW_TAP:-kreuzberg-dev/homebrew-tap}"
CLI_FORMULA="${TSLP_BREW_CLI_FORMULA:-ts-pack}"
FFI_FORMULA="${TSLP_BREW_LIB_FORMULA:-libts-pack}"
FFI_FORMULA_QUALIFIED="$TAP/$FFI_FORMULA"
CLI_FORMULA_QUALIFIED="$TAP/$CLI_FORMULA"

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

PASS=0
FAIL=0

pass() {
  echo "PASS: $1"
  PASS=$((PASS + 1))
}

fail() {
  echo "FAIL: $1 — $2" >&2
  FAIL=$((FAIL + 1))
}

# Skip on Linux for now (homebrew semantics differ; this test targets macOS bottles).
if [[ "$OSTYPE" != "darwin"* ]]; then
  echo "SKIP: homebrew test_app only supported on macOS"
  exit 0
fi

# Install formulae. `brew bundle install` does not upgrade already-installed
# formulae, so force an upgrade afterward to ensure the test exercises the
# pinned $VERSION rather than a cached older bottle.
brew bundle install --file="$SCRIPT_DIR/Brewfile"
brew upgrade --force "$CLI_FORMULA_QUALIFIED" "$FFI_FORMULA_QUALIFIED" 2>/dev/null || true

# Test: CLI version output contains VERSION.
cli_version=$("$CLI_FORMULA" --version 2>&1 || true)
if [[ "$cli_version" == *"$VERSION"* ]]; then
  pass "cli-version"
else
  fail "cli-version" "expected '$VERSION' in version output, got: $cli_version"
fi

# Test: CLI help works.
cli_help=$("$CLI_FORMULA" --help 2>&1 || true)
if [[ "$cli_help" == *"ts-pack"* ]]; then
  pass "cli-help"
else
  fail "cli-help" "expected 'ts-pack' in help output, got: $cli_help"
fi

# Test: FFI formula — compile and run ffi_smoke.c.
TMP_DIR=$(mktemp -d)
trap 'rm -rf "$TMP_DIR"' EXIT

if command -v pkg-config &>/dev/null; then
  FFI_CFLAGS=$(pkg-config --cflags "tree-sitter-language-pack" 2>/dev/null || true)
  FFI_LIBS=$(pkg-config --libs "tree-sitter-language-pack" 2>/dev/null || true)
else
  # Fallback: use brew --prefix to locate headers and libs.
  FFI_PREFIX=$(brew --prefix "$FFI_FORMULA_QUALIFIED" 2>/dev/null || true)
  FFI_CFLAGS="-I$FFI_PREFIX/include"
  FFI_LIBS="-L$FFI_PREFIX/lib -lts_pack_core_ffi"
fi

# shellcheck disable=SC2086
if cc $FFI_CFLAGS -o "$TMP_DIR/ffi_smoke" "$SCRIPT_DIR/ffi_smoke.c" $FFI_LIBS 2>&1; then
  if "$TMP_DIR/ffi_smoke"; then
    pass "ffi-smoke"
  else
    fail "ffi-smoke" "ffi_smoke binary exited non-zero"
  fi
else
  fail "ffi-smoke" "compilation of ffi_smoke.c failed"
fi

echo ""
echo "Results: $PASS passed, $FAIL failed"
[ "$FAIL" -eq 0 ]
