## MODIFIED Requirements

### Requirement: Filament entity structure
A Filament SHALL have: id (random u32), manufacturer (Option<String>), material (Option<String>), material_modifier (Option<String>), diameter (f32, mm, default 1.75), density (f32, g/cm³), min_print_temp / max_print_temp (Option<i32>, °C), min_bed_temp / max_bed_temp (Option<i32>, °C), min_chamber_temp / max_chamber_temp (Option<i32>, °C), spool_weight (Option<f32>, grams), registered (DateTime UTC), comment (Option<String>). Filament SHALL NOT have single-value `print_temp` or `bed_temp` fields in written data. Filament SHALL NOT have color fields. Filament SHALL NOT have a net_weight field.

#### Scenario: Filament created without color
- **WHEN** a filament is created
- **THEN** the stored filament has no color fields; colors belong to spools referencing this filament

#### Scenario: Migrate v2 single temperatures
- **WHEN** the server loads a schema_version 2 file whose filament has `print_temp: 215`, `bed_temp: 60` and no min/max temps
- **THEN** min_print_temp = max_print_temp = 215 and min_bed_temp = max_bed_temp = 60, `print_temp`/`bed_temp` are no longer written, and schema_version becomes 3

#### Scenario: Migration keeps existing ranges
- **WHEN** a v2 filament has `print_temp: 215` and `min_print_temp: 200`
- **THEN** min_print_temp stays 200 and max_print_temp becomes 215
