## 1. State — ColorThresholds context

- [x] 1.1 Add `ColorThresholds` struct to `state.rs` with nine `RwSignal<f32>` fields (`ciede2000_same`, `ciede2000_close`, `ciede2000_ballpark`, `oklab_same`, `oklab_close`, `oklab_ballpark`, `din99d_same`, `din99d_close`, `din99d_ballpark`)
- [x] 1.2 Add `pub fn color_thresholds() -> ColorThresholds` context accessor to `state.rs`
- [x] 1.3 Add `get(level: &str, algo: ColorAlgorithm) -> f32` method to `ColorThresholds` that reads the matching signal
- [x] 1.4 Provide `ColorThresholds` from `App` in `app.rs`, reading initial values from the settings map (falling back to hardcoded defaults when absent)

## 2. Color utility — remove `threshold_for` static function

- [x] 2.1 In `utils/color.rs`, deprecate/remove the static `threshold_for()` function (or keep it as a `default_threshold_for()` helper used only for defaults)
- [x] 2.2 Update all call sites of `threshold_for()` in `pages/spool.rs` to use `color_thresholds().get(level, algo)` instead

## 3. Settings page — threshold editor

- [x] 3.1 Add local signals for the nine threshold values to `pages/settings.rs`, initialised from `color_thresholds()` context signals
- [x] 3.2 Add a "Color Search Thresholds" section to the Settings form showing three `<input type="number" step="any" min="0">` fields (Same, Close, Ballpark) labelled for the active algorithm; fields update reactively when the algorithm selector changes
- [x] 3.3 Add a per-algorithm range hint below each field (e.g. "typical: 0.05 – 0.50" for OKLab)
- [x] 3.4 In the save handler, issue `put_setting` calls for all nine `color_threshold_{algo}_{level}` keys in addition to the existing settings keys
- [x] 3.5 On successful save, update the corresponding `ColorThresholds` context signals so the spool list reacts without a page reload

## 4. Verification

- [ ] 4.1 Confirm `cargo check -p spoolman-client --target wasm32-unknown-unknown` passes
- [ ] 4.2 Manually verify: Settings page shows correct default values, saving persists them, spool list immediately reflects new thresholds
- [ ] 4.3 Manually verify: Switching algorithm on Settings page updates the threshold fields to show that algorithm's values
- [ ] 4.4 Manually verify: Absent keys on a fresh data file still yield original filter behaviour (defaults apply)
