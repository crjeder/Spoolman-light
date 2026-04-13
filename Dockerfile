# syntax=docker/dockerfile:1
# Multi-stage build: compile Rust workspace → minimal runtime image.
# Replaces the previous Python/Node multi-stage build.

# ── Stage 1: build ────────────────────────────────────────────────────────────
FROM rust:1-bookworm AS builder

# Install cargo-leptos build tool, the WASM compilation target, and
# wasm-bindgen-cli. cargo-leptos downloads wasm-bindgen as a pre-built binary
# from GitHub Releases at build time — pre-installing it via cargo puts it on
# $PATH so cargo-leptos uses it directly without a network download.
RUN rustup target add wasm32-unknown-unknown \
 && cargo install cargo-leptos --locked \
 && cargo install wasm-bindgen-cli --version 0.2.117 --locked

WORKDIR /build
COPY . .

# Build the full workspace: spoolman-server binary + spoolman-client WASM.
RUN cargo leptos build --release

# cargo-leptos 0.3.x renames spoolman-server_bg.wasm → spoolman-server.wasm in
# the site output, but the generated JS still references the _bg name. Alias it.
RUN cp target/site/pkg/spoolman-server.wasm target/site/pkg/spoolman-server_bg.wasm

# cargo-leptos skips index.html generation when a server binary is present (SSR
# assumption). Since spoolman-server is a plain Axum file server, generate the
# CSR bootstrap HTML manually.
RUN printf '<!DOCTYPE html>\n<html lang="en">\n<head>\n  <meta charset="utf-8" />\n  <meta name="viewport" content="width=device-width, initial-scale=1" />\n  <title>Spoolman</title>\n  <link rel="icon" type="image/png" href="/spoolman-light-logo.png" />\n  <link rel="stylesheet" href="/pkg/spoolman-server.css" />\n</head>\n<body>\n  <script type="module">\n    import init from "/pkg/spoolman-server.js";\n    init();\n  </script>\n</body>\n</html>\n' > target/site/index.html

# ── Stage 2: runtime ──────────────────────────────────────────────────────────
# distroless/cc includes glibc + libstdc++ but no shell or package manager,
# minimising attack surface. The built-in nonroot user has uid/gid 65532.
FROM gcr.io/distroless/cc-debian12 AS runtime

# Copy the compiled server binary.
COPY --from=builder --chown=65532:65532 /build/target/release/spoolman-server /spoolman

# Copy the compiled WASM frontend assets served by the binary at runtime.
COPY --from=builder --chown=65532:65532 /build/target/site /site

LABEL org.opencontainers.image.source=https://github.com/Donkie/Spoolman
LABEL org.opencontainers.image.description="Keep track of your inventory of 3D-printer filament spools."
LABEL org.opencontainers.image.licenses=MIT

ENV SPOOLMAN_HOST=0.0.0.0 \
    SPOOLMAN_PORT=8000 \
    SPOOLMAN_DATA_FILE=/data/spoolman.json \
    LEPTOS_SITE_ROOT=/site

EXPOSE 8000
VOLUME ["/data"]

USER 65532
CMD ["/spoolman"]
