## ADDED Requirements

### Requirement: Import full database backup via API
The system SHALL expose `POST /api/v1/import` accepting a JSON body matching the `DataStore` schema (as produced by `GET /api/v1/export`), replacing all filaments, spools, locations, and settings currently stored.

#### Scenario: Successful import
- **WHEN** a client POSTs a valid `DataStore` JSON body to `/api/v1/import`
- **THEN** the server writes it to the data file, replaces the in-memory store, and responds `204 No Content`

#### Scenario: Malformed JSON body
- **WHEN** a client POSTs a body that does not deserialize into `DataStore` (missing required fields, wrong types)
- **THEN** the server responds `400 Bad Request` and leaves the existing data file and in-memory store unchanged

#### Scenario: Unsupported schema version
- **WHEN** a client POSTs a `DataStore` whose `meta.schema_version` is newer than the server's current schema version
- **THEN** the server responds `400 Bad Request` and leaves the existing data file and in-memory store unchanged

#### Scenario: Older schema version is migrated
- **WHEN** a client POSTs a `DataStore` whose `meta.schema_version` is older than current
- **THEN** the server applies the same migration steps used on startup load before persisting the imported data

### Requirement: Restore database from Settings UI
The Settings page SHALL provide a control to select a local JSON backup file and restore it via the import endpoint, with a confirmation step before the destructive action proceeds.

#### Scenario: User restores a backup
- **WHEN** a user selects a valid backup file in Settings and confirms the restore action
- **THEN** the client calls `POST /api/v1/import` with the file contents and shows a success message on completion

#### Scenario: User cancels restore confirmation
- **WHEN** a user selects a backup file but cancels the confirmation prompt
- **THEN** no request is sent and the existing data is unchanged

#### Scenario: Import fails
- **WHEN** the import request returns an error (e.g. malformed file, unsupported schema version)
- **THEN** the Settings page displays the error message and does not treat the restore as successful
