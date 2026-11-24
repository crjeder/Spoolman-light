## 1. Implementation

- [x] 1.1 Add `clear_session_filters()` to `crates/spoolman-client/src/state.rs` removing all `sessionStorage` keys with prefix `filter.`
- [x] 1.2 Call it from the `nav-links` `on:click` handler in `crates/spoolman-client/src/components/layout.rs`

## 2. Verification

- [x] 2.1 Add E2E test: set material filter on Color tab, switch to Spools, assert Material filter is "All"
- [x] 2.2 Add E2E test: filter survives spool detail round trip
- [x] 2.3 `cargo check` passes (server, wasm client); E2E suite not run
- [x] 2.4 Update `CHANGELOG.md` and `TODO.md` on archive
