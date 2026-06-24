---
description: "Run ts-pack in Docker — a statically-linked Alpine image with all parsers compiled in."
---

# Docker

The Docker image ships a statically-linked `ts-pack` binary on Alpine Linux.
The build process compiles all parsers at image build time; the container needs
no internet access or runtime downloads.

## Quick start

```bash
docker pull ghcr.io/xberg-io/tree-sitter-language-pack:latest

# Parse a file by mounting the current directory
docker run --rm \
  -v "$(pwd):/work" -w /work \
  ghcr.io/xberg-io/tree-sitter-language-pack:latest \
  parse src/main.py

# From stdin
echo "def hello(): pass" | docker run --rm -i \
  ghcr.io/xberg-io/tree-sitter-language-pack:latest \
  parse - --language python
```

## Image contents

The image is two layers:

1. A Rust/Alpine builder that compiles `ts-pack-cli` with all parsers statically linked via `TSLP_LINK_MODE=static`
2. A minimal `alpine:latest` runtime containing `/usr/local/bin/ts-pack`

The binary links statically against musl libc, so it runs on any Linux machine without extra dependencies.

## Building locally

Before building, you need the parser C sources cloned locally:

```bash
uv run scripts/clone_vendors.py
```

Then build the image from the repository root (the full context must be present):

```bash
docker build -f docker/Dockerfile -t ts-pack .
```

The build takes several minutes — it compiles every grammar in `sources/language_definitions.json` from C.

## Verify the image

```bash
docker run --rm ts-pack --version
docker run --rm ts-pack list | wc -l
```

## Use in CI

```yaml
# GitHub Actions
jobs:
  analyze:
    runs-on: ubuntu-latest
    container:
      image: ghcr.io/xberg-io/tree-sitter-language-pack:latest
    steps:
      - uses: actions/checkout@v4
      - name: Extract structure
        run: ts-pack process src/main.py --structure
```

## Build a smaller image with a parser subset

To target a language subset, set `TSLP_LANGUAGES` at build time:

```dockerfile
FROM rust:alpine AS builder
RUN apk add --no-cache musl-dev gcc g++ python3 bash
WORKDIR /build
COPY . .
RUN TSLP_LANGUAGES=python,javascript,typescript \
    TSLP_LINK_MODE=static \
    PROJECT_ROOT=/build \
    cargo build --release -p ts-pack-cli && \
    strip target/release/ts-pack

FROM alpine:latest
COPY --from=builder /build/target/release/ts-pack /usr/local/bin/ts-pack
ENTRYPOINT ["ts-pack"]
```

Run `uv run scripts/clone_vendors.py --languages python,javascript,typescript`
first to fetch the needed grammar sources.

## Multi-arch

The published image targets `linux/amd64` and `linux/arm64`. The `ci-docker.yaml`
and `publish-docker.yaml` workflows handle this via `docker buildx`.
