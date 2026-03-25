## 1. Dev-dependency setup

- [x] 1.1 Add `tokio` (features: `macros`, `rt-multi-thread`), `serde_json`, `http-body-util`, `tempfile`, and `tower` (feature: `util`) to `[dev-dependencies]` in `crates/spoolman-server/Cargo.toml`
- [x] 1.2 Verify `cargo check -p spoolman-server` still passes after adding dev-dependencies

## 2. Test helper module

- [x] 2.1 Create `crates/spoolman-server/tests/common/mod.rs` with a `make_app()` helper that: creates a `NamedTempFile`, calls `JsonStore::load`, builds a default `Config`, calls `routes::build_router`, and returns `(Router, NamedTempFile)` (keeping the temp file alive for the test's duration)
- [x] 2.2 Add a `request(app, method, path, body)` helper in `common` that constructs an `http::Request`, sets `Content-Type: application/json` and `Accept-Encoding: identity`, dispatches via `tower::ServiceExt::oneshot`, collects the body with `http_body_util::BodyExt::collect`, and returns `(StatusCode, serde_json::Value)`

## 3. Health endpoint tests

- [x] 3.1 Create `crates/spoolman-server/tests/health.rs` with a `#[tokio::test]` that calls `GET /health` and asserts status 200

## 4. Filament CRUD tests

- [x] 4.1 Create `crates/spoolman-server/tests/filament.rs`
- [x] 4.2 Test: `POST /api/v1/filament` with minimal valid body → 201, response contains non-zero `id`
- [x] 4.3 Test: `GET /api/v1/filament/{id}` with existing ID → 200, body matches created filament
- [x] 4.4 Test: `GET /api/v1/filament/{id}` with unknown ID → 404
- [x] 4.5 Test: `GET /api/v1/filament` after creating two filaments → 200, `items` contains both
- [x] 4.6 Test: `PATCH /api/v1/filament/{id}` with partial update → 200, changed fields reflected
- [x] 4.7 Test: `DELETE /api/v1/filament/{id}` (no referencing spools) → 204, subsequent GET → 404
- [x] 4.8 Test: `DELETE /api/v1/filament/{id}` when spool references it → 409

## 5. Spool CRUD tests

- [x] 5.1 Create `crates/spoolman-server/tests/spool.rs`
- [x] 5.2 Test: `POST /api/v1/spool` with valid filament ID → 201, response contains nested filament
- [x] 5.3 Test: `POST /api/v1/spool` with unknown filament ID → 404
- [x] 5.4 Test: `GET /api/v1/spool/{id}` with existing ID → 200
- [x] 5.5 Test: `GET /api/v1/spool/{id}` with unknown ID → 404
- [x] 5.6 Test: `GET /api/v1/spool` after creating two spools → 200, `items` contains both
- [x] 5.7 Test: `PATCH /api/v1/spool/{id}` with `current_weight` → 200, `current_weight` updated, `last_used` set
- [x] 5.8 Test: `POST /api/v1/spool/{id}/clone` → 201, cloned spool has different ID, same `filament_id`
- [x] 5.9 Test: `DELETE /api/v1/spool/{id}` → 204, subsequent GET → 404

## 6. Location CRUD tests

- [x] 6.1 Create `crates/spoolman-server/tests/location.rs`
- [x] 6.2 Test: `POST /api/v1/location` with `{ "name": "Shelf A" }` → 201, response contains non-zero `id`
- [x] 6.3 Test: `GET /api/v1/location/{id}` with existing ID → 200, contains `spool_count`
- [x] 6.4 Test: `GET /api/v1/location/{id}` with unknown ID → 404
- [x] 6.5 Test: `GET /api/v1/location` after creating a location → 200, location appears in list
- [x] 6.6 Test: `PATCH /api/v1/location/{id}` with new name → 200, updated name in response
- [x] 6.7 Test: `DELETE /api/v1/location/{id}` (no referencing spools) → 204
- [x] 6.8 Test: `DELETE /api/v1/location/{id}` when spool is assigned → 409

## 7. Settings endpoint tests

- [x] 7.1 Create `crates/spoolman-server/tests/settings.rs`
- [x] 7.2 Test: `PUT /api/v1/setting/{key}` then `GET /api/v1/setting` → the key-value pair appears in the response

## 8. Final verification

- [x] 8.1 Run `cargo test -p spoolman-server` and confirm all tests pass
- [x] 8.2 Update `TODO.md` to mark the integration testing item as complete and move it to `CHANGELOG.md` under the next version
