## 1. Remove Database Dependencies

- [x] 1.1 Remove `sqlalchemy`, `alembic`, `aiosqlite`, `asyncpg`, `aiomysql`, and related packages from `pyproject.toml`
- [x] 1.2 Delete the `migrations/` (Alembic) directory and `alembic.ini` if present
- [x] 1.3 Remove database-related environment variable handling from `spoolman/env.py` (`SPOOLMAN_DB_*` vars, `DatabaseType` enum, DB connection helpers)
- [x] 1.4 Add `SPOOLMAN_DATA_FILE` env var support to `spoolman/env.py`

## 2. Implement JSON Storage Layer

- [x] 2.1 Create `spoolman/storage/` package with `__init__.py`
- [x] 2.2 Create `spoolman/storage/models.py` with Pydantic models: `VendorModel`, `FilamentModel`, `SpoolModel`, `SettingModel`, and a root `DataStore` model
- [x] 2.3 Create `spoolman/storage/store.py` with `JsonStore` class:
  - `__init__(path: Path)` — stores path, initializes empty data
  - `load()` — reads and parses JSON file, creates default if missing
  - `_flush()` — atomically writes data to disk via temp file + rename
  - CRUD methods for vendors, filaments, spools, and settings
  - Auto-increment ID logic per entity type
- [x] 2.4 Add `extra` dict support (`dict[str, str]`) inline on each entity model

## 3. Wire Storage into Application Startup

- [x] 3.1 Create a `spoolman/storage/dependencies.py` with a FastAPI dependency (`get_store`) that returns the singleton `JsonStore` instance
- [x] 3.2 Initialize `JsonStore` in `spoolman/main.py` (or app startup event), replacing the existing database engine setup
- [x] 3.3 Remove `get_db_session` and all `AsyncSession` dependency usage from the app

## 4. Update API Route Handlers

- [x] 4.1 Rewrite `spoolman/api/v1/vendor.py` to use `JsonStore` instead of `spoolman.database.vendor`
- [x] 4.2 Rewrite `spoolman/api/v1/filament.py` to use `JsonStore` instead of `spoolman.database.filament`
- [x] 4.3 Rewrite `spoolman/api/v1/spool.py` to use `JsonStore` instead of `spoolman.database.spool`
- [x] 4.4 Rewrite `spoolman/api/v1/setting.py` to use `JsonStore` instead of `spoolman.database.setting`
- [x] 4.5 Update `spoolman/api/v1/field.py` (extra fields) to use inline dict storage on each entity
- [x] 4.6 Update `spoolman/api/v1/other.py` (health/info endpoints) to remove DB version/status checks

## 5. Update Prometheus Metrics

- [x] 5.1 Update `spoolman/prometheus/metrics.py` to read filament and spool counts from `JsonStore` instead of DB session

## 6. Delete Old Database Module

- [x] 6.1 Delete `spoolman/database/database.py`
- [x] 6.2 Delete `spoolman/database/models.py`
- [x] 6.3 Delete `spoolman/database/filament.py`, `vendor.py`, `spool.py`, `setting.py`, `utils.py`
- [x] 6.4 Delete `spoolman/database/__init__.py` and the `spoolman/database/` directory

## 7. Update Tests

- [x] 7.1 Remove database fixture setup (DbType enum, get_db_type) from `tests_integration/tests/conftest.py`
- [x] 7.2 Remove DB-specific docker-compose files (postgres, mysql, cockroachdb)
- [x] 7.3 Update `test_backup.py` to remove SQLite-only guard
- [x] 7.4 Run full test suite and fix any remaining failures

## 8. Documentation and Config

- [x] 8.1 Update `README.md` to remove database configuration docs and add `SPOOLMAN_DATA_FILE` env var documentation
- [x] 8.2 Update `docker-compose.yml` / `Dockerfile` volume mounts if needed (JSON file instead of DB file)
- [x] 8.3 Add a release note warning about the breaking change and lack of automatic data migration
