#!/usr/bin/env bash
# Skills name `scripts/mcp-launch.sh`; this entry point runs the plugin launcher.
set -euo pipefail

exec "$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)/plugin/scripts/mcp-launch.sh" "$@"
