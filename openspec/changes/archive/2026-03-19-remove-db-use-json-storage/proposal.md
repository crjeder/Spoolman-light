## Why

Spoolman currently depends on SQLAlchemy + Alembic with SQLite/MySQL/PostgreSQL/CockroachDB backends, which adds significant operational complexity for a simple self-hosted filament tracker. Replacing the database with a flat JSON file eliminates the database engine dependency, removes migration management, and makes the data directly human-readable and portable.

## What Changes

- **BREAKING**: Remove SQLAlchemy, Alembic, and all database engine dependencies
- **BREAKING**: Remove support for MySQL, PostgreSQL, and CockroachDB backends
- Replace `spoolman/database/` module with a JSON-based storage layer
- Data persisted to a single `spoolman.json` file (path configurable via env var)
- All CRUD operations read/write from in-memory dict backed by the JSON file
- Remove Alembic migrations directory and related config
- Remove database-related environment variables (`SPOOLMAN_DB_*`) and replace with `SPOOLMAN_DATA_FILE`

## Capabilities

### New Capabilities
- `json-storage`: Read/write spool, filament, and vendor data to/from a JSON file with atomic writes and in-memory caching

### Modified Capabilities
<!-- No existing specs to delta against -->

## Impact

- **Backend**: `spoolman/database/` fully replaced; `spoolman/api/` route handlers updated to use new storage layer; `spoolman/settings.py` updated for new env vars
- **Dependencies**: Remove `sqlalchemy`, `alembic`, `asyncpg`, `aiomysql`, `aiosqlite` from `pyproject.toml`
- **Docker/deployment**: Persistent volume only needs to store one JSON file instead of a DB file or connection
- **Data migration**: No automatic migration from existing DBs; users must export data manually
