## 1. Infrastructure

- [x] 1.1 Update `tests_integration/run.py`: remove `postgres`, `mariadb`, `cockroachdb` from `VALID_TARGETS`; simplify to a single `sqlite` target
- [x] 1.2 Update `tests_integration/docker-compose-sqlite.yml`: replace the `environment` block with `SPOOLMAN_DATA_FILE=/data/spoolman.json` and `SPOOLMAN_LOGGING_LEVEL=INFO`; add a named volume `/data` so the JSON store persists between test containers starting
- [x] 1.3 Verify `tests_integration/Dockerfile` (tester image) requires no changes — it only installs Python deps and runs pytest

## 2. conftest.py

- [x] 2.1 Rewrite `random_filament_impl` fixture: use `manufacturer="TestBrand"`, `material="PLA"`, `density=1.24`, `diameter=1.75`, `print_temp=210`, `bed_temp=60`, `comment="abcdefghåäö"` — remove `name`, `vendor`, `settings_extruder_temp`, `settings_bed_temp`
- [x] 2.2 Rewrite `random_empty_filament_impl` fixture: use only `density=1.24`, `diameter=1.75`
- [x] 2.3 Rewrite `random_spool_impl` fixture: use `filament_id`, `initial_weight=1250`, `colors=[{"r":255,"g":0,"b":0,"a":255}]` — remove `spool_weight`, `color_hex`, `price`; update cleanup to DELETE by ID
- [x] 2.4 Add `random_location_impl` context manager and `random_location` / `random_location_mod` fixtures (POST `{"name": "Test Location"}`, DELETE on teardown)
- [x] 2.5 Remove `length_from_weight` helper (no longer returned by API)
- [x] 2.6 Keep `assert_dicts_compatible`, `assert_lists_compatible`, `assert_httpx_success`, `assert_httpx_code` unchanged

## 3. Filament tests

- [x] 3.1 `tests/filament/test_add.py` — rewrite: assert `manufacturer`, `material`, `density`, `diameter`, `print_temp`, `bed_temp`, `comment` round-trip; assert `registered` is recent; assert 201 status; assert absent optional fields are `null`
- [x] 3.2 `tests/filament/test_get.py` — rewrite: assert single GET returns full Filament object; assert 404 for missing ID
- [x] 3.3 `tests/filament/test_update.py` — rewrite: PATCH individual fields (`material`, `comment`, `net_weight`) and assert response reflects change; assert non-patched fields unchanged
- [x] 3.4 `tests/filament/test_delete.py` — rewrite: DELETE returns 204; subsequent GET returns 404; delete of non-existent returns 404
- [x] 3.5 `tests/filament/test_find.py` — rewrite: list with `?material=PLA` filters correctly; list with no params returns all; `X-Total-Count` header matches list length; `?sort` / `?order` / `?limit` / `?offset` work as expected

## 4. Spool tests

- [x] 4.1 `tests/spool/test_add.py` — rewrite: create spool with `initial_weight`, `colors`, `color_name`; assert `current_weight == initial_weight`; assert `used_weight == 0`; assert `remaining_filament` and `remaining_pct` are correct when filament has `net_weight`; assert `remaining_filament == null` when filament has no `net_weight`; assert embedded `filament` object is correct; assert 201 status
- [x] 4.2 `tests/spool/test_get.py` — rewrite: GET returns SpoolResponse with embedded filament; 404 for missing ID
- [x] 4.3 `tests/spool/test_update.py` — rewrite: PATCH `current_weight` and verify `used_weight` updates; PATCH `colors`, `comment`, `archived`, `location_id`; assert non-patched fields unchanged
- [x] 4.4 `tests/spool/test_delete.py` — rewrite: DELETE returns 204; subsequent GET returns 404; deleting a spool decrements `location.spool_count`
- [x] 4.5 `tests/spool/test_find.py` — rewrite: filter by `filament_id`; filter by `location_id`; `allow_archived=true` includes archived spools; `allow_archived=false` (default) excludes them; `X-Total-Count` matches
- [x] 4.6 `tests/spool/test_find_by_color.py` — removed: no color-proximity filter endpoint exists in the Rust API
- [x] 4.7 `tests/spool/test_measure.py` — rewrite: the `use` endpoint is gone; test `PATCH /api/v1/spool/:id` with `current_weight` to simulate filament consumption; assert `used_weight` and `remaining_filament` update correctly
- [x] 4.8 `tests/spool/test_use.py` — deleted: the `/api/v1/spool/:id/use` endpoint was removed; consumption is now modelled by PATCHing `current_weight`

## 5. Location tests (new)

- [x] 5.1 Create `tests_integration/tests/location/__init__.py`
- [x] 5.2 Create `tests_integration/tests/location/test_add.py`: POST `{"name": "Shelf A"}`; assert 201 and `{"id": ..., "name": "Shelf A", "spool_count": 0}`
- [x] 5.3 Create `tests_integration/tests/location/test_get.py`: GET by ID returns location; 404 for missing
- [x] 5.4 Create `tests_integration/tests/location/test_update.py`: PATCH `name`; assert response updated
- [x] 5.5 Create `tests_integration/tests/location/test_delete.py`: DELETE returns 204; verify 404 after; assert DELETE fails (409 or similar) when location has spools assigned
- [x] 5.6 Create `tests_integration/tests/location/test_find.py`: list all locations; assert `spool_count` increments when a spool is assigned

## 6. Settings tests

- [x] 6.1 `tests/setting/test_get.py` — verify GET `/api/v1/setting` returns a dict; after a PUT it reflects the new value
- [x] 6.2 `tests/setting/test_set.py` — rewrite: PUT `/api/v1/setting/currency_symbol` with body `{"value": "€"}` returns 204; subsequent GET shows the value; overwriting with a new value works

## 7. Fields tests

- [x] 7.1 Delete `tests_integration/tests/fields/` directory entirely (extra-fields system removed)

## 8. Backup test

- [x] 8.1 `tests/test_backup.py` — no manual backup endpoint in Rust (automatic background task, stub); replaced with `GET /api/v1/export` test which validates the full data store JSON shape

## 9. Verification

- [ ] 9.1 Run `python tests_integration/run.py sqlite` (requires Docker) and confirm all tests pass
- [ ] 9.2 Fix any assertion mismatches found during the test run
