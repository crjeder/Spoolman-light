## Context

The converter script was written alongside an early version of the server. Since then, the server's data model has been finalized with different field names, a structured RGBA color type, and a separate `locations` collection. The script now produces JSON the server either rejects or silently misreads.

The script is pure Python with no external dependencies, making it easy to update in-place. The test suite (`test_convert_export.py`) runs directly with `python scripts/test_convert_export.py` and covers all public functions.

## Goals / Non-Goals

**Goals:**
- All filament field names match `Filament` struct in `spoolman-types`
- All spool field names match `Spool` struct in `spoolman-types`
- Colors are emitted as `[{r, g, b, a}]` RGBA arrays
- Locations are collected into a top-level `locations` array; spools reference them by `location_id`
- `current_weight` is derived from `initial_weight - used_weight`
- Tests pass and cover all new mapping logic

**Non-Goals:**
- Validating or normalizing material strings beyond passing them through (the server accepts any string via `MaterialType::Other`)
- Handling arbitrary old-format variations not present in the documented export format
- GUI or interactive mode

## Decisions

### Rename `vendor` → `manufacturer`, `settings_extruder_temp` → `print_temp`, `settings_bed_temp` → `bed_temp`
Direct rename; no ambiguity. The old Spoolman used different naming conventions.

### Derive `material_modifier` from `filament.name`
The old Spoolman had a free-text `name` field (e.g. "PLA Basic", "Galaxy Black PETG") that served as a product variant label. The new model splits this into `material` (type abbreviation) and `material_modifier` (the variant suffix). Conversion rule: if `name` starts with the material string (case-insensitive), strip it and trim; otherwise use the whole name as the modifier. If the name is null or equal to the material string alone, set `material_modifier` to null.

### Hex strings → RGBA objects
`color_hex` is a 6-character hex string (no `#`). Convert to `{r, g, b, a: 255}`. `multi_color_hexes` is a list of such strings; convert each one. Take up to 4 (the server's documented limit). If both are absent/null, emit `colors: []`.

### Collect locations, assign synthetic IDs
Iterate all spools, collect unique non-null `location` strings, assign sequential IDs starting at 1. Build a `locations` array. Each spool with a non-null `location` gets a `location_id` referencing the matching Location.

### `current_weight = initial_weight - used_weight`
`initial_weight` in the old format was the net filament weight (no spool tare). `used_weight` was consumed filament. The new model's `initial_weight` and `current_weight` are both gross scale readings (including spool), but without a reliable spool_weight to add, we keep the same semantics and just set `current_weight = initial_weight - (used_weight or 0)`. This preserves the remaining-weight ratio correctly, even if absolute values differ from a real scale reading.

### Drop `extra`, `spool_weight` (on spool), `multi_color_direction`
These fields have no equivalent in the current server model. Silently dropping them is the safest approach — unknown fields in the JSON are ignored by serde anyway, but emitting them creates confusion.

## Risks / Trade-offs

- [Risk] `material_modifier` derivation from `name` may produce unexpected results for unusual name strings → users can edit the generated JSON if needed; the script logs a warning when it can't cleanly strip the material prefix
- [Risk] `current_weight` without a real spool tare may be slightly inaccurate vs. a real scale → acceptable for migration purposes; users can re-weigh spools after import
- [Risk] Location ID collisions if the user has existing location data → not a concern for fresh imports; for append scenarios, a `--location-id-start` flag could be added later

## Migration Plan

1. User exports old spoolman data via `/api/v1/export/spools?fmt=json` (and optionally `/api/v1/export/filaments?fmt=json`)
2. User runs updated `convert_export.py`
3. User places output at `SPOOLMAN_DATA_FILE` path and starts new Spoolman
4. No rollback needed — old export files are unchanged; re-run the script to regenerate
