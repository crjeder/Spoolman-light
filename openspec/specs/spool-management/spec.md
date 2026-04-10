## ADDED Requirements

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

### Requirement: Create spool
The system SHALL allow creating a new spool by providing filament_id, colors, initial_weight, and optionally location_id, color_name, net_weight, comment.

#### Scenario: Spool created successfully
- **WHEN** POST /api/v1/spool is called with valid filament_id and initial_weight
- **THEN** a new spool is created with a random u32 id, current_weight set to initial_weight, and registered set to now

#### Scenario: Spool created with net_weight
- **WHEN** POST /api/v1/spool is called with net_weight provided
- **THEN** the spool stores the net_weight and weight percentage metrics are included in the response

#### Scenario: Filament not found
- **WHEN** POST /api/v1/spool is called with a non-existent filament_id
- **THEN** the system returns 404

### Requirement: Edit spool
The system SHALL allow updating any mutable spool field: colors, color_name, location_id, current_weight, net_weight, first_used, last_used, comment.

#### Scenario: Update current weight
- **WHEN** PATCH /api/v1/spool/<id> is called with a new current_weight
- **THEN** the spool's current_weight is updated and last_used is set to now

#### Scenario: Update net weight
- **WHEN** PATCH /api/v1/spool/<id> is called with a new net_weight
- **THEN** the spool's net_weight is updated and weight percentage metrics reflect the new value

#### Scenario: Spool not found
- **WHEN** PATCH /api/v1/spool/<id> is called with a non-existent id
- **THEN** the system returns 404

### Requirement: Derive weight metrics
The system SHALL derive and return weight metrics in spool responses. Metrics are computed from stored fields, never stored redundantly. `net_weight` is read from the spool, not from the filament. The derived price metric SHALL be named `price_per_kg` and SHALL equal `spool.price / (net_weight_grams / 1000.0)`, using `net_weight` when available and falling back to `initial_weight`.

#### Scenario: Used weight derived
- **WHEN** a spool is retrieved
- **THEN** `used_weight = initial_weight - current_weight` is included in the response

#### Scenario: Remaining filament derived when net_weight known
- **WHEN** a spool is retrieved and the spool has a `net_weight`
- **THEN** `remaining_filament = spool.net_weight - used_weight` is included in the response

#### Scenario: No remaining_filament when net_weight absent
- **WHEN** a spool is retrieved and `spool.net_weight` is `None`
- **THEN** `remaining_filament` is omitted (`None`) from the response

#### Scenario: Price per kg derived when price and weight known
- **WHEN** a spool has `price = 20.0` and `net_weight = 1000 g`
- **THEN** `price_per_kg = 20.0` is included in the response

#### Scenario: Price per kg uses initial_weight fallback
- **WHEN** a spool has `price = 20.0` and no `net_weight` and `initial_weight = 500 g`
- **THEN** `price_per_kg = 40.0` (20.0 / 0.5 kg) is included in the response

#### Scenario: Price per kg absent when no price set
- **WHEN** a spool has no price
- **THEN** `price_per_kg` is `null` in the response

### Requirement: Spool detail view displays price, price per kg, and net weight
The spool detail view SHALL display the following read-only fields: `net_weight` (formatted as grams), purchase `price` (formatted as currency), and `price_per_kg` (formatted as currency). When any field is absent (`None`), it SHALL display as `"—"`.

#### Scenario: Net weight shown when present
- **WHEN** a spool has `net_weight = 1000` g
- **THEN** the detail view shows `"Net weight"` row with value `"1,000 g"` (locale-formatted)

#### Scenario: Net weight shows dash when absent
- **WHEN** a spool has no `net_weight`
- **THEN** the detail view shows `"Net weight"` row with value `"—"`

#### Scenario: Price shown with locale currency symbol
- **WHEN** a spool has `price = 24.99` and `currency_symbol` is `"€"` and the browser locale is `de-DE`
- **THEN** the detail view shows `"Price"` row with value `"24,99 €"`

#### Scenario: Price per kg shown with locale currency symbol
- **WHEN** a spool has `price = 20.0` and `net_weight = 1000 g` and `currency_symbol` is `"$"` and browser locale is `en-US`
- **THEN** the detail view shows `"Price/kg"` row with value `"$20.00"`

### Requirement: Clone spool
The system SHALL allow cloning an existing spool to create a new spool pre-filled with the same filament_id, colors, color_name, net_weight, and initial_weight.

