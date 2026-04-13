## Why

`scripts/convert_export.py` produces output that no longer matches the current server's data model: field names diverged (e.g. `vendor` vs `manufacturer`, `settings_extruder_temp` vs `print_temp`), color fields remain as hex strings instead of the required `Vec<Rgba>`, spool locations are stored as strings rather than a separate `locations` array with `location_id` references, `used_weight` is left as-is instead of being folded into `current_weight`, and the `locations` key is missing from the output entirely. Anyone running the script today gets a file the server will reject or silently misread.

## What Changes

- **Filament extraction**: rename `vendor` → `manufacturer`; drop `name` but derive `material_modifier` from it by stripping the material-type prefix; rename `settings_extruder_temp` → `print_temp` and `settings_bed_temp` → `bed_temp`; drop `extra`
- **Spool extraction**: convert `color_hex` / `multi_color_hexes` hex strings → `colors: [{r,g,b,a}]` RGBA objects; compute `current_weight = initial_weight - used_weight` (fall back to `initial_weight` when `used_weight` is null); rename `location` string → `location_id` referencing a generated Location object; drop `spool_weight`, `multi_color_direction`, `extra`
- **Output assembly**: add a `locations` array of `{id, name}` objects to the top-level store
- **Test suite**: update `test_convert_export.py` to match the new output shape

## Capabilities

### New Capabilities
- none

### Modified Capabilities
- `legacy-export-converter`: field mappings, color format, location handling, `current_weight` derivation, and `locations` array all need updating to match the current server data model

## Impact

- `scripts/convert_export.py` — all mapping functions rewritten
- `scripts/test_convert_export.py` — test assertions updated to match new field names and types
- No server code changes; no API changes
- Users running the script on old exports must re-run after this fix
