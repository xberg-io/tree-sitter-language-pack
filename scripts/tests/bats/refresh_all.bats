#!/usr/bin/env bats
#
# Contract tests for mise-tasks/refresh/all.sh, the body of `mise run update` and `mise run upgrade`.
# Every package manager is a stub that records its working directory and argv, so each test pins
# the full ordered sequence of refresh commands for one mode. ~keep

setup() {
	bats_load_library xberg-bats
	xberg_setup

	SCRIPT="$(cd "$BATS_TEST_DIRNAME/../../.." && pwd -P)/mise-tasks/refresh/all.sh"
	local tool
	for tool in cargo uv uvx pnpm bundle go mvn dotnet composer mix dart swift zig gradle mise; do
		xberg_stub "$tool" "printf '%s %s %s\\n' \"\${PWD##*/}\" '$tool' \"\$*\" >>\"\$XBERG_TRACE\""
	done
	mkdir -p "$XBERG_WORK"/packages/{ruby,go,java,csharp,elixir,zig,kotlin-android} "$XBERG_WORK/e2e/php"
	cd "$XBERG_WORK"
}

@test "update refreshes every ecosystem within its major versions, in order" {
	usage_mode=update run bash "$SCRIPT"
	xberg_assert_status 0
	local rules="-DallowSnapshots=false -Dmaven.version.rules=file:///$XBERG_WORK/packages/java/maven-version-rules.xml"
	local csproj=TreeSitterLanguagePack/TreeSitterLanguagePack.csproj
	xberg_assert_trace \
		"work cargo upgrade" \
		"work cargo update" \
		"work cargo upgrade --manifest-path e2e/rust/Cargo.toml" \
		"work cargo update --manifest-path e2e/rust/Cargo.toml" \
		"work uv sync --all-packages --all-extras --no-install-workspace --no-install-project" \
		"work uv pip list --outdated" \
		"work pnpm up -r" \
		"work pnpm up -w" \
		"ruby bundle update --conservative" \
		"ruby bundle outdated" \
		"go go get -t ./..." \
		"go go mod tidy" \
		"go go install github.com/golangci/golangci-lint/v2/cmd/golangci-lint@latest" \
		"java mvn $rules -DallowMajorUpdates=false versions:use-next-versions -DprocessDependencies=true -DprocessParent=false -DgenerateBackupPoms=false" \
		"java mvn $rules -DallowMajorUpdates=false versions:update-properties -DgenerateBackupPoms=false" \
		"java mvn $rules versions:display-dependency-updates" \
		"csharp dotnet tool install --global dotnet-outdated-tool" \
		"csharp dotnet outdated $csproj --upgrade --version-lock Major" \
		"csharp dotnet list $csproj package --outdated" \
		"work composer update -w" \
		"work composer outdated --direct" \
		"php composer update -w" \
		"php composer outdated --direct" \
		"elixir mix hex.outdated" \
		"elixir mix deps.update --all" \
		"elixir mix hex.outdated" \
		"work mise run deps update dart" \
		"work mise run deps update swift" \
		"zig zig build --fetch" \
		"kotlin-android gradle dependencyUpdates -Drevision=release --no-daemon" \
		"work uv run --no-sync scripts/check_grammar_updates.py --dry-run" \
		"work uvx --from gh-actions-updater gau --update --recursive --pin-style major --missing-ref fallback .github"
}

@test "upgrade takes breaking releases and routes rust, dart, swift and zig through the shared engine" {
	usage_mode=upgrade run bash "$SCRIPT"
	xberg_assert_status 0
	local rules="-DallowSnapshots=false -Dmaven.version.rules=file:///$XBERG_WORK/packages/java/maven-version-rules.xml"
	local csproj=TreeSitterLanguagePack/TreeSitterLanguagePack.csproj
	xberg_assert_trace \
		"work mise run deps upgrade rust" \
		"work uv sync --all-packages --all-extras --upgrade --no-install-workspace --no-install-project" \
		"work uv pip list --outdated" \
		"work pnpm up --latest -r" \
		"work pnpm up --latest -w" \
		"ruby bundle update --all --conservative=false" \
		"ruby bundle outdated" \
		"go go get -u -t ./..." \
		"go go mod tidy" \
		"go go install github.com/golangci/golangci-lint/v2/cmd/golangci-lint@latest" \
		"java mvn $rules -DallowMajorUpdates=true versions:use-latest-releases -DprocessDependencies=true -DprocessParent=false -DgenerateBackupPoms=false" \
		"java mvn $rules -DallowMajorUpdates=true versions:update-properties -DgenerateBackupPoms=false" \
		"java mvn $rules versions:display-dependency-updates" \
		"java mvn $rules versions:display-plugin-updates" \
		"csharp dotnet tool install --global dotnet-outdated-tool" \
		"csharp dotnet outdated $csproj --upgrade" \
		"csharp dotnet list $csproj package --outdated" \
		"work composer update --with-all-dependencies" \
		"work composer outdated --direct" \
		"php composer update --with-all-dependencies" \
		"php composer outdated --direct" \
		"elixir mix hex.outdated" \
		"elixir mix deps.unlock --all" \
		"elixir mix deps.update --all" \
		"elixir mix hex.outdated" \
		"work mise run deps upgrade dart" \
		"work mise run deps upgrade swift" \
		"work mise run deps upgrade zig" \
		"kotlin-android gradle dependencyUpdates -Drevision=latest.release --no-daemon" \
		"work uv run --no-sync scripts/check_grammar_updates.py --dry-run" \
		"work uvx --from gh-actions-updater gau --update --recursive --pin-style major --missing-ref fallback .github"
}

@test "an unknown mode exits 2 before running anything" {
	usage_mode=refresh run bash "$SCRIPT"
	xberg_assert_status 2
	xberg_assert_output "mode must be update or upgrade, got: refresh"
	xberg_assert_trace_empty
}
