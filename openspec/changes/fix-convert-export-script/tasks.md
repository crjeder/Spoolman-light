## 1. Filament extraction fixes

- [x] 1.1 Rename `vendor` → `manufacturer` in `_extract_filament` and `_extract_filament_from_filament_export`
- [x] 1.2 Rename `settings_extruder_temp` → `print_temp` in both filament extraction functions
- [x] 1.3 Rename `settings_bed_temp` → `bed_temp` in both filament extraction functions
- [x] 1.4 Add `material_modifier` derivation: strip material-type prefix from `filament.name` (case-insensitive); fall back to full name; set null when name is absent or equals material alone
- [x] 1.5 Remove `name` and `extra` fields from both filament extraction functions

## 2. Spool extraction fixes

- [x] 2.1 Add `_hex_to_rgba(hex_str)` helper: converts 6-char hex string to `{r, g, b, a: 255}` dict
- [x] 2.2 Replace `color_hex` / `multi_color_hexes` output with a `colors` list: single hex → `[rgba]`, multi → `[rgba, ...]` up to 4, both null → `[]`
- [x] 2.3 Add `current_weight` computation: `initial_weight - (used_weight or 0)`; keep `initial_weight` as-is
- [x] 2.4 Remove `spool_weight`, `used_weight`, `multi_color_direction`, `extra` from spool output
- [x] 2.5 Change `location` string field to `location_id` (wire up after location collection in step 3)

## 3. Location collection

- [x] 3.1 Add `collect_locations(spool_records)` function: returns `(locations_list, name_to_id_map)` — unique non-null location strings → `[{id, name}]` with sequential IDs from 1
- [x] 3.2 Update `extract_spools` to accept the `name_to_id_map` and emit `location_id` instead of `location`
- [x] 3.3 Update `assemble_store` to accept and include `locations` array in the output dict

## 4. Wire up in main()

- [x] 4.1 Call `collect_locations` before `extract_spools` in `main()`
- [x] 4.2 Pass `name_to_id_map` to `extract_spools`
- [x] 4.3 Pass `locations` list to `assemble_store`

## 5. Test suite updates

- [x] 5.1 Update `MINIMAL_SPOOL_EXPORT` fixture to reflect expected new output shape in `test_basic_conversion`
- [x] 5.2 Assert `manufacturer` (not `vendor`), `print_temp`, `bed_temp`, `material_modifier` on filament
- [x] 5.3 Assert `colors` RGBA list (not `color_hex`) on spool
- [x] 5.4 Assert `current_weight` on spool
- [x] 5.5 Assert `location_id` (not `location` string) on spool
- [x] 5.6 Assert `locations` array present in store output
- [x] 5.7 Assert `extra` not present on filament or spool
- [x] 5.8 Add a test for `_hex_to_rgba` covering basic conversion and multi-color
- [x] 5.9 Add a test for `material_modifier` derivation edge cases (null name, name equals material, name with prefix, name without prefix)
- [x] 5.10 Run `python scripts/test_convert_export.py` and confirm all tests pass
