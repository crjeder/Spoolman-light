## MODIFIED Requirements

### Requirement: List spools
The system SHALL provide an endpoint to list spools with server-side filtering, sorting, and pagination. Archived spools SHALL be excluded by default and included only when explicitly requested. The client SHALL support filtering by location by passing `location_id` to the list endpoint.

#### Scenario: List active spools
- **WHEN** GET /api/v1/spool is called without parameters
- **THEN** the response contains all non-archived spools with total count in X-Total-Count header

#### Scenario: List including archived
- **WHEN** GET /api/v1/spool?allow_archived=true is called
- **THEN** the response includes both archived and non-archived spools

#### Scenario: Filter by filament
- **WHEN** GET /api/v1/spool?filament_id=<id> is called
- **THEN** only spools with that filament_id are returned

#### Scenario: Filter by location
- **WHEN** GET /api/v1/spool?location_id=<id> is called
- **THEN** only spools with that location_id are returned

#### Scenario: Sort by field
- **WHEN** GET /api/v1/spool?sort=registered&order=desc is called
- **THEN** spools are returned sorted by registered date descending

## ADDED Requirements

### Requirement: Location filter on spool list page
The frontend spool list page SHALL display a location filter dropdown populated from the locations API. Selecting a location reloads the spool list showing only spools in that location. Selecting the empty option clears the filter.

#### Scenario: No filter selected
- **WHEN** user opens the spool list page without selecting a location filter
- **THEN** all non-archived spools are displayed

#### Scenario: Filter by location
- **WHEN** user selects a location from the filter dropdown
- **THEN** the spool list reloads showing only spools with that location_id

#### Scenario: Clear location filter
- **WHEN** user selects the empty option in the location filter dropdown
- **THEN** the spool list reloads showing all non-archived spools

### Requirement: Location name in spool list table
The spool list table Location column SHALL display the location name rather than the raw location_id integer. The name SHALL be resolved from the locations list fetched on page load.

#### Scenario: Location name shown
- **WHEN** a spool has a location_id and locations have been loaded
- **THEN** the Location column shows the location name

#### Scenario: No location assigned
- **WHEN** a spool has no location_id
- **THEN** the Location column shows an empty value or dash
