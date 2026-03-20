## MODIFIED Requirements

### Requirement: JSON file initialization
The system SHALL create a new, empty JSON data file on first startup if none exists at the configured path. The file SHALL have a `meta` key with a `schema_version` field.

#### Scenario: First run with no data file
- **WHEN** the application starts and no data file exists at `SPOOLMAN_DATA_FILE`
- **THEN** the system creates a valid empty JSON file with `{"meta": {"schema_version": 2}, "filaments": [], "spools": [], "settings": {}}`

#### Scenario: Subsequent run with existing file
- **WHEN** the application starts and a data file already exists
- **THEN** the system loads the file into memory without overwriting it

### Requirement: Auto-increment integer IDs
The system SHALL assign auto-incrementing integer IDs to new entities, tracking the next available ID per entity type in the JSON file.

#### Scenario: New entity gets next sequential ID
- **WHEN** a new filament or spool is created
- **THEN** it receives an integer ID one greater than the current maximum ID for that entity type

#### Scenario: IDs remain stable after deletion
- **WHEN** an entity is deleted and a new entity of the same type is created
- **THEN** the new entity receives a new ID higher than any previously assigned ID (no ID reuse)

### Requirement: Extra fields stored as inline dict
The system SHALL store `extra` fields for filaments and spools as a `dict[str, str]` directly on the entity object in JSON, not as a separate relation.

#### Scenario: Create entity with extra fields
- **WHEN** a POST request includes `extra` key-value pairs
- **THEN** the entity is stored with `"extra": {"key": "value", ...}` inline in the JSON

#### Scenario: Extra fields round-trip correctly
- **WHEN** an entity with extra fields is retrieved via GET
- **THEN** the response contains the same extra key-value pairs that were set

### Requirement: API contract reflects simplified model
The system SHALL expose only Filament and Spool resources. Filament fields SHALL be limited to material properties. Spool fields SHALL include color, weight, and price.

#### Scenario: Filament CRUD operations use simplified schema
- **WHEN** any filament API endpoint is called with a valid request
- **THEN** the response contains only material-formula fields (no color, price, weight, article_number, external_id)

#### Scenario: Spool CRUD operations include color and price
- **WHEN** any spool API endpoint is called with a valid request
- **THEN** the response includes color_hex, initial_weight, spool_weight, and price fields

## REMOVED Requirements

### Requirement: API contract unchanged
**Reason**: The API contract is intentionally changed by this redesign. Vendor endpoints are removed; Filament and Spool schemas are restructured.
**Migration**: Clients using `/vendor` endpoints must remove those calls. Clients reading color/price from Filament responses must read them from Spool responses instead.
