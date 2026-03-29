## 1. Create the `format` module

- [x] 1.1 Add `mod format;` to `crates/spoolman-client/src/lib.rs` (or `main.rs`) and create `crates/spoolman-client/src/format.rs`
- [x] 1.2 Confirm `js-sys` is available as a workspace dependency (check `Cargo.lock`); add explicit declaration to `spoolman-client/Cargo.toml` if missing
- [x] 1.3 Implement `pub fn format_weight(grams: f32) -> String` using `js_sys::Intl::NumberFormat` with `style: "decimal"`, `maximumFractionDigits: 1`, append `" g"`
- [x] 1.4 Implement `pub fn format_density(g_per_cm3: f32) -> String` using `Intl.NumberFormat` with `maximumFractionDigits: 3`, append `" g/cm³"`
- [x] 1.5 Implement `pub fn format_date(date: chrono::NaiveDate) -> String` using `js_sys::Intl::DateTimeFormat` with `dateStyle: "medium"`
- [x] 1.6 Implement `pub fn format_currency(amount: f64, symbol_override: &str) -> String` — if override non-empty use it as literal prefix with decimal-style Intl, else use currency-style Intl

## 2. Update spool display sites

- [x] 2.1 Replace `format!("{:.0}g", w)` / `format!("{:.1}g", w)` in the spool table (`pages/spool.rs`, `rem` variable) with `format::format_weight`
- [x] 2.2 Replace weight `format!` calls in the spool detail view (`initial_weight`, `current_weight`, `used_weight`, `remaining_filament`) with `format::format_weight`
- [x] 2.3 Replace `.format("%Y-%m-%d")` calls for `registered`, `first_used`, `last_used` in spool table and detail view with `format::format_date`

## 3. Update filament display sites

- [x] 3.1 Replace `format!("{:.2}mm", d)` diameter display in filament table with a locale-formatted call (use `format_weight` pattern with `" mm"` suffix, or a dedicated `format_mm` helper)
- [x] 3.2 Replace `format!("{:.3}", f.density)` in filament table with `format::format_density`
- [x] 3.3 Replace `.format("%Y-%m-%d")` for `registered` in filament table with `format::format_date`
- [x] 3.4 Replace equivalent `format!` calls in the filament detail view (diameter, density, spool_weight, print_temp, bed_temp if applicable)

## 4. Wire currency symbol setting into context

- [x] 4.1 Expose `currency_symbol` as a reactive signal in app-wide context (similar to how `diameter_settings` works) so display components can read it without prop-drilling
- [x] 4.2 Populate the signal from the settings API response on app startup (e.g. in the root component or a settings resource)
- [x] 4.3 Document `format_currency` usage in a code comment (no call sites yet — price fields don't exist — but the helper must be exercised or it will rot)

## 5. Verification

- [x] 5.1 Run `cargo check -p spoolman-client --target wasm32-unknown-unknown` and fix any compile errors
- [x] 5.2 Run `cargo clippy -p spoolman-client` and address any warnings introduced by this change
- [ ] 5.3 Manually verify in browser (via `cargo leptos watch` in WSL) that dates, weights, and densities render with the browser locale and do not show raw ISO or Rust-format strings
