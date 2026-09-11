## ADDED Requirements

### Requirement: Filament selector disabled when search yields no results
When the SpoolmanDB search input on the Spool Create form is non-empty and returns zero results, the system SHALL disable the filament dropdown and SHALL display an explanatory label. When the search input is empty or returns at least one result, the filament dropdown SHALL be enabled.

#### Scenario: No results disables filament dropdown
- **WHEN** the user has typed a non-empty query into the SpoolmanDB search input on the Spool Create form and the query matches zero entries
- **THEN** the filament dropdown is disabled and a label reads "Not found in database -- filament selection disabled"

#### Scenario: Empty query leaves filament dropdown enabled
- **WHEN** the SpoolmanDB search input is empty
- **THEN** the filament dropdown is enabled and no explanatory label is shown

#### Scenario: Results present leaves filament dropdown enabled
- **WHEN** the SpoolmanDB search input is non-empty and matches at least one entry
- **THEN** the filament dropdown is enabled

#### Scenario: Clearing the query re-enables the filament dropdown
- **WHEN** the user clears the SpoolmanDB search input after it had no results
- **THEN** the filament dropdown becomes enabled again

### Requirement: Search text pre-fills spool name when no results found
When the SpoolmanDB search on Spool Create yields no results and the search input is non-empty, the system SHALL copy the search input text into the spool name field. The spool name field SHALL remain editable. Clearing the search input SHALL NOT automatically clear the name field.

#### Scenario: Search text pre-fills name
- **WHEN** the user types "Sunlu Matte Black" and no SpoolmanDB entries match
- **THEN** the spool name field is set to "Sunlu Matte Black"

#### Scenario: Name field remains editable after pre-fill
- **WHEN** the spool name field has been pre-filled from the search query
- **THEN** the user can still modify the name field freely before submitting

#### Scenario: Clearing search does not clear name
- **WHEN** the user clears the SpoolmanDB search input after the name was pre-filled
- **THEN** the name field retains its last pre-filled value
