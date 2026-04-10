## Why

The spool list currently supports filtering by filament but not by location, making it tedious to see only spools stored in a specific spot (shelf, box, drawer). Users with many spools spread across multiple locations need a quick way to narrow the list by where spools are physically kept.

## What Changes

- Add a `location_id` query parameter to `GET /api/v1/spool` for server-side filtering by location
- Add a location filter dropdown to the spool list page in the frontend, populated from the locations API
- When a location filter is active, only spools with that `location_id` are displayed

## Capabilities

### New Capabilities
<!-- None — this extends existing spool management and does not introduce a new top-level capability -->

### Modified Capabilities
- `spool-management`: Add `location_id` filter parameter to the list spools endpoint and frontend filter controls

## Impact

- `crates/spoolman-server/` — extend spool list handler to accept and apply `location_id` filter
- `crates/spoolman-client/` — add location filter dropdown to spool list page; fetch locations list on load
- No data model changes; no breaking API changes (additive query param)
