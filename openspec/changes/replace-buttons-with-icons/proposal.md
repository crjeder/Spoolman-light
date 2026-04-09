## Why

Action buttons across the filament and location pages still use plain text labels ("Edit", "Delete", "Sure?", "Cancel", "← Prev", "Next →"), while the spool table was already converted to icon buttons in a previous pass. Completing this conversion makes the UI consistent and saves horizontal space in compact table rows.

## What Changes

- Replace text action buttons in the filament list with icon buttons (`✏`, `🗑`, confirm `✓`/`✗`)
- Replace text action buttons in the location list with icon buttons (`✏`, `🗑`, `💾`, confirm `✓`/`✗`)
- Replace "← Prev" / "Next →" pagination buttons with arrow icons (`‹` / `›` or Unicode arrows)
- Replace "Edit" / "Delete" text in spool detail view with icon buttons matching the table style
- Form submit and cancel buttons ("Create", "Save", "Cancel", "Add") stay as text — these are primary form actions, not row-level actions, and text labels are appropriate there

## Capabilities

### New Capabilities

- `icon-action-buttons`: Consistent icon-only action buttons for row-level operations (view, edit, delete, confirm, cancel, save) across all list pages and detail views.

### Modified Capabilities

- `filament-management`: Row action buttons change from text to icons.
- `spool-management`: Detail view action buttons change from text to icons (list already done).
- `location-management`: Row action buttons change from text to icons.

## Impact

- `crates/spoolman-client/src/pages/filament.rs` — row action buttons
- `crates/spoolman-client/src/pages/location.rs` — row action buttons
- `crates/spoolman-client/src/pages/spool.rs` — detail view action buttons
- `crates/spoolman-client/src/components/pagination.rs` — prev/next buttons
- CSS: existing `.btn-icon` class (already defined for spool table) reused; no new styles expected
