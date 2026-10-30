setup() {
	bats_load_library xberg-bats
	xberg_setup
	BIN="${TS_PACK_BIN:-ts-pack}"
}

@test "CLI version command succeeds" {
	run "$BIN" --version
	xberg_assert_status 0
	xberg_assert_output_contains ts-pack
}

@test "CLI help command succeeds" {
	run "$BIN" --help
	xberg_assert_status 0
	xberg_assert_output_contains Usage
}

@test "CLI lists at least one language rather than only a footer" {
	run "$BIN" list
	xberg_assert_status 0
	count=$(printf '%s\n' "$output" | awk 'NF && !/language\(s\)/ { count++ } END { print count + 0 }')
	[ "$count" -gt 0 ] || {
		printf 'expected at least 1 language, got %s\n' "$count" >&2
		return 1
	}
}

@test "CLI filters languages by name" {
	run "$BIN" list --filter python
	xberg_assert_status 0
	xberg_assert_output_contains python
}

@test "CLI reports information about a known language" {
	run "$BIN" info python
	xberg_assert_status 0
	xberg_assert_output_contains python
}

@test "CLI parses Python from standard input" {
	run bash -c 'printf "%s\n" "x = 1" | "$1" parse - --language python' _ "$BIN"
	xberg_assert_status 0
	xberg_assert_output_contains module
}
