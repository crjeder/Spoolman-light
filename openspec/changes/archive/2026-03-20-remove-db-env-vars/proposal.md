## Why

Spoolman is migrating from a SQLAlchemy-backed relational database to a JSON file storage backend. Now that the JSON storage spec is in place, the database-specific environment variables (`SPOOLMAN_DB_TYPE`, `SPOOLMAN_DB_HOST`, `SPOOLMAN_DB_PORT`, `SPOOLMAN_DB_NAME`, `SPOOLMAN_DB_USERNAME`, `SPOOLMAN_DB_PASSWORD`, `SPOOLMAN_DB_PASSWORD_FILE`) are no longer relevant and should be removed to reduce confusion and surface area.

## What Changes

- **BREAKING** Remove `SPOOLMAN_DB_TYPE` environment variable (and all its accepted values: `sqlite`, `postgres`, `mysql`, `cockroachdb`)
- **BREAKING** Remove `SPOOLMAN_DB_HOST` environment variable
- **BREAKING** Remove `SPOOLMAN_DB_PORT` environment variable
- **BREAKING** Remove `SPOOLMAN_DB_NAME` environment variable
- **BREAKING** Remove `SPOOLMAN_DB_USERNAME` environment variable
- **BREAKING** Remove `SPOOLMAN_DB_PASSWORD` environment variable
- **BREAKING** Remove `SPOOLMAN_DB_PASSWORD_FILE` environment variable
- Remove all parsing, validation, and connection-string construction logic tied to these variables from `spoolman/env.py`
- Update documentation (README, CLAUDE.md) to remove references to these variables
- Remove any startup warnings or checks that reference DB env vars

## Capabilities

### New Capabilities
- None

### Modified Capabilities
- `json-storage`: Configuration contract changes — `SPOOLMAN_DATA_FILE` is now the only storage-related env var; the DB env vars are explicitly unsupported and should be ignored or raise a warning if set

## Impact

- `spoolman/env.py` — remove DB var parsing/validation logic
- `README.md` — remove DB env var documentation
- `CLAUDE.md` — update env var table
- `Dockerfile` — remove any DB-related `ENV` defaults if present
- Integration tests — remove DB-variant test configs (postgres, mariadb, cockroachdb compose files already deleted per git status)
- Users running postgres/mysql/cockroachdb deployments will need to migrate to the JSON storage backend before upgrading
