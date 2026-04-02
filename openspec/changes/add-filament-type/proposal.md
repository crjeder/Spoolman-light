## Why

The `material` field on Filament (e.g. "PLA", "PETG", "ABS") already exists in the data model and is stored/returned by the API. However, the UI offers no guidance when typing a material: the input is a plain text box, so users inevitably create inconsistent spellings ("pla", "Pla", "PLA+", "PLA Plus"). Filtering and sorting by material in the UI is impractical as a result.

## What Changes

- **`crates/spoolman-types/src/`** — add `MaterialType` enum with variants for common filament types (PLA, PETG, ABS, ASA, TPU, NYLON, PC, PVA, HIPS, WOOD, METAL, CARBON_FIBER, OTHER, etc.) and helper methods (`all_known()`, `abbreviation()`).
- **`crates/spoolman-client/src/pages/filament.rs`** — add `MaterialSelect` component rendering a `<select>` from `MaterialType::all_known()`; wire it into `FilamentCreate` and `FilamentEdit`.
- **`FilamentList` component** — replace free-text filter with a material `<select>` dropdown populated from `MaterialType::all_known()`; selecting a value calls `GET /api/v1/filament?material=<value>` for server-side filtering.
- **`SpoolList` component** — text filter now also matches `filament.material.abbreviation()`.

## Capabilities

### New Capabilities

- `material-select`: Material input on filament create/edit is a `<select>` restricted to known `MaterialType` variants — no free-text entry, no inconsistent spellings.
- `material-filter`: Filament list can be filtered by material via a dropdown driven by the same enum; spool list text filter now matches on material abbreviation.

### Modified Capabilities

- `filament-list`: Gains a material filter dropdown alongside the existing text search.
- `spool-list`: Text filter now also matches `filament.material.abbreviation()`.

## Impact

- `MaterialType` enum lives in `crates/spoolman-types/` (shared); the `<select>` UI components live in `crates/spoolman-client/`.
- No new API endpoints — uses the existing `GET /api/v1/filament?material=<value>`.
- No breaking changes to the API or storage format (material is still stored as a string).
