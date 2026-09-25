## Why

The API already supports archiving a spool (`PATCH archived=true`), but the UI offers no way to do it; users must edit raw data. Archiving a spool that still holds filament is usually a mistake, so the UI should warn.

## What Changes

- Add an Archive icon button to each spool list row and to the spool detail page header.
- When the spool is archived, the same button becomes Unarchive (restore).
- If the spool is not empty (remaining filament > 0; `current_weight > 0` when net weight is unknown), archiving requires an explicit confirmation showing a warning; otherwise it archives immediately.
- No backend or type changes.

## Capabilities

### New Capabilities

### Modified Capabilities
- `spool-management`: UI archive/unarchive action with a non-empty-spool warning.

## Impact

- `crates/spoolman-client/src/pages/spool.rs` (list row actions, `SpoolShow` header).
- Uses existing `api::update_spool` with `UpdateSpool { archived: Some(_), .. }`.
- E2E: add a Playwright test for archive flow.
