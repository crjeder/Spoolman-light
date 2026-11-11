## Why

Editing a spool through the edit form does not refresh "last used" when the spool already has a `last_used` date: the form always resubmits the old date, and the server only stamps `last_used` when none is supplied. Users updating the current weight expect the last-used date to become today.

## What Changes

- Saving the spool edit form sets `last_used` to today's date unless the user explicitly changed the `last_used` field.
- The form's `last_used` input still lets the user override the date manually.
- No API or data-model change; `PATCH /api/v1/spool/<id>` keeps its existing behaviour (stamps `last_used` when `current_weight` is sent without `last_used`).

## Capabilities

### New Capabilities
<!-- none -->

### Modified Capabilities
- `spool-management`: the "Edit spool" requirement gains a scenario for the edit form refreshing `last_used` to today when the date field is left untouched.

## Impact

- `crates/spoolman-client/src/pages/spool.rs` (edit form submit handler, and tracking of the loaded `last_used` value).
- `openspec/specs/spool-management/spec.md` (delta only).
- E2E: one Playwright test for the edit flow.
