## 1. Dependency

- [x] 1.1 Add `oklab` crate to `crates/spoolman-client/Cargo.toml`

## 2. Core Color Distance Logic

- [x] 2.1 Add `ColorAlgorithm` enum (`Ciede2000`, `OkLab`, `Din99d`) to `crates/spoolman-client/src/utils/color.rs`
- [x] 2.2 Add `oklab_distance(a: &Rgba, b: &Rgba) -> f32` using the `oklab` crate
- [x] 2.3 Add `din99d_distance(a: &Rgba, b: &Rgba) -> f32` implementing the DIN 6176:2001 transform on the existing CIE L\*a\*b\* values
- [x] 2.4 Update `color_distance(a, b)` to `color_distance(a, b, algo: ColorAlgorithm) -> f32` dispatching to the three implementations
- [x] 2.5 Add `threshold_for(level: &str, algo: ColorAlgorithm) -> Option<f32>` returning per-algorithm threshold constants
- [x] 2.6 Add unit tests: identical colors → 0.0 for each algorithm; red vs blue > expected minimum for each algorithm; DIN99d reference pair against known value

## 3. Reactive State

- [x] 3.1 Add `ColorAlgorithm` (re-export or newtype) and `ColorDistanceAlgorithm(pub RwSignal<ColorAlgorithm>)` to `crates/spoolman-client/src/state.rs`
- [x] 3.2 Add `color_distance_algorithm() -> ColorDistanceAlgorithm` context accessor to `state.rs`

## 4. App Wiring

- [x] 4.1 In `crates/spoolman-client/src/app.rs`: create the `ColorDistanceAlgorithm` signal and `provide_context` it (default `ColorAlgorithm::Ciede2000`)
- [x] 4.2 In `app.rs` settings effect: read `color_distance_algorithm` key and update the signal when the settings resource resolves

## 5. Settings Page

- [x] 5.1 In `crates/spoolman-client/src/pages/settings.rs`: add a local `algo` signal initialised from the context
- [x] 5.2 Populate `algo` from the `color_distance_algorithm` settings key in the load effect
- [x] 5.3 Add a labeled `<select>` with options ciede2000 / oklab / din99d (display: "CIEDE2000 (default)" / "OKLab" / "DIN99d") bound to `algo`
- [x] 5.4 In `on_submit`: call `api::put_setting("color_distance_algorithm", …)` and update the context signal on success

## 6. Spool List Color Filter

- [x] 6.1 In `crates/spoolman-client/src/pages/spool.rs`: read `color_distance_algorithm()` from context
- [x] 6.2 Replace bare `color_distance(a, b)` calls with `color_distance(a, b, algo)` using the context value
- [x] 6.3 Replace inline threshold constants with `threshold_for(level, algo)` calls

## 7. Validation

- [x] 7.1 Run `cargo clippy -p spoolman-client` and fix any warnings (especially enum exhaustiveness)
- [x] 7.2 Run `cargo check -p spoolman-server -p spoolman-types` to confirm no unintended cross-crate breakage
- [ ] 7.3 Manual smoke test: change algorithm in Settings, verify color filter produces visibly different groupings for the same spool list
