## Why

When the SpoolmanDB search returns no results for a user's query, the Spool Create form still shows an active filament dropdown, which is confusing: the user typed a name but must separately hunt for a filament. The search input text should become the spool name and the filament selector should be hidden when no DB match is found, allowing a clean "manual entry" path.

## What Changes

- When the SpoolmanDB search input is non-empty but matches zero entries, the filament dropdown on Spool Create is disabled (or hidden) and a message explains why.
- The content of the search input is propagated to the parent form and pre-filled into the spool name field (or used as the default name on submit).
- When the search input is cleared or a result is selected, normal behavior resumes.

## Capabilities

### New Capabilities

- `spool-create-unfound-mode`: Spool Create form enters a "manual" mode when SpoolmanDB search yields no results -- filament selector disabled, search text used as spool name.

### Modified Capabilities

- `spoolmandb-lookup`: The search component must expose its current query and a "no results" signal to the parent so the parent can react. This is a spec-level behavior change (the component contract gains an output signal).

## Impact

- `crates/spoolman-client/src/components/spoolmandb_search.rs` -- expose query + no-results state via additional props/callbacks.
- `crates/spoolman-client/src/pages/spool.rs` -- react to no-results state: disable filament dropdown, pre-fill name field.
- No API, backend, or data-model changes.
