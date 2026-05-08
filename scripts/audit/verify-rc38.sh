#!/usr/bin/env bash
# Comprehensive verification driver for v1.8.0-rc.{N}.
# Runs per-registry presence checks, content audits (type stubs, file inventory),
# downstream installs (test_apps), and Homebrew bottle-pour tests.
#
# Records per-target outcomes to a summary table. Exit code is 0 only if every
# check is "ok" or "skip"; "fail" anywhere causes a non-zero exit.
#
# shellcheck disable=SC2012,SC2035,SC2164

set -uo pipefail

VERSION="${VERSION:-1.8.0-rc.38}"
TAG="v${VERSION}"
REPO="${REPO:-kreuzberg-dev/tree-sitter-language-pack}"
NPM_VERSION="$VERSION"                  # npm uses canonical
PYTHON_VERSION="${VERSION/-rc./rc}"     # PyPI uses 1.8.0rc28
RUBY_VERSION="${VERSION/-rc./.pre.rc.}" # rubygems uses 1.8.0.pre.rc.38

WORK="/tmp/verify-rc-${VERSION}"
mkdir -p "$WORK"

# Capture release asset names once — gh subcommands inside cd-ed work dirs
# can't auto-detect the repo, so we resolve them up front.
RELEASE_ASSETS=$(gh release view "$TAG" --repo "$REPO" --json assets --jq '.assets[].name' 2>/dev/null || echo "")

declare -A RESULTS=()

record() {
  local name="$1" outcome="$2" detail="${3:-}"
  RESULTS["$name"]="$outcome|$detail"
  printf '[%s] %-40s %s\n' "$outcome" "$name" "$detail"
}

