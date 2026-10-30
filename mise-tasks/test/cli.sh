#!/usr/bin/env bash
#MISE description="Run CLI Bats contracts against TS_PACK_BIN or the installed ts-pack"
set -euo pipefail
export BATS_LIB_PATH="$MISE_CONFIG_ROOT/scripts/tests/lib"
bats --print-output-on-failure "$MISE_CONFIG_ROOT/test_apps/cli"
