## Why

The integration test suite (`tests_integration/`) was written against the old Python/FastAPI backend. The backend has since been rewritten in Rust (Axum), which carries several breaking changes: the Filament and Spool data models were restructured, the Vendor and extra-fields subsystems were removed, a new Location entity was added, weight-tracking semantics changed (scale-based `current_weight` instead of `used_weight`), and JSON file storage replaced the multi-database setup. As a result every test file fails or tests behaviour that no longer exists.

## What Changes

- **`tests_integration/tests/conftest.py`** — rewrite fixtures to match the Rust data model (`manufacturer`/`material` instead of `name`/`vendor`; `colors` Vec<Rgba> instead of `color_hex`; `initial_weight`/`current_weight` semantics).
- **`tests_integration/tests/filament/`** — rewrite all five test files for the new Filament schema (no `name`, `vendor`, `color_hex`, `price`, `extra`; new fields `manufacturer`, `material`, `material_modifier`, `net_weight`, `print_temp`, `bed_temp`).
- **`tests_integration/tests/spool/`** — rewrite all eight test files for the new Spool schema (`colors`, `color_name`, `location_id`, `current_weight`; derived `used_weight`, `remaining_filament`, `remaining_pct`; no `used_length`, `remaining_length`, `price`, `extra`).
- **`tests_integration/tests/fields/`** — delete entirely (extra-fields system removed).
- **`tests_integration/tests/location/`** — add new test module for Location CRUD (`GET/POST/PATCH/DELETE /api/v1/location`).
- **`tests_integration/tests/setting/`** — update for the `PUT /api/v1/setting/:key` endpoint (body now `{"value": "..."}` with a typed JSON schema).
- **`tests_integration/run.py`** — remove `postgres`, `mariadb`, `cockroachdb` targets; JSON file storage is the only backend.
- **`tests_integration/docker-compose-sqlite.yml`** — update environment variables and volume mount to match the Rust image (`SPOOLMAN_DATA_FILE=/data/spoolman.json`).

## Capabilities

### New Capabilities

- `integration-tests`: Up-to-date test coverage for the Rust backend API (filament, spool, location, setting, backup).

### Modified Capabilities

- `integration-tests`: Existing test files updated to use the new Rust data model and API contracts.

## Impact

- All changes are confined to `tests_integration/`; no application code is touched.
- The single Docker target (`sqlite`) becomes the only target in `run.py`.
- Test infrastructure (Python tester image, `conftest.py` patterns) remains structurally the same — only schema details change.
