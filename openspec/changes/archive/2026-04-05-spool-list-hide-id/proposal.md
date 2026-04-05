## Why

The ID column in the spools list view exposes an internal implementation detail (random u32) that has no meaning to the user. It clutters the table and takes up horizontal space better used for filament or status information.

## What Changes

- Remove the "ID" column header and its corresponding data cell from the spools list table
- The spool ID remains accessible via the detail page URL and is still used internally for routing

## Capabilities

### New Capabilities
<!-- none -->

### Modified Capabilities
- `spool-management`: The spool list table no longer displays an ID column.

## Impact

- `crates/spoolman-client/src/pages/spool.rs`: remove `ColHeader` for "ID" and the `<td>` cell rendering `id`
- No API changes, no data model changes, no backend changes
