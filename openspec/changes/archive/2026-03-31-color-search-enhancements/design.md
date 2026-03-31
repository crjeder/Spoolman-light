## Context

Color search in `spool.rs` uses a `threshold: RwSignal<u8>` (0–255) and an `<input type="range">` to let users tune the CIEDE2000 match distance. In practice, users have no intuition for what "60 delta-E units" means, making the slider unusable. Additionally, a separate color-picker button above the table duplicates the Color column header's click-to-focus behaviour, and there is no visual indication when a color filter is active.

Current signals:
- `color_pick: RwSignal<Option<String>>` — selected hex color
- `threshold: RwSignal<u8>` — raw CIEDE2000 threshold (0–255)

## Goals / Non-Goals

**Goals:**
- Replace `threshold` signal with a `color_level` signal holding one of four named levels.
- Map each level to a fixed CIEDE2000 threshold constant.
- Remove the toolbar color-picker button element from the view.
- Render ■ (U+25A0) in the Color `<th>` whenever `color_level` is not "off".
- Keep all changes inside `spool.rs`; no backend or data-model changes.

**Non-Goals:**
- Making threshold user-configurable beyond the four levels.
- Persisting the selected level across sessions (the current color filter already resets on reload).
- Changing how `color_distance` or `hex_to_rgba` work.

## Decisions

### Named levels instead of freeform slider

**Decision:** Use a `<select>` with four options: Off / Fine / Medium / Coarse, mapped to CIEDE2000 thresholds approximately:

| Level  | Threshold |
|--------|-----------|
| Off    | N/A (filter disabled) |
| Fine   | 10.0      |
| Medium | 30.0      |
| Coarse | 60.0      |

**Rationale:** CIEDE2000 has a perceptual "just noticeable difference" of ~1–2 units. 10 captures near-identical colours; 30 covers the same hue family; 60 is broad. These three named levels cover the useful range without exposing the raw number to users.

**Alternative considered:** Keep slider, add labelled tick marks — rejected because the DOM range input offers no standard way to show semantic labels, and the mental model remains opaque.

### Selector drives both filter and header indicator

When `color_level` is not "off", the filter is active. The same signal is read by the `<th>` render to decide whether to append ■. This avoids a separate boolean.

When the user selects "Off", `color_pick` is reset to `None` and the colour picker input is cleared.

### Remove toolbar button, keep header affordance

The toolbar `<span class="color-filter">` currently holds the `<input type="color">`, the clear `×` button, and the slider. After this change, the `<input type="color">` stays in that span (still activated by clicking the column header), but the toolbar button (if any separate one exists) is removed. The `<select>` replaces the range input in the same span.

## Risks / Trade-offs

- [Threshold mapping is opaque] → Chosen values (10/30/60) are not exposed to users; if they want finer control they have no escape hatch. Acceptable given the scope of this change; a future enhancement could add a "Custom" level.
- [Colour picker still lives in toolbar] → The `<input type="color">` remains hidden in the DOM but activated via `.focus()` from the header click. This is unchanged behaviour; no risk.
