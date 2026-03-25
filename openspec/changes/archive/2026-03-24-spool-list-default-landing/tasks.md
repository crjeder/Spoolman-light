## 1. Route Table

- [x] 1.1 In `crates/spoolman-client/src/app.rs`, change the `"/"` route's `view` from `HomePage` to `SpoolList`
- [x] 1.2 Remove the `use crate::pages::home::HomePage` import from `app.rs`

## 2. Nav Active-Link

- [x] 2.1 In the nav bar component, update the Spools link so it is highlighted as active when the current path is `"/"` or `"/spools"`

## 3. Cleanup

- [x] 3.1 Delete `crates/spoolman-client/src/pages/home.rs`
- [x] 3.2 Remove `pub mod home;` from `crates/spoolman-client/src/pages/mod.rs`

## 4. Verification

- [x] 4.1 Run `cargo check -p spoolman-server` and `cargo check` (client requires wasm32 target — check server only on Windows) to confirm no remaining references to `HomePage` or `home` module
