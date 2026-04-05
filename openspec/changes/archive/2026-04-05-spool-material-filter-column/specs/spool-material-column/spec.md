## ADDED Requirements

### Requirement: Material column displays filament material
The spool list table SHALL include a "Material" column displaying the filament's material abbreviation (e.g. "PLA", "PETG"). When the filament has no material set, the cell SHALL be blank.

#### Scenario: Material abbreviation shown
- **WHEN** a spool's filament has a material set (e.g. PLA)
- **THEN** the Material column cell displays the material abbreviation (e.g. "PLA")

#### Scenario: Blank cell for unset material
- **WHEN** a spool's filament has no material set
- **THEN** the Material column cell is blank

### Requirement: Material column header indicates active filter
The Material column header SHALL display a ■ character (U+25A0) alongside the label "Material" when a material filter is active.

#### Scenario: Active filter indicator shown
- **WHEN** a material other than "All" is selected in the material filter dropdown
- **THEN** the "Material" column header displays "Material ■"

#### Scenario: Active filter indicator hidden when All selected
- **WHEN** "All" is selected in the material filter dropdown
- **THEN** the "Material" column header displays only "Material" with no ■
