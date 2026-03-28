## MODIFIED Requirements

### Requirement: Edit filament
The system SHALL allow updating any filament field: manufacturer, material, material_modifier, diameter, density, print_temp, bed_temp, spool_weight, min/max temps, comment. net_weight SHALL NOT be accepted on filament create or edit endpoints.

#### Scenario: Update filament specs
- **WHEN** PATCH /api/v1/filament/<id> is called with updated fields
- **THEN** the filament is updated; all spools referencing it reflect the new values on next read

#### Scenario: net_weight field ignored on filament edit
- **WHEN** PATCH /api/v1/filament/<id> is called with a net_weight field
- **THEN** the field is ignored (not stored on the filament)

### Requirement: Create filament
The system SHALL allow creating a filament with material specification fields. Color fields SHALL NOT be accepted on the filament create endpoint. net_weight SHALL NOT be accepted on the filament create endpoint.

#### Scenario: Filament created with required fields
- **WHEN** POST /api/v1/filament is called with density and diameter
- **THEN** a new filament is created with a random u32 id and registered set to now

#### Scenario: net_weight field ignored on filament create
- **WHEN** POST /api/v1/filament is called with a net_weight field
- **THEN** the field is ignored (not stored on the filament)

### Requirement: Filament import from SpoolmanDB
The frontend SHALL allow searching SpoolmanDB when creating a filament or spool. Material spec fields SHALL populate the filament form; color fields and net_weight from the search result SHALL populate the spool form.

#### Scenario: Import populates filament and spool forms
- **WHEN** user searches SpoolmanDB and selects a result (e.g., "eSun PLA Blue")
- **THEN** filament form is pre-filled with manufacturer, material, density, diameter, temps; spool form is pre-filled with colors, color_name, and net_weight

#### Scenario: Reuse existing filament on import
- **WHEN** a SpoolmanDB search result matches an existing filament (same manufacturer + material + modifier)
- **THEN** the user is offered the option to link the new spool to the existing filament instead of creating a duplicate

#### Scenario: Manual fallback
- **WHEN** the search returns no results or the user skips search
- **THEN** all filament and spool fields are available for manual entry
