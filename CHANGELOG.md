# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [1.4.0] - 2026-03-24

### Removed

- Python/FastAPI backend (`spoolman/`) — the Rust/Axum server is now the sole backend. No API changes for consumers.
- Docker-based Python integration test suite (`tests_integration/`) — no longer applicable to the Rust stack. Rust integration tests are tracked as a follow-up.
- `entrypoint.sh` — the uvicorn launcher is not needed; the Rust binary binds `SPOOLMAN_HOST`/`SPOOLMAN_PORT` directly.
- `scripts/install.sh` and `scripts/start.sh` — Python/venv install helpers; superseded by `docker compose up` or `cargo leptos build`.
- `pyproject.toml`, `pdm.lock`, `uv.lock` — Python package management files.
- CI jobs `style` (Python pre-commit) and `build-client` (Node/pdm) replaced by `cargo check` and `cargo clippy` jobs.

### Changed

- `PUID`/`PGID` env vars no longer supported — the Docker image runs as a fixed `app` user (uid 1000). Ensure volume mounts are accessible by uid 1000.

## [1.3.0] - 2026-03-24

### Removed

- React/TypeScript frontend (`client/`) — the Leptos WASM frontend (`crates/spoolman-client`) is now the sole UI. The `client/` directory, npm tooling, and all Node.js build configuration have been deleted.

## [1.2.0] - 2026-03-24

### Added

- Color proximity filter on spool list: a color picker and threshold slider let users find spools by color similarity using CIEDE2000 (ΔE\*00) — a perceptually uniform metric that matches human color vision. Default threshold is 10 ΔE (≈ "acceptably similar"). Replaces the earlier Euclidean RGB distance.

## [1.1.0] - 2026-03-22

### Added

- `MaterialType` enum in `spoolman-types` based on the OpenPrintTag `material_type_enum` spec (42 named variants + `Other(String)` catch-all). Serializes as uppercase abbreviation (e.g. `"PLA"`); unknown strings round-trip without error.
- Material `<select>` on filament create/edit forms — replaces free-text input with a dropdown of all 42 spec-defined types plus a "select" blank option.
- Material filter dropdown on the Filament list page; filters are applied server-side via `?material=` query param.
- Spool list text filter now also matches on `filament.material` abbreviation (e.g. typing "PLA" narrows spool results).
- `GET /api/v1/material` client wrapper (`api::list_materials`) for future datalist/autocomplete use.
- `.env` file support via `dotenvy`: the server silently loads a `.env` file from the working directory on startup, before reading environment variables. Missing file is not an error.

## [1.0.0] - 2026-03-23

### Added

- Full Rust rewrite: Axum backend + Leptos WASM frontend in a Cargo workspace (`crates/spoolman-types`, `crates/spoolman-server`, `crates/spoolman-client`). Build with `cargo leptos build --release`.
- `docker-compose.yml` at repo root for quick local deployment; data persisted in a named Docker volume at `/data/spoolman.json`.
- `rust-toolchain.toml` declares the stable toolchain with `wasm32-unknown-unknown` target so `rustup` installs it automatically.
- `spoolman-types` shared crate: `Spool`, `Filament`, `Location`, `DataStore` types used by both server and client, ensuring compile-time API contract consistency.
- `Location` as a first-class entity with full CRUD (`GET/POST/PATCH/DELETE /api/v1/location`) — replaces the previous freeform string field on Spool.
- `GET /api/v1/filament/search?q=` endpoint: proxies SpoolmanDB on demand (no background scheduler or local cache).
- `GET /api/v1/export` endpoint: downloads the full data store as JSON (useful for backup and migration).
- Dark mode toggle with CSS variable switching, persisted in localStorage.
- Spool clone action (`POST /api/v1/spool/<id>/clone`).
- `SPOOLMAN_DATA_FILE` environment variable to configure the path of the JSON data file (default: `~/.local/share/spoolman/spoolman.json`).

### Changed

- **BREAKING**: Entire stack replaced — Python/FastAPI backend and React/Refine/Ant Design frontend superseded by a Rust Cargo workspace (Axum + Leptos WASM). Docker image no longer requires Python runtime or Node.js build artifacts.
- **BREAKING**: JSON storage format redesigned for Rust/serde ergonomics. No existing data to migrate (format was unconstrained).
- **BREAKING**: Spool and Filament IDs are now random `u32` values (previously sequential integers), stable across export/reimport for NFC tag URL durability.
- **BREAKING**: Colors represented as `Vec<RGBA>` (OpenTag3D/OpenPrintTag compatible) instead of hex strings. Color lives on Spool, not Filament.
- **BREAKING**: Weight tracked as `initial_weight` + `current_weight` (full scale readings). Three-mode weight entry (used/remaining/measured) removed.
- **BREAKING**: The `Vendor` entity removed; vendor is a plain string on Filament.
- **BREAKING**: `article_number`, `external_id`, `lot_nr`, `extra` fields removed from all entities.
- **BREAKING**: WebSocket live-update endpoint removed.
- **BREAKING**: Backup download endpoint removed (backup still runs automatically in background).
- Spool NFC Online Data URL maps to `/api/v1/spool/<id>` (OpenTag3D-compatible).
- Dockerfile: `SPOOLMAN_DATA_FILE` defaults to `/data/spoolman.json` and `LEPTOS_SITE_ROOT` defaults to `/site` in the container, matching the volume mount convention.
- Static asset path in the server is now read from `LEPTOS_SITE_ROOT` env var (fallback: `target/site`), enabling the production Docker layout without recompilation.
- Integration test suite (`tests_integration/`) rewritten for the Rust API: new Filament/Spool/Location models, RGBA color schema, `current_weight` weight tracking, new Location CRUD tests, settings tests updated for `PUT /api/v1/setting/:key`, `fields/` tests deleted, `test_use.py` and `test_find_by_color.py` deleted, `test_measure.py` rewritten as `current_weight` PATCH tests, `test_backup.py` replaced with export test.

### Fixed

- `entrypoint.sh` had Windows-style CRLF line endings, causing the Docker container to fail to start on Linux with "no such file or directory".
- `JsonStore._flush` could raise `FileNotFoundError` under concurrent requests due to multiple threads racing to rename the same `.tmp` file; writes are now serialized with a reentrant lock.
- Filtering spools by empty `filament.name`, `filament.material`, or `filament.vendor` now correctly returns spools whose filament has no value set for that field.

### Removed

- `Vendor` entity and all `/api/v1/vendor` endpoints.
- `color_hex`, `multi_color_hexes`, `multi_color_direction` from Filament (moved to Spool as `colors: Vec<RGBA>`).
- `price`, `weight`, `spool_weight`, `article_number`, `external_id` from Filament.
- `lot_nr`, `external_id` from Spool.
- `/api/v1/spool/find-by-color` endpoint (no replacement; color filter is a planned enhancement).
- `/api/v1/field/*` extra-fields system.
- Database backends: SQLite, PostgreSQL, MySQL, CockroachDB.
- Python dependencies: `SQLAlchemy`, `alembic`, `aiosqlite`, `asyncpg`, `aiomysql`, `psycopg2-binary`, `sqlalchemy-cockroachdb`.
- Alembic migration directory (`migrations/`) and `alembic.ini`.
- Prometheus metrics endpoint (`GET /metrics`) and the `SPOOLMAN_METRICS_ENABLED` environment variable.
- WebSocket support on all REST endpoints — use polling instead.

[Unreleased]: https://github.com/crjeder/Spoolman-light/compare/v1.0.0...HEAD
[1.0.0]: https://github.com/crjeder/Spoolman-light/compare/v0.22.1...v1.0.0
