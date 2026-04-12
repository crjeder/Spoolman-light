## Why

DIN99d is a perceptually uniform color space specifically designed to provide consistent ΔE values matching human color perception, making it a better default than CIEDE2000 for spool color matching. The current defaults for DIN99d (10/20/35) are inherited from CIEDE2000 and do not reflect good sensitivity calibration for filament matching — values of 13/19/25 provide better discrimination between "same", "close", and "ballpark" matches in practice.

## What Changes

- The default color distance algorithm changes from `Ciide2000` to `Din99d`
- The default DIN99d thresholds change from 10.0/20.0/35.0 to 13.0/19.0/25.0 (same/close/ballpark)
- The settings page algorithm selector label changes from "CIIDE2000 (default)" to "CIIDE2000" and "DIN99d" gains "(default)"

## Capabilities

### New Capabilities

_(none)_

### Modified Capabilities

- `color-distance-algorithm`: Default algorithm changes from CIEDE2000 to DIN99d
- `color-threshold-settings`: Default DIN99d thresholds change to 13/19/25

## Impact

- `crates/spoolman-client/src/utils/color.rs` — `default_threshold_for()` DIN99d values updated
- `crates/spoolman-client/src/state.rs` — default algorithm signal changed to `Din99d`
- `crates/spoolman-client/src/pages/settings.rs` — selector option labels updated
- `crates/spoolman-client/src/app.rs` — settings-load fallback algorithm updated
- Existing users with no persisted settings will now see DIN99d selected with 13/19/25 thresholds
