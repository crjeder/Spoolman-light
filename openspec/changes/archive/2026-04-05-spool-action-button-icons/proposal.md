## Why

The spool list action column uses plain text labels ("Edit", "Delete") that take up unnecessary horizontal space and look inconsistent with modern table UIs. There is also no quick way to navigate from the list to a spool's detail page — users have to know to click the spool name or manually navigate.

## What Changes

- Replace the "Edit" text link in the spool list actions column with a pencil icon button.
- Replace the "Delete" text button (and its "Sure?"/"Cancel" confirmation state) with a trash icon button (confirmation state retains icon or minimal text).
- Add a new "View" icon button (eye icon) in the actions column that links to `/spools/:id` (the existing `SpoolShow` page).

## Capabilities

### New Capabilities

None — no new backend capabilities are introduced.

### Modified Capabilities

- `spool-management`: The spool list UI requirement changes — the actions column gains a View button and switches from text labels to icons for Edit and Delete.

## Impact

- `crates/spoolman-client/src/pages/spool.rs` — `SpoolList` component, actions cell in the row loop
- CSS styles for icon buttons (size, spacing, hover states) — likely in the existing stylesheet
- `openspec/specs/spool-management/spec.md` — delta spec to update the "Spool list UI" requirement
