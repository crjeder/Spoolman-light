## 1. Replace threshold slider with level selector

- [x] 1.1 In `crates/spoolman-client/src/pages/spool.rs`, replace `let threshold = create_rw_signal(60u8);` with `let color_level = create_rw_signal("off".to_string());`
- [x] 1.2 Add threshold constants near the top of `SpoolList`: `const FINE: f32 = 10.0; const MEDIUM: f32 = 30.0; const COARSE: f32 = 60.0;`
- [x] 1.3 Update the filter closure: replace `threshold.get() as f32` with a match on `color_level.get()` returning the appropriate constant (and treat "off" as disabling the colour filter entirely, even if `color_pick` has a value)
- [x] 1.4 In the view, replace the `<input type="range" …>` element with a `<select>` bound to `color_level`, containing options: `off`/Off, `fine`/Fine, `medium`/Medium, `coarse`/Coarse
- [x] 1.5 When `color_level` is "off", show neither the colour picker nor the `×` clear button (conditioned on `color_level != "off"`)
- [x] 1.6 When the user switches `color_level` back to "off", reset `color_pick` to `None` and clear the colour input element value

## 2. Remove toolbar color-picker button

- [x] 2.1 Locate any standalone toolbar `<button>` or element above the table that opens or represents the colour picker (distinct from the `<input type="color">` itself) and remove it from the view

## 3. Add active-filter indicator to Color column header

- [x] 3.1 In the Color `<th>` render, add a reactive expression: when `color_level` is not "off", append `" ■"` (space + U+25A0) after "Color"; otherwise render "Color" alone
- [x] 3.2 Verify the indicator appears and disappears correctly as the level selector changes

## 4. Verification

- [x] 4.1 Run `cargo check -p spoolman-client` (via wasm32 target or CI) to confirm no compile errors
- [x] 4.2 Manually test: select Fine/Medium/Coarse, pick a colour, confirm filtering; switch to Off, confirm filter cleared and indicator hidden
- [x] 4.3 Mark the three TODO items as complete in `TODO.md`
