# tree-sitter-language-pack CLI — static musl binary with all 248 parsers
# Build: docker build -f docker/Dockerfile -t ts-pack .
# Run:   docker run --rm ts-pack parse --language python -
#
# NOTE: The `parsers/` directory must exist in the build context.
# Run `uv run scripts/clone_vendors.py` before building, or use ci-docker.yaml.

# ─── Builder ─────────────────────────────────────────────────────────
FROM rust:1.95-alpine3.22 AS builder

SHELL ["/bin/ash", "-eo", "pipefail", "-c"]

RUN apk add --no-cache \
    bash=5.2.37-r0 \
    g++=14.2.0-r6 \
    gcc=14.2.0-r6 \
    musl-dev=1.2.5-r12 \
    python3=3.12.13-r0

# Alpine's gcc already targets musl; cargo config specifies musl-gcc which doesn't exist on Alpine.
ENV CARGO_TARGET_X86_64_UNKNOWN_LINUX_MUSL_LINKER=gcc

WORKDIR /build
COPY . .

# Build CLI with all parsers statically linked
RUN LANGS=$(python3 -c "import json; d=json.load(open('sources/language_definitions.json')); print(','.join(sorted(d.keys())))") && \
    echo "Building with $(echo "$LANGS" | tr ',' '\n' | wc -l) languages" && \
    TSLP_LANGUAGES="$LANGS" \
    TSLP_LINK_MODE=static \
    PROJECT_ROOT=/build \
    cargo build --release -p ts-pack-cli && \
    strip target/release/ts-pack

# ─── Runtime ─────────────────────────────────────────────────────────
FROM alpine:3.22

SHELL ["/bin/ash", "-eo", "pipefail", "-c"]

COPY --from=builder /build/target/release/ts-pack /usr/local/bin/ts-pack

# Smoke test — parsers are statically linked, no downloads needed
RUN ts-pack --version && \
    echo "def hello(): pass" | ts-pack parse - --language python

ENTRYPOINT ["ts-pack"]
