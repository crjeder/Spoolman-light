## Context

### Rust API surface (current)

**Filament** — `POST /api/v1/filament`, `GET /api/v1/filament`, `GET /api/v1/filament/:id`, `PATCH /api/v1/filament/:id`, `DELETE /api/v1/filament/:id`, `GET /api/v1/filament/search`

Request body (`CreateFilament`):
```json
{
  "manufacturer": "string|null",
  "material": "string|null",
  "material_modifier": "string|null",
  "diameter": 1.75,
  "net_weight": "number|null",
  "density": 1.24,
  "print_temp": "integer|null",
  "bed_temp": "integer|null",
  "spool_weight": "number|null",
  "min_print_temp": "integer|null",
  "max_print_temp": "integer|null",
  "min_bed_temp": "integer|null",
  "max_bed_temp": "integer|null",
  "comment": "string|null"
}
```
Response adds: `id`, `registered` (ISO-8601 UTC).

**Spool** — `POST /api/v1/spool`, `GET /api/v1/spool`, `GET /api/v1/spool/:id`, `PATCH /api/v1/spool/:id`, `DELETE /api/v1/spool/:id`, `POST /api/v1/spool/:id/clone`

Request body (`CreateSpool`):
```json
{
  "filament_id": 1,
  "colors": [{"r": 255, "g": 0, "b": 0, "a": 255}],
  "color_name": "string|null",
  "location_id": "integer|null",
  "initial_weight": 1250.0,
  "first_used": "datetime|null",
  "last_used": "datetime|null",
  "comment": "string|null"
}
```
Response (`SpoolResponse`) adds: `id`, `registered`, `archived`, `current_weight` (starts = `initial_weight`), plus derived read-only fields `used_weight` (= `initial_weight − current_weight`), `remaining_filament`, `remaining_pct`. Also embeds full `filament` object.

**Location** — `POST /api/v1/location`, `GET /api/v1/location`, `GET /api/v1/location/:id`, `PATCH /api/v1/location/:id`, `DELETE /api/v1/location/:id`

Request body: `{"name": "string"}`. Response adds `id` and `spool_count`.

**Settings** — `GET /api/v1/setting` → `{"key": "value", ...}`. `PUT /api/v1/setting/:key` with body `{"value": "string"}` → 204.

**Other** — `GET /api/v1/material` → `["PLA", ...]`. `GET /api/v1/export` → full `DataStore` JSON. `GET /health` → 200. `GET /info` → `{"version": "...", "data_file": "..."}`.

**Removed** — `/api/v1/vendor/*`, `/api/v1/field/*`, WebSocket, Prometheus, per-query deprecated underscore params.

### Infrastructure

The Rust image is built with the root `Dockerfile` (multi-stage Rust build). The tester image uses `tests_integration/Dockerfile` (Python tester). `docker-compose-sqlite.yml` wires them together; the Rust server needs:
```yaml
environment:
  - SPOOLMAN_DATA_FILE=/data/spoolman.json
  - SPOOLMAN_LOGGING_LEVEL=INFO
volumes:
  - spoolman_data:/data
```

## Goals / Non-Goals

**Goals:**
- Every existing test that covers still-supported behaviour is rewritten to pass against the Rust backend.
- New test module added for Location CRUD.
- Tests for removed features (extra fields) are deleted.
- `run.py` only offers a single `sqlite` target.

**Non-Goals:**
- Adding exhaustive property-based or load tests.
- Testing the WASM frontend or WebSocket (removed).
- Porting the `vendor` tests (entity removed — no replacement needed).

## Decisions

### Decision: Keep Python pytest tester; do not rewrite tests in Rust

The existing tester infrastructure (Python + httpx + pytest) is clean and still appropriate for black-box HTTP integration testing. Rewriting in Rust provides no meaningful benefit and would double the work.

**Alternative considered**: Rust integration tests using `reqwest`. Rejected — higher migration cost, no correctness benefit.

### Decision: Replace `color_hex: "FF0000"` with `colors: [{"r":255,"g":0,"b":0,"a":255}]`

The new API uses a typed RGBA color list. All test fixtures and assertions must use this representation.

**Alternative considered**: keep `color_hex` as a convenience field. Rejected — the Rust API does not expose it.

### Decision: Weight semantics change — use `current_weight` not `used_weight` on create

`CreateSpool` takes `initial_weight` (total scale weight at creation). `UpdateSpool` takes `current_weight` (latest scale reading). `used_weight` is always derived (`initial_weight − current_weight`). Tests that previously set `used_weight` on create must be updated to set `initial_weight` and then PATCH `current_weight` to simulate consumption.

### Decision: Remove `length_from_weight` helper from conftest

`used_length` and `remaining_length` are no longer returned. The helper is dead code. Remove it and update assertions to use the new derived fields (`remaining_filament`, `remaining_pct`).

### Decision: Delete `tests/fields/` entirely

The extra-fields system was removed. No stub tests are needed. A comment in `CHANGELOG.md` already records the removal.

### Decision: Rename `docker-compose-sqlite.yml` target in `run.py`

The `run.py` `VALID_TARGETS` list still contains `postgres`, `mariadb`, `cockroachdb`. Remove them. The script becomes a thin wrapper around the single compose file and can drop the `--targets` loop.

## Risks / Trade-offs

- **[Risk] Test isolation** — integration tests share a running server; test order may matter for IDs. Mitigation: each test/fixture creates and then deletes its own data (already established pattern).
- **[Risk] `spool_count` on Location** — LocationResponse embeds a derived `spool_count`. Tests that delete spools before asserting location count must be careful about ordering.
- **[Risk] `remaining_filament` is null when `net_weight` is null** — tests using bare filaments (no `net_weight`) must not assert on `remaining_filament`.
