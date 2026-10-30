setup() {
	bats_load_library xberg-bats
	xberg_setup
	ROOT="$(cd "$BATS_TEST_DIRNAME/../../.." && pwd -P)"
	mkdir -p "$XBERG_WORK/scripts" "$XBERG_WORK/packages/kotlin-android" "$XBERG_WORK/crates/ts-pack-core-wasm"
	cp "$ROOT"/scripts/*.sh "$XBERG_WORK/scripts/"
	cd "$XBERG_WORK" || return
}

@test "Swift bridge staging preserves its guarded header and imports" {
	out=target/release/build/tree-sitter-language-pack-swift-fixture/out
	mkdir -p "$out/tree-sitter-language-pack-swift"
	printf 'core header\n' >"$out/SwiftBridgeCore.h"
	printf 'bridge header\n' >"$out/tree-sitter-language-pack-swift/tree-sitter-language-pack-swift.h"
	printf 'core swift\n' >"$out/SwiftBridgeCore.swift"
	printf 'bridge swift\n' >"$out/tree-sitter-language-pack-swift/tree-sitter-language-pack-swift.swift"
	run bash scripts/setup-swift-bridge.sh
	xberg_assert_status 0
	header=packages/swift/Sources/RustBridgeC/RustBridgeC.h
	grep -q '^#ifndef RUST_BRIDGE_C_H' "$header"
	grep -q '^#endif /\* RUST_BRIDGE_C_H \*/' "$header"
	grep -q '^core header$' "$header"
	grep -q '^bridge header$' "$header"
	xberg_assert_file packages/swift/Sources/RustBridge/SwiftBridgeCore.swift $'import RustBridgeC\ncore swift'
	xberg_assert_file packages/swift/Sources/RustBridge/tree-sitter-language-pack-swift.swift $'import RustBridgeC\nbridge swift'
}

@test "Swift bridge staging rejects missing compiler output" {
	mkdir -p target/release/build
	run bash scripts/setup-swift-bridge.sh
	xberg_assert_status 1
	xberg_assert_output_contains 'Could not find swift-bridge build output'
}

@test "Go and Zig staging build grammar-free FFI before using native artifacts" {
	xberg_stub uname 'case "$1" in -s) printf Linux;; -m) printf aarch64;; esac'
	xberg_stub cargo 'test "$TSLP_OFFLINE:$TSLP_LINK_MODE:$PROJECT_ROOT" = "1:dynamic:$XBERG_WORK" || exit 44
printf "%s\n" "$*" >>"$XBERG_TRACE"
mkdir -p target/release
printf "native fixture\n" >target/release/libts_pack_core_ffi.so'
	run bash scripts/stage_go_native.sh
	xberg_assert_status 0
	xberg_assert_file packages/go/.lib/linux-aarch64/libts_pack_core_ffi.so 'native fixture'
	run bash scripts/stage_zig_ffi.sh
	xberg_assert_status 0
	xberg_assert_trace $'build --locked --release -p ts-pack-core-ffi\nbuild --locked --release -p ts-pack-core-ffi'
}

@test "Go and Zig staging reject successful builds that produced no library" {
	xberg_stub uname 'case "$1" in -s) printf Linux;; -m) printf x86_64;; esac'
	xberg_stub_exit cargo 0
	run bash scripts/stage_go_native.sh
	xberg_assert_status 1
	xberg_assert_output_contains 'does not exist'
	run bash scripts/stage_zig_ffi.sh
	xberg_assert_status 1
	xberg_assert_output_contains 'no libts_pack_core_ffi'
}

@test "Kotlin staging records the explicit SDK before compiling classes" {
	export ANDROID_HOME="$XBERG_WORK/android-sdk"
	mkdir -p "$ANDROID_HOME/platforms"
	xberg_stub_trace gradlew
	cp "$XBERG_STUB_BIN/gradlew" packages/kotlin-android/gradlew
	run bash scripts/stage_kotlin_android_sdk.sh
	xberg_assert_status 0
	xberg_assert_file packages/kotlin-android/local.properties "sdk.dir=$ANDROID_HOME"
	xberg_assert_trace 'gradlew compileDebugKotlin --no-daemon'
}

@test "Kotlin staging propagates compilation failure" {
	export ANDROID_HOME="$XBERG_WORK/android-sdk"
	mkdir -p "$ANDROID_HOME/platforms"
	xberg_stub_exit gradlew 31
	cp "$XBERG_STUB_BIN/gradlew" packages/kotlin-android/gradlew
	run bash scripts/stage_kotlin_android_sdk.sh
	xberg_assert_status 31
}

wasm_tools() {
	xberg_stub_exit rustup 0
	xberg_stub_exit tree-sitter 0
	mkdir -p "$XBERG_WORK/canonical/bin"
	cp "$XBERG_STUB_BIN/tree-sitter" "$XBERG_WORK/canonical/bin/tree-sitter"
	xberg_stub_exit tree-sitter 99
	xberg_stub mise 'test "$#" -eq 4 && test "$1" = -C && test "$2" = "$XBERG_WORK" && test "$3" = which && test "$4" = tree-sitter || exit 89
printf "%s/canonical/bin/tree-sitter\n" "$XBERG_WORK"'
	xberg_stub npm 'printf "unexpected global install\n" >&2; exit 99'
	xberg_stub uv 'test "$TSLP_LANGUAGES" = mojo,nim,norg || exit 43
test "$(command -v tree-sitter)" = "$XBERG_WORK/canonical/bin/tree-sitter" || exit 45
tree-sitter --version
printf "uv %s\n" "$*" >>"$XBERG_TRACE"'
}

@test "Wasm staging prepares its three skipped grammars and declaration package" {
	wasm_tools
	xberg_stub wasm-pack 'test "$TSLP_LANGUAGES:$TSLP_LINK_MODE:$PROJECT_ROOT" = "mojo,nim,norg:static:$XBERG_WORK" || exit 44
printf "wasm-pack %s\n" "$*" >>"$XBERG_TRACE"
mkdir -p pkg/nodejs
printf "export declare function process(): void;\n" >pkg/nodejs/ts_pack_core_wasm.d.ts'
	run bash scripts/stage_wasm_types.sh
	xberg_assert_status 0
	xberg_assert_trace $'uv run --no-sync scripts/clone_vendors.py\nwasm-pack build --release --target nodejs --out-dir pkg/nodejs'
	xberg_assert_file crates/ts-pack-core-wasm/pkg/nodejs/ts_pack_core_wasm.d.ts 'export declare function process(): void;'
}

@test "Wasm staging refuses a successful build that omitted declarations" {
	wasm_tools
	xberg_stub_exit wasm-pack 0
	run bash scripts/stage_wasm_types.sh
	xberg_assert_status 1
	xberg_assert_output_contains 'does not exist'
}
