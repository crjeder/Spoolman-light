## Why

Density is shown with up to 3 fractional digits and trailing zeros suppressed (`1.2 g/cm³`, `1.24 g/cm³`, `1.245 g/cm³`), so the filaments table column looks ragged. A fixed `x.xx` format is easier to scan.

## What Changes

- Density is displayed rounded to 2 decimal places, always with 2 digits (`1.20`, `1.24`, `1.25`), in the browser locale's decimal separator.
- `format_density` is shared by the filaments table and the filament detail view, so both change together.
- Form inputs (`step="0.001"`) and stored values are unchanged; only display rounds.

## Capabilities

### New Capabilities

### Modified Capabilities
- `intl-formatting`: density display requirement changes from max 3 fraction digits (trailing zeros suppressed) to exactly 2 fraction digits.

## Impact

- `crates/spoolman-client/src/format.rs` (`format_density`)
- E2E tests asserting density text, if any (`tests/e2e/tests/`)
- No API, data model, or dependency changes.
