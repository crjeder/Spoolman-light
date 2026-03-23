# syntax=docker/dockerfile:1
# Multi-stage build: compile Rust workspace → minimal runtime image.
# Replaces the previous Python/Node multi-stage build.

# ── Stage 1: build ────────────────────────────────────────────────────────────
FROM rust:1-bookworm AS builder

# Install cargo-leptos build tool and the WASM compilation target.
RUN rustup target add wasm32-unknown-unknown \
 && cargo install cargo-leptos --locked

WORKDIR /build
COPY . .

# Build the full workspace: spoolman-server binary + spoolman-client WASM.
RUN cargo leptos build --release

# ── Stage 2: runtime ──────────────────────────────────────────────────────────
FROM debian:bookworm-slim AS runtime

RUN apt-get update \
 && apt-get install -y --no-install-recommends ca-certificates \
 && rm -rf /var/lib/apt/lists/*

# Non-root user matching the previous image's uid/gid convention.
RUN groupmod -g 1000 users \
 && useradd -u 1000 -U app \
 && usermod -G users app \
 && mkdir -p /home/app/.local/share/spoolman \
 && chown -R app:app /home/app/.local/share/spoolman

# Copy the compiled server binary.
COPY --from=builder --chown=app:app /build/target/release/spoolman-server /spoolman

# Copy the compiled WASM frontend assets served by the binary at runtime.
COPY --from=builder --chown=app:app /build/target/site /site

LABEL org.opencontainers.image.source=https://github.com/Donkie/Spoolman
LABEL org.opencontainers.image.description="Keep track of your inventory of 3D-printer filament spools."
LABEL org.opencontainers.image.licenses=MIT

ENV SPOOLMAN_HOST=0.0.0.0 \
    SPOOLMAN_PORT=8000 \
    SPOOLMAN_DATA_FILE=/data/spoolman.json \
    LEPTOS_SITE_ROOT=/site

EXPOSE 8000
VOLUME ["/data"]

USER app
CMD ["/spoolman"]
