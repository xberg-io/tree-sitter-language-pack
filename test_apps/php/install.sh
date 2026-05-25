#!/usr/bin/env bash
# alef-generated installer for registry-mode PHP test_app.
# Installs the kreuzberg-dev/tree-sitter-language-pack extension via PIE before `composer install` runs.
# Requires `composer` and `php` on PATH; bootstraps `php/pie` if needed.
set -euo pipefail

VERSION="${1:?usage: $0 <version>}"

# PIE >= 1.3.7 supports the array-form `php-ext.download-url-method`
# our composer.json emits; 1.4.x is preferred. Install pie globally if
# we don't already have a recent enough version.
need_pie_install=true
if command -v pie >/dev/null 2>&1; then
  current="$(pie --version 2>&1 | grep -oE '[0-9]+\.[0-9]+\.[0-9]+' | head -1 || echo '0.0.0')"
  if printf '%s\n%s\n' "1.3.7" "$current" | sort -V -C; then
    need_pie_install=false
  fi
fi
if [[ "$need_pie_install" == "true" ]]; then
  composer global require php/pie:^1.4
  PIE="$(composer config -g home)/vendor/bin/pie"
else
  PIE="pie"
fi

# Install the extension binary into the running PHP's extension dir.
"$PIE" install "kreuzberg-dev/tree-sitter-language-pack:$VERSION" --skip-enable-extension

# Verify the .so loads.
EXT_DIR="$(php -r 'echo ini_get("extension_dir");')"
test -f "$EXT_DIR/tree_sitter_language_pack.so" || test -f "$EXT_DIR/tree_sitter_language_pack.dylib" || test -f "$EXT_DIR/tree_sitter_language_pack.dll"

# Load it explicitly for the smoke test (the verify-install action runs
# phpunit with this same `-dextension=` flag in CI).
if ! php -dextension=tree_sitter_language_pack -m | grep -qi tree_sitter_language_pack; then
  echo "::error::tree_sitter_language_pack extension failed to load after PIE install" >&2
  exit 1
fi
echo "tree_sitter_language_pack extension installed and loaded"
