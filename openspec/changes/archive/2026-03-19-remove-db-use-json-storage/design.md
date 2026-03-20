## Context

Spoolman's backend currently uses SQLAlchemy async ORM with Alembic migrations, supporting SQLite, MySQL, PostgreSQL, and CockroachDB. The data model has three primary entities — `Vendor`, `Filament`, `Spool` — plus `*Field` extension tables for extra attributes, and a `Setting` table. The database layer lives in `spoolman/database/` and is consumed by FastAPI route handlers in `spoolman/api/`.

The goal is to replace the entire database stack with a single JSON file, eliminating the ORM, migration tooling, and multi-engine support.

## Goals / Non-Goals

**Goals:**
- Replace `spoolman/database/` with a `JsonStore` class that persists all data to a single `spoolman.json` file
- Maintain the same API contracts (routes, request/response shapes) so the frontend is unaffected
- Atomic writes: data file is written to a temp file then renamed to prevent corruption
- In-memory store loaded once at startup, flushed to disk on every mutation
- Auto-increment integer IDs preserved (no UUID change)
- Retain `extra` fields (currently in `*Field` relationship tables) as inline dicts in JSON

**Non-Goals:**
- Data migration from existing SQLite/Postgres/MySQL databases (out of scope)
- Multi-process safety / concurrent writer support
- Keeping Alembic or any SQL migration tooling
- Keeping MySQL/PostgreSQL/CockroachDB support

## Decisions

### 1. Single JSON file, not per-entity files

**Decision:** One `spoolman.json` with top-level keys `vendors`, `filaments`, `spools`, `settings`, `meta` (schema version).

**Rationale:** Simpler atomicity — one write keeps everything consistent. Per-entity files require coordinated writes across multiple files. With small filament spool data (hundreds of records at most), a single file is practical.

**Alternative considered:** SQLite kept but hidden from user — rejected because it still requires SQLAlchemy and Alembic.

### 2. Synchronous `JsonStore`, async wrappers via `asyncio.to_thread`

**Decision:** Core `JsonStore` methods are synchronous (file I/O). Route handlers call them via `asyncio.to_thread()` to stay non-blocking.

**Rationale:** File I/O is straightforward sync; async file libraries (aiofiles) add complexity without meaningful benefit for this workload. Keeps `JsonStore` easy to test.

### 3. In-memory dict as primary store, disk as persistence

**Decision:** Load JSON into a `dict` at startup. All reads/writes operate on the dict. Every mutation calls `_flush()` which atomically writes to disk.

**Rationale:** Avoids repeated file reads. For a local filament tracker the entire dataset fits easily in memory.

### 4. Pydantic models replace SQLAlchemy models

**Decision:** Define `VendorModel`, `FilamentModel`, `SpoolModel`, `SettingModel` as `pydantic.BaseModel` classes in a new `spoolman/storage/models.py`.

**Rationale:** Pydantic is already used in the FastAPI layer. Replacing SQLAlchemy mapped classes with Pydantic models is a natural fit. Existing API schemas (`spoolman/api/v1/models/`) stay unchanged.

### 5. `extra` fields stored as `dict[str, str]` inline

**Decision:** Drop the `*Field` relationship tables. Store `extra` as a plain dict on each entity.

**Rationale:** The only purpose of the join tables was typed extensibility in SQL. In JSON this is native.

## Risks / Trade-offs

- **Data loss on crash during write** → Mitigated by atomic rename (write to `.tmp`, then `os.replace`)
- **No concurrent access** → Acceptable for single-user self-hosted tool; document the limitation
- **Large datasets slow JSON serialization** → Unlikely given use case (hundreds of spools max); if needed, add background flush with dirty-flag
- **No query filtering at storage level** → All filtering done in Python after loading full dataset; adequate for expected data sizes
- **Breaking change for existing users** → No auto-migration; users must export data via existing API before upgrading, documented in release notes

## Migration Plan

1. Users export their data using the existing `/api/v1/backup` or export endpoints before upgrading
2. After upgrade, start fresh (empty JSON file created automatically on first run)
3. Users re-import via API if an import endpoint exists, or manually

Rollback: keep old version image; JSON format is new so there is no automatic downgrade path.

## Open Questions

- Should a one-time importer from the old SQLite DB be provided as a separate script? (Recommended but out of scope for this change)
- Does the Prometheus metrics endpoint (`spoolman/prometheus/`) need changes? (Likely yes — it reads from the DB session; needs to read from `JsonStore` instead)
