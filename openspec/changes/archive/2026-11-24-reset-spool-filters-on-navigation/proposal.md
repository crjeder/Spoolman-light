## Why

The Spools and Color tabs both render `SpoolList` and share the same `sessionStorage` keys (`filter.spools.*`) for material, location, color, and archived filters. A material filter set on the Color tab therefore leaks into the Spools tab (visible when grouped by location), and the Material column shows a selection the user never made there. Filters should not survive a switch between tabs / menu items.

## What Changes

- Selecting any sidebar menu item (Spools, Color, Filaments, Locations, Settings, Help) clears all persisted list filters (`filter.*` keys in `sessionStorage`) before the target page mounts, so every list opens at its default state.
- Filters still persist across reloads and across detail-page round trips (open a spool, go back) since those are not menu changes.

## Capabilities

### New Capabilities

### Modified Capabilities
- `list-filter-persistence`: filters persist for the tab session but are reset whenever the user changes tab / menu item.

## Impact

- `crates/spoolman-client/src/state.rs`: new `clear_session_filters()` helper.
- `crates/spoolman-client/src/components/layout.rs`: call it from the nav-links click handler.
- E2E: one test covering tab-switch reset.
