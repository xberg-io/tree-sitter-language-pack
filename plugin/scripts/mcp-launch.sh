#!/usr/bin/env bash
set -euo pipefail

REPO="xberg-io/tree-sitter-language-pack"
# ~keep Rewritten by scripts/sync_plugin_version.py on every release. The plugin only accepts a
# ~keep binary at or above this version, and downloads exactly this release, so a plugin update
# ~keep cannot keep running an older ts-pack that happens to be installed.
PINNED_VERSION="1.27.4"

log() { printf 'ts-pack-launch: %s\n' "$*" >&2; }
die() {
  log "error: $*"
  exit 1
}

LAUNCHER="${TS_PACK_LAUNCHER:-auto}"
case "$LAUNCHER" in
auto | npx | brew | download) ;;
*) die "invalid TS_PACK_LAUNCHER='$LAUNCHER' (expected auto|npx|brew|download)" ;;
esac

want() { [ "$LAUNCHER" = "auto" ] || [ "$LAUNCHER" = "$1" ]; }
# brew is never part of auto: it needs network and a package manager a sandbox does not have.
want_explicit() { [ "$LAUNCHER" = "$1" ]; }

PLUGIN_ROOT="${CLAUDE_PLUGIN_ROOT:-}"
if [ -z "$PLUGIN_ROOT" ]; then
  PLUGIN_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
fi

BINARY_NAME="ts-pack"
case "$(uname -s)" in
MINGW* | MSYS* | CYGWIN* | Windows_NT) BINARY_NAME="ts-pack.exe" ;;
esac
BIN_DIR="$PLUGIN_ROOT/bin"
BIN="$BIN_DIR/$BINARY_NAME"

have() { command -v "$1" >/dev/null 2>&1; }

# True when $1 runs and reports a version at or above PINNED_VERSION. A prerelease of a version
# sorts below that version (SemVer), so 1.27.0-rc.1 does not satisfy a 1.27.0 pin. ~keep
runnable() {
  local out found base
  [ -x "$1" ] || return 1
  out="$("$1" --version 2>/dev/null)" || return 1
  found="$(printf '%s\n' "$out" | grep -Eo '[0-9]+\.[0-9]+\.[0-9]+(-[0-9A-Za-z.-]+)?' | head -n 1)"
  [ -n "$found" ] || return 1
  base="${found%%-*}"
  if [ "$(printf '%s\n%s\n' "$PINNED_VERSION" "$base" | sort -V | head -n 1)" != "$PINNED_VERSION" ] ||
    { [ "$base" = "$PINNED_VERSION" ] && [ "$found" != "$base" ]; }; then
    log "ignoring $1: version $found is older than the pinned $PINNED_VERSION"
    return 1
  fi
}

if runnable "$BIN"; then
  log "using cached ts-pack at $BIN"
  exec "$BIN" "$@"
fi
if have "$BINARY_NAME"; then
  PATH_BIN="$(command -v "$BINARY_NAME")"
  if runnable "$PATH_BIN"; then
    log "using ts-pack on PATH ($PATH_BIN)"
    exec "$PATH_BIN" "$@"
  fi
fi

NPM_PACKAGE="@xberg-io/ts-pack-cli"
# ~keep npx prints its own progress on stdout unless told otherwise, which would corrupt the MCP
# ~keep stdio stream, so its stdout is never touched: --yes skips the install prompt, and npm's
# ~keep own chatter is silenced or sent to stderr. The version is exact, never a range.
if want npx && have node && have npx; then
  log "running $NPM_PACKAGE@$PINNED_VERSION via npx ..."
  export npm_config_update_notifier=false npm_config_fund=false npm_config_audit=false
  exec npx --yes --loglevel=error "$NPM_PACKAGE@$PINNED_VERSION" "$@"
fi

