## Context

`Filament` has `print_temp`, `bed_temp` plus four min/max fields that no form sets (forms send `None`). Data is a JSON file with `meta.schema_version` (currently 2) and a `migrate()` in `store.rs`.

## Goals / Non-Goals

**Goals:** ranges only, chamber range added, existing data keeps its values.
**Non-Goals:** unit conversion, per-spool temperatures.

## Decisions

- **Drop single fields, no extra "nominal" field.** Min/max is the single source of truth. A single value is entered as min = max.
- **Legacy read like `net_weight`:** keep `print_temp`/`bed_temp` on `Filament` as `#[serde(default, skip_serializing)]` so v2 files load; `migrate()` (v2 to v3) copies them into min/max where those are `None`, clears them, and bumps `schema_version` to 3. A separate legacy struct was rejected as more code.
- **Requests:** remove `print_temp`/`bed_temp` from create/update requests; add chamber min/max. `apply_option` handling unchanged.
- **Validation:** server rejects min > max (422) per pair when both are set.
- **Display:** detail view shows a range, e.g. `190-220°C`; a single value when min = max or only one is set.
- **Imports:** SpoolmanDB (`extruder_temp`, `bed_temp`), Orca (`nozzle_temperature`, plate temp) and the legacy converter set min = max. `SpoolmanDbEntry` and `slicer_profile` keep single-value fields (external formats).

## Risks / Trade-offs

- API break for external clients: documented in CHANGELOG as breaking.
- Migration is one-way; older servers reject v3 files via the existing schema_version check.
