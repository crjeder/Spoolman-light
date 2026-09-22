## 1. Server: download endpoint

- [x] 1.1 Add `download_database` handler in `crates/spoolman-server/src/routes/other.rs` reading the file at `store.data_file_path()` and returning it with `Content-Type: application/json` and `Content-Disposition: attachment; filename="spoolman.json"`.
- [x] 1.2 Handle the missing-file case (fresh install) with an error response instead of panicking.
- [x] 1.3 Register `GET /database/download` in the router.

## 2. Server: upload endpoint

- [x] 2.1 Add a `JsonStore` method that: deserializes the given bytes into `DataStore`, writes them to a temp file beside the data file, renames the temp file over the target path, then calls `self.reload()`. (reused the existing `flush()` atomic-write helper instead of duplicating temp-file logic)
- [x] 2.2 Add `upload_database` handler in `routes/other.rs` that reads the request body, calls the new store method, and maps errors to the existing `routes::error::Result` error response.
- [x] 2.3 Register `POST /database/upload` in the router.

## 3. Client: API calls

- [x] 3.1 Add `download_database_url()` or equivalent helper in `crates/spoolman-client/src/api.rs` (or construct the URL inline) for the anchor-based download.
- [x] 3.2 Add `upload_database(bytes: Vec<u8>) -> Result<(), ...>` in `api.rs` that POSTs the raw bytes to `/database/upload` with `Content-Type: application/json`, following the existing error-handling pattern used by `reload_database`. (took a `String` — the file is read as UTF-8 text via `File::text()`, no `Vec<u8>` round trip needed)

## 4. Client: Settings page UI

- [x] 4.1 Add a "Database file" section to `crates/spoolman-client/src/pages/settings.rs` (near or merged with the existing "Database" reload section) with a "Download database" link/button that navigates to the download endpoint.
- [x] 4.2 Add a file `<input type="file" accept="application/json">` and "Upload database" button with its own status signals (mirroring `reload_saved` / `reload_error`).
- [x] 4.3 Wire a confirmation step (native `confirm()` via `web_sys`) before the upload request fires.
- [x] 4.4 On successful upload, show a success message; on failure, show the server's error message.

## 5. Verification

- [x] 5.1 `cargo check -p spoolman-server` and `cargo check -p spoolman-types` pass. (also checked `spoolman-client` for `wasm32-unknown-unknown` and ran clippy on server/types — all clean)
- [x] 5.2 Manually verify download produces a valid file and upload of that same file round-trips without data loss. (covered by `crates/spoolman-server/tests/database_file.rs::upload_replaces_store`; full-browser manual check not run — `cargo-leptos` build is blocked on Windows per repo gotchas and Docker Desktop's daemon isn't running in this environment)
- [x] 5.3 Manually verify uploading a malformed file is rejected and leaves the existing data intact. (covered by `database_file.rs::upload_rejects_malformed_body`)
- [x] 5.4 Update `CHANGELOG.md` under a new version entry when archiving this change.
