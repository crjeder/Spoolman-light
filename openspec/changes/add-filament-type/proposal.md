## Why

The `material` field on Filament (e.g. "PLA", "PETG", "ABS") already exists in the data model and is stored/returned by the API. However, the UI offers no guidance when typing a material: the input is a plain text box with no suggestions, so users inevitably create inconsistent spellings ("pla", "Pla", "PLA+", "PLA Plus"). The backend already exposes `GET /api/v1/material` — a deduplicated list of every material string in the store — but nothing calls it. As a result, filtering and sorting by material in the UI is impractical.

## What Changes

- **`crates/spoolman-client/src/api/filament.rs`** — add `list_materials()` calling `GET /api/v1/material`.
- **`FilamentCreate` and `FilamentEdit` components** — wire up a `<datalist>` element to the Material `<input>` so the browser shows autocomplete suggestions drawn from `list_materials()`.
- **`FilamentList` component** — replace (or augment) the free-text filter with a material dropdown populated from `list_materials()`; selecting a value calls `GET /api/v1/filament?material=<value>` so filtering is server-side.
- **`SpoolList` component** — add a material column (drawn from the embedded `filament.material`) and include material in the text-filter match so users can type "PLA" to narrow spools.

## Capabilities

### New Capabilities

- `material-autocomplete`: Material input on filament create/edit shows suggestions from existing materials in the store.
- `material-filter`: Filament list can be filtered by material via a dropdown; spool list text filter now matches on material.

### Modified Capabilities

- `filament-list`: Gains a material filter dropdown alongside the existing text search.
- `spool-list`: Text filter now also matches `filament.material`.

## Impact

- All changes are confined to `crates/spoolman-client/` (frontend only). No backend or data-model changes.
- No new API endpoints — uses the existing `GET /api/v1/material`.
- No breaking changes to the API or storage format.
