## 1. Server: import endpoint

- [x] 1.1 Add `JsonStore::import(&self, data: DataStore) -> Result<()>` in `crates/spoolman-server/src/store.rs`: reject `data.meta.schema_version` newer than current, run `migrate()`, `flush()`, then swap `self.inner`
- [x] 1.2 Add `import` handler + `POST /import` route in `crates/spoolman-server/src/routes/other.rs`, taking `Json<DataStore>`, returning `204 No Content` on success and existing `Result<StatusCode>` error mapping on validation failure

## 2. Client: API + Settings UI

- [x] 2.1 Add `import_database(data: &str) -> Result<(), ApiError>` (or typed `DataStore`) in `crates/spoolman-client/src/api/mod.rs` mirroring `reload_database`
- [x] 2.2 Add "Restore" control to the Database section in `crates/spoolman-client/src/pages/settings.rs`: file input, confirmation prompt before submit, status signals mirroring `reload_saved`/`reload_error`, and a page reload/state refresh on success

## 3. Verification

- [x] 3.1 `cargo check -p spoolman-types -p spoolman-server`
- [x] 3.2 `cargo clippy -p spoolman-types -p spoolman-server`
- [x] 3.3 Manual check: export, modify data, import the export back, confirm data matches
- [x] 3.4 Manual check: POST malformed JSON and a newer schema_version to `/api/v1/import`, confirm 400 and store unchanged
