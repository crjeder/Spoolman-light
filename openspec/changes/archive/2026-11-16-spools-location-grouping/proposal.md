## Why

With many spools spread over many storage locations, the flat spool table makes it hard to see what is stored where. Clicking the "Location" header currently only sorts; users want a compact per-location overview they can expand on demand.

## What Changes

- Sorting the spool table by Location (clicking the "Location" header) switches the table into a grouped mode: one collapsed row per location instead of one row per spool.
- A group row shows `+ <Location name>: <n> entries` (`- ` when expanded) in the Location cell. Clicking the row expands/collapses it, revealing that location's normal spool rows beneath it.
- The remaining cells of a group row are empty, except that every column with an active filter shows the matching filter value (search text under Filament, selected material(s) under Material, picked colour under Color).
- Spools without a location are collected in a `No location` group.
- Sorting by any other column returns to the flat table. The colour swatch view is unaffected.

## Capabilities

### New Capabilities
- `spool-location-grouping`: grouped (collapsed per-location) rendering of the spool table when sorted by Location, including count label, expand/collapse and filter-value display.

### Modified Capabilities
(none) - grouping is additive; existing spool-list requirements stay valid for the flat table.

## Impact

- `crates/spoolman-client/src/pages/spool.rs` (`SpoolList`): group computation, group rows, expanded-set signal, pagination over groups.
- `crates/spoolman-client/` stylesheet: group-row styling.
- No server, API or data model change. E2E page object/tests for the spool table gain grouping cases.
