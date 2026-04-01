## 1. Client-Side Sort Logic

- [x] 1.1 In `sorted()` in `spool.rs`, capture `color_pick` and `color_level` signals (already in scope) and check if level is active and hex is valid
- [x] 1.2 When color is active, compute `min_delta` per spool using `color_distance` over all `spool.colors`, falling back to `f32::MAX` for empty color lists
- [x] 1.3 Sort items by `min_delta` ascending when delta-sort mode is active, bypassing the existing `sort_field` / `sort_asc` branch
- [x] 1.4 Ensure the existing column-sort branch is used unchanged when level is "off" or hex parse fails

## 2. Verification

- [ ] 2.1 Manually verify: set level to Fine with a red color — red/orange spools appear first, blue/green spools at the bottom
- [ ] 2.2 Manually verify: switch level back to Off — default sort order (ID ascending) is restored
- [ ] 2.3 Manually verify: spool with multiple colors is ranked by its closest color, not its first color
- [x] 2.4 Run `cargo check -p spoolman-client --target wasm32-unknown-unknown` (or equivalent) — no compile errors
- [x] 2.5 Run `cargo clippy -p spoolman-client` — no new warnings

## 3. Changelog & Docs

- [x] 3.1 Add entry to `CHANGELOG.md` under the current unreleased version: "Sort spools by closest color match when color search is active"
- [x] 3.2 Mark the TODO.md item `[ ] sort spools according to delta when a color is selected` as done
