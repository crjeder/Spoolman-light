## 1. Filter Signal and Logic

- [x] 1.1 Add `material_filter: RwSignal<String>` (empty = all) to `SpoolList` in `spool.rs`
- [x] 1.2 Compute `available_materials` signal: derive distinct material abbreviations from the full (unfiltered) `spools` resource, sorted alphabetically
- [x] 1.3 Add material predicate to `filtered()` closure: skip spools whose filament material abbreviation doesn't match `material_filter` when filter is non-empty

## 2. Column Header UI

- [x] 2.1 Insert a `<th class="material-head">` after the Filament column in the table header
- [x] 2.2 Render label "Material" with ■ indicator when `material_filter` is non-empty
- [x] 2.3 Render `<select>` inside the header with "All" option (value = "") and one `<option>` per entry in `available_materials`
- [x] 2.4 Wire `on:change` to set `material_filter` from the selected value

## 3. Column Data Cell

- [x] 3.1 Insert a `<td>` for material after the Filament cell in each table row, showing `filament.material.map(|m| m.abbreviation()).unwrap_or_default()`

## 4. Verification

- [x] 4.1 Run `cargo check -p spoolman-client --target wasm32-unknown-unknown` — no errors
- [ ] 4.2 Manually verify: load spool list, confirm Material column shows correct abbreviations and blank for unset
- [ ] 4.3 Manually verify: select a material from the dropdown, confirm list filters correctly and ■ appears in header
- [ ] 4.4 Manually verify: select "All", confirm filter clears and ■ disappears
- [ ] 4.5 Manually verify: material filter combines correctly with text search and color filter
