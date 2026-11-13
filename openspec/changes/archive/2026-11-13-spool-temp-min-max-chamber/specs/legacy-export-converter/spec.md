## MODIFIED Requirements

### Requirement: Reconstruct filaments from spool export
The script SHALL extract unique filaments from the flat spool records (keyed by `filament.id`) and write them as the `filaments` array, using the field names of the current `Filament` data model. Temperatures SHALL be written as min/max pairs; `print_temp` and `bed_temp` SHALL NOT be written.

#### Scenario: Vendor name mapped to manufacturer
- **WHEN** a spool record has a `filament.vendor.name` key
- **THEN** the filament's `manufacturer` field is set to that string value (not `vendor`)

#### Scenario: Filament with no vendor
- **WHEN** a spool record has no `filament.vendor.name` key (or it is null)
- **THEN** the filament's `manufacturer` field is set to `null`

#### Scenario: Extruder temp mapped to print range
- **WHEN** a spool record has `filament.settings_extruder_temp` set to 210
- **THEN** the filament's `min_print_temp` and `max_print_temp` are both 210

#### Scenario: Bed temp mapped to bed range
- **WHEN** a spool record has `filament.settings_bed_temp` set to 60
- **THEN** the filament's `min_bed_temp` and `max_bed_temp` are both 60

#### Scenario: material_modifier derived from filament name
- **WHEN** a spool record has `filament.name` set and the name starts with the material string (case-insensitive)
- **THEN** the leading material prefix and surrounding whitespace are stripped and the remainder is set as `material_modifier`

#### Scenario: material_modifier uses full name when no material prefix
- **WHEN** a spool record has `filament.name` set but it does not start with the material string
- **THEN** `material_modifier` is set to the full name value

#### Scenario: material_modifier is null when name matches material exactly or is absent
- **WHEN** the name is null, empty, or identical to the material string alone
- **THEN** `material_modifier` is set to `null`

#### Scenario: extra field is not emitted
- **WHEN** a filament record has an `extra` key
- **THEN** the output filament does NOT contain an `extra` field

#### Scenario: Duplicate filament IDs are deduplicated
- **WHEN** multiple spool records share the same `filament.id`
- **THEN** only one filament entry is written, using data from the first occurrence