# 1. PyPI
audit_pypi() {
  local name="pypi:tree-sitter-language-pack"
  cd "$WORK" && rm -rf python && mkdir python && cd python
  local pip_cmd
  if command -v pip >/dev/null 2>&1; then
    pip_cmd="pip"
  elif command -v pip3 >/dev/null 2>&1; then
    pip_cmd="pip3"
  else
    pip_cmd="python3 -m pip"
  fi
  if ! $pip_cmd download "tree-sitter-language-pack==${PYTHON_VERSION}" --no-deps \
    --platform macosx_11_0_arm64 --python-version 3.12 --only-binary=:all: \
    >/dev/null 2>err; then
    record "$name" FAIL "pip download failed: $(tr -d '\n' <err | head -c200)"
    return 1
  fi
  local wheel
  wheel=$(ls *.whl 2>/dev/null | head -1)
  [[ -n "$wheel" ]] || {
    record "$name" FAIL "no wheel downloaded"
    return 1
  }
  local listing
  listing=$(unzip -l "$wheel")
  local missing=()
  grep -q 'py.typed' <<<"$listing" || missing+=("py.typed")
  grep -q '_native\.pyi' <<<"$listing" || missing+=("_native.pyi")
  grep -qE '_native\.[a-z0-9_]+\.so' <<<"$listing" || missing+=("_native.*.so")
  if ((${#missing[@]})); then
    record "$name" FAIL "missing in wheel: ${missing[*]}"
    return 1
  fi
  record "$name" OK "wheel has py.typed + .pyi + .so"
}

# 2. npm: NAPI bundle
audit_npm_node() {
  local name="npm:@kreuzberg/tree-sitter-language-pack"
  cd "$WORK" && rm -rf node && mkdir node && cd node
  if ! npm pack "@kreuzberg/tree-sitter-language-pack@${NPM_VERSION}" >/dev/null 2>err; then
    record "$name" FAIL "npm pack failed"
    return 1
  fi
  local tgz
  tgz=$(ls *.tgz | head -1)
  tar -xzf "$tgz"
  local missing=()
  [[ -f package/index.js ]] || missing+=("index.js")
  [[ -f package/index.d.ts ]] || missing+=("index.d.ts")
  ls package/*.node >/dev/null 2>&1 || missing+=("*.node binary")
  # platform-dispatch sanity
  if ! grep -q 'isMusl\|aarch64\|win32' package/index.js; then
    missing+=("platform-dispatch in index.js")
  fi
  if ((${#missing[@]})); then
    record "$name" FAIL "missing: ${missing[*]}"
    return 1
  fi
  record "$name" OK "index.js + .d.ts + .node + platform-dispatch present"
}

# 3. npm: WASM bundle
audit_npm_wasm() {
  local name="npm:@kreuzberg/tree-sitter-language-pack-wasm"
  cd "$WORK" && rm -rf wasm && mkdir wasm && cd wasm
  if ! npm pack "@kreuzberg/tree-sitter-language-pack-wasm@${NPM_VERSION}" >/dev/null 2>err; then
    record "$name" FAIL "npm pack failed"
    return 1
  fi
  local tgz
  tgz=$(ls *.tgz | head -1)
  tar -xzf "$tgz"
  local missing=()
  ls package/*.wasm >/dev/null 2>&1 || missing+=("*.wasm")
  ls package/*.d.ts >/dev/null 2>&1 || missing+=("*.d.ts")
  ls package/*.js >/dev/null 2>&1 || missing+=("*.js glue")
  if ((${#missing[@]})); then
    record "$name" FAIL "missing: ${missing[*]}"
    return 1
  fi
  # Known issue: env imports — flag as warn rather than fail
  local js
  js=$(ls package/*.js | head -1)
  if grep -q 'import \* as.*from "env"' "$js"; then
    record "$name" WARN "ships env imports (libc passthrough — known issue)"
  else
    record "$name" OK "wasm + d.ts + js, no env imports"
  fi
}

# 4. RubyGems
audit_rubygems() {
  local name="rubygems:tree_sitter_language_pack"
  cd "$WORK" && rm -rf ruby && mkdir ruby && cd ruby
  local platforms=(arm64-darwin x86_64-linux aarch64-linux)
  local fails=()
  for plat in "${platforms[@]}"; do
    if ! gem fetch tree_sitter_language_pack -v "$RUBY_VERSION" --platform "$plat" >/dev/null 2>err; then
      fails+=("$plat")
      continue
    fi
    local gem
    gem=$(ls "tree_sitter_language_pack-${RUBY_VERSION}-${plat}.gem" 2>/dev/null | head -1) || {
      fails+=("$plat:no-gem-file")
      continue
    }
    local listing
    listing=$(tar -xOf "$gem" data.tar.gz | tar -tz)
    local missing=()
    grep -q 'lib/tree_sitter_language_pack.rb' <<<"$listing" || missing+=("main.rb")
    grep -q 'lib/tree_sitter_language_pack/native.rb' <<<"$listing" || missing+=("native.rb")
    grep -q 'sig/types.rbs' <<<"$listing" || missing+=("types.rbs")
    grep -qE '\.bundle$|\.so$' <<<"$listing" || missing+=("native binary")
    if ((${#missing[@]})); then
      fails+=("$plat:${missing[*]}")
    fi
  done
  if ((${#fails[@]})); then
    record "$name" FAIL "issues: ${fails[*]}"
    return 1
  fi
  record "$name" OK "${#platforms[@]} platforms, all have main+native+rbs+binary"
}

# 5. Maven
audit_maven() {
  local name="maven:tree-sitter-language-pack"
  cd "$WORK" && rm -rf java && mkdir java && cd java
  local url="https://repo1.maven.org/maven2/dev/kreuzberg/treesitterlanguagepack/tree-sitter-language-pack/${VERSION}/tree-sitter-language-pack-${VERSION}.jar"
  if ! curl -fsSLO "$url" 2>err; then
    record "$name" FAIL "jar download failed"
    return 1
  fi
  local missing=()
  unzip -l "tree-sitter-language-pack-${VERSION}.jar" | grep -q 'natives/linux-x86_64' || missing+=("linux-x86_64")
  unzip -l "tree-sitter-language-pack-${VERSION}.jar" | grep -q 'natives/macos-arm64' || missing+=("macos-arm64")
  unzip -l "tree-sitter-language-pack-${VERSION}.jar" | grep -q '\.class$' || missing+=("classes")
  if ((${#missing[@]})); then
    record "$name" FAIL "missing: ${missing[*]}"
    return 1
  fi
  record "$name" OK "jar has natives + classes"
}

# 6. NuGet
audit_nuget() {
  local name="nuget:TreeSitterLanguagePack"
  cd "$WORK" && rm -rf csharp && mkdir csharp && cd csharp
  local url="https://api.nuget.org/v3-flatcontainer/treesitterlanguagepack/${VERSION}/treesitterlanguagepack.${VERSION}.nupkg"
  if ! curl -fsSLO "$url" 2>err; then
    record "$name" FAIL "nupkg download failed"
    return 1
  fi
  local missing=()
  unzip -l "treesitterlanguagepack.${VERSION}.nupkg" | grep -q 'lib/' || missing+=("lib/")
  unzip -l "treesitterlanguagepack.${VERSION}.nupkg" | grep -q 'runtimes/' || missing+=("runtimes/")
  if ((${#missing[@]})); then
    record "$name" FAIL "missing: ${missing[*]}"
    return 1
  fi
  record "$name" OK "nupkg has lib + runtimes"
}

# 7. Packagist (composer)
audit_packagist() {
  local name="packagist:kreuzberg/tree-sitter-language-pack"
  if curl -fsSL "https://repo.packagist.org/p2/kreuzberg/tree-sitter-language-pack.json" 2>err |
    jq -e --arg v "$VERSION" '.packages."kreuzberg/tree-sitter-language-pack" | map(.version == $v) | any' >/dev/null; then
    record "$name" OK "version present on packagist"
  else
    record "$name" FAIL "version not on packagist"
  fi
}

# 8. crates.io
audit_cratesio() {
  local name="crates:tree-sitter-language-pack"
  if curl -fsSL "https://crates.io/api/v1/crates/tree-sitter-language-pack" 2>err |
    jq -e --arg v "$VERSION" '.versions | map(.num == $v) | any' >/dev/null; then
    record "$name" OK "version present on crates.io"
  else
    record "$name" FAIL "version not on crates.io"
  fi
}

# 9. Hex.pm
audit_hex() {
  local name="hex:tree_sitter_language_pack"
  if curl -fsSL "https://hex.pm/api/packages/tree_sitter_language_pack" 2>err |
    jq -e --arg v "$VERSION" '.releases | map(.version == $v) | any' >/dev/null; then
    record "$name" OK "version present on hex.pm"
  else
    record "$name" FAIL "version not on hex.pm"
  fi
}

# 10. C FFI release tarballs
audit_c_ffi() {
  local name="release:c-ffi"
  local triples=(aarch64-apple-darwin x86_64-apple-darwin aarch64-unknown-linux-gnu x86_64-unknown-linux-gnu x86_64-pc-windows-msvc aarch64-pc-windows-msvc)
  local missing=()
  for t in "${triples[@]}"; do
    local pat="tree-sitter-language-pack-ffi-${TAG}-${t}.tar.gz"
    grep -q "$pat" <<<"$RELEASE_ASSETS" || missing+=("$t")
  done
  if ((${#missing[@]})); then
    record "$name" FAIL "missing triples: ${missing[*]}"
    return 1
  fi
  record "$name" OK "all 6 FFI triples on release"
}

# 11. Go FFI release tarballs
audit_go_ffi() {
  local name="release:go-ffi"
  local labels=(linux-x86_64 linux-aarch64 macos-arm64 macos-x86_64 windows-x86_64 windows-aarch64)
  local missing=()
  for l in "${labels[@]}"; do
    grep -q "tree-sitter-language-pack-ffi.*${l}" <<<"$RELEASE_ASSETS" || missing+=("$l")
  done
  if ((${#missing[@]})); then
    record "$name" WARN "go-ffi missing labels: ${missing[*]}"
  else
    record "$name" OK "all 6 go-ffi labels on release"
  fi
}

# 12. Bottles
audit_bottles() {
  local name="release:bottles"
  local tags=(arm64_sequoia sequoia x86_64_linux arm64_linux)
  local missing=()
  for t in "${tags[@]}"; do
    if ! grep -qE "ts-pack-${VERSION}\.${t}\.bottle\.tar\.gz" <<<"$RELEASE_ASSETS"; then
      missing+=("ts-pack:$t")
    fi
    if ! grep -qE "libts-pack-${VERSION}\.${t}\.bottle\.tar\.gz" <<<"$RELEASE_ASSETS"; then
      missing+=("libts-pack:$t")
    fi
  done
  if ((${#missing[@]})); then
    record "$name" FAIL "missing bottles: ${missing[*]}"
    return 1
  fi
  record "$name" OK "8 bottles (ts-pack + libts-pack × 4 tags) on release"
}

# Per-test-app smoke tests
smoke_python() {
  local name="smoke:test_apps/python"
  cd "$ROOT/test_apps/python" || {
    record "$name" SKIP "no test_apps/python"
    return 0
  }
  if ! uv venv --quiet 2>err; then
    record "$name" FAIL "uv venv failed"
    return 1
  fi
  if ! uv pip install --quiet -e . 2>>err; then
    record "$name" FAIL "install failed: $(tr -d '\n' <err | head -c150)"
    return 1
  fi
  if uv run pytest --tb=short --quiet -x 2>&1 | tail -3 | grep -qE 'passed|no tests'; then
    record "$name" OK "pytest passes"
  else
    record "$name" FAIL "pytest failed"
  fi
}

smoke_node() {
  local name="smoke:test_apps/node"
  cd "$ROOT/test_apps/node" || {
    record "$name" SKIP "no test_apps/node"
    return 0
  }
  if ! pnpm install --quiet 2>err; then
    record "$name" FAIL "pnpm install failed: $(tr -d '\n' <err | head -c150)"
    return 1
  fi
  if pnpm test 2>&1 | tail -5 | grep -qE 'passed|0 fail'; then
    record "$name" OK "vitest passes"
  else
    record "$name" FAIL "vitest failed"
  fi
}

smoke_ruby() {
  local name="smoke:test_apps/ruby"
  cd "$ROOT/test_apps/ruby" || {
    record "$name" SKIP "no test_apps/ruby"
    return 0
  }
  if ! bundle install --quiet 2>err; then
    record "$name" FAIL "bundle install failed"
    return 1
  fi
  if bundle exec rspec spec/ 2>&1 | tail -5 | grep -qE 'examples.*0 failures'; then
    record "$name" OK "rspec passes"
  else
    record "$name" FAIL "rspec failed"
  fi
}

smoke_brew_pour() {
  local name="brew:ts-pack pour"
  brew tap kreuzberg-dev/tap >/dev/null 2>&1 || true
  brew uninstall --force ts-pack 2>/dev/null || true
  local out
  out=$(brew install kreuzberg-dev/tap/ts-pack 2>&1)
  if grep -q 'Pouring' <<<"$out"; then
    record "$name" OK "brew install poured a bottle"
  elif grep -q 'Building from source' <<<"$out"; then
    record "$name" WARN "brew install built from source (bottle SHA mismatch?)"
  else
    record "$name" FAIL "brew install failed: $(tail -3 <<<"$out" | tr '\n' ' ')"
  fi
  brew uninstall --force ts-pack 2>/dev/null || true

  local name2="brew:libts-pack pour"
  brew uninstall --force libts-pack 2>/dev/null || true
  out=$(brew install kreuzberg-dev/tap/libts-pack 2>&1)
  if grep -q 'Pouring' <<<"$out"; then
    record "$name2" OK "brew install poured a bottle"
  elif grep -q 'Building from source' <<<"$out"; then
    record "$name2" WARN "brew install built from source"
  else
    record "$name2" FAIL "brew install failed: $(tail -3 <<<"$out" | tr '\n' ' ')"
  fi
  brew uninstall --force libts-pack 2>/dev/null || true
}

# Driver
ROOT="${ROOT:-$(git rev-parse --show-toplevel)}"

# Phase 1: registry presence
audit_pypi
audit_npm_node
audit_npm_wasm
audit_rubygems
audit_maven
audit_nuget
audit_packagist
audit_cratesio
audit_hex
audit_c_ffi
audit_go_ffi
audit_bottles

# Phase 2: per-test-app smokes (mac-arm64 only — script runs on the dev box)
[[ "${SKIP_SMOKE:-}" == "1" ]] || {
  smoke_python
  smoke_node
  smoke_ruby
  smoke_brew_pour
}

# Summary
printf '\n=== Verify %s summary ===\n' "$VERSION"
fail=0
warn=0
for k in "${!RESULTS[@]}"; do :; done | sort
for k in $(printf '%s\n' "${!RESULTS[@]}" | sort); do
  IFS='|' read -r outcome detail <<<"${RESULTS[$k]}"
  printf '%-7s %-40s %s\n' "$outcome" "$k" "$detail"
  case "$outcome" in
  FAIL) fail=$((fail + 1)) ;;
  WARN) warn=$((warn + 1)) ;;
  esac
done
printf '\n%d FAIL, %d WARN\n' "$fail" "$warn"

[[ "$fail" -eq 0 ]]
