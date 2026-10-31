setup() {
	bats_load_library xberg-bats
	xberg_setup
	ROOT="$(cd "$BATS_TEST_DIRNAME/../../.." && pwd -P)"
	mkdir -p "$XBERG_WORK/packages/elixir"
	cd "$XBERG_WORK" || return
}

@test "PHP source build rejects unsupported series and relative prefixes before download" {
	xberg_stub_curl_offline
	run bash "$ROOT/scripts/ci/build-php-source.sh" 8.1 "$XBERG_WORK/php"
	xberg_assert_status 1
	xberg_assert_output_contains 'Unsupported PHP series'
	run bash "$ROOT/scripts/ci/build-php-source.sh" 8.4 relative-prefix
	xberg_assert_status 1
	xberg_assert_output_contains 'Installation prefix must be absolute'
}

@test "PHP source build verifies the archive before configuring and installing tools" {
	prefix="$XBERG_WORK/php"
	source="$prefix-build/php-8.4.25"
	mkdir -p "$source" "$prefix/bin"
	printf 'archive fixture\n' >"$XBERG_WORK/archive"
	xberg_stub_curl_file "$XBERG_WORK/archive"
	xberg_stub shasum 'cat >/dev/null; printf "shasum %s\n" "$*" >>"$XBERG_TRACE"'
	xberg_stub_trace tar
	xberg_stub_trace configure
	cp "$XBERG_STUB_BIN/configure" "$source/configure"
	xberg_stub_trace make
	xberg_stub sysctl 'printf 2'
	for tool in php php-config phpize; do
		xberg_stub_trace "$tool"
		cp "$XBERG_STUB_BIN/$tool" "$prefix/bin/$tool"
	done
	run bash "$ROOT/scripts/ci/build-php-source.sh" 8.4 "$prefix"
	xberg_assert_status 0
	xberg_assert_trace "$(printf '%s\n' \
		'shasum -a 256 --check' \
		"tar -xJf $prefix-build/php-8.4.25.tar.xz -C $prefix-build" \
		"configure --prefix=$prefix --disable-all --enable-cli --disable-cgi --disable-phpdbg --without-pear" \
		'make -j 2' 'make install' \
		'php -n -r if (PHP_ZTS || PHP_DEBUG) { exit(1); } echo PHP_VERSION, PHP_EOL;' \
		'php-config --version' 'phpize --version')"
	xberg_assert_github_path "$prefix/bin"
	xberg_assert_github_env "PHP_CONFIG=$prefix/bin/php-config"
}

@test "PHP source build rejects archive checksum mismatch before extraction" {
	printf 'invalid archive\n' >"$XBERG_WORK/archive"
	xberg_stub_curl_file "$XBERG_WORK/archive"
	xberg_stub_trace tar
	run bash "$ROOT/scripts/ci/build-php-source.sh" 8.4 "$XBERG_WORK/php"
	xberg_assert_status 1
	xberg_assert_output_contains 'FAILED'
	xberg_assert_trace_empty
}

@test "E2E manifest wrapper reads the workspace version and forwards bundle paths" {
	mkdir -p "$XBERG_WORK/scripts" "$XBERG_WORK/bundle" "$XBERG_WORK/sources"
	printf 'version = "1.2.3"\n' >Cargo.toml
	touch bundle/parsers-linux-x86_64.tar.zst
	cat >scripts/build_e2e_manifest.py <<'PY'
import os
import pathlib
import sys
pathlib.Path(os.environ['XBERG_TRACE']).write_text('\n'.join(sys.argv[1:])+'\n')
PY
	export PLATFORM_LABEL=linux-x86_64 BUNDLE_DIR="$XBERG_WORK/bundle" GITHUB_WORKSPACE="$XBERG_WORK"
	run bash "$ROOT/scripts/ci/prepare_e2e_manifest.sh"
	xberg_assert_status 0
	xberg_assert_trace "$(printf '%s\n' --version 1.2.3 --platform-label linux-x86_64 \
		--bundle "$BUNDLE_DIR/parsers-linux-x86_64.tar.zst" --definitions "$XBERG_WORK/sources/language_definitions.json" \
		--output "$BUNDLE_DIR/parsers.json")"
	xberg_assert_github_env "TREE_SITTER_LANGUAGE_PACK_MANIFEST_URL=file://$BUNDLE_DIR/parsers.json"
}

