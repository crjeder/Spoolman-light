## Why

The current landing page (`/`) renders a generic `HomePage` component that adds an unnecessary navigation step before users can view their spools. Since spool tracking is the primary use case, users should land directly on the Spool list every time they open the app.

## What Changes

- The `"/"` route renders `SpoolList` instead of `HomePage`
- `HomePage` component and its module are removed
- The nav bar's active-link logic is updated so `"/"` and `"/spools"` both highlight the Spools link

## Capabilities

### New Capabilities

_(none — this is a routing change, not a new functional capability)_

### Modified Capabilities

- `spool-management`: The spool list is now the entry point of the application (routing/UX change within the existing capability)

## Impact

- `crates/spoolman-client/src/app.rs` — route table change (`"/"` → `SpoolList`, remove `HomePage` import)
- `crates/spoolman-client/src/pages/home.rs` — file deleted
- `crates/spoolman-client/src/pages/mod.rs` — `pub mod home` removed
- No API changes, no backend changes, no data-model changes
