## MODIFIED Requirements

### Requirement: Selecting a SpoolmanDB entry auto-fills filament form fields
When a user selects a SpoolmanDB entry from search results on the Filament Create or Filament Edit form, the system SHALL populate the following fields: manufacturer, material (base type), material modifier, diameter, density, min and max print temp (both from the entry's extruder temp), and min and max bed temp (both from the entry's bed temp). Fields SHALL remain editable after auto-fill. The material string SHALL be parsed by stripping known composite suffixes (`+`, `-CF`, `-GF`, `-HF`, `-ESD`) to derive a base `MaterialType` and an optional `material_modifier`.

#### Scenario: Auto-fill simple material
- **WHEN** the user selects an entry with `material: "PLA"` on the Filament Create form
- **THEN** the Material field is set to `PLA` and the Modifier field remains empty

#### Scenario: Auto-fill composite material
- **WHEN** the user selects an entry with `material: "PETG-CF"` on the Filament Create form
- **THEN** the Material field is set to `PETG` and the Modifier field is set to `CF`

#### Scenario: Auto-fill physical properties
- **WHEN** the user selects an entry with defined density, diameter, extruder_temp 215, and bed_temp 60
- **THEN** density and diameter are populated, min and max print temp are 215, and min and max bed temp are 60

#### Scenario: Fields remain editable after auto-fill
- **WHEN** form fields have been auto-filled from a SpoolmanDB selection
- **THEN** the user can still modify any field before submitting
