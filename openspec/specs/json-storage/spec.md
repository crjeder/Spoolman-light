# JSON Storage Spec

## Requirements

### Requirement: JSON file initialization
The system SHALL create a new, empty JSON data file on first startup if none exists at the configured path. The file SHALL have a `meta` key with a `schema_version` field.

#### Scenario: First run with no data file
- **WHEN** the application starts and no data file exists at `SPOOLMAN_DATA_FILE`
- **THEN** the system creates a valid empty JSON file with `{"meta": {"schema_version": 1}, "vendors": [], "filaments": [], "spools": [], "settings": {}}`

#### Scenario: Subsequent run with existing file
- **WHEN** the application starts and a data file already exists
- **THEN** the system loads the file into memory without overwriting it

### Requirement: Configurable data file path
The system SHALL read the data file path from the `SPOOLMAN_DATA_FILE` environment variable, defaulting to `<data_dir>/spoolman.json`.

#### Scenario: Custom path via environment variable
- **WHEN** `SPOOLMAN_DATA_FILE=/custom/path/data.json` is set
- **THEN** the system reads and writes data to `/custom/path/data.json`

#### Scenario: Default path when env var is unset
- **WHEN** `SPOOLMAN_DATA_FILE` is not set
- **THEN** the system uses `<SPOOLMAN_DATA_DIR>/spoolman.json` as the data file path

### Requirement: Atomic writes
The system SHALL write data atomically by writing to a temporary file first, then renaming it to the target path, to prevent data corruption on crash.

#### Scenario: Successful mutation persists atomically
- **WHEN** any create, update, or delete operation completes
- **THEN** the updated data is written to a `.tmp` file adjacent to the data file and then renamed to the data file path

#### Scenario: Crash during write does not corrupt data
- **WHEN** the process crashes after the tmp file is written but before rename
- **THEN** the original data file remains intact and readable on next startup

### Requirement: In-memory data store
The system SHALL maintain an in-memory representation of all data, loading from disk at startup and flushing to disk on every mutation.

#### Scenario: Read operations use in-memory data
- **WHEN** a GET request is made for any entity
- **THEN** the response is served from the in-memory store without reading the file

#### Scenario: Write operations flush to disk
- **WHEN** a POST, PUT, PATCH, or DELETE request mutates data
- **THEN** the in-memory store is updated and the data file is flushed before the response is returned

### Requirement: Auto-increment integer IDs
The system SHALL assign auto-incrementing integer IDs to new entities, tracking the next available ID per entity type in the JSON file.

#### Scenario: New entity gets next sequential ID
- **WHEN** a new vendor, filament, or spool is created
- **THEN** it receives an integer ID one greater than the current maximum ID for that entity type

#### Scenario: IDs remain stable after deletion
- **WHEN** an entity is deleted and a new entity of the same type is created
- **THEN** the new entity receives a new ID higher than any previously assigned ID (no ID reuse)

### Requirement: Extra fields stored as inline dict
The system SHALL store `extra` fields for vendors, filaments, and spools as a `dict[str, str]` directly on the entity object in JSON, not as a separate relation.

#### Scenario: Create entity with extra fields
- **WHEN** a POST request includes `extra` key-value pairs
- **THEN** the entity is stored with `"extra": {"key": "value", ...}` inline in the JSON

#### Scenario: Extra fields round-trip correctly
- **WHEN** an entity with extra fields is retrieved via GET
- **THEN** the response contains the same extra key-value pairs that were set

### Requirement: API contract unchanged
The system SHALL preserve all existing REST API routes, request schemas, and response schemas so that API clients and the frontend require no changes.

#### Scenario: Vendor CRUD operations work identically
- **WHEN** any vendor API endpoint is called with a valid request
- **THEN** the response matches the existing API contract (same fields, same HTTP status codes)

#### Scenario: Filament CRUD operations work identically
- **WHEN** any filament API endpoint is called with a valid request
- **THEN** the response matches the existing API contract

#### Scenario: Spool CRUD operations work identically
- **WHEN** any spool API endpoint is called with a valid request
- **THEN** the response matches the existing API contract

### Requirement: Settings persistence
The system SHALL persist application settings as a flat `dict[str, str]` under the `settings` key in the JSON file.

#### Scenario: Setting is saved and retrieved
- **WHEN** a setting key is written via the settings API
- **THEN** the value is persisted to the JSON file and retrievable after restart

### Requirement: DB env vars are not recognized
The system SHALL NOT read or act on `SPOOLMAN_DB_TYPE`, `SPOOLMAN_DB_HOST`, `SPOOLMAN_DB_PORT`, `SPOOLMAN_DB_NAME`, `SPOOLMAN_DB_USERNAME`, `SPOOLMAN_DB_PASSWORD`, or `SPOOLMAN_DB_PASSWORD_FILE`. These variables MUST be absent from all project documentation and contributor guides.

#### Scenario: Legacy DB env var set by user has no effect
- **WHEN** a user sets `SPOOLMAN_DB_TYPE=postgres` (or any other legacy DB var) in their environment
- **THEN** the application starts normally using JSON file storage and the variable is silently ignored

#### Scenario: CLAUDE.md env var table contains no DB rows
- **WHEN** a contributor reads the CLAUDE.md environment variable reference table
- **THEN** no `SPOOLMAN_DB_*` rows are present