#### Scenario: Clone creates new spool
- **WHEN** POST /api/v1/spool/<id>/clone is called
- **THEN** a new spool is created with a new random id, same filament, color, and net_weight; current_weight set to initial_weight; registered set to now

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
The frontend SHALL provide a spool list page with sortable columns, server-side filtering, pagination, and column visibility toggle. Archived spools SHALL be togglable via a button. The spool list page SHALL be the default landing page of the application, rendered at both `"/"` and `"/spools"`. The `"/"` route SHALL render the spool list component directly without a redirect. The Spools navigation link SHALL appear active when the current path is either `"/"` or `"/spools"`. The page SHALL include a text search input labeled "Search" (placeholder "Search…") that filters rows client-side. A clear ("×") button SHALL appear inside the search input when it has a value; clicking it SHALL empty the input and reset the list. The table SHALL NOT include a column displaying the internal spool ID. Each row SHALL have an actions cell containing three icon buttons: View (navigates to the spool detail page), Edit (navigates to the spool edit page), and Delete (initiates inline confirmation). Icon buttons SHALL use icon characters or inline SVG — no text labels. The Delete button SHALL use a two-step confirmation: the first click arms it, the second click executes the delete; a Cancel button SHALL disarm it.

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

#### Scenario: Search filters spools
- **WHEN** the user types in the search input
- **THEN** only spools whose display name contains the typed text (case-insensitive) are shown

#### Scenario: Clear button appears with input
- **WHEN** the search input contains at least one character
- **THEN** a "×" clear button is visible inside the input

#### Scenario: Clear button hidden when empty
- **WHEN** the search input is empty
- **THEN** no clear button is shown

#### Scenario: Clear button resets list
- **WHEN** the user clicks the "×" clear button
- **THEN** the search input is emptied and all spools are shown

#### Scenario: ID column not shown in spool list
- **WHEN** the spool list table is rendered
- **THEN** no column with the internal spool ID is present in the table

#### Scenario: View button navigates to detail page
- **WHEN** the user clicks the View icon button in a spool row
- **THEN** the browser navigates to `/spools/:id` for that spool

#### Scenario: Edit button navigates to edit page
- **WHEN** the user clicks the Edit icon button in a spool row
- **THEN** the browser navigates to `/spools/:id/edit` for that spool

#### Scenario: Delete button arms on first click
- **WHEN** the user clicks the Delete icon button in a spool row
- **THEN** a confirmation state is shown with a confirm icon/button and a Cancel button; no deletion occurs yet

#### Scenario: Delete confirmed on second click
- **WHEN** the delete is armed and the user clicks the confirm button
- **THEN** the spool is deleted and the row is removed from the list

#### Scenario: Delete cancelled
- **WHEN** the delete is armed and the user clicks Cancel
- **THEN** the row returns to its normal state with no deletion

#### Scenario: Action buttons show no text labels
- **WHEN** the spool list table is rendered
- **THEN** the actions column buttons display only icons (no "Edit", "Delete", or "View" text)

### Requirement: Location required in spool forms
The spool create and edit forms SHALL require a location to be selected before submission. The submit button SHALL be disabled when no location is selected.

#### Scenario: Submit disabled without location on create
- **WHEN** the spool create form is open and no location is selected
- **THEN** the submit button is disabled

#### Scenario: Submit enabled after location selected on create
- **WHEN** the user selects a location in the spool create form
- **THEN** the submit button becomes enabled

#### Scenario: Submit disabled without location on edit
- **WHEN** the spool edit form is open and no location is selected
- **THEN** the submit button is disabled

#### Scenario: Submit enabled after location selected on edit
- **WHEN** the user selects a location in the spool edit form
- **THEN** the submit button becomes enabled

### Requirement: Date-only timestamp display
The frontend SHALL display all spool timestamps (`registered`, `first_used`, `last_used`) as date-only values in `YYYY-MM-DD` format. Time-of-day SHALL NOT be shown anywhere in the spool UI.

#### Scenario: Registered date shown without time
- **WHEN** a spool's detail panel is open
- **THEN** `registered` is displayed as `YYYY-MM-DD` with no time component

#### Scenario: First used shown without time
- **WHEN** a spool has a `first_used` value and the detail panel is open
- **THEN** `first_used` is displayed as `YYYY-MM-DD` with no time component

#### Scenario: Last used shown without time
- **WHEN** a spool has a `last_used` value and the detail panel is open
- **THEN** `last_used` is displayed as `YYYY-MM-DD` with no time component

### Requirement: Date-only edit inputs for spool timestamps
The spool edit form SHALL use date-only inputs (not datetime-local) for `first_used` and `last_used`. When those values are submitted to the API, the time component SHALL be fixed to `00:05:00 UTC`.

#### Scenario: Edit form shows date picker for first_used
- **WHEN** the spool edit form is open
- **THEN** the `first_used` field is a date input (not a datetime-local input)

#### Scenario: Edit form shows date picker for last_used
- **WHEN** the spool edit form is open
- **THEN** the `last_used` field is a date input (not a datetime-local input)

#### Scenario: Submitted date gets fixed time component
- **WHEN** the user sets `first_used` to `2024-03-15` and saves
- **THEN** the API receives `2024-03-15T00:05:00Z` for `first_used`

#### Scenario: Existing datetime populates date field
- **WHEN** a spool with `first_used = 2024-03-15T14:32:00Z` is loaded into the edit form
- **THEN** the date input shows `2024-03-15` (date only, time discarded)

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
