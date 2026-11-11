## ADDED Requirements

### Requirement: Filament import from slicer profile
The New Filament form SHALL provide an "Import profile" button that lets the user select an OrcaSlicer filament profile (`.json`) from disk. The file SHALL be read and parsed in the browser only; on success the form fields manufacturer, material, modifier, diameter, density, print temp and bed temp SHALL be pre-filled from the profile where the profile provides a usable value. Fields the profile does not provide SHALL keep their current value. Import SHALL NOT create the filament; the user submits the form as usual.

Profile field mapping (first element of each array value):
- `filament_vendor` -> manufacturer
- `filament_type` -> material
- `name`, with any ` @...` suffix and leading vendor name removed, and a trailing material abbreviation removed -> modifier
- `filament_diameter` -> diameter (ignored when uniform diameter mode is on)
- `filament_density` -> density
- `nozzle_temperature` -> print temp
- the `*_plate_temp` entry matching `bed_type` (fallback `hot_plate_temp`) -> bed temp

Values that are missing, empty, `"nil"` or non-numeric (for numeric fields) SHALL be ignored.

#### Scenario: Import populates the form
- **WHEN** the user selects the Polymaker HT-PLA Orca profile (vendor `Polymaker`, type `PLA`, name `Polymaker HT-PLA @User`, density `1.28`, diameter `1.75`, nozzle temperature `220`, bed type `Cool Plate`, cool plate temp `60`)
- **THEN** the form shows manufacturer `Polymaker`, material `PLA`, modifier `HT`, diameter `1.75`, density `1.28`, print temp `220`, bed temp `60`

#### Scenario: Imported values stay editable
- **WHEN** a profile has been imported
- **THEN** the user can change any field before clicking Create, and no filament exists until Create is clicked

#### Scenario: Missing values are skipped
- **WHEN** the profile has `"nil"` or no value for a field
- **THEN** that form field keeps its previous value

#### Scenario: Invalid file rejected
- **WHEN** the selected file is not valid JSON or has none of the keys `filament_type`, `filament_vendor`, `filament_density`
- **THEN** an inline error is shown and the form is left unchanged
