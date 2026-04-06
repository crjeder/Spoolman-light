## Why

Color search compares stored spool colors directly against the user's target, but surface finish (matte vs. gloss) shifts perceived color — matte desaturates and lightens, gloss saturates and slightly darkens. Without accounting for finish, searches miss spools whose stored color is accurate but whose printed appearance diverges from the target.

## What Changes

- Add `SurfaceFinish` enum (`Matte | Standard | Gloss`) to `spoolman-types`
- Add `finish: SurfaceFinish` field to `Spool` model (serde default: `Standard`)
- Apply HSV modifiers to spool color before Lab-space comparison in color search:
  - Matte: S×0.85, V×1.10
  - Standard: identity (no-op)
  - Gloss: S×1.15, V×0.95
- Add finish selector to spool add/edit form
- Display finish badge in spool table and detail view
- API: `finish` field added to spool create/update request and response bodies

## Capabilities

### New Capabilities

- `spool-finish`: Surface finish classification on spools and its application to color search

### Modified Capabilities

- `color-search`: Color search now pre-processes spool colors through finish modifiers before distance comparison

## Impact

- `crates/spoolman-types/src/models.rs` — new `SurfaceFinish` enum, `Spool.finish` field
- `crates/spoolman-types/src/requests.rs` — `finish` on create/update request types
- `crates/spoolman-types/src/responses.rs` — `finish` on spool response type
- `crates/spoolman-client/src/utils/color.rs` — HSV modifier + finish-aware distance function
- `crates/spoolman-client/src/pages/spool.rs` — color search filter, form, table display
- No server-side logic change — finish is stored and returned, comparison is client-only
- No migration needed — `serde(default)` gives existing spools `Standard` on first read
