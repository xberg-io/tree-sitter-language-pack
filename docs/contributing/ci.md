---
description: "CI/CD workflow reference — what each GitHub Actions workflow does, when it runs, and how publishing works."
---

# CI/CD reference

The project has 8 GitHub Actions workflows in `.github/workflows/`.

## Overview

| Workflow | Purpose | Trigger |
|----------|---------|---------|
| `ci.yaml` | Main CI — builds and tests all bindings | push/PR to `main` |
| `ci-cli.yaml` | CLI-specific tests | push/PR to `main` |
| `ci-docker.yaml` | Docker image build and tests | push/PR to `main` |
| `docs.yaml` | Build and deploy documentation | push to `main`, manual |
| `publish.yaml` | Publish packages to all registries | manual, release |
| `publish-docker.yaml` | Build and push Docker image | manual, release |
| `validate-issues.yml` | Validate issue format | issue opened/edited |
| `validate-pr.yml` | Validate PR format | PR opened/edited/synced |

---

## CI workflows

### `ci.yaml` — main CI

Runs on push to `main` and pull requests when relevant paths change:
`crates/**`, `packages/**`, `e2e/**`, `sources/**`, `scripts/**`, `docs/snippets/**`, `Cargo.toml`, `Taskfile.yml`.

This is the primary workflow that builds and tests all language bindings.

### `ci-cli.yaml` — CLI

Runs on push to `main` and pull requests when CLI or core paths change:
`crates/ts-pack-cli/**`, `crates/ts-pack-core/**`, `test_apps/cli/**`.

### `ci-docker.yaml` — Docker

Runs on push to `main` and pull requests when Docker or core paths change:
`docker/**`, `crates/ts-pack-core/**`, `crates/ts-pack-cli/**`.

---

## Docs workflow

`docs.yaml` runs on push to `main` when docs files change, and you can also trigger it manually via `workflow_dispatch`. It builds the docs site and deploys it.

Triggers on changes to: `docs/**`, `zensical.toml`, `pyproject.toml`, `sources/language_definitions.json`, and `scripts/generate_grammar_table.py`.

---

## Publishing workflows

Both publish workflows run automatically on a GitHub release event, and you can also trigger them manually via `workflow_dispatch`.

### `publish.yaml` — package releases

Takes a release tag (for example `vX.Y.Z`), an optional `dry_run` flag, and an optional `targets` list (comma-separated, defaults to `all`). On a real run, it publishes to all registered package registries simultaneously.

### `publish-docker.yaml` — Docker image

Takes a release tag and optional `dry_run`. Builds the multi-arch image (amd64 + arm64) using `docker buildx` and pushes to `ghcr.io`.

---

## Validation workflows

### `validate-issues.yml`

Validates the format of newly opened or edited issues using a reusable workflow from `kreuzberg-dev/actions`.

### `validate-pr.yml`

Validates the format of pull requests when opened, edited, or synchronized using a reusable workflow from `kreuzberg-dev/actions`.
