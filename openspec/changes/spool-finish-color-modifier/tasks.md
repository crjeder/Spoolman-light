## 1. Shared Types

- [ ] 1.1 Add `SurfaceFinish` enum (`Matte | Standard | Gloss`) to `crates/spoolman-types/src/models.rs` with `Serialize`/`Deserialize` (lowercase string: `"matte"`, `"standard"`, `"gloss"`) and `Default` impl returning `Standard`
- [ ] 1.2 Add `finish: SurfaceFinish` field to `Spool` struct with `#[serde(default)]`
- [ ] 1.3 Add `finish: Option<SurfaceFinish>` to spool create/update request types in `requests.rs` (absent = keep existing / default to Standard)
- [ ] 1.4 Verify `cargo check -p spoolman-types` passes

## 2. Color Utilities

- [ ] 2.1 Add `rgba_to_hsv` / `hsv_to_rgba` conversion helpers in `crates/spoolman-client/src/utils/color.rs`
- [ ] 2.2 Add `apply_finish_modifier(color: &Rgba, finish: SurfaceFinish) -> Rgba` function applying S/V multipliers (clamp V to [0,1]; H unchanged)
- [ ] 2.3 Add unit tests: Standard is identity; Matte red has lower S and higher V than input; Gloss red has higher S and lower V; V clamp holds for bright colors

## 3. Server — Spool Store

- [ ] 3.1 Update spool create handler in `spoolman-server` to persist `finish` from request (default `Standard` when absent)
- [ ] 3.2 Update spool update handler to accept and persist `finish`
- [ ] 3.3 Verify `cargo check -p spoolman-server` passes

## 4. Client — Color Search

- [ ] 4.1 In `spool.rs` filter closure, replace `color_distance(c, &target, cda)` with `color_distance(&apply_finish_modifier(c, s.spool.finish), &target, cda)`
- [ ] 4.2 In `spool.rs` sort-by-color-distance, apply the same finish modifier before computing sort key distance

## 5. Client — Spool Form

- [ ] 5.1 Add finish `<select>` (Matte / Standard / Gloss) to the add-spool form in `spool.rs`, defaulting to Standard
- [ ] 5.2 Wire finish selector value into the create spool API call
- [ ] 5.3 Populate finish selector from existing spool data in the edit-spool form
- [ ] 5.4 Wire finish selector value into the update spool API call

## 6. Client — Spool Table Display

- [ ] 6.1 In the spool table color cell, render a small text badge for `Matte` and `Gloss` finish; render nothing for `Standard`

## 7. Verification

- [ ] 7.1 `cargo check -p spoolman-types -p spoolman-server` passes clean
- [ ] 7.2 Manual smoke: add a Matte red spool, search for a muted red — spool appears; Standard red spool at same stored color appears at wider threshold
