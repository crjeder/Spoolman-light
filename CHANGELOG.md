# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- `SPOOLMAN_DATA_FILE` environment variable to configure the path of the JSON data file (default: `~/.local/share/spoolman/spoolman.json`).

### Changed

- Docker builder stage now uses uv instead of pdm to install production dependencies, reducing build time and layer size.
- **BREAKING**: The `Vendor` entity has been removed. Filaments now have a plain `vendor: string` field instead of a foreign-key reference.
- **BREAKING**: Color fields (`color_hex`, `multi_color_hexes`, `multi_color_direction`) have moved from Filament to Spool. Each physical spool can now have its own color.
- **BREAKING**: `price` has moved from Filament to Spool. Each physical spool can record its own purchase price.
- **BREAKING**: `weight` and `spool_weight` have been removed from Filament. These are per-spool properties — set `initial_weight` and `spool_weight` on the Spool instead.
- **BREAKING**: `article_number` and `external_id` have been removed from Filament. `lot_nr` and `external_id` have been removed from Spool.
- The `/filament/find-by-color` endpoint has moved to `/spool/find-by-color`.
- All vendor CRUD endpoints (`GET/POST/PATCH/DELETE /api/v1/vendor`) have been removed.
- **BREAKING**: All data is now stored in a single JSON file (`spoolman.json`) instead of a relational database. SQLite, PostgreSQL, MySQL, and CockroachDB backends have been removed.
- **BREAKING**: There is no automatic migration from an existing database. You must export your data manually before upgrading (e.g. via the export endpoints) and re-import it after.
- The `SPOOLMAN_DB_*` environment variables (`SPOOLMAN_DB_TYPE`, `SPOOLMAN_DB_HOST`, etc.) are no longer recognized and can be removed from your configuration.
- The `/api/v1/info` response field `db_type` has been replaced with `data_file` (the resolved path to the JSON file).
- Docker volume mounts previously pointing at `/home/app/.local/share/spoolman` (for `spoolman.db`) still work — the JSON file is written to the same directory as `spoolman.json`.

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

### Fixed

- Filtering spools by empty `filament.name`, `filament.material`, or `filament.vendor` now correctly returns spools whose filament has no value set for that field.

### Security

[Unreleased]: https://github.com/Donkie/Spoolman/compare/v0.22.1...HEAD