if want_explicit brew && have brew; then
  log "installing via 'brew install xberg-io/tap/ts-pack' ..."
  if brew install xberg-io/tap/ts-pack >&2; then
    if have "$BINARY_NAME"; then
      BREW_BIN="$(command -v "$BINARY_NAME")"
      runnable "$BREW_BIN" && exec "$BREW_BIN" "$@"
    fi
    log "brew install reported success but ts-pack is not on PATH; falling through"
  else
    log "brew install failed; falling through"
  fi
fi

try_download() {
  local arch triple ext base_url asset asset_url tmp ex src_dir
  arch="$(uname -m)"
  case "$(uname -s)" in
  Darwin)
    case "$arch" in
    arm64 | aarch64) triple="aarch64-apple-darwin" ;;
    *)
      log "no prebuilt macOS archive for $arch (only Apple Silicon is published); falling through"
      return 1
      ;;
    esac
    ;;
  Linux)
    case "$arch" in
    aarch64 | arm64) triple="aarch64-unknown-linux-gnu" ;;
    x86_64) triple="x86_64-unknown-linux-gnu" ;;
    *)
      log "no prebuilt Linux archive for $arch; falling through"
      return 1
      ;;
    esac
    ;;
  MINGW* | MSYS* | CYGWIN* | Windows_NT) triple="x86_64-pc-windows-msvc" ;;
  *)
    log "no prebuilt archive for $(uname -s)/$arch; falling through"
    return 1
    ;;
  esac
  case "$triple" in
  *windows*) ext="zip" ;;
  *) ext="tar.gz" ;;
  esac

  base_url="https://github.com/${REPO}/releases/download/v${PINNED_VERSION}"
  asset="ts-pack-${triple}.${ext}"
  asset_url="${base_url}/${asset}"

  if have curl; then
    fetch() { curl -fsSL --retry 3 -o "$2" "$1"; }
  elif have wget; then
    fetch() { wget -q -O "$2" "$1"; }
  else
    log "no curl or wget available for download; falling through"
    return 1
  fi

  tmp="$(mktemp -d)"
  # shellcheck disable=SC2064  # expand $tmp now so the trap removes this exact dir
  trap "rm -rf '$tmp'" RETURN

  log "downloading $asset from release v$PINNED_VERSION ..."
  if ! fetch "$asset_url" "$tmp/$asset"; then
    log "download failed or asset not found: $asset_url; falling through"
    return 1
  fi

  log "warning: no published checksum for $asset; integrity is verified by HTTPS/TLS only, not a content hash"

  ex="$tmp/extracted"
  mkdir -p "$ex"
  case "$ext" in
  tar.gz) tar -xzf "$tmp/$asset" -C "$ex" || {
    log "extraction failed; falling through"
    return 1
  } ;;
  zip)
    if have unzip; then
      unzip -qo "$tmp/$asset" -d "$ex" || {
        log "extraction failed; falling through"
        return 1
      }
    else
      log "need unzip to extract $asset; falling through"
      return 1
    fi
    ;;
  esac

  src_dir="$ex/ts-pack-${triple}"
  [ -d "$src_dir" ] || src_dir="$ex"
  if [ ! -f "$src_dir/$BINARY_NAME" ]; then
    log "binary $BINARY_NAME not found inside $asset; falling through"
    return 1
  fi

  rm -rf "$BIN_DIR"
  mkdir -p "$BIN_DIR"
  mv "$src_dir"/* "$BIN_DIR"/
  chmod +x "$BIN"
  log "installed ts-pack to $BIN"
  return 0
}

if want download; then
  if try_download; then
    exec "$BIN" "$@"
  fi
fi

die "could not locate or install ts-pack. Install it manually with one of:
  npm install -g ${NPM_PACKAGE}@${PINNED_VERSION}  (or run it with npx; needs node)
  brew install xberg-io/tap/ts-pack
  cargo install ts-pack-cli
  or download a prebuilt archive from https://github.com/${REPO}/releases/tag/v${PINNED_VERSION}
then ensure 'ts-pack' is on PATH and retry."
