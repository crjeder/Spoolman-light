## 1. Preparation

- [x] 1.1 Create a git worktree on a new branch `feat/din99-default-color-model` under `.worktrees/`

## 2. Update DIN99d Default Thresholds

- [x] 2.1 In `crates/spoolman-client/src/utils/color.rs`, update `default_threshold_for()` DIN99d arm: change `"same"` from `10.0` to `13.0`, `"close"` from `20.0` to `19.0`, `"ballpark"` from `35.0` to `25.0`

## 3. Change Default Algorithm

- [x] 3.1 In `crates/spoolman-client/src/state.rs`, change the initial value of the color distance algorithm signal from `ColorAlgorithm::Ciide2000` to `ColorAlgorithm::Din99d`
- [x] 3.2 In `crates/spoolman-client/src/app.rs`, update the settings-load fallback for `color_distance_algorithm`: change the catch-all `_ =>` arm to produce `ColorAlgorithm::Din99d` instead of `Ciide2000`

## 4. Update Settings Page Labels

- [x] 4.1 In `crates/spoolman-client/src/pages/settings.rs`, update the algorithm selector option labels: remove "(default)" from the CIEDE2000 option and add "(default)" to the DIN99d option

## 5. Verification

- [x] 5.1 Run `cargo check -p spoolman-types` and `cargo check -p spoolman-server` — both must pass without errors
- [x] 5.2 Verify with `grep` that no reference to `ColorAlgorithm::Ciide2000` or `"ciede2000"` remains as a hardcoded default fallback in `state.rs` or `app.rs`
- [x] 5.3 Verify with `grep` that `default_threshold_for` in `color.rs` returns 13.0 / 19.0 / 25.0 for the DIN99d arms
