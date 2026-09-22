## Why

Operators currently have no way to back up or restore the `spoolman.json` data file except by reaching into the host filesystem (or the container volume). The `POST /api/v1/reload` endpoint (see `database-reload` spec) already lets an operator pick up an externally-replaced file, but there is no in-app way to get the file out for safekeeping or put a new one in. A download/upload pair on the Settings page closes that gap using the existing reload machinery.

## What Changes

- Add a `GET /api/v1/database/download` endpoint that streams the raw data file (`application/json`, `Content-Disposition: attachment`) to the client.
- Add a `POST /api/v1/database/upload` endpoint that accepts a JSON file body, validates it parses as a `DataStore` before touching disk, atomically replaces the data file, and reloads the in-memory store (reusing the existing migrate-on-load path).
- Add a "Database file" section to the Settings page with a "Download database" link/button and an "Upload database" file picker + button, following the existing "Reload database" section's status-message pattern.
- Uploading requires explicit confirmation (it overwrites the live data file) and reports success/failure the same way the existing reload control does.

## Capabilities

### New Capabilities
- `database-file-transfer`: download and upload of the raw JSON data file via HTTP endpoints and a Settings page UI, with server-side validation and reload-on-upload.

### Modified Capabilities
(none — `database-reload` already covers reload semantics; this reuses that machinery via the store rather than changing its requirements)

## Impact

- `crates/spoolman-server/src/routes/other.rs`: two new routes.
- `crates/spoolman-server/src/store.rs`: a method to validate + atomically write an uploaded file, then reload.
- `crates/spoolman-client/src/api.rs`: two new client calls (download via anchor/blob, upload via multipart or raw body fetch).
- `crates/spoolman-client/src/pages/settings.rs`: new "Database file" section.
- No new dependencies expected (Axum body streaming and `web-sys`/`gloo` file APIs already in use elsewhere in the client).
