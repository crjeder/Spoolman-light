## 1. Grouping logic

- [x] 1.1 In `SpoolList`, derive `grouped = mode == Spool && ts.sort_field == "location"`
- [x] 1.2 Add `group_by_location(items)` returning `(Option<u32>, Vec<SpoolResponse>)` from the already location-sorted list
- [x] 1.3 Add expanded-set signal `RwSignal<HashSet<Option<u32>>>` (in memory only)
- [x] 1.4 Derive `total` and page slice per mode (groups when grouped, spools otherwise)

## 2. Rendering

- [x] 2.1 Extract the per-spool `<tr>` into a closure shared by flat and expanded rows
- [x] 2.2 Render group row: `+`/`-` prefix, name (`No location` for none), count with `entry`/`entries`, click toggles expansion
- [x] 2.3 Fill Filament/Material/Color cells of the group row from active search, material and colour filters; leave others empty
- [x] 2.4 Style group rows in `style/spoolman.css` (clickable, bold, subtle background)

## 3. Verification

- [x] 3.1 Client compiles for wasm32 (cargo check/clippy)
- [x] 3.2 Add Playwright tests in `tests/e2e/tests/` for group rows, expand/collapse, count under filter, filter values, return to flat table
- [ ] 3.3 Run `./scripts/run-e2e.sh`
- [ ] 3.4 Update `CHANGELOG.md` / `TODO.md` on archive
