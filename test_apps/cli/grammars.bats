setup() {
	bats_load_library xberg-bats
	xberg_setup
	BIN="${TS_PACK_BIN:-ts-pack}"
}

source_for() {
	case "$1" in
	python | wolfram) printf 'x = 1' ;;
	rust) printf 'fn main() {}' ;;
	javascript) printf 'const x = 1;' ;;
	typescript) printf 'const x: number = 1;' ;;
	go) printf 'package main' ;;
	html) printf '<html></html>' ;;
	css) printf 'body { color: red; }' ;;
	json) printf '{"key": "value"}' ;;
	mojo) printf 'fn main(): None: pass' ;;
	nim) printf 'echo "hi"' ;;
	norg) printf '* Heading' ;;
	*) printf '// %s' "$1" ;;
	esac
}

@test "selected static grammars parse their source snippets" {
	IFS=',' read -r -a languages <<<"${TS_PACK_LANGUAGES:-python,rust,javascript,typescript,go,html,css,json}"
	[ "${#languages[@]}" -gt 0 ]
	for language in "${languages[@]}"; do
		[ -n "$language" ]
		source="$(source_for "$language")"
		run bash -c 'printf "%s\n" "$1" | "$2" parse - --language "$3"' _ "$source" "$BIN" "$language"
		if [ "$status" -ne 0 ]; then
			printf 'grammar failed: %s\n' "$language" >&2
		fi
		xberg_assert_status 0
	done
}
