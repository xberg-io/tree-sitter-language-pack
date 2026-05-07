# tree-sitter-language-pack CLI — static musl binary with all 248 parsers
# Build: docker build -f docker/Dockerfile -t ts-pack .
# Run:   docker run --rm ts-pack parse --language python -
#
# NOTE: The `parsers/` directory must exist in the build context.
# Run `uv run scripts/clone_vendors.py` before building, or use ci-docker.yaml.

# ─── Builder ─────────────────────────────────────────────────────────
FROM rust:alpine AS builder

RUN apk add --no-cache musl-dev gcc g++ python3 bash

# Alpine's gcc already targets musl; cargo config specifies musl-gcc which doesn't exist on Alpine.
ENV CARGO_TARGET_X86_64_UNKNOWN_LINUX_MUSL_LINKER=gcc

WORKDIR /build
COPY . .

# Build CLI with all parsers statically linked
RUN LANGS=$(python3 -c "import json; d=json.load(open('sources/language_definitions.json')); print(','.join(sorted(d.keys())))") && \
    echo "Building with $(echo $LANGS | tr ',' '\n' | wc -l) languages" && \
    TSLP_LANGUAGES="$LANGS" \
    TSLP_LINK_MODE=static \
    PROJECT_ROOT=/build \
    cargo build --release -p ts-pack-cli && \
    strip target/release/ts-pack

# ─── Runtime ─────────────────────────────────────────────────────────
FROM alpine:latest

COPY --from=builder /build/target/release/ts-pack /usr/local/bin/ts-pack

# Smoke test — parsers are statically linked, no downloads needed
RUN ts-pack --version && \
    echo "def hello(): pass" | ts-pack parse - --language python

ENTRYPOINT ["ts-pack"]
