## 1. Color utility module

- [ ] 1.1 Create `crates/spoolman-client/src/utils/color.rs` with `pub fn rgb_distance(a: &Rgba, b: &Rgba) -> f32` (Euclidean, alpha ignored) and `pub fn hex_to_rgba(hex: &str) -> Option<Rgba>` (parses `#rrggbb`)
- [ ] 1.2 Declare `pub mod color;` in `crates/spoolman-client/src/utils/mod.rs`

## 2. Color picker and threshold slider in SpoolList

- [ ] 2.1 In `crates/spoolman-client/src/pages/spool.rs`, add signals: `color_pick: RwSignal<Option<String>>` (default `None`) and `threshold: RwSignal<u8>` (default `60`)
- [ ] 2.2 Add an `<input type="color">` to the page-actions bar wired to `color_pick`; add a "×" clear button that sets `color_pick` to `None`
- [ ] 2.3 Add an `<input type="range" min="0" max="255">` slider wired to `threshold`; show it only when `color_pick` is `Some`
- [ ] 2.4 Extend the `filtered` closure: when `color_pick` is `Some(hex)`, parse it with `hex_to_rgba`, then keep only spools where any `spool.colors` entry has `rgb_distance` ≤ `threshold` value

## 3. Update CHANGELOG and TODO

- [ ] 3.1 Add entry to `CHANGELOG.md` under `[Unreleased] → Added`: "Color proximity filter on spool list: a color picker and threshold slider let users find spools by RGBA similarity (client-side Euclidean RGB distance)."
- [ ] 3.2 Remove the "Color search on spool list" item from `TODO.md`
