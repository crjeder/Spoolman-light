## MODIFIED Requirements

### Requirement: Material column header contains a filter dropdown
The Material column header SHALL contain a `<select>` dropdown. The dropdown SHALL list "All" as the first option, followed by the distinct material abbreviations present in the full (unfiltered) spool list, sorted alphabetically. Filaments with no material set SHALL NOT appear as a selectable option. When the shared material filter set holds more than one material, the dropdown SHALL additionally render a selected `"Multiple (n)"` option so that an active filter is never displayed as "All".

#### Scenario: Dropdown populated with distinct materials
- **WHEN** the spool list contains spools with materials PLA, PETG, and PLA (duplicate)
- **THEN** the dropdown options are: All, PETG, PLA (alphabetical, no duplicates)

#### Scenario: All option present
- **WHEN** the dropdown is rendered
- **THEN** "All" is listed as the first option

#### Scenario: No-material spools excluded from options
- **WHEN** some spools have no material set on their filament
- **THEN** no blank or empty option appears in the dropdown (those spools always pass the filter when no material is selected)

#### Scenario: Multi-material selection is shown as Multiple
- **WHEN** PLA and PETG are both selected in the shared material filter set
- **THEN** the dropdown shows a selected `"Multiple (2)"` option rather than "All"

#### Scenario: Picking a concrete material replaces a multi-material selection
- **WHEN** the filter set holds PLA and PETG and the user selects PLA from the dropdown
- **THEN** the filter set holds PLA alone and the `"Multiple (n)"` option is no longer rendered

### Requirement: Selecting a material filters the spool list
The material filter SHALL be a set of selected material abbreviations, shared by the spool table and the colour swatch view. The spool list SHALL show only spools whose filament material abbreviation is a member of the set. An empty set SHALL impose no material restriction. Selecting "All" in the table dropdown SHALL empty the set; selecting a concrete material SHALL replace the set with that single material.

#### Scenario: Filter by selected material
- **WHEN** the user selects "PLA" from the material dropdown
- **THEN** only spools with filament material PLA are displayed

#### Scenario: Filter by two materials
- **WHEN** the material filter set holds PLA and PETG
- **THEN** spools of both PLA and PETG are displayed and spools of other materials are hidden

#### Scenario: Clear filter by selecting All
- **WHEN** the user selects "All" from the material dropdown
- **THEN** the set is emptied and spools of all materials are displayed

#### Scenario: Material filter combines with text search
- **WHEN** a material filter is active and the user types in the text search box
- **THEN** only spools matching both the material filter and the text search are displayed

#### Scenario: Material filter combines with color filter
- **WHEN** a material filter is active and a color filter is active
- **THEN** only spools matching both the material filter and the color filter are displayed

### Requirement: Material filter selection persists for the session
The selected material set SHALL be stored in `sessionStorage` under the existing `filter.spools.material` key as a comma-separated list, and restored when either view is re-created or reloaded within the same tab session. A stored value holding a single material abbreviation with no separator SHALL be read back as a one-element set, and an empty stored value as an empty set. The "Clear filters" control SHALL empty the set and remove its stored value.

#### Scenario: Selected material restored after navigation
- **WHEN** the user selects "PETG" and navigates away from the spool list and back
- **THEN** the Material dropdown shows "PETG" and only PETG spools are displayed

#### Scenario: Multi-material selection restored after navigation
- **WHEN** the user ticks PLA and PETG in the swatch view and navigates away and back
- **THEN** both materials are still selected and the grid shows PLA and PETG spools

#### Scenario: Value stored by an earlier version is still honoured
- **WHEN** `filter.spools.material` holds the single value `"PLA"` written before this change
- **THEN** it is read back as a one-element set and only PLA spools are displayed

#### Scenario: Clear filters resets the material dropdown
- **WHEN** a material filter is active and the user clicks "Clear filters"
- **THEN** the Material dropdown returns to "All", every material checkbox is unticked, and spools of every material are displayed
