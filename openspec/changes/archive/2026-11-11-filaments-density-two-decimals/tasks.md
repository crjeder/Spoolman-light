## 1. Implementation

- [x] 1.1 In `crates/spoolman-client/src/format.rs`, change `format_density` to use `sm_format_decimal(f64::from(g_per_cm3), 2, 2)` and update its doc comment ("exactly 2 fractional digits")
- [x] 1.2 Update `intl-formatting` main spec comment/purpose references if they mention 3 digits (none expected)

## 2. Verification

- [ ] 2.1 `cargo check -p spoolman-types -p spoolman-server` and confirm client change compiles in review (client needs wasm32 target)
- [ ] 2.2 Visually confirm the filaments table shows densities like `1.24 g/cm³` and `1.20 g/cm³`
