# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- Full Rust rewrite: Axum backend + Leptos WASM frontend in a Cargo workspace (`crates/spoolman-types`, `crates/spoolman-server`, `crates/spoolman-client`). Build with `LEPTOS_WASM_BINDGEN_VERSION=0.2.114 cargo leptos build`.
- `docker-compose.yml` at repo root for quick local deployment; data persisted in a named Docker volume at `/data/spoolman.json`.
- `rust-toolchain.toml` declares the stable toolchain with `wasm32-unknown-unknown` target so `rustup` installs it automatically.
- `spoolman-types` shared crate: `Spool`, `Filament`, `Location`, `DataStore` types used by both server and client, ensuring compile-time API contract consistency.
- `Location` as a first-class entity with full CRUD — replaces the previous freeform string field on Spool.
- `GET /api/v1/filament/search?q=` endpoint: proxies SpoolmanDB on demand (no background scheduler or local cache).
- `GET /api/v1/export` endpoint: downloads the full data store as JSON.
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
- README updated with Rust build instructions and revised environment variable reference.
- Dockerfile: `SPOOLMAN_DATA_FILE` defaults to `/data/spoolman.json` and `LEPTOS_SITE_ROOT` defaults to `/site` in the container, matching the volume mount convention.
- Static asset path in the server is now read from `LEPTOS_SITE_ROOT` env var (fallback: `target/site`), enabling the production Docker layout without recompilation.

### Fixed

- `entrypoint.sh` had Windows-style CRLF line endings, causing the Docker container to fail to start on Linux with "no such file or directory".
- `JsonStore._flush` could raise `FileNotFoundError` under concurrent requests due to multiple threads racing to rename the same `.tmp` file; writes are now serialized with a reentrant lock.

### Deprecated

### Removed

- `Vendor` entity and all `/api/v1/vendor` endpoints.
- `color_hex`, `multi_color_hexes`, `multi_color_direction` from Filament (moved to Spool).
- `price`, `weight`, `spool_weight`, `article_number`, `external_id` from Filament.
- `lot_nr`, `external_id` from Spool.
- `/api/v1/filament/find-by-color` endpoint (moved to `/api/v1/spool/find-by-color`).
- Database backends: SQLite, PostgreSQL, MySQL, CockroachDB.
- Python dependencies: `SQLAlchemy`, `alembic`, `aiosqlite`, `asyncpg`, `aiomysql`, `psycopg2-binary`, `sqlalchemy-cockroachdb`.
- Alembic migration directory (`migrations/`) and `alembic.ini`.
- Prometheus metrics endpoint (`GET /metrics`) and the `SPOOLMAN_METRICS_ENABLED` environment variable. The `prometheus-client` dependency has been removed.
- **BREAKING**: WebSocket support has been removed. The endpoints `GET /api/v1/` (root), `/api/v1/spool`, `/api/v1/spool/{id}`, `/api/v1/filament`, `/api/v1/filament/{id}`, `/api/v1/setting`, and `/api/v1/setting/{key}` no longer accept WebSocket connections. Use polling on the corresponding REST endpoints instead.

### Fixed

- Filtering spools by empty `filament.name`, `filament.material`, or `filament.vendor` now correctly returns spools whose filament has no value set for that field.

### Security

[Unreleased]: https://github.com/Donkie/Spoolman/compare/v0.22.1...HEAD
