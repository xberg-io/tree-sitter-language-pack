#!/usr/bin/env bash
#MISE description="Update or upgrade every ecosystem, then report grammar updates and repin actions"
#MISE hide=true
#USAGE arg "<mode>" help="update stays within major versions; upgrade takes the latest, breaking ones included"
# shellcheck disable=SC2154 # usage_* are set by mise from the #USAGE lines
set -euo pipefail

mode="$usage_mode"
case "$mode" in
update | upgrade) ;;
*)
  echo "mode must be update or upgrade, got: $mode" >&2
  exit 2
  ;;
esac

csharp_project=TreeSitterLanguagePack/TreeSitterLanguagePack.csproj

# Dart, Swift and Zig upgrades match the shared deps engine; the other ecosystems keep their own
# steps (extra manifests, report steps, Maven version rules). ~keep
refresh() {
  local lang="$1"
  case "$lang:$mode" in
  rust:update)
    # `cargo upgrade` without --incompatible raises requirements within their major version
    # first, which the shared engine's update mode does not do. ~keep
    cargo upgrade
    cargo update
    cargo upgrade --manifest-path e2e/rust/Cargo.toml
    cargo update --manifest-path e2e/rust/Cargo.toml
    ;;
  rust:upgrade) mise run deps upgrade rust ;;
  python:*)
    local upgrade=()
    if [ "$mode" = upgrade ]; then upgrade=(--upgrade); fi
    uv sync --all-packages --all-extras ${upgrade[@]+"${upgrade[@]}"} --no-install-workspace --no-install-project
    uv pip list --outdated || true
    ;;
  node:update) pnpm up -r && pnpm up -w ;;
  node:upgrade) pnpm up --latest -r && pnpm up --latest -w ;;
  ruby:update) (cd packages/ruby && bundle update --conservative && { bundle outdated || true; }) ;;
  # True cross-major upgrades require manual Gemfile edits. ~keep
  ruby:upgrade) (cd packages/ruby && bundle update --all --conservative=false && { bundle outdated || true; }) ;;
  go:*)
    local flags=(-t)
    if [ "$mode" = upgrade ]; then flags=(-u -t); fi
    (
      cd packages/go
      go get "${flags[@]}" ./...
      go mod tidy
      go install github.com/golangci/golangci-lint/v2/cmd/golangci-lint@latest || true
    )
    ;;
  java:*) (cd packages/java && refresh_java) ;;
  csharp:*)
    local lock=()
    if [ "$mode" = update ]; then lock=(--version-lock Major); fi
    (
      cd packages/csharp
      dotnet tool install --global dotnet-outdated-tool || dotnet tool update --global dotnet-outdated-tool
      dotnet outdated "$csharp_project" --upgrade ${lock[@]+"${lock[@]}"}
      dotnet list "$csharp_project" package --outdated || true
    )
    ;;
  php:*)
    local with=-w
    if [ "$mode" = upgrade ]; then with=--with-all-dependencies; fi
    composer update "$with" || true
    composer outdated --direct || true
    (cd e2e/php && { composer update "$with" || true; } && { composer outdated --direct || true; })
    ;;
  # Elixir requires manual mix.exs edits for major version updates. ~keep
  elixir:*)
    (
      cd packages/elixir
      mix hex.outdated || true
      if [ "$mode" = upgrade ]; then mix deps.unlock --all; fi
      mix deps.update --all
      mix hex.outdated || true
    )
    ;;
  # Dart, Swift and Zig are not mise tools, so each fails first when its binary is missing. SwiftPM
  # resolves within Package.swift ranges: edit them by hand for a major upgrade. ~keep
  dart:*)
    command -v dart >/dev/null 2>&1
    mise run deps "$mode" dart
    ;;
  swift:*)
    command -v swift >/dev/null 2>&1
    mise run deps "$mode" swift
    ;;
  zig:update)
    command -v zig >/dev/null 2>&1
    (cd packages/zig && zig build --fetch)
    ;;
  zig:upgrade)
    command -v zig >/dev/null 2>&1
    mise run deps upgrade zig
    ;;
  # Gradle has no built-in apply-updates: this reports current vs available versions; edit
  # build.gradle(.kts) by hand to apply them. ~keep
  kotlin-android:*)
    local revision=release
    if [ "$mode" = upgrade ]; then revision=latest.release; fi
    command -v gradle >/dev/null 2>&1
    (cd packages/kotlin-android && { gradle dependencyUpdates -Drevision="$revision" --no-daemon || true; })
    ;;
  esac
}

refresh_java() {
  local rules majors goal
  rules=(-DallowSnapshots=false "-Dmaven.version.rules=file:///${PWD}/maven-version-rules.xml")
  if [ "$mode" = update ]; then
    majors=-DallowMajorUpdates=false goal=versions:use-next-versions
  else
    majors=-DallowMajorUpdates=true goal=versions:use-latest-releases
  fi
  mvn "${rules[@]}" "$majors" "$goal" -DprocessDependencies=true -DprocessParent=false -DgenerateBackupPoms=false
  mvn "${rules[@]}" "$majors" versions:update-properties -DgenerateBackupPoms=false || true
  mvn "${rules[@]}" versions:display-dependency-updates
  if [ "$mode" = upgrade ]; then mvn "${rules[@]}" versions:display-plugin-updates; fi
}

for lang in rust python node ruby go java csharp php elixir dart swift zig kotlin-android; do
  echo "Running $lang:$mode..."
  refresh "$lang"
done

# Grammar pins are NOT bumped automatically: a new pin makes `tree-sitter generate` execute that
# revision's grammar.js under Node, so every bump is arbitrary upstream code execution and needs
# human review. Report only; run `uv run --no-sync scripts/pin_vendors.py` explicitly to apply,
# then review the diff. ~keep
echo "Checking for language grammar updates (report only)..."
uv run --no-sync scripts/check_grammar_updates.py --dry-run
echo "Updating GitHub Actions pins..."
uvx --from gh-actions-updater gau --update --recursive --pin-style major --missing-ref fallback .github
