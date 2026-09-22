## Why

Spools without a location are harder to find physically. Enforcing a location at save time prevents accidentally creating or editing spools that have no storage location assigned.

## What Changes

- The spool create form SHALL require a location to be selected before submission is allowed.
- The spool edit form SHALL require a location to be selected before submission is allowed.
- Both forms SHALL display a validation error or disable the submit button when no location is selected.

## Capabilities

### New Capabilities

*(none)*

### Modified Capabilities

- `spool-management`: The create and edit spool UI forms gain a required-field constraint on `location_id`.

## Impact

- `crates/spoolman-client/` — spool form components (new spool form, edit spool form)
- No API or data-model changes needed — location_id is already an optional field on the server; this is frontend-only enforcement
- E2E tests that create or edit spools without a location will need updating
