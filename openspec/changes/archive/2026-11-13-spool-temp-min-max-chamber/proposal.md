## Why

Filaments store a single nominal `print_temp` and `bed_temp` next to separate, unused min/max fields. Slicers and OpenPrintTag express temperatures as ranges, and enclosed printers need a chamber temperature. Having both single and min/max values is redundant and confusing.

## What Changes

- **BREAKING**: remove `print_temp` and `bed_temp` from `Filament`; temperatures are stored only as min/max pairs (`min_print_temp`/`max_print_temp`, `min_bed_temp`/`max_bed_temp`).
- Add `min_chamber_temp` / `max_chamber_temp` (`Option<i32>`, degrees C) to `Filament`.
- Schema v2 to v3 migration: a legacy `print_temp`/`bed_temp` fills min and max when those are unset.
- Create/edit filament forms and detail view show min and max inputs for print, bed and chamber temperature.
- SpoolmanDB lookup, OrcaSlicer profile import and the legacy export converter map their single values to min = max.

Note: temperatures live on `Filament` (a spool references a filament); nothing is added to `Spool`.

## Capabilities

### New Capabilities

### Modified Capabilities
- `data-model`: Filament entity drops single temps, gains chamber min/max, schema v3.
- `filament-management`: edit/forms handle min/max for print, bed, chamber.
- `spoolmandb-lookup`: auto-fill sets min and max from the single DB values.
- `legacy-export-converter`: extruder/bed temp map to min and max.

## Impact

- `crates/spoolman-types` (models, requests, `SpoolmanDbEntry` unchanged), `crates/spoolman-server/src/store.rs` (create/update/migrate), `crates/spoolman-client` (`pages/filament.rs`, `pages/spool.rs`, `slicer_profile.rs`), `scripts/convert_export.py` + its test, `assets/spoolman.json` fixture, e2e `filament-import-profile.spec.ts`, README, CHANGELOG.
- API break: clients sending `print_temp`/`bed_temp` must use the min/max fields.
