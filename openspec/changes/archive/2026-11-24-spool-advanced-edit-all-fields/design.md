## Context

`SpoolEdit` (`crates/spoolman-client/src/pages/spool.rs`) splits the form into quick fields (current weight, location, last used, comment) and a collapsed "Advanced" `<details>`. Advanced shows a read-only ID, net weight, price, colors, color name, finish, first used, and initial weight only when it is 0. The server's `update_spool` mirrors that: `initial_weight` is applied only when the stored value is 0, and `UpdateSpool` has no `filament_id` or `registered`.

Spool fields not editable today: `filament_id`, `registered`, `archived`, `initial_weight` (when non-zero). `swatch_image` is set indirectly via the filamentcolors.xyz lookup already present in Advanced.

## Goals / Non-Goals

**Goals:**
- Every stored spool field except `id` can be changed from the edit form.
- Server accepts and validates the newly editable fields.

**Non-Goals:**
- Clearing optional values (net weight, price, first/last used) back to "unset". `UpdateSpool` uses `None` = unchanged; tri-state semantics are a separate change.
- Free-form editing of `swatch_image` (stays driven by the colour lookup).
- Changing the quick-edit field set or moving fields out of Advanced.

## Decisions

- **Extend `UpdateSpool` with `filament_id: Option<u32>` and `registered: Option<DateTime<Utc>>`.** Same `None` = unchanged convention as the other fields; old clients keep working. Alternative (separate "relink filament" endpoint) adds an endpoint for one field.
- **Validate `filament_id` exists before mutating**, next to the existing `location_id` check, returning `StoreError::Validation`. Prevents dangling references that would make the response's filament lookup fail after the write.
- **Always apply `initial_weight`.** The 0-guard existed to stop the edit form overwriting it; now the form edits it deliberately. An untouched field resends the loaded value, so it is a no-op write.
- **Initial weight placement depends on the loaded value.** When the loaded initial weight is 0 it is shown in the quick section above current weight (it still needs entering, so it belongs up front) and mirrors into current weight as before. When it is non-zero it moves into Advanced as a plain correction field, decoupled from current weight, so the quick section stays clean and correcting the registration reading does not overwrite the latest scale reading.
- **Registered input as `datetime-local`**, interpreted as UTC and sent only when changed from the loaded value, so stored seconds are not truncated on every save. A `date` input would drop the time of day.
- **Archived as a checkbox in Advanced, sent as `archived: Some(value)`.** The server already clears `location_id` when archiving. The client "Location is required" check and the Save button's disabled state become `location_id.is_none() && !archived`.
- **Filament select** reuses `api::list_filaments(None)` and `display_name()` as in `SpoolCreate`, pre-selected with the spool's current `filament_id`.

## Risks / Trade-offs

- [Changing `filament_id` may leave net weight / colors inconsistent with the new filament] -> Accepted; these are per-spool values editable in the same section.
- [API clients relying on `initial_weight` being ignored when non-zero] -> Only the bundled client sends it; flagged BREAKING in the proposal and CHANGELOG.
- [Unarchiving leaves the spool without a location] -> The location-required check applies again once archived is unchecked, so a location must be picked before saving.
