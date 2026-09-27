## Why

Color naming and hex values are currently typed manually, which is imprecise and inconsistent across spools of the same real-world filament. filamentcolors.xyz publishes measured (spectrophotometer) colors with product photos for thousands of filament SKUs, and its API is CORS-open with no proxy needed. Wiring it into spool create/edit gives users an accurate, one-click color fill plus a real swatch photo instead of a flat color chip.

## What Changes

- Add an optional "Search filamentcolors.xyz" lookup panel to the spool create and spool edit forms, alongside the existing SpoolmanDB lookup panel.
- On selecting a filamentcolors.xyz result: set the spool's color from the swatch's measured hex value, and download the swatch image, storing it server-side associated with the spool.
- Add a new server-side route to store and serve downloaded swatch images (new local storage on disk, no external dependency).
- Add an optional `swatch_image` reference field to the Spool model.
- Wherever a spool's color is currently rendered as a flat color chip (list table, detail view, `/colors` grid), render the stored swatch image instead when one is present; fall back to the existing chip when absent.
- Cache filamentcolors.xyz search results (not images) in localStorage, following the existing SpoolmanDB cache pattern, respecting the anonymous API's documented rate limit.

## Capabilities

### New Capabilities
- `filamentcolors-color-lookup`: fetches and caches filamentcolors.xyz swatch data, provides a search UI on spool create/edit, applies the selected swatch's hex color, and downloads/stores the swatch image server-side.

### Modified Capabilities
- `data-model`: Spool entity gains an optional `swatch_image` field referencing a stored image.
- `spool-management`: spool list and detail views render the stored swatch image in place of the flat color chip when present.
- `spool-color-swatch-view`: `/colors` grid cards render the stored swatch image as card fill when present.

## Impact

- `crates/spoolman-types/src/models.rs`: new optional field on `Spool`.
- `crates/spoolman-server/src/routes/`: new route(s) to accept/store/serve swatch images; new on-disk image directory next to the JSON store.
- `crates/spoolman-client/src/filamentcolors.rs` (new): fetch + localStorage cache, mirroring `spoolmandb.rs`.
- `crates/spoolman-client/src/components/filamentcolors_search.rs` (new): search UI, mirroring `spoolmandb_search.rs`.
- `crates/spoolman-client/src/pages/spool.rs`: wire the new search panel into create/edit forms; update color rendering in list/detail.
- `crates/spoolman-client/src/components/swatch_grid.rs`: use swatch image as card fill when present.
- No breaking changes: existing spools without a swatch image keep rendering the current flat color chip.
