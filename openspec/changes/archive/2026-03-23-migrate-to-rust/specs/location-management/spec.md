## ADDED Requirements

### Requirement: Location CRUD
The system SHALL provide endpoints to create, read, update, and delete Location entities.

#### Scenario: Create location
- **WHEN** POST /api/v1/location is called with a non-empty name
- **THEN** a new location is created with a random u32 id

#### Scenario: Create location with empty name rejected
- **WHEN** POST /api/v1/location is called with an empty name
- **THEN** the system returns 422

#### Scenario: List locations
- **WHEN** GET /api/v1/location is called
- **THEN** all locations are returned sorted by name

#### Scenario: Rename location
- **WHEN** PATCH /api/v1/location/<id> is called with a new name
- **THEN** the location name is updated; all spools with location_id referencing it display the new name on next read

#### Scenario: Delete unused location
- **WHEN** DELETE /api/v1/location/<id> is called and no spool references it
- **THEN** the location is removed and returns 204

#### Scenario: Delete referenced location
- **WHEN** DELETE /api/v1/location/<id> is called and spools reference it
- **THEN** the system returns 409, or optionally unsets location_id on all referencing spools if the user confirms

### Requirement: Location dropdown in spool edit
The frontend spool create and edit forms SHALL include a dropdown to assign a location, populated from the list of Location entities.

#### Scenario: Location assigned to spool
- **WHEN** user selects a location from the dropdown when creating or editing a spool
- **THEN** the spool's location_id is set to the selected location's id

#### Scenario: No location assigned
- **WHEN** user leaves the location dropdown blank
- **THEN** the spool's location_id is null

### Requirement: Location management UI
The frontend SHALL provide a location management page where users can view, create, rename, and delete locations.

#### Scenario: View locations
- **WHEN** user navigates to the locations page
- **THEN** all locations are listed with their names and the count of spools assigned to each

#### Scenario: Delete location with confirmation
- **WHEN** user attempts to delete a location that has spools assigned
- **THEN** the system shows a confirmation dialog before proceeding
