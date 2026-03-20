## 1. Documentation Cleanup

- [x] 1.1 Remove the `SPOOLMAN_DB_TYPE`, `SPOOLMAN_DB_HOST`, `SPOOLMAN_DB_PORT`, `SPOOLMAN_DB_NAME`, `SPOOLMAN_DB_USERNAME`, and `SPOOLMAN_DB_PASSWORD` rows from the env var table in `CLAUDE.md`
- [x] 1.2 Verify no other project-owned files (outside `.venv/`) still reference `SPOOLMAN_DB_*` variables

## 2. Verification

- [x] 2.1 Confirm `spoolman/env.py` contains no `SPOOLMAN_DB_*` parsing logic
- [x] 2.2 Confirm `Dockerfile` contains no `SPOOLMAN_DB_*` references
