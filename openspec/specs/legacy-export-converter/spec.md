# legacy-export-converter Specification

## Purpose
Defines the script that converts a legacy flat spool export into spoolman.json.

## Requirements

### Requirement: Convert spool export to spoolman.json
The script SHALL accept a path to the old flat spool export JSON as its primary positional argument and write a valid `spoolman.json` to the specified output path.

#### Scenario: Output contains all required top-level keys
- **WHEN** the script is run with a valid old spool export JSON file
- **THEN** it writes a `spoolman.json` with `meta.schema_version: 2`, a `filaments` array, a `spools` array, a `locations` array, and a `settings` object

#### Scenario: Output path configurable via --output flag
- **WHEN** the user passes `--output /path/to/spoolman.json`
- **THEN** the converted file is written to that path

#### Scenario: Output defaults to spoolman.json in current directory
- **WHEN** no `--output` flag is provided
- **THEN** the script writes to `./spoolman.json`

### Requirement: Reconstruct filaments from spool export
The script SHALL extract unique filaments from the flat spool records (keyed by `filament.id`) and write them as the `filaments` array, using the field names of the current `Filament` data model.

#### Scenario: Vendor name mapped to manufacturer
- **WHEN** a spool record has a `filament.vendor.name` key
- **THEN** the filament's `manufacturer` field is set to that string value (not `vendor`)

#### Scenario: Filament with no vendor
- **WHEN** a spool record has no `filament.vendor.name` key (or it is null)
- **THEN** the filament's `manufacturer` field is set to `null`

#### Scenario: Extruder temp mapped to print_temp
- **WHEN** a spool record has `filament.settings_extruder_temp` set
- **THEN** the filament's `print_temp` field is set to that value (not `settings_extruder_temp`)

#### Scenario: Bed temp mapped to bed_temp
- **WHEN** a spool record has `filament.settings_bed_temp` set
- **THEN** the filament's `bed_temp` field is set to that value (not `settings_bed_temp`)

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

### Requirement: Map color fields to RGBA objects on spool
The script SHALL convert old hex-string color fields into the `colors` array of `{r, g, b, a}` objects expected by the current `Spool` data model.

#### Scenario: Single color_hex converted to colors array
- **WHEN** a spool record has a non-null `color_hex` (or `filament.color_hex`) value
- **THEN** the output spool has `colors: [{r, g, b, a: 255}]` derived from that hex string

#### Scenario: multi_color_hexes converted to colors array
- **WHEN** a spool record has a non-null `multi_color_hexes` list
- **THEN** the output spool has `colors` containing one RGBA entry per hex string (up to 4)

#### Scenario: No color produces empty colors array
- **WHEN** both `color_hex` and `multi_color_hexes` are null or absent
- **THEN** the output spool has `colors: []`

#### Scenario: multi_color_direction is dropped
- **WHEN** a spool record has `multi_color_direction` set
- **THEN** the output spool does NOT contain `multi_color_direction`

### Requirement: Collect locations into a top-level array
The script SHALL gather all unique location strings from spool records, create `Location` objects with synthetic IDs, and emit them as the `locations` array in the output store.

#### Scenario: Unique location strings become Location objects
- **WHEN** spool records contain non-null `location` strings
- **THEN** each unique string appears exactly once in the output `locations` array as `{id, name}`

#### Scenario: Spools reference location by ID
- **WHEN** a spool record has a non-null `location` string
- **THEN** the output spool has a `location_id` field set to the ID of the matching Location object (not a `location` string field)

#### Scenario: Spool with no location
- **WHEN** a spool record has a null or absent `location`
- **THEN** the output spool has `location_id: null`

#### Scenario: locations array present in output even when empty
- **WHEN** no spool has a non-null location
- **THEN** the output store still contains an empty `locations` array

### Requirement: Derive current_weight from used_weight
The script SHALL compute `current_weight` on each output spool so that the remaining-weight ratio is preserved.

#### Scenario: current_weight computed when used_weight is present
- **WHEN** a spool record has both `initial_weight` and `used_weight` set
- **THEN** the output spool has `current_weight = initial_weight - used_weight`

#### Scenario: current_weight equals initial_weight when used_weight absent
- **WHEN** a spool record has `initial_weight` but `used_weight` is null or absent
- **THEN** the output spool has `current_weight = initial_weight`

### Requirement: Map price from filament to spool
The script SHALL copy the old filament-level `price` to the spool when the spool has no price of its own.

#### Scenario: price falls through from filament
- **WHEN** a spool record has a null `price` but `filament.price` is set
- **THEN** the output spool has `price` equal to the filament value

#### Scenario: spool price takes precedence
- **WHEN** a spool record has a non-null `price`
- **THEN** the output spool keeps its own `price` regardless of `filament.price`

### Requirement: Map filament weight to spool initial_weight
The script SHALL copy the old `filament.weight` field to `initial_weight` on the spool when the spool has no `initial_weight`.

#### Scenario: initial_weight populated from filament.weight
- **WHEN** a spool record has null `initial_weight` and `filament.weight` is set
- **THEN** the output spool has `initial_weight` equal to `filament.weight`

### Requirement: Accept optional filament export for filaments without spools
The script SHALL accept an optional `--filaments` argument pointing to the old flat filament export JSON, and include those filaments (not already present in the spool export) in the output.

#### Scenario: Orphan filaments included
- **WHEN** `--filaments filaments_export.json` is passed and the filament export contains filaments not referenced by any spool
- **THEN** those filaments appear in the output `filaments` array

#### Scenario: Duplicate filaments not repeated
- **WHEN** a filament ID appears in both the spool export and the filament export
- **THEN** the filament appears only once in the output (spool-export version takes precedence)

### Requirement: Drop fields absent from the current data model
The script SHALL NOT emit fields that no longer exist in the server's `Spool` or `Filament` structs, and SHALL ignore such fields in the input without raising errors.

#### Scenario: spool_weight not emitted on spool
- **WHEN** a spool record has a `spool_weight` field
- **THEN** the output spool does NOT contain `spool_weight`

#### Scenario: extra not emitted on spool
- **WHEN** a spool record has an `extra` field
- **THEN** the output spool does NOT contain `extra`

#### Scenario: used_weight not emitted on spool
- **WHEN** a spool record has a `used_weight` field
- **THEN** the output spool does NOT contain `used_weight`

#### Scenario: Removed filament fields ignored
- **WHEN** spool records contain `filament.article_number`, `filament.external_id`, or `filament.spool_weight`
- **THEN** those fields are not written to any output model and no error is raised

#### Scenario: Removed spool fields ignored
- **WHEN** spool records contain `lot_nr` or `external_id`
- **THEN** those fields are not written to the output spool and no error is raised
