## 1. Types and server

- [x] 1.1 Add `filament_id: Option<u32>` and `registered: Option<DateTime<Utc>>` to `UpdateSpool` in `crates/spoolman-types/src/requests.rs`
- [x] 1.2 In `store.rs` `update_spool`, validate a provided `filament_id` exists (validation error otherwise) before mutating
- [x] 1.3 Apply `filament_id` and `registered` in `update_spool`
- [x] 1.4 Remove the `initial_weight == 0` guard so a provided `initial_weight` is always applied
- [x] 1.5 Add server tests: change filament, unknown filament rejected, non-zero initial weight updated (current weight untouched), registered updated

## 2. Edit form

- [x] 2.1 Load filaments in `SpoolEdit` and add a filament `<select>` to Advanced, pre-selected with the spool's filament; send `filament_id`
- [x] 2.2 Initial weight input: above current weight (quick section, mirrors into current weight) when the loaded value is 0; in Advanced (decoupled) otherwise
- [x] 2.3 Add a `datetime-local` "Registered" input; send `registered` only when changed from the loaded value
- [x] 2.4 Add an "Archived" checkbox; send `archived`
- [x] 2.5 Relax the location-required check and Save button disabled state to `location_id.is_none() && !archived`
- [x] 2.6 Keep ID read-only in Advanced

## 3. Verification and docs

- [x] 3.1 `cargo check -p spoolman-types -p spoolman-server`, `cargo clippy`, server tests pass; type-check client for wasm32 if the target is available
- [~] 3.2 Extend the E2E spool edit test (if present) to cover an Advanced field such as filament or registered
- [x] 3.3 CHANGELOG entry (note the `initial_weight` API behaviour change) and TODO.md update
