## MODIFIED Requirements

### Requirement: Edit filament
The system SHALL allow updating any filament field: manufacturer, material, material_modifier, diameter, density, spool_weight, min/max print temp, min/max bed temp, min/max chamber temp, comment. net_weight SHALL NOT be accepted on filament create or edit endpoints. `print_temp` and `bed_temp` SHALL NOT be accepted. A request where min exceeds max for any temperature pair SHALL be rejected with 422.

#### Scenario: Update filament specs
- **WHEN** PATCH /api/v1/filament/<id> is called with updated fields
- **THEN** the filament is updated; all spools referencing it reflect the new values on next read

#### Scenario: net_weight field ignored on filament edit
- **WHEN** PATCH /api/v1/filament/<id> is called with a net_weight field
- **THEN** the field is ignored (not stored on the filament)

#### Scenario: Set chamber range
- **WHEN** a filament is updated with min_chamber_temp 40 and max_chamber_temp 60
- **THEN** both values are stored and returned

#### Scenario: Min greater than max rejected
- **WHEN** a filament is created or updated with min_print_temp 230 and max_print_temp 200
- **THEN** the server responds 422 and nothing is stored

## ADDED Requirements

### Requirement: Filament temperature inputs
The filament create and edit forms SHALL provide min and max number inputs for print, bed and chamber temperature (°C). The filament detail view SHALL show each as a range, or a single value when min equals max or only one is set.

#### Scenario: Range displayed
- **WHEN** a filament has min_print_temp 190 and max_print_temp 220
- **THEN** the detail view shows `190-220°C` for print temp
