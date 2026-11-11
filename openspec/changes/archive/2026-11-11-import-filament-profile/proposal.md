## Why

Manufacturers publish slicer filament profiles (e.g. Polymaker for OrcaSlicer / Bambu Studio). These already contain manufacturer, material, density, diameter and temperatures, so users should not retype them when creating a filament. SpoolmanDB only covers filaments it happens to list; a profile file is available for almost any commercial filament.

## What Changes

- Add an "Import profile" button to the New Filament form that opens a file picker for a `.json` slicer profile.
- Parse the OrcaSlicer filament profile format client-side and pre-fill the form fields (manufacturer, material, modifier, diameter, density, print temp, bed temp). The user can review and edit before clicking Create.
- Show an inline error if the file is not valid JSON or does not look like an Orca filament profile.
- No backend, API or data-model change; nothing is persisted until the user submits the form.

## Capabilities

### New Capabilities

### Modified Capabilities
- `filament-management`: adds a requirement for importing a slicer (OrcaSlicer) filament profile into the create-filament form.

## Impact

- `crates/spoolman-client/src/pages/filament.rs` (`FilamentCreate`): button, hidden file input, handler.
- New `crates/spoolman-client/src/slicer_profile.rs`: pure parser with unit tests.
- No new dependencies (`gloo-file` and `serde_json` already used by the client).
- Specs: `openspec/specs/filament-management`.
