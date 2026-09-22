## Why

`GET /api/v1/export` produces a full backup of the JSON data store, but there is no way to load one back in. Users who export before an upgrade, or who want to migrate between installs, have no path to restore that data (TODO.md: "There is GET /api/v1/export but no way back in").

## What Changes

- Add `POST /api/v1/import` accepting a full `DataStore` JSON body, validating it, and atomically replacing the on-disk data file and in-memory store.
- Reject imports with an unsupported `meta.schema_version` or malformed structure (400), rather than silently corrupting the store.
- Add a "Restore" control in Settings next to the existing "Reload database" button: file picker + confirmation, calls the new endpoint, then reloads client state.

## Capabilities

### New Capabilities
- `database-import`: accepting, validating, and applying a full `DataStore` JSON backup via the API and Settings UI.

### Modified Capabilities
(none — no existing capability's requirements change)

## Impact

- `crates/spoolman-server/src/routes/other.rs`: new `import` handler + route.
- `crates/spoolman-server/src/store.rs`: new `JsonStore::import` method (validate, flush, swap in-memory data).
- `crates/spoolman-client/src/api/mod.rs`: new `import_database` API call.
- `crates/spoolman-client/src/pages/settings.rs`: Restore button + file input + confirmation.
