# syntax=docker/dockerfile:1
# Multi-stage build: compile Rust workspace → minimal runtime image.
# Replaces the previous Python/Node multi-stage build.

# ── Stage 1: site (WASM frontend) ─────────────────────────────────────────────
# The WASM output is architecture-independent, so build it once on the native
# build host. Running cargo-leptos under QEMU for arm targets takes ~6h and its
# LTO link runs out of memory on armv7.
FROM --platform=$BUILDPLATFORM rust:1-bookworm AS site

# Install cargo-leptos build tool, the WASM compilation target, and
# wasm-bindgen-cli. cargo-leptos downloads wasm-bindgen as a pre-built binary
# from GitHub Releases at build time — pre-installing it via cargo puts it on
# $PATH so cargo-leptos uses it directly without a network download.
RUN rustup target add wasm32-unknown-unknown \
 && cargo install cargo-leptos --locked \
 && cargo install wasm-bindgen-cli --version 0.2.117 --locked

WORKDIR /build
COPY . .

# Empty dir copied into the runtime image to establish /data ownership.
RUN mkdir -p /build/data

# Build spoolman-client WASM (the native server binary built here is discarded).
RUN cargo leptos build --release

# cargo-leptos 0.3.x renames spoolman-server_bg.wasm → spoolman-server.wasm in
# the site output, but the generated JS still references the _bg name. Alias it.
RUN cp target/site/pkg/spoolman-server.wasm target/site/pkg/spoolman-server_bg.wasm

# cargo-leptos skips index.html generation when a server binary is present (SSR
# assumption). Since spoolman-server is a plain Axum file server, generate the
# CSR bootstrap HTML manually.
RUN printf '<!DOCTYPE html>\n<html lang="en">\n<head>\n  <meta charset="utf-8" />\n  <meta name="viewport" content="width=device-width, initial-scale=1" />\n  <title>Spoolman</title>\n  <link rel="icon" type="image/png" href="/spoolman-light-logo.png" />\n  <link rel="stylesheet" href="/pkg/spoolman-server.css" />\n</head>\n<body>\n  <script type="module">\n    import init from "/pkg/spoolman-server.js";\n    init();\n  </script>\n</body>\n</html>\n' > target/site/index.html

# ── Stage 2: server binary (per target platform) ──────────────────────────────
# Plain cargo build, no cargo-leptos: the server only reads LEPTOS_SITE_ROOT at
# runtime. Workspace release profile has no LTO, so armv7 stays within memory.
FROM rust:1-bookworm AS server

WORKDIR /build
COPY . .

RUN cargo build --release --locked -p spoolman-server

# ── Stage 3: runtime ──────────────────────────────────────────────────────────
# distroless/cc includes glibc + libstdc++ but no shell or package manager,
# minimising attack surface. The built-in nonroot user has uid/gid 65532.
FROM gcr.io/distroless/cc-debian12 AS runtime

# Copy the compiled server binary.
COPY --from=server --chown=65532:65532 /build/target/release/spoolman-server /spoolman

# Copy the compiled WASM frontend assets served by the binary at runtime.
COPY --from=site --chown=65532:65532 /build/target/site /site

# Seed /data owned by the nonroot user so a freshly created volume inherits
# writable ownership instead of root:root (the default for a VOLUME
# mountpoint created before USER switches away from root).
COPY --from=site --chown=65532:65532 /build/data /data

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
