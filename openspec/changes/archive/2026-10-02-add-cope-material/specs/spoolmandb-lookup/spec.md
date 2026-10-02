## ADDED Requirements

### Requirement: CoPE material alias
The system SHALL treat the material string "CoPE" or "COPE" as the `CPE` (Copolyester) material type when parsing a material abbreviation, including SpoolmanDB material strings.

#### Scenario: Auto-fill CoPE entry
- **WHEN** the user selects a SpoolmanDB entry with `material: "CoPE"` on the Filament Create form
- **THEN** the Material field is set to `CPE`

#### Scenario: Composite CoPE entry
- **WHEN** the user selects an entry with `material: "CoPE-CF"`
- **THEN** the Material field is set to `CPE` and the Modifier field is set to `CF`
