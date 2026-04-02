## Context

Color search level thresholds are returned by `threshold_for(level, algo)` in `spoolman-client/src/utils/color.rs` as compile-time constants. The Settings page persists other user preferences (currency symbol, default diameter, color distance algorithm) as individual key/value pairs via `PUT /api/v1/settings/{key}`. The `App` component reads settings at startup and injects them into Leptos context as reactive signals (e.g. `ColorDistanceAlgorithm`).

Nine threshold values exist: three levels (Same, Close, Ballpark) × three algorithms (CIEDE2000, OKLab, DIN99d). Current hardcoded defaults:

| Level    | CIEDE2000 | OKLab | DIN99d |
|----------|-----------|-------|--------|
| Same     | 10.0      | 0.10  | 10.0   |
| Close    | 20.0      | 0.20  | 20.0   |
| Ballpark | 35.0      | 0.35  | 35.0   |

## Goals / Non-Goals

**Goals:**
- User can override any of the nine threshold values from the Settings page.
- Overrides persist across reloads via existing settings key/value store.
- `threshold_for()` replaced by a reactive context lookup; the filter reacts immediately when thresholds change.
- Defaults match current hardcoded values, so no behaviour change on first load.

**Non-Goals:**
- Per-spool or per-filament threshold overrides.
- Exposing thresholds via the REST API to external clients.
- Input validation beyond basic range checks (values must be > 0).

## Decisions

### D1 — Nine individual settings keys, not one structured key

**Chosen:** `color_threshold_{algo}_{level}` (e.g. `color_threshold_ciede2000_same`, `color_threshold_oklab_ballpark`).

**Rationale:** Consistent with existing pattern (one `put_setting` call per key). Avoids introducing JSON-within-JSON in the flat key/value store. Absent keys fall back to hardcoded defaults without any migration step.

**Alternative considered:** A single `color_thresholds` key containing a JSON object. Rejected because it requires JSON serialisation/deserialisation in the client and a custom server-side handler — more complexity for no gain.

### D2 — Show only the active algorithm's thresholds on the Settings page

**Chosen:** The threshold editor section displays three `<input type="number">` fields (Same, Close, Ballpark) for the currently selected algorithm only, updating dynamically when the algorithm selector changes.

**Rationale:** Showing all 9 fields simultaneously is confusing — the inactive algorithms' thresholds have no visible effect until the algorithm changes. Grouping by active algorithm keeps the UI compact and contextual.

**Alternative considered:** A tabbed UI showing all three algorithms. Overkill for a settings page; adds complexity.

### D3 — Reactive `ColorThresholds` context struct, parallel to `ColorDistanceAlgorithm`

**Chosen:** Add a `ColorThresholds` struct to `state.rs` holding nine `RwSignal<f32>` values, provided via Leptos context from `App`. `threshold_for()` is replaced by a `ColorThresholds::get(level)` method that reads the active algorithm's signal.

**Rationale:** Matches the existing `ColorDistanceAlgorithm` pattern exactly. The spool list reactive computation already subscribes to `cda.0.get()` — adding `thresholds.get(level)` is a natural extension with no architectural change.

## Risks / Trade-offs

- **9 settings keys on first save** — Saving settings will issue up to 9 additional `PUT` requests (one per threshold) in addition to the existing ones. These are fire-and-forget; a single failure leaves one threshold at its hardcoded default. Mitigation: batch all puts into a single `join_all` and report any errors in the save status message.
- **OKLab scale mismatch** — OKLab thresholds are in [0, 1] while CIEDE2000/DIN99d are in [0, 100]. A user could accidentally enter 10.0 for OKLab (equivalent to Ballpark × 28). Mitigation: display the current default value as placeholder text and add a brief per-algorithm hint (e.g. "typical range 0.05–0.50").
- **No threshold ordering enforcement** — Same should be ≤ Close ≤ Ballpark, but this is not validated. Inverted values produce confusing filter behaviour. Mitigation: document expected ordering in the UI label; no hard enforcement for now.

## Migration Plan

No data migration needed. Absent keys are transparently resolved to hardcoded defaults by the context struct. The change is purely additive — existing installations behave identically until the user explicitly saves new threshold values.
