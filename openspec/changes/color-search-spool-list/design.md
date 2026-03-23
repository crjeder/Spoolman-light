## Context

### Relevant data model

`Spool.colors: Vec<Rgba>` — 1 to 4 RGBA values per spool (sRGB, u8 channels). Alpha is always 255 for solid filament colors in practice. The `color_name: Option<String>` field is a human label; only `colors` is used for proximity matching.

### Relevant frontend code

- `crates/spoolman-client/src/pages/spool.rs` — `SpoolList` component. Client-side `filtered` closure currently matches on `display_name`, `color_name`, and `material`. All spools are loaded in a single resource; pagination/sorting are done over this in-memory vec.
- `crates/spoolman-client/src/utils/` — utility modules; no color helper exists yet.

### HTML color input mechanics

`<input type="color">` yields a `#rrggbb` hex string. In Leptos:
```rust
let color_pick: RwSignal<Option<String>> = create_rw_signal(None);
view! {
  <input type="color"
    on:input=move |ev| color_pick.set(Some(event_target_value(&ev)))
  />
}
```
A companion reset button sets `color_pick` back to `None` to disable the filter.

The threshold slider:
```rust
let threshold: RwSignal<u8> = create_rw_signal(60u8);  // default 60 out of 255
view! {
  <input type="range" min="0" max="255" step="1"
    prop:value=threshold
    on:input=move |ev| threshold.set(event_target_value(&ev).parse().unwrap_or(60))
  />
}
```

## Goals / Non-Goals

**Goals:**
- Color picker + threshold slider in the spool list page-actions bar.
- Client-side filtering: spool passes if any of its `colors` is within `threshold` Euclidean RGB distance of the picked color.
- Picking a color and adjusting the threshold immediately re-filters the list.
- A clear/reset control disables color filtering.

**Non-Goals:**
- Perceptual color distance (CIEDE2000, CIELAB) — Euclidean RGB is sufficient for a home app.
- Backend color-filter query param — not needed since all spools are in memory.
- Color filter combined with server-side pagination — the current design loads all spools then filters client-side; no change to that model.

## Decisions

### Decision: Euclidean RGB distance, alpha ignored

`distance = sqrt((r1-r2)² + (g1-g2)² + (b1-b2)²)`

Max value: `sqrt(3 × 255²) ≈ 441`. The threshold slider maps 0–255 to "tight–loose". Default 60 ≈ 14% of max — picks up same-hue variants without matching across hue families.

**Alternative considered**: CIELAB ΔE — more perceptually uniform but requires a non-trivial color-space conversion. Rejected — overkill for a home filament tracker.

### Decision: match if ANY spool color is within threshold

Multi-color spools (tie-dye, dual-color) should surface when the user searches for either color. Requiring ALL colors to match would hide valid results.

### Decision: color filter is additive with the text filter

Both filters apply simultaneously (AND logic). A spool must match the text filter AND (if a color is picked) the color proximity filter. This is consistent with how other filters compose in the list.

### Decision: `<input type="color">` with a separate "×" clear button

The native color picker has no "no selection" state — it always holds a color. A small "×" button next to it sets `color_pick` to `None`, re-enabling all spools. The picker is visually greyed / labelled "Filter by color" when inactive.

### Decision: utility module `utils/color.rs`

Keeps the distance logic testable and reusable. Two functions:
- `pub fn rgb_distance(a: &Rgba, b: &Rgba) -> f32`
- `pub fn hex_to_rgba(hex: &str) -> Option<Rgba>` — parses `#rrggbb`

The module is declared in `utils/mod.rs`.

## Risks / Trade-offs

- **[Risk] Large spool lists** — filtering is O(n × colors_per_spool) in-memory. For a home user with < 500 spools this is imperceptible; no concern.
- **[Risk] Color picker default colour** — when first activated the picker shows black (`#000000`), which would match only very dark spools. The UX is: color filter is OFF until the user explicitly picks a colour (i.e., `color_pick` starts as `None`).
- **[Risk] Threshold default too tight/loose** — default 60 is a reasonable starting point; the slider gives full control.
