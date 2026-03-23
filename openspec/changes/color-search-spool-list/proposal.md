## Why

The spool list already shows a color swatch for each spool, but there is no way to filter by color. A user who wants to find "all my blue spools" must scroll visually or remember color names. The `colors: Vec<Rgba>` field (1–4 values per spool) is already stored and rendered — the data is there; the UI just does not expose it as a filter criterion.

## What Changes

- **`crates/spoolman-client/src/pages/spool.rs`** — add a color picker (`<input type="color">`) and a proximity threshold slider to the spool list page-actions bar. When a color is selected, the existing client-side filter closure is extended to also check Euclidean RGB distance: a spool passes if any of its `colors` is within the threshold. Clearing the picker (resetting to no selection) disables the color filter and shows all spools again.
- **`crates/spoolman-client/src/utils/color.rs`** (new) — small utility module with `rgb_distance(a: &Rgba, b: &Rgba) -> f32` (Euclidean in RGB space, alpha ignored) and `hex_to_rgba(hex: &str) -> Option<Rgba>` (parses `#rrggbb` from the color input).
- **No backend changes** — all spools are already loaded client-side for the existing text filter; color proximity filtering adds no extra requests.

## Capabilities

### New Capabilities

- `spool-color-filter`: Spool list can be filtered by RGBA proximity — a color picker + threshold slider let the user find spools whose color(s) are close to a chosen color.

### Modified Capabilities

- `spool-list`: Page-actions bar gains a color picker and threshold slider alongside the existing text search.

## Impact

- All changes are confined to `crates/spoolman-client/` (frontend only). No backend, API, or storage changes.
- No new API endpoints.
- No breaking changes.
