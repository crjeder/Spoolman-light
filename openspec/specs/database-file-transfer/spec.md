# database-file-transfer

## Purpose
Lets a user download the current Spoolman data file as a backup, via the API and the Settings page UI. Restoring a backup is covered by the separate `database-import` capability (`POST /api/v1/import` and the "Restore from backup" control), which this pairs with.

## Requirements

### Requirement: Server can serve the raw data file for download
The system SHALL provide a `GET /api/v1/database/download` endpoint that returns the current data file's raw bytes with `Content-Type: application/json` and a `Content-Disposition: attachment` header naming the file.

#### Scenario: Downloading the current database
- **WHEN** a client sends `GET /api/v1/database/download`
- **THEN** the server responds with the exact bytes currently on disk at the configured data file path, as a downloadable attachment

#### Scenario: Downloading when no data file exists yet
- **WHEN** the configured data file has never been created (fresh install) and a client sends `GET /api/v1/database/download`
- **THEN** the server returns an error response indicating there is no file to download

### Requirement: Settings page exposes a database download control
The Settings page SHALL include a "Download database" control next to "Reload database" that fetches the current file as a downloadable attachment.

#### Scenario: User downloads the database file
- **WHEN** the user activates "Download database"
- **THEN** the browser saves the current data file to disk under a recognizable filename
