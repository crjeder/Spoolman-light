## ADDED Requirements

### Requirement: README positions the project as home-use
The README SHALL describe Spoolman as a lightweight, self-hosted filament tracker optimized for home use with one or two printers and tens of spools.

#### Scenario: Reader understands the scope
- **WHEN** a reader opens the README
- **THEN** the intro paragraph makes clear this is a home-scale tool, not an enterprise/fleet solution

### Requirement: README feature list matches the actual codebase
The README's feature list SHALL only describe capabilities present in the current codebase. Removed features MUST NOT appear.

#### Scenario: No mention of removed database backends
- **WHEN** a reader reviews the Features section
- **THEN** there is no mention of SQLite, PostgreSQL, MySQL, or CockroachDB backends

#### Scenario: No mention of removed entities
- **WHEN** a reader reviews the Features section
- **THEN** there is no mention of Vendor management or a vendor entity

#### Scenario: No mention of removed integrations as first-class features
- **WHEN** a reader reviews the Features section
- **THEN** Prometheus monitoring, WebSocket real-time updates, and SpoolmanDB community database are not listed as features

#### Scenario: JSON storage is described accurately
- **WHEN** a reader reviews the Features section
- **THEN** data storage is described as a single JSON file with no database server required

### Requirement: README environment variable table is accurate
The README SHALL list only environment variables that are currently recognized by the application.

#### Scenario: DB variables are absent
- **WHEN** a reader reviews the environment variable table
- **THEN** `SPOOLMAN_DB_TYPE`, `SPOOLMAN_DB_HOST`, `SPOOLMAN_DB_PORT`, `SPOOLMAN_DB_NAME`, `SPOOLMAN_DB_USERNAME`, and `SPOOLMAN_DB_PASSWORD` do not appear

#### Scenario: Data file variable is present
- **WHEN** a reader reviews the environment variable table
- **THEN** `SPOOLMAN_DATA_FILE` is listed with its default path and purpose

### Requirement: README does not advertise unsupported third-party integrations
The README SHALL NOT list specific third-party clients (Moonraker, OctoPrint, etc.) as supported integrations without a caveat. A general note about REST API compatibility is acceptable.

#### Scenario: No specific integration list
- **WHEN** a reader reviews the README
- **THEN** there is no bulleted list of third-party integrations presented as officially supported
