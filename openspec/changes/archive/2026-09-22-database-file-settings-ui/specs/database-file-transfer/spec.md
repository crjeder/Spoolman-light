## ADDED Requirements

### Requirement: Server can serve the raw data file for download
The system SHALL provide a `GET /api/v1/database/download` endpoint that returns the current data file's raw bytes with `Content-Type: application/json` and a `Content-Disposition: attachment` header naming the file.

#### Scenario: Downloading the current database
- **WHEN** a client sends `GET /api/v1/database/download`
- **THEN** the server responds with the exact bytes currently on disk at the configured data file path, as a downloadable attachment

#### Scenario: Downloading when no data file exists yet
- **WHEN** the configured data file has never been created (fresh install) and a client sends `GET /api/v1/database/download`
- **THEN** the server returns an error response indicating there is no file to download

### Requirement: Server can accept an uploaded data file and replace the live database
The system SHALL provide a `POST /api/v1/database/upload` endpoint that accepts a JSON request body, validates it deserializes into the server's data store schema before writing anything to disk, atomically replaces the configured data file with the uploaded content, and reloads the in-memory store (applying any pending schema migration) from the new file.

#### Scenario: Uploading a valid replacement database
- **WHEN** a client sends `POST /api/v1/database/upload` with a body that parses as a valid data store
- **THEN** the server replaces the on-disk data file with the uploaded content, reloads the in-memory store from it, and subsequent API responses reflect the uploaded data

#### Scenario: Uploading an invalid or malformed file
- **WHEN** a client sends `POST /api/v1/database/upload` with a body that is not valid JSON or does not match the data store schema
- **THEN** the server returns an error response, leaves the on-disk data file and in-memory store unchanged

### Requirement: Settings page exposes database download and upload controls
The Settings page SHALL include a "Database file" section with a "Download database" control that fetches the current file, and an "Upload database" control that lets the user pick a file and confirm before it is sent, reporting the outcome of an upload attempt to the user.

#### Scenario: User downloads the database file
- **WHEN** the user activates "Download database"
- **THEN** the browser saves the current data file to disk under a recognizable filename

#### Scenario: User uploads a replacement database file
- **WHEN** the user picks a file, confirms the destructive-replace warning, and the server accepts the upload
- **THEN** the page shows a success message and reflects the newly uploaded data on next navigation/query

#### Scenario: User cancels the upload confirmation
- **WHEN** the user picks a file but declines the destructive-replace confirmation
- **THEN** no request is sent and the live database is left unchanged

#### Scenario: Uploaded file is rejected by the server
- **WHEN** the user confirms an upload and the server rejects the file as invalid
- **THEN** the page shows an error message describing the failure, and no data is lost
