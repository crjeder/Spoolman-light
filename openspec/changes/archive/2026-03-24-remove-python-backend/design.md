## Context

The Rust rewrite (Axum + Leptos WASM) is the active stack. The Python/FastAPI backend in `spoolman/` was retained as a reference while the Rust stack was being developed and verified. The `Dockerfile` has already been converted to a pure Rust build — the Python removal is now primarily about deleting dead code and cleaning up auxiliary files.

Files to remove:
- `spoolman/` — entire Python package
- `tests_integration/` — Docker-based integration test suite (Python/pytest)
- `pyproject.toml`, `pdm.lock`, `uv.lock` — Python package management
- `entrypoint.sh` — launches uvicorn; replaced by the Rust binary's built-in listener
- `.github/workflows/ci.yml` — contains Python lint/style/build steps that no longer apply

## Goals / Non-Goals

**Goals:**
- Delete all Python source code and tooling from the repository
- Remove integration tests that test the Python stack
- Clean up CI so it only builds/tests the Rust stack
- Update `entrypoint.sh` or remove it if the Rust binary handles startup directly
- Update project docs (`CLAUDE.md`, `README.md`, `TODO.md`) to reflect Rust-only stack

**Non-Goals:**
- Writing new Rust integration tests (separate concern)
- Migrating Python integration tests to Rust (out of scope)
- Changing any API contracts or data formats

## Decisions

### Delete `entrypoint.sh` rather than rewrite it
`entrypoint.sh` runs `uvicorn spoolman.main:app`. The Rust binary (`spoolman-server`) accepts `SPOOLMAN_HOST` and `SPOOLMAN_PORT` env vars directly and binds them on startup — no shell wrapper needed. The Docker image already uses `CMD ["/spoolman"]` without the entrypoint script.

**Alternative considered:** Rewrite `entrypoint.sh` for the Rust binary (PUID/PGID mapping). However, the `Dockerfile` already handles user setup at build time with a fixed `app` user. The PUID/PGID dynamic remapping in the old script was a Python-era convenience; the Rust Docker image doesn't use it.

### Remove `tests_integration/` entirely
The integration tests are written in Python/pytest and test the Python HTTP API. They cannot be reused against the Rust server without a full rewrite. Deleting them now avoids confusion about whether they apply to the current stack.

**Alternative considered:** Keep them as "reference tests". Rejected — they import Python client code from `spoolman/` and would be broken/misleading without it.

### Rewrite CI rather than patch it
The `ci.yml` workflow has Python-specific jobs (`style`, `build-client` with `pdm export`). These should be replaced with Rust-centric jobs (`cargo check`, `cargo clippy`, `cargo test`, Docker build smoke test). A clean rewrite is simpler than incrementally removing Python steps.

## Risks / Trade-offs

- **Risk**: `entrypoint.sh` removal breaks Docker deployments that set `PUID`/`PGID`. → Mitigation: Document in CHANGELOG that PUID/PGID env vars are no longer supported; the Rust image runs as a fixed `app` user (uid 1000).
- **Risk**: Removing integration tests leaves the Rust server with no automated API coverage. → Mitigation: Acceptable for now; noted in `TODO.md` as follow-up to add Rust integration tests.
- **Risk**: CI job names referenced by branch protection rules change. → Mitigation: Keep the same job names where possible, or update branch protection rules.

## Migration Plan

1. Delete Python source tree and tooling files
2. Delete integration tests
3. Remove or replace `entrypoint.sh`
4. Rewrite `.github/workflows/ci.yml` for Rust
5. Update `CLAUDE.md`, `README.md`, `TODO.md`
6. Verify `cargo check` passes after removals (no accidental cross-references)

No rollback strategy needed — all changes are deletions in a git repo; revert is trivial.

## Open Questions

- Should PUID/PGID support be added to the Rust Docker image for users who relied on it? (Likely a follow-up issue, not a blocker here.)
