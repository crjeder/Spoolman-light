## 1. State — add context signals

- [x] 1.1 In `crates/spoolman-client/src/state.rs`, add `date_format: RwSignal<String>` and `time_format: RwSignal<String>` to the app-level context struct (alongside `currency_symbol`)
- [x] 1.2 Expose accessor functions `date_format_setting()` and `time_format_setting()` matching the existing accessor pattern

## 2. App bootstrap — seed signals from settings

- [x] 2.1 In `crates/spoolman-client/src/app.rs`, read `date_format` from fetched settings; default to `"medium"` when absent; set the context signal
- [x] 2.2 Read `time_format` from fetched settings; default to `"none"` when absent; set the context signal

## 3. Formatting — parameterise the JS shim and Rust wrapper

- [x] 3.1 In `crates/spoolman-client/src/format.rs`, replace the `sm_format_date_medium(timestamp_ms)` JS shim with `sm_format_date(timestamp_ms, date_style, time_style)` — omit `timeStyle` in the JS when `time_style` is an empty string
- [x] 3.2 Update the `extern "C"` binding declaration to match the new signature
- [x] 3.3 Change `format_date(dt: DateTime<Utc>)` to `format_date(dt: DateTime<Utc>, date_style: &str, time_style: &str)` and call the new shim

## 4. Call sites — pass format signals

- [x] 4.1 In `crates/spoolman-client/src/pages/spool.rs`, read the two context signals and pass them to every `format::format_date(...)` call (table cell, detail panel registered/first_used/last_used)
- [x] 4.2 In `crates/spoolman-client/src/pages/filament.rs`, do the same for the registered column cell

## 5. Settings page — add selectors

- [x] 5.1 In `crates/spoolman-client/src/pages/settings.rs`, add `date_format` and `time_format` `RwSignal<String>` locals, seeded from fetched settings on load
- [x] 5.2 Add a "Date format" `<select>` with options `short`, `medium`, `long`, `full`
- [x] 5.3 Add a "Time format" `<select>` with options `none`, `short`, `medium`
- [x] 5.4 In `on_submit`, call `api::put_setting("date_format", ...)` and `api::put_setting("time_format", ...)` alongside existing keys
- [x] 5.5 On successful save, update the app-level context signals so date cells re-render immediately

## 6. Verification

- [x] 6.1 Run `cargo check -p spoolman-types -p spoolman-server` — must compile clean
- [ ] 6.2 Build the Docker image (`docker build .`) to confirm the WASM client also compiles
- [ ] 6.3 Manually verify in the browser: changing and saving "Date format" to `long` updates all date cells without a reload
- [ ] 6.4 Manually verify: setting "Time format" to `short` appends a time component; reverting to `none` removes it
- [ ] 6.5 Manually verify: a fresh load with no settings stored shows dates in `medium` format with no time component (unchanged from previous behaviour)
