## ADDED Requirements

### Requirement: Convert spool export to spoolman.json
The script SHALL accept a path to the old flat spool export JSON as its primary positional argument and write a valid `spoolman.json` to the specified output path.

#### Scenario: Basic conversion produces valid output
- **WHEN** the script is run with a valid old spool export JSON file
- **THEN** it writes a `spoolman.json` with `meta.schema_version: 2`, a `filaments` array, a `spools` array, and a `settings` object

#### Scenario: Output path configurable via --output flag
- **WHEN** the user passes `--output /path/to/spoolman.json`
- **THEN** the converted file is written to that path

#### Scenario: Output defaults to spoolman.json in current directory
- **WHEN** no `--output` flag is provided
- **THEN** the script writes to `./spoolman.json`

### Requirement: Reconstruct filaments from spool export
The script SHALL extract unique filaments from the flat spool records (keyed by `filament.id`) and write them as the `filaments` array.

#### Scenario: Duplicate filament IDs are deduplicated
- **WHEN** multiple spool records share the same `filament.id`
- **THEN** only one `FilamentModel` entry is written, using data from the first occurrence

#### Scenario: Vendor name is inlined as string
- **WHEN** a spool record has a `filament.vendor.name` key
- **THEN** the filament's `vendor` field is set to that string value

#### Scenario: Filament with no vendor
- **WHEN** a spool record has no `filament.vendor.name` key (or it is null)
- **THEN** the filament's `vendor` field is set to `null`

### Requirement: Map color fields from filament to spool
The script SHALL copy color fields from the old filament-level keys to the corresponding spool record.

#### Scenario: color_hex moved to spool
- **WHEN** a spool record has `filament.color_hex` set and the spool's own `color_hex` is absent or null
- **THEN** the output spool has `color_hex` equal to the filament value

#### Scenario: multi_color_hexes moved to spool
- **WHEN** a spool record has `filament.multi_color_hexes` set
- **THEN** the output spool has `multi_color_hexes` equal to that value

#### Scenario: multi_color_direction moved to spool
- **WHEN** a spool record has `filament.multi_color_direction` set
- **THEN** the output spool has `multi_color_direction` equal to that value

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

### Requirement: Drop removed fields silently
The script SHALL ignore old fields that no longer exist in the new schema without raising errors.

#### Scenario: Removed filament fields ignored
- **WHEN** spool records contain `filament.article_number`, `filament.external_id`, or `filament.spool_weight`
- **THEN** those fields are not written to any output model and no error is raised

#### Scenario: Removed spool fields ignored
- **WHEN** spool records contain `lot_nr` or `external_id`
- **THEN** those fields are not written to the output spool and no error is raised

### Requirement: Preserve extra fields
The script SHALL pass through the `extra` dict on both filaments and spools unchanged.

#### Scenario: Extra fields round-trip
- **WHEN** a spool or filament record has an `extra` object
- **THEN** the output model contains the same key-value pairs under `extra`

#### Scenario: Missing extra defaults to empty dict
- **WHEN** a record has no `extra` key
- **THEN** the output model has `extra: {}`
