## 1. Worktree Setup

- [x] 1.1 Create a git worktree at `.worktrees/spoolmandb-filament-lookup` on a new branch `feat/spoolmandb-filament-lookup`

## 2. SpoolmanDB Types and Cache Utility

- [x] 2.1 Add a `SpoolmanEntry` struct in `crates/spoolman-client/src/` (or a new `spoolmandb.rs` module) with fields: `manufacturer`, `name`, `material`, `density`, `diameter`, `extruder_temp`, `bed_temp`, `weight`, `spool_weight`, `color_hex` — use `#[serde(default)]` on all optional fields, lenient deserialization
- [x] 2.2 Implement `parse_material(s: &str) -> (MaterialType, Option<String>)` that strips known suffixes (`+`, `-CF`, `-GF`, `-HF`, `-ESD`) and maps the base via `MaterialType::from_abbreviation()`
- [x] 2.3 Implement `load_spoolmandb() -> Result<Vec<SpoolmanEntry>, String>` async function: check localStorage `spoolmandb_cache`, return cached data if within 24h; otherwise fetch with `If-None-Match` ETag header; handle 304 (bump timestamp), 200 (replace cache), and error (return stale or error)
- [x] 2.4 Write unit tests for `parse_material` covering: plain material, `+` suffix, `-CF` suffix, unknown base string

## 3. SpoolmanDbSearch Component

- [x] 3.1 Implement `#[component] SpoolmanDbSearch(on_select: Callback<SpoolmanEntry>) -> impl IntoView` in the client crate
- [x] 3.2 On component mount, call `load_spoolmandb()` and store result in a signal; show "Loading…" while pending, "Database unavailable" on error
- [x] 3.3 Render a text input; on input, filter the cached entries (case-insensitive match on manufacturer + material + name) and show up to 10 results
- [x] 3.4 Each result renders as a clickable row showing "Manufacturer · Material · Color name"; clicking calls `on_select` with the entry
- [x] 3.5 Empty query shows no results; no-match query shows "No results"

## 4. Filament Create — SpoolmanDB Integration

- [x] 4.1 Add `SpoolmanDbSearch` to `FilamentCreate` in `pages/filament.rs`; wire `on_select` callback to set manufacturer, material, modifier, diameter, density, print_temp, bed_temp signals from the selected entry using `parse_material`

## 5. Filament Edit — SpoolmanDB Integration

- [x] 5.1 Add `SpoolmanDbSearch` to `FilamentEdit` in `pages/filament.rs`; same callback wiring as Filament Create

## 6. Spool Create — SpoolmanDB Integration

- [x] 6.1 Add `SpoolmanDbSearch` to `SpoolCreate` in `pages/spool.rs`; on select, set `color_hex`, `color_name`, and `net_weight` signals from the entry
- [x] 6.2 On entry selection, search the already-fetched `filaments` list for a match (case-insensitive manufacturer, same `MaterialType`, diameter within ±0.01mm)
- [x] 6.3 If a match is found, set `filament_id` to the matched filament's id
- [x] 6.4 If no match is found, call `api::create_filament()` with the entry's physical properties (via `parse_material`), set `filament_id` to the new filament's id, and set a notification signal
- [x] 6.5 Render the auto-create notification as a dismissable info banner: "Filament '[Manufacturer] [Material]' was created automatically."

## 7. Verification

- [x] 7.1 `cargo check -p spoolman-client` (requires wasm32 target) — confirm no compile errors
- [ ] 7.2 Manual smoke test: Filament Create — search "Prusament PLA", select entry, verify all fields populated correctly
- [ ] 7.3 Manual smoke test: Filament Edit — same search on an existing filament, verify fields overwritten
- [ ] 7.4 Manual smoke test: Spool Create — select an entry whose filament does not exist, verify filament is auto-created and notification shown; select an entry whose filament does exist, verify no duplicate is created
- [ ] 7.5 Manual smoke test: close tab, reopen — verify data loads from localStorage cache without a new network request (check DevTools Network tab)
