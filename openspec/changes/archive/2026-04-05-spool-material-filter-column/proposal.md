## Why

The spool list table shows filament name but not material type, making it hard to quickly scan for spools of a specific material (e.g., PLA vs PETG). Adding a material column with a clickable header filter lets users narrow the list without typing.

## What Changes

- Add a **Material** column to the spool list table displaying the filament's `material` abbreviation (e.g., "PLA", "PETG", or blank if unset)
- The **Material** column header is interactive: clicking it opens a dropdown filter listing all distinct materials present in the current spool list
- Selecting a material from the dropdown filters the spool list to show only spools whose filament matches that material; selecting "All" clears the filter
- An active-filter indicator (■) is shown on the column header when a material filter is active

## Capabilities

### New Capabilities

- `spool-material-column`: Material column in the spool list table with value display and active-filter indicator
- `spool-material-filter`: Dropdown filter activated by clicking the Material column header, filtering spools by filament material type

### Modified Capabilities

<!-- No existing spec-level requirements change -->

## Impact

- `crates/spoolman-client/src/pages/spool.rs` — spool list table rendering and filter state
- No backend changes needed; material is already available on the filament embedded in each spool response
