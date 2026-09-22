## Why

In the spool detail view, the filament name is displayed as plain text with no way to navigate directly to the corresponding filament record. Users must manually navigate to the Filaments page and search for it, which is friction that should not exist given the filament page already has a dedicated route (`/filaments/:id`).

## What Changes

- The filament name in the spool detail view (`/spools/:id`) becomes a hyperlink to `/filaments/<filament_id>`.
- The filament name in the spool list table also becomes a hyperlink to the same destination, for consistency.

## Capabilities

### New Capabilities

- `spool-filament-link`: Filament name in spool detail and spool list is rendered as a navigable link to the filament detail page.

### Modified Capabilities

<!-- None — no existing spec-level requirements change. -->

## Impact

- `crates/spoolman-client/src/pages/spool.rs`: Two render sites updated — detail view (`SpoolShow`) and list table row.
- No API, backend, or data model changes required.
- No new dependencies.
