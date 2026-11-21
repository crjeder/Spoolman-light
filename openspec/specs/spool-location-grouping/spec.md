# Spec: Spool Location Grouping

## Purpose

Show the spool table as collapsed per-location rows with entry counts when sorted by Location.

## Requirements

### Requirement: Grouped spool table when sorted by Location
When the spool table is sorted by the Location column, it SHALL render one collapsed row per location instead of one row per spool. Clicking the Location column header SHALL select Location as the sort field (a further click toggles ascending/descending group order). Sorting by any other column SHALL restore the flat table. The colour swatch view SHALL NOT be grouped. Spools without a location SHALL form a single group labelled "No location", ordered last.

#### Scenario: Header click groups the table
- **WHEN** the user clicks the "Location" column header
- **THEN** the table shows one row per location and no individual spool rows

#### Scenario: Other column restores flat table
- **WHEN** the table is grouped and the user clicks another sortable column header
- **THEN** the table shows one row per spool again

#### Scenario: Spools without location
- **WHEN** the table is grouped and some spools have no location
- **THEN** a "No location" group row is shown last, with their count

### Requirement: Group row label and count
Each group row SHALL show in its Location cell a prefix, the location name, a colon and the number of spools in that group after all active filters, in the form `+ Box 3: 7 entries`. A count of one SHALL read `1 entry`. The prefix SHALL be `+` when the group is collapsed and `-` when expanded.

#### Scenario: Collapsed label
- **WHEN** location "Box 3" holds 7 matching spools and the group is collapsed
- **THEN** its row reads "+ Box 3: 7 entries"

#### Scenario: Count reflects filters
- **WHEN** a material filter hides 4 of the 7 spools in "Box 3"
- **THEN** its row reads "+ Box 3: 3 entries"

#### Scenario: Singular
- **WHEN** a location holds one matching spool
- **THEN** its row reads "+ <name>: 1 entry"

### Requirement: Expand and collapse a group
Clicking a group row SHALL toggle that group. An expanded group SHALL show its spools as normal spool rows directly beneath the group row. Groups SHALL start collapsed; expansion state SHALL NOT be persisted.

#### Scenario: Expand
- **WHEN** the user clicks the collapsed row "+ Box 3: 7 entries"
- **THEN** the row reads "- Box 3: 7 entries" and the 7 spool rows appear beneath it

#### Scenario: Collapse
- **WHEN** the user clicks an expanded group row
- **THEN** its spool rows are hidden and the prefix returns to "+"

### Requirement: Active filter values in group rows
The cells of a group row other than the Location cell SHALL be empty unless a filter tied to that column is active, in which case the cell SHALL show the matching filter value: the search text in the Filament cell, the selected material abbreviations (comma separated) in the Material cell, and the picked colour swatch in the Color cell.

#### Scenario: No filters
- **WHEN** no filter is active
- **THEN** every cell of a group row except the Location cell is empty

#### Scenario: Material filter active
- **WHEN** the material filter is set to PLA
- **THEN** each group row shows "PLA" in the Material cell

#### Scenario: Search text active
- **WHEN** the search input contains "red"
- **THEN** each group row shows "red" in the Filament cell

#### Scenario: Colour filter active
- **WHEN** the colour filter level is not "off"
- **THEN** each group row shows the picked colour swatch in the Color cell

### Requirement: Pagination over groups
In grouped mode the pagination unit SHALL be a group row; expanded spool rows SHALL NOT count toward the page size.

#### Scenario: Page size counts groups
- **WHEN** the page size is 10, 15 locations match and one group is expanded
- **THEN** the first page shows 10 group rows plus the expanded group's spool rows
