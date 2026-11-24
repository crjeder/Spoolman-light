## Why

The spool edit form's "Advanced" section still leaves several stored fields uneditable: the filament link, the registered date, the archived flag, and the initial weight (editable only while it is 0, and the server ignores it otherwise). Correcting a data-entry mistake (wrong filament picked, wrong scale reading at registration) currently requires deleting and recreating the spool, losing its history.

## What Changes

- The "Advanced" section of the spool edit form exposes every stored spool field except `id`, which stays read-only.
- New editable fields in the form: filament (select), registered (date/time), archived (checkbox).
- Initial weight is always editable: above current weight when it is 0 (as now, it still needs entering), inside Advanced when it already has a value, to keep the quick form clean.
- `PATCH /api/v1/spool/<id>` (`UpdateSpool`) accepts `filament_id` and `registered`; `filament_id` is validated to exist.
- **BREAKING** (API behaviour): `initial_weight` in `UpdateSpool` is always applied; previously it was silently ignored unless the stored value was 0.
- The client-side "Location is required" check is skipped when the spool is marked archived (archiving clears the location).

## Capabilities

### New Capabilities

### Modified Capabilities
- `spool-management`: "Edit spool" requirement widens the set of mutable fields to all fields except `id`, and the edit form's Advanced section exposes all of them.

## Impact

- `crates/spoolman-types/src/requests.rs` - `UpdateSpool` gains `filament_id`, `registered`.
- `crates/spoolman-server/src/store.rs` - `update_spool` applies the new fields, validates `filament_id`, drops the `initial_weight == 0` guard.
- `crates/spoolman-client/src/pages/spool.rs` - `SpoolEdit` Advanced section adds filament select, registered input, archived checkbox, unconditional initial weight input.
- Server unit tests / API integration tests for spool update; E2E edit test if one covers Advanced.
