## ADDED Requirements

### Requirement: List spools
The system SHALL provide an endpoint to list spools with server-side filtering, sorting, and pagination. Archived spools SHALL be excluded by default and included only when explicitly requested.

#### Scenario: List active spools
- **WHEN** GET /api/v1/spool is called without parameters
- **THEN** the response contains all non-archived spools with total count in X-Total-Count header

#### Scenario: List including archived
- **WHEN** GET /api/v1/spool?allow_archived=true is called
- **THEN** the response includes both archived and non-archived spools

#### Scenario: Filter by filament
- **WHEN** GET /api/v1/spool?filament_id=<id> is called
- **THEN** only spools with that filament_id are returned

#### Scenario: Sort by field
- **WHEN** GET /api/v1/spool?sort=registered&order=desc is called
- **THEN** spools are returned sorted by registered date descending

### Requirement: Create spool
The system SHALL allow creating a new spool by providing filament_id, colors, initial_weight, and optionally location_id, color_name, comment.

#### Scenario: Spool created successfully
- **WHEN** POST /api/v1/spool is called with valid filament_id and initial_weight
- **THEN** a new spool is created with a random u32 id, current_weight set to initial_weight, and registered set to now

#### Scenario: Filament not found
- **WHEN** POST /api/v1/spool is called with a non-existent filament_id
- **THEN** the system returns 404

### Requirement: Edit spool
The system SHALL allow updating any mutable spool field: colors, color_name, location_id, current_weight, first_used, last_used, comment.

#### Scenario: Update current weight
- **WHEN** PATCH /api/v1/spool/<id> is called with a new current_weight
- **THEN** the spool's current_weight is updated and last_used is set to now

#### Scenario: Spool not found
- **WHEN** PATCH /api/v1/spool/<id> is called with a non-existent id
- **THEN** the system returns 404

### Requirement: Derive weight metrics
The system SHALL derive and return weight metrics in spool responses. Metrics are computed from stored fields, never stored redundantly.

#### Scenario: Used weight derived
- **WHEN** a spool is retrieved
- **THEN** used_weight = initial_weight - current_weight is included in the response

#### Scenario: Remaining filament derived when net_weight known
- **WHEN** a spool is retrieved and its filament has a net_weight
- **THEN** remaining_filament = net_weight - used_weight is included in the response

#### Scenario: Remaining percentage derived
- **WHEN** a spool is retrieved and its filament has a net_weight
- **THEN** remaining_pct = remaining_filament / net_weight × 100 is included in the response

### Requirement: Clone spool
The system SHALL allow cloning an existing spool to create a new spool pre-filled with the same filament_id, colors, color_name, and initial_weight.

#### Scenario: Clone creates new spool
- **WHEN** POST /api/v1/spool/<id>/clone is called
- **THEN** a new spool is created with a new random id, same filament and color, current_weight set to initial_weight, registered set to now

### Requirement: Archive and unarchive spool
The system SHALL allow marking a spool as archived (no longer in active use) and restoring it.

#### Scenario: Archive spool
- **WHEN** PATCH /api/v1/spool/<id> is called with archived=true
- **THEN** the spool is marked archived and excluded from default list results

#### Scenario: Unarchive spool
- **WHEN** PATCH /api/v1/spool/<id> is called with archived=false
- **THEN** the spool is marked non-archived and included in default list results

### Requirement: Delete spool
The system SHALL allow permanently deleting a spool.

#### Scenario: Delete spool
- **WHEN** DELETE /api/v1/spool/<id> is called
- **THEN** the spool is removed from the store and returns 204

### Requirement: NFC tag URL
Each spool SHALL be addressable at a stable URL suitable for use as the OpenTag3D Online Data URL field (stored without https:// prefix).

#### Scenario: Spool API URL
- **WHEN** an NFC tag is written for a spool with id 12345
- **THEN** the Online Data URL field contains "<host>/api/v1/spool/12345"

### Requirement: Spool list UI
The frontend SHALL provide a spool list page with sortable columns, server-side filtering, pagination, and column visibility toggle. Archived spools SHALL be togglable via a button. The spool list page SHALL be the default landing page of the application, rendered at both `"/"` and `"/spools"`. The `"/"` route SHALL render the spool list component directly without a redirect. The Spools navigation link SHALL appear active when the current path is either `"/"` or `"/spools"`.

#### Scenario: Default list shows active spools
- **WHEN** the spool list page loads
- **THEN** only non-archived spools are shown with sensible default sort (by registered date, descending)

#### Scenario: Table state is optionally persisted
- **WHEN** the user changes sort or filter and returns to the page
- **THEN** the previous state is restored from localStorage if persistence is enabled

#### Scenario: Root path renders spool list
- **WHEN** the user navigates to `"/"`
- **THEN** the spool list is displayed without a redirect

#### Scenario: Nav link is active at root path
- **WHEN** the current path is `"/"`
- **THEN** the Spools navigation link is highlighted as active
