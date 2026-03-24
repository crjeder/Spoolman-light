## 1. Add Dependency

- [x] 1.1 Add `deltae = "0.3.2"` to `crates/spoolman-client/Cargo.toml`

## 2. Color Distance Implementation

- [x] 2.1 Add sRGB-to-linear conversion helper in `crates/spoolman-client/src/utils/color.rs` using the exact IEC 61966-2-1 piecewise EOTF inverse
- [x] 2.2 Add linear-RGB-to-XYZ-D65 conversion using the ITU-R BT.709 matrix
- [x] 2.3 Add XYZ-to-Lab conversion using the CIE cube-root transform (D65 white point)
- [x] 2.4 Expose a public `color_distance(a: &Rgba, b: &Rgba) -> f32` function that converts both inputs to Lab and calls `deltae::DE2000::new()` (or equivalent `deltae` API) to return ΔE\*00

## 3. Rename and Clean Up

- [x] 3.1 Replace the `rgb_distance` implementation in `color.rs` with `color_distance` (remove the old Euclidean code)
- [x] 3.2 Update the import/use of `rgb_distance` in `crates/spoolman-client/src/pages/spool.rs` to `color_distance`

## 4. Threshold Update

- [x] 4.1 Change the default threshold `RwSignal` in `spool.rs` from `60u8` to `10u8` (ΔE\*00 scale)

## 5. Verification

- [x] 5.1 Run `cargo check -p spoolman-client` and confirm zero errors
- [x] 5.2 Confirm `cargo tree -p spoolman-client` shows `deltae` with no transitive dependencies
