## Why

The color search UI has three rough edges: the raw 0–255 threshold slider is hard to reason about, a redundant color-picker button clutters the toolbar above the table, and there is no visual indicator showing that a color filter is active. These are quick, self-contained UX improvements that complete the color-search feature.

## What Changes

- Replace the `<input type="range">` sensitivity slider with a `<select>` offering four labelled levels: **Off** (filter disabled, default), **Fine**, **Medium**, and **Coarse** — each mapped to a fixed CIEDE2000 threshold.
- Remove the standalone color-picker button that currently appears above the table (the Color column header already activates the picker).
- Display a filled square (U+25A0 ■) in the Color column header when color search is set to any level other than "Off", giving users a clear active-filter indicator.

## Capabilities

### New Capabilities

- `color-search-selector`: Replaces the numeric range slider with a named selector (Off / Fine / Medium / Coarse); drives the CIEDE2000 threshold used when filtering spools by color.

### Modified Capabilities

- `color-column-head-activates-filter`: The column header now also shows an active-filter indicator (■) when color search is not "Off", and the toolbar color-picker button is removed.

## Impact

- **`crates/spoolman-client/src/pages/spool.rs`** — replace `threshold` signal + range input with a `color_level` signal + `<select>`; remove the toolbar color-picker button element; add ■ indicator to the Color `<th>` when `color_level != "off"`.
- **`crates/spoolman-client/src/utils/color.rs`** — no logic changes needed; threshold values are defined in `spool.rs` as constants.
- No API, data-model, or backend changes required.
