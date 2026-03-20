## ADDED Requirements

### Requirement: DB env vars are not recognized
The system SHALL NOT read or act on `SPOOLMAN_DB_TYPE`, `SPOOLMAN_DB_HOST`, `SPOOLMAN_DB_PORT`, `SPOOLMAN_DB_NAME`, `SPOOLMAN_DB_USERNAME`, `SPOOLMAN_DB_PASSWORD`, or `SPOOLMAN_DB_PASSWORD_FILE`. These variables MUST be absent from all project documentation and contributor guides.

#### Scenario: Legacy DB env var set by user has no effect
- **WHEN** a user sets `SPOOLMAN_DB_TYPE=postgres` (or any other legacy DB var) in their environment
- **THEN** the application starts normally using JSON file storage and the variable is silently ignored

#### Scenario: CLAUDE.md env var table contains no DB rows
- **WHEN** a contributor reads the CLAUDE.md environment variable reference table
- **THEN** no `SPOOLMAN_DB_*` rows are present
