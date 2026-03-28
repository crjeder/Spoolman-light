## MODIFIED Requirements

### Requirement: Spool entity structure
A Spool SHALL have: id (random u32), filament_id (u32), location_id (Option<u32>), colors (Vec<Rgba>, len 1–4), color_name (Option<String>), initial_weight (f32, grams), current_weight (f32, grams), net_weight (Option<f32>, grams), registered (DateTime UTC), first_used (Option<DateTime>), last_used (Option<DateTime>), comment (Option<String>), archived (bool).

#### Scenario: Spool created with required fields
- **WHEN** a spool is created with filament_id, colors, initial_weight, and current_weight
- **THEN** the system stores the spool with a unique random u32 id and registered timestamp

#### Scenario: Spool id collision is handled
- **WHEN** a randomly generated u32 id already exists in the store
- **THEN** the system generates a new random u32 until a unique id is found

#### Scenario: Spool stores its own net weight
- **WHEN** a spool is created with net_weight provided
- **THEN** the spool stores the net_weight independently of its filament

#### Scenario: Spool net weight is optional
- **WHEN** a spool is created without net_weight
- **THEN** net_weight is stored as None and weight percentage metrics are omitted from the response

### Requirement: Filament entity structure
A Filament SHALL have: id (random u32), manufacturer (Option<String>), material (Option<String>), material_modifier (Option<String>), diameter (f32, mm, default 1.75), density (f32, g/cm³), print_temp (Option<i32>, °C), bed_temp (Option<i32>, °C), spool_weight (Option<f32>, grams), min_print_temp (Option<i32>), max_print_temp (Option<i32>), min_bed_temp (Option<i32>), max_bed_temp (Option<i32>), registered (DateTime UTC), comment (Option<String>). Filament SHALL NOT have color fields. Filament SHALL NOT have a net_weight field.

#### Scenario: Filament created without color
- **WHEN** a filament is created
- **THEN** the stored filament has no color fields; colors belong to spools referencing this filament

#### Scenario: Filament display name is derived
- **WHEN** a filament is displayed
- **THEN** the display name is derived as "{manufacturer} {material} {material_modifier}" with absent fields omitted

## ADDED Requirements

### Requirement: Schema migration v1 to v2
On startup, if the DataStore schema_version is 1, the server SHALL automatically migrate to version 2 by moving net_weight from each filament to its referencing spools.

#### Scenario: Migration copies net_weight to spools
- **WHEN** the server starts with schema_version 1 and filaments have net_weight values
- **THEN** each spool's net_weight is set to its filament's net_weight, all filament net_weight fields are cleared, schema_version is bumped to 2, and the store is saved atomically

#### Scenario: Migration with None net_weight
- **WHEN** a filament's net_weight is None at migration time
- **THEN** all spools referencing that filament retain net_weight = None after migration

#### Scenario: Already migrated store is untouched
- **WHEN** the server starts with schema_version 2
- **THEN** no migration is performed
