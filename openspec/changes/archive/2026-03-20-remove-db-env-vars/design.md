## Context

The migration from SQLAlchemy-backed relational databases to JSON file storage is largely complete. The `spoolman/env.py` and `Dockerfile` have already been cleaned up — no DB env var parsing code exists in the running application. However, `CLAUDE.md` still documents the old `SPOOLMAN_DB_*` variables in its env var reference table, creating confusion for contributors.

`CHANGELOG.md` already contains a user-facing notice that these variables are removed.

## Goals / Non-Goals

**Goals:**
- Remove the `SPOOLMAN_DB_*` rows from the `CLAUDE.md` env var table so contributor documentation is accurate
- Confirm no other project-owned files still reference these variables

**Non-Goals:**
- Modifying `env.py` (already clean)
- Modifying `Dockerfile` (already clean)
- Adding runtime warnings for legacy env vars (out of scope; the changelog notice is sufficient)
- Touching `.venv/` or third-party packages

## Decisions

**Edit CLAUDE.md directly** — The env var table in `CLAUDE.md` is the only remaining project-owned reference. A targeted row deletion is the minimum correct change.

**No deprecation shim** — These env vars are no longer read anywhere in the codebase. Adding a warning would require reintroducing code purely to emit a message; the changelog entry is the appropriate migration notice.

## Risks / Trade-offs

- **Risk:** A contributor using an old config with `SPOOLMAN_DB_TYPE=postgres` will silently get JSON storage instead. → Mitigation: already documented in `CHANGELOG.md`; no further action needed in this change.
