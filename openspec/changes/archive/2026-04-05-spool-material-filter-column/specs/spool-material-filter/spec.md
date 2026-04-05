## ADDED Requirements

### Requirement: Material column header contains a filter dropdown
The Material column header SHALL contain a `<select>` dropdown. The dropdown SHALL list "All" as the first option, followed by the distinct material abbreviations present in the full (unfiltered) spool list, sorted alphabetically. Filaments with no material set SHALL NOT appear as a selectable option.

#### Scenario: Dropdown populated with distinct materials
- **WHEN** the spool list contains spools with materials PLA, PETG, and PLA (duplicate)
- **THEN** the dropdown options are: All, PETG, PLA (alphabetical, no duplicates)

#### Scenario: All option present
- **WHEN** the dropdown is rendered
- **THEN** "All" is listed as the first option

#### Scenario: No-material spools excluded from options
- **WHEN** some spools have no material set on their filament
- **THEN** no blank or empty option appears in the dropdown (those spools always pass the filter when "All" is selected)

### Requirement: Selecting a material filters the spool list
When a material is selected in the dropdown, the spool list SHALL show only spools whose filament material abbreviation matches the selected value. Selecting "All" SHALL remove the material filter and show all spools.

#### Scenario: Filter by selected material
- **WHEN** the user selects "PLA" from the material dropdown
- **THEN** only spools with filament material PLA are displayed

#### Scenario: Clear filter by selecting All
- **WHEN** the user selects "All" from the material dropdown
- **THEN** spools of all materials are displayed

#### Scenario: Material filter combines with text search
- **WHEN** a material filter is active and the user types in the text search box
- **THEN** only spools matching both the material filter and the text search are displayed

#### Scenario: Material filter combines with color filter
- **WHEN** a material filter is active and a color filter is active
- **THEN** only spools matching both the material filter and the color filter are displayed
