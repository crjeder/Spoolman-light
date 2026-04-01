## Why

The spool detail view displays color swatches for each color but shows no text representation of the color value. Users who want to record or reference an exact color (e.g., for ordering matching filament) have no way to read the hex code from the UI.

## What Changes

- Each color swatch in the spool detail view gains an adjacent hex label (e.g., `#ff6a00`) showing its RGB value.

## Capabilities

### New Capabilities

- `spool-color-hex-label`: Display the hex color code alongside each color swatch in the spool detail view.

### Modified Capabilities

*(none — no existing spec-level requirements change)*

## Impact

- `crates/spoolman-client/src/pages/spool.rs`: the Colors `<dd>` in the detail view template (around line 408) needs a hex label appended next to each swatch.
- No API changes, no new dependencies.
