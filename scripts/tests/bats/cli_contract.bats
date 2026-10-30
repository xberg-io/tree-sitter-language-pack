setup() {
	bats_load_library xberg-bats
	xberg_setup
	ROOT="$(cd "$BATS_TEST_DIRNAME/../../.." && pwd -P)"
	export CLI_TRACE="$XBERG_WORK/cli-trace"
	xberg_stub ts-pack 'printf "%s\n" "$*" >>"$CLI_TRACE"
case "$1" in
  --version) printf "ts-pack 1.0.0\n" ;;
  --help) printf "Usage: ts-pack COMMAND\n" ;;
  list)
    if [ "${CLI_EMPTY_REGISTRY:-0}" = 1 ]; then printf "0 language(s)\n"
    elif [ "$#" -gt 1 ]; then printf "python\n1 language(s)\n"
    else printf "python\nrust\n2 language(s)\n"; fi ;;
  info) printf "Language: python\n" ;;
  parse)
    [ -n "$(cat)" ] || exit 43
    [ "${4:-}" != "${CLI_FAIL_LANGUAGE:-}" ] || exit 42
    printf "(module)\n" ;;
  *) exit 44 ;;
esac'
	export TS_PACK_BIN="$XBERG_STUB_BIN/ts-pack" TS_PACK_LANGUAGES=python,rust
}

@test "CLI Bats ports exercise all smoke commands and selected grammars" {
	run env MISE_CONFIG_ROOT="$ROOT" bash "$ROOT/mise-tasks/test/cli.sh"
	xberg_assert_status 0
	xberg_assert_output_contains '1..7'
	[ "$(wc -l <"$CLI_TRACE" | tr -d ' ')" -eq 8 ]
	[ "$(grep -c '^parse - --language' "$CLI_TRACE")" -eq 3 ]
}

@test "CLI grammar Bats gate rejects a parser failure for the selected language" {
	run env CLI_FAIL_LANGUAGE=rust bats "$ROOT/test_apps/cli/grammars.bats"
	xberg_assert_status 1
	xberg_assert_output_contains 'grammar failed: rust'
	[ "$(wc -l <"$CLI_TRACE" | tr -d ' ')" -eq 2 ]
}

@test "CLI smoke Bats gate rejects an empty registry despite a footer" {
	run env CLI_EMPTY_REGISTRY=1 bats "$ROOT/test_apps/cli/smoke.bats"
	xberg_assert_status 1
	xberg_assert_output_contains 'expected at least 1 language, got 0'
}
