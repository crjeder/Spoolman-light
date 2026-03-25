## Why

The Docker-based Python/pytest integration test suite has been removed, leaving the Axum REST API with no automated regression coverage. A Rust-native integration test suite is needed to catch regressions before they reach production.

## What Changes

- Add a `tests/` directory (or `spoolman-server/tests/`) containing Rust integration tests that spin up the Axum server in-process and exercise the full HTTP API via `reqwest`.
- Cover all CRUD endpoints for spools, filaments, and locations, plus error cases (404, 400, conflict).
- Tests run with `cargo test` — no Docker required.

## Capabilities

### New Capabilities

- `api-integration-tests`: HTTP-level integration tests for the Axum REST API, covering spool, filament, and location CRUD operations, validation errors, and data persistence across requests within a test run.

### Modified Capabilities

<!-- No existing spec-level requirements are changing — this adds test infrastructure only. -->

## Impact

- New dev-dependency: `reqwest` (with `json` feature), `tokio` (test runtime), possibly `tempfile` for isolated data files per test.
- Tests live alongside `spoolman-server` — either as `spoolman-server/tests/` (Rust integration test convention) or a separate `spoolman-tests` workspace crate.
- No changes to production code, API contracts, or data model.
- CI would gain a `cargo test` step (not in scope for this change but enabled by it).
