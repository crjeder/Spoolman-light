## 1. Types crate — data model

- [x] 1.1 Remove `net_weight: Option<f32>` from `Filament` struct in `spoolman-types/src/models.rs`
- [x] 1.2 Add `net_weight: Option<f32>` to `Spool` struct in `spoolman-types/src/models.rs`
- [x] 1.3 Remove `net_weight` from `CreateFilament` in `spoolman-types/src/requests.rs`
- [x] 1.4 Remove `net_weight` from `UpdateFilament` in `spoolman-types/src/requests.rs`
- [x] 1.5 Add `net_weight: Option<f32>` to `CreateSpool` in `spoolman-types/src/requests.rs`
- [x] 1.6 Add `net_weight: Option<f32>` to `UpdateSpool` in `spoolman-types/src/requests.rs`
- [x] 1.7 Update `SpoolResponse::new` in `spoolman-types/src/responses.rs` to derive `remaining_filament` and `remaining_pct` from `spool.net_weight` instead of `filament.net_weight`
- [x] 1.8 Run `cargo check -p spoolman-types` and fix any compilation errors

## 2. Server — migration and route handlers

- [x] 2.1 Bump `DEFAULT_SCHEMA_VERSION` (or equivalent constant) to 2 in `spoolman-server`
- [x] 2.2 Implement schema v1→v2 migration in `JsonStore`: for each spool copy `filament.net_weight` into `spool.net_weight`, then clear all `filament.net_weight` values
- [x] 2.3 Invoke migration in `JsonStore::load` (or equivalent startup path) when `schema_version < 2`, then atomically save
- [x] 2.4 Update `CreateSpool` handler to store `request.net_weight` on the new spool
- [x] 2.5 Update `UpdateSpool` handler to apply `request.net_weight` patch to the spool
- [x] 2.6 Update clone-spool handler to copy `source.net_weight` into the new spool
- [x] 2.7 Run `cargo check -p spoolman-server` and fix any compilation errors
- [x] 2.8 Run `cargo clippy -p spoolman-types -p spoolman-server` and resolve warnings

## 3. Client — forms and display

- [x] 3.1 Remove `net_weight` field from the filament create/edit form in `spoolman-client`
- [x] 3.2 Add `net_weight` field (optional, numeric, grams) to the spool create form
- [x] 3.3 Add `net_weight` field to the spool edit form
- [x] 3.4 Update clone-spool flow to include `net_weight` in the pre-filled form (server handler already copies it; no UI input needed)
- [x] 3.5 Update weight display component to source `net_weight` from `spool` (verified: `remaining_filament`/`remaining_pct` flow through `SpoolResponse` — no change needed)
- [x] 3.6 Update SpoolmanDB import: route `net_weight` from the search result to the spool form (SpoolmanDB import UI not yet built — N/A)

## 4. Test data and validation

- [x] 4.1 Update `assets/spoolman.json` test data: remove `net_weight` from filament objects, add `net_weight` to spool objects (already in schema v2 format)
- [x] 4.2 Start the server against the updated test data and confirm migration completes without error
- [x] 4.3 Run Playwright tests; fix any failures caused by form field changes (no test suite exists yet — N/A)
- [x] 4.4 Manually verify: create a spool with net_weight, confirm remaining % displays correctly
- [x] 4.5 Manually verify: edit a spool's net_weight, confirm metrics update
