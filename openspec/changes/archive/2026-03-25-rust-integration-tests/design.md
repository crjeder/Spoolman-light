## Context

The Axum REST API has no automated tests. `routes::build_router` is already `pub` and takes a `JsonStore` + `&Config` — both trivially constructable in test code. No TCP listener is required; Axum supports in-process request dispatch via `tower::ServiceExt::oneshot`.

The previous Python/pytest suite required Docker. The new suite must run with plain `cargo test`.

## Goals / Non-Goals

**Goals:**
- Cover all CRUD endpoints: filament, spool, location, health, settings
- Cover error paths: 404 on missing IDs, 409 on referential integrity violations, 400 on invalid input
- Run with `cargo test` — no Docker, no external processes, no network ports
- Each test gets an isolated, empty in-memory store (via a temp file)

**Non-Goals:**
- Frontend / WASM testing
- Load or performance testing
- Coverage of the backup subsystem

## Decisions

### In-process dispatch over real TCP server

**Decision:** Use `tower::ServiceExt::oneshot` (already in the Axum dependency tree) to call the router directly in tests, rather than `reqwest` against a bound TCP port.

**Rationale:** No port allocation, no race conditions, no cleanup needed. `oneshot` calls the full middleware stack (compression, CORS, trace layers) exactly as a real request would.

**Alternative considered:** `axum-test` crate (provides a higher-level `TestServer`). Adds a new dependency for marginal ergonomic gain; `tower` + `hyper` primitives are already present.

### Integration tests as a separate Cargo crate

**Decision:** Create `crates/spoolman-tests/` as a new workspace crate with `[dev-dependencies]` on `spoolman-server` and `spoolman-types`.

**Rationale:** Keeps test code out of `spoolman-server/src/` and avoids inflating the server binary. Rust's `tests/` directory (adjacent to `src/`) would also work but would require making `store` and `config` modules `pub`, which we want to avoid for production code encapsulation.

**Alternative considered:** `spoolman-server/tests/` (Rust integration test convention). This is simpler but requires pub-exposing internal modules. The separate crate approach keeps the server's module visibility unchanged — tests call only `routes::build_router` and `store::JsonStore::load` which are already `pub`.

Actually, reconsidering: `spoolman-server/tests/` is the idiomatic Rust approach for integration tests of a crate — they already have access to the crate's public API. `build_router` and `JsonStore::load` are already pub. This is simpler and requires no new workspace crate.

**Revised decision:** Use `spoolman-server/tests/` (standard Rust integration test directory). Tests have access to `spoolman_server`'s public API without any structural changes. Dev-dependencies go in `spoolman-server/Cargo.toml`.

### Test isolation via `tempfile`

**Decision:** Each test creates a named temp file via the `tempfile` crate. `JsonStore::load` is called with this path; the temp file is automatically deleted when the `TempDir`/`NamedTempFile` guard drops.

**Rationale:** Guarantees no state leaks between tests even when run in parallel. Pure file I/O — no mocking required.

### HTTP client in tests: `hyper` + `http-body-util`

**Decision:** Use `hyper::Request` to construct test requests, `tower::ServiceExt::oneshot` to dispatch, and `http-body-util::BodyExt::collect` to read the response body. Both are already transitive dependencies.

**New dev-dependencies:**
- `tokio` (feature: `macros`, `rt-multi-thread`) — `#[tokio::test]` runtime
- `serde_json` — already used by the server crate, needed in tests for JSON construction/parsing
- `http-body-util` — response body collection
- `tempfile` — isolated temp files per test

## Risks / Trade-offs

- **Windows path handling** — `tempfile` on Windows uses `%TEMP%`, which works fine with `JsonStore`'s path canonicalization logic.
- **Parallel test isolation** — Each test gets its own `TempFile`; no shared global state, so `cargo test` parallelism is safe.
- **Middleware side effects** — `CompressionLayer` may compress responses; tests must either set `Accept-Encoding: identity` or decompress. Starting without compression in test `Config` (no env-var change needed — just build router without compression) is an option, but using the real router is preferred for fidelity. Tests will set `Accept-Encoding: identity`.
- **`tracing_subscriber` global init** — `tracing_subscriber::registry().init()` panics if called twice in the same process. Tests must not call `main`'s init; avoid initializing tracing in tests entirely (the server only logs; tests don't need it).