@test "E2E manifest wrapper refuses missing bundle before running the generator" {
	mkdir -p bundle
	export PLATFORM_LABEL=linux-x86_64 BUNDLE_DIR="$XBERG_WORK/bundle" GITHUB_WORKSPACE="$XBERG_WORK"
	run bash "$ROOT/scripts/ci/prepare_e2e_manifest.sh"
	xberg_assert_status 1
	xberg_assert_output_contains 'bundle missing'
	xberg_assert_trace_empty
}

@test "CSharp runtime packaging uses concrete graph entries in sorted order" {
	project=packages/csharp/TreeSitterLanguagePack
	mkdir -p "$project"
	printf '<Project><PropertyGroup><PackageId>Pack</PackageId></PropertyGroup></Project>' >"$project/TreeSitterLanguagePack.csproj"
	printf '{"runtimes":{"win-x64":{"Pack":{}},"linux-x64":{"Pack":{}},"alias":{"#import":["linux-x64"]}}}' >"$project/runtime.json.template"
	xberg_stub_trace dotnet
	run bash "$ROOT/scripts/publish/csharp/pack-runtimes.sh" 1.2.3 output
	xberg_assert_status 0
	base='dotnet pack packages/csharp/TreeSitterLanguagePack.Runtime/TreeSitterLanguagePack.Runtime.csproj -c Release -p:Version=1.2.3'
	xberg_assert_trace "$base -p:PublishedRID=linux-x64 -o output"$'\n'"$base -p:PublishedRID=win-x64 -o output"
}

@test "Elixir checksums cover all sixteen target and NIF filename combinations" {
	export NIF_FIXTURE="$XBERG_WORK/nif"
	printf 'nif fixture\n' >"$NIF_FIXTURE"
	xberg_stub curl 'while [ "$#" -gt 0 ]; do
  if [ "$1" = -o ]; then output="$2"; shift; else url="$1"; fi
  shift
done
printf "%s\n" "$url" >>"$XBERG_TRACE"
cp "$NIF_FIXTURE" "$output"'
	run env -u GH_TOKEN -u GITHUB_TOKEN /bin/bash "$ROOT/scripts/publish/elixir/generate_checksums.sh" 1.2.3
	xberg_assert_status 0
	target=packages/elixir/checksum-Elixir.TreeSitterLanguagePack.Native.exs
	[ "$(grep -c '=> "sha256:' "$target")" -eq 16 ]
	[ "$(sort -u "$XBERG_TRACE" | wc -l | tr -d ' ')" -eq 16 ]
	digest="$(shasum -a 256 "$NIF_FIXTURE" | cut -d' ' -f1)"
	[ "$(grep -c "$digest" "$target")" -eq 16 ]
	xberg_assert_output_contains 'Generated checksums for 16 files'
}

@test "Elixir checksums exhaust ten retries and reject incomplete publication" {
	xberg_stub curl 'printf "curl\n" >>"$XBERG_TRACE"; exit 22'
	xberg_stub sleep 'printf "sleep %s\n" "$*" >>"$XBERG_TRACE"'
	run env -u GH_TOKEN -u GITHUB_TOKEN bash "$ROOT/scripts/publish/elixir/generate_checksums.sh" 1.2.3
	xberg_assert_status 1
	[ "$(grep -c '^curl$' "$XBERG_TRACE")" -eq 10 ]
	[ "$(grep -c '^sleep 15$' "$XBERG_TRACE")" -eq 10 ]
	[ ! -e packages/elixir/checksum-Elixir.TreeSitterLanguagePack.Native.exs ]
}

@test "shared Bats helper sync rejects mutable refs and detects byte drift" {
	xberg_stub_curl_file "$ROOT/scripts/tests/lib/xberg-bats/load.bash"
	export BATS_LIB_DEST="$XBERG_WORK/lib" BATS_LIB_REF=main
	run bash "$ROOT/scripts/task/sync-bats-lib.sh" sync
	xberg_assert_status 1
	export BATS_LIB_REF=v1.29.0
	run bash "$ROOT/scripts/task/sync-bats-lib.sh" sync
	xberg_assert_status 0
	run bash "$ROOT/scripts/task/sync-bats-lib.sh" --check
	xberg_assert_status 0
	printf '\nchanged\n' >>"$BATS_LIB_DEST/load.bash"
	run bash "$ROOT/scripts/task/sync-bats-lib.sh" --check
	xberg_assert_status 1
	xberg_assert_output_contains 'has drifted'
}
