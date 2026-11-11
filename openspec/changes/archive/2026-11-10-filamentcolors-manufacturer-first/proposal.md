## Why

The filamentcolors.xyz search on the New Spool page ranks results in API order, so a color-only query like `Black` returns the first 10 black swatches of any brand. The spool's filament already names its manufacturer; swatches from that manufacturer are the relevant ones and should surface first, with other brands only as a fallback.

## What Changes

- The filamentcolors.xyz search panel accepts the manufacturer of the currently selected filament.
- Matching results whose swatch manufacturer matches the filament's manufacturer are listed first; remaining matches (other brands / generics) fill the rest of the 10-result list.
- When no filament is selected or the filament has no manufacturer, ordering is unchanged.
- The token filter itself is unchanged: the ranking only reorders what already matches.
- The Edit Spool page passes its spool's filament manufacturer too, since it uses the same component (no extra cost).

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `filamentcolors-color-lookup`: the search panel requirement gains manufacturer-first ranking of results.

## Impact

- `crates/spoolman-client/src/components/filamentcolors_search.rs` - new `manufacturer` signal prop, ranking in `results`.
- `crates/spoolman-client/src/pages/spool.rs` - Spool Create derives the manufacturer from `filament_id` + `filaments_list`; Spool Edit from the loaded spool's filament.
- No server, API, data model, or dependency changes.
