<picture>
    <img alt="Icon of a filament spool" src="./assets/spoolman-light-logo.png">
</picture>

<br/>


# Spoolman light

_A lightweight filament tracker for home 3D printing._

Spoolman light is a self-hosted web service for tracking your 3D printer filament spools. It is a simplified fork of [Donkie/Spoolman](https://github.com/Donkie/Spoolman) designed for home use — one or two printers and a shelf of spools — with no database server, no vendor management, and no external integrations required.

## Features

* **Filament & Spool Tracking**: Keep records of filament types and individual spools, including RGBA color(s) on the spool.
* **Location Management**: Organise spools into named locations (shelves, dry boxes, etc.) with live spool counts.
* **Weight Tracking**: Record initial and current weight (scale readings); used weight and remaining percentage are derived automatically.
* **SpoolmanDB Search**: Filament lookup via the SpoolmanDB online database (`GET /api/v1/filament/search`).
* **Data Export**: Download the full data store as JSON via `GET /api/v1/export`.
* **REST API**: Clean REST API compatible with Spoolman-aware clients (Klipper plugins, OrcaSlicer, etc.).
* **Web Client**: Built-in browser UI with dark mode support.
* **Simple Storage**: All data stored in a single JSON file — no database server required.

## Installation

### Docker (recommended)

```bash
docker build -t spoolman-light .
docker run -p 8000:8000 -v spoolman_data:/data spoolman-light
```

Or use the included `docker-compose.yml`:

```bash
docker compose up
```

The web UI is available at `http://localhost:8000`.

### Build from source

Requirements: Rust stable (see `rust-toolchain.toml`), `cargo-leptos`, `wasm32-unknown-unknown` target.

```bash
rustup target add wasm32-unknown-unknown
cargo install cargo-leptos --locked

cargo leptos build --release
./target/release/spoolman-server
```

## Stack

| Layer | Technology |
|-------|-----------|
| Backend | Rust 1.82+, Axum, Tokio |
| Frontend | Rust, Leptos (WASM), compiled into the server binary |
| Storage | JSON file (`spoolman.json`) — no database |
| Types | `spoolman-types` crate shared by server and client |

The entire application ships as a **single self-contained binary** with no Python runtime, no Node.js, and no external database.

## Configuration

All variables can also be set in a `.env` file in the working directory — the server loads it silently on startup (a missing file is not an error).

| Variable | Default | Purpose |
|----------|---------|---------|
| `SPOOLMAN_DATA_FILE` | `<platform data dir>/spoolman.json` | Path to JSON data file |
| `SPOOLMAN_HOST` | `0.0.0.0` | Bind host |
| `SPOOLMAN_PORT` | `8000` | Bind port |
| `SPOOLMAN_CORS_ORIGIN` | `FALSE` | CORS allowed origin (`FALSE` = disabled) |
| `SPOOLMAN_BASE_PATH` | `""` | URL base path prefix |
| `SPOOLMAN_DEBUG_MODE` | `FALSE` | Enable debug mode |
| `SPOOLMAN_LOGGING_LEVEL` | `info` | Log level (`trace`/`debug`/`info`/`warn`/`error`) |
| `SPOOLMAN_AUTOMATIC_BACKUP` | `TRUE` | Enable daily backup rotation |

## API

| Method | Path | Description |
|--------|------|-------------|
| GET/POST | `/api/v1/filament` | List / create filaments |
| GET/PATCH/DELETE | `/api/v1/filament/:id` | Get / update / delete a filament |
| GET | `/api/v1/filament/search` | Search SpoolmanDB |
| GET/POST | `/api/v1/spool` | List / create spools |
| GET/PATCH/DELETE | `/api/v1/spool/:id` | Get / update / delete a spool |
| POST | `/api/v1/spool/:id/clone` | Clone a spool |
| GET/POST | `/api/v1/location` | List / create locations |
| GET/PATCH/DELETE | `/api/v1/location/:id` | Get / update / delete a location |
| GET | `/api/v1/material` | List distinct filament materials |
| GET | `/api/v1/export` | Full data store JSON download |
| GET | `/api/v1/setting` | List all settings |
| PUT | `/api/v1/setting/:key` | Set a key-value setting |
| GET | `/health` | Health check |
| GET | `/info` | Server version and data file path |

## What's removed vs upstream Spoolman

This fork deliberately omits features that add complexity without value for home use:

- No Vendor entity (vendor is a plain string on Filament)
- No extra-fields system
- No WebSocket live-updates (use polling)
- No Prometheus metrics
- No multi-database support (JSON file only)
- No QR / label printing page
