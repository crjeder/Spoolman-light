## ADDED Requirements

### Requirement: List filaments
The system SHALL provide an endpoint to list filaments with server-side filtering, sorting, and pagination.

#### Scenario: List all filaments
- **WHEN** GET /api/v1/filament is called
- **THEN** all filaments are returned with total count in X-Total-Count header

#### Scenario: Filter by material
- **WHEN** GET /api/v1/filament?material=PLA is called
- **THEN** only filaments with material "PLA" are returned

### Requirement: Create filament
The system SHALL allow creating a filament with material specification fields. Color fields SHALL NOT be accepted on the filament create endpoint.

#### Scenario: Filament created with required fields
- **WHEN** POST /api/v1/filament is called with density and diameter
- **THEN** a new filament is created with a random u32 id and registered set to now

### Requirement: Edit filament
The system SHALL allow updating any filament field: manufacturer, material, material_modifier, diameter, net_weight, density, print_temp, bed_temp, spool_weight, min/max temps, comment.

#### Scenario: Update filament specs
- **WHEN** PATCH /api/v1/filament/<id> is called with updated fields
- **THEN** the filament is updated; all spools referencing it reflect the new values on next read

### Requirement: Delete filament
The system SHALL allow deleting a filament. Deletion SHALL be rejected if any spool references the filament.

#### Scenario: Delete unused filament
- **WHEN** DELETE /api/v1/filament/<id> is called and no spool references it
- **THEN** the filament is removed and returns 204

#### Scenario: Delete referenced filament rejected
- **WHEN** DELETE /api/v1/filament/<id> is called and at least one spool references it
- **THEN** the system returns 409 with a message indicating which spools prevent deletion

### Requirement: List materials
The system SHALL provide an endpoint returning the distinct set of material strings across all filaments, for use in filter dropdowns.

#### Scenario: Materials derived from filaments
- **WHEN** GET /api/v1/material is called
- **THEN** a deduplicated list of material strings is returned, sorted alphabetically

### Requirement: Search SpoolmanDB
The system SHALL provide a proxy endpoint that searches the SpoolmanDB filament catalog and returns matching entries. The search is pull-on-demand; no local cache is maintained.

#### Scenario: Search returns matches
- **WHEN** GET /api/v1/filament/search?q=eSun+PLA is called
- **THEN** the backend fetches from SpoolmanDB and returns matching filament entries with all spec fields

#### Scenario: Search fails gracefully
- **WHEN** SpoolmanDB is unreachable
- **THEN** the endpoint returns 503 and the frontend falls back to manual entry

### Requirement: Filament import from SpoolmanDB
The frontend SHALL allow searching SpoolmanDB when creating a filament or spool. Material spec fields SHALL populate the filament form; color fields from the search result SHALL populate the spool form.

#### Scenario: Import populates filament and spool forms
- **WHEN** user searches SpoolmanDB and selects a result (e.g., "eSun PLA Blue")
- **THEN** filament form is pre-filled with manufacturer, material, density, diameter, temps; spool form is pre-filled with colors and color_name

#### Scenario: Reuse existing filament on import
- **WHEN** a SpoolmanDB search result matches an existing filament (same manufacturer + material + modifier)
- **THEN** the user is offered the option to link the new spool to the existing filament instead of creating a duplicate

#### Scenario: Manual fallback
- **WHEN** the search returns no results or the user skips search
- **THEN** all filament and spool fields are available for manual entry
