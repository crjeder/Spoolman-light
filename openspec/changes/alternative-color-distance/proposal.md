## Why

The current color search always uses CIEDE2000 (ΔE\*00), which is accurate but perceptually non-uniform at high chroma — particularly for saturated filament colors like vivid reds, greens, and blues. OKLab (2020) and DIN99d offer better perceptual uniformity and can yield more intuitive search results for users comparing bright filament spools. Making the algorithm user-configurable lets power users pick the metric that matches their perception.

## What Changes

- Add an `oklab` dependency to `spoolman-client` providing OKLab Euclidean distance (ΔE_ok).
- Refactor `crates/spoolman-client/src/utils/color.rs` to support three interchangeable distance algorithms: CIEDE2000, OKLab, and DIN99d.
- Add a `color_distance_algorithm` application setting (persisted key-value, values: `ciede2000` | `oklab` | `din99d`; default: `ciede2000`).
- Expose the setting on the Settings page as a labeled `<select>` element.
- Propagate the selected algorithm via Leptos context so the spool list color filter uses the live value without a page reload.

## Capabilities

### New Capabilities

- `color-distance-algorithm`: User-configurable color distance metric (CIEDE2000, OKLab, DIN99d) stored as an application setting and used by the color search filter.

### Modified Capabilities

- `ciede2000-color-distance`: The requirement changes from "the system SHALL use CIEDE2000" to "the system SHALL use the configured algorithm (defaulting to CIEDE2000)". The sRGB→Lab conversion path and threshold scale are also affected.

## Impact

- **`crates/spoolman-client/Cargo.toml`**: add `oklab` crate (pure Rust, no unsafe, no C deps — safe on WASM).
- **`crates/spoolman-client/src/utils/color.rs`**: refactor `color_distance()` to accept or dispatch on an algorithm enum.
- **`crates/spoolman-client/src/state.rs`**: add `ColorDistanceAlgorithm` signal to app context.
- **`crates/spoolman-client/src/app.rs`**: read `color_distance_algorithm` setting on load; provide context.
- **`crates/spoolman-client/src/pages/settings.rs`**: add algorithm selector to Settings form.
- **`crates/spoolman-client/src/pages/spool.rs`**: consume algorithm from context for color filter.
- **No server changes**: setting persistence reuses the existing `PUT /api/v1/settings/{key}` endpoint.
- **No `spoolman-types` changes**: no new API surface or data model.
