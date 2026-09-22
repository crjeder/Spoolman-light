# location-management Specification

## Purpose
Defines Location CRUD and how locations are presented and selected in the UI.
## Requirements
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

### Requirement: Location list row action icon buttons
The location management page SHALL display row-level actions as icon-only buttons using the standard icon set. Each location row SHALL have an Edit icon button (✏) and a Delete icon button (🗑). When the row enters inline-edit mode, the edit controls SHALL show a Save icon button (💾) and a Cancel icon button (✕). The Delete button SHALL use a two-step inline confirmation: the first click shows a confirm (🗑) and cancel (✕) button; the second click executes deletion. All icon buttons SHALL carry `title` attributes.

#### Scenario: Edit button enters inline edit mode
- **WHEN** the user clicks the ✏ icon button in a location row
- **THEN** the row switches to inline edit mode showing an editable name field, a 💾 Save button, and a ✕ Cancel button

#### Scenario: Save button commits inline edit
- **WHEN** the user is in inline edit mode and clicks the 💾 Save button
- **THEN** the updated location name is submitted and the row returns to view mode

#### Scenario: Cancel button exits inline edit
- **WHEN** the user is in inline edit mode and clicks the ✕ Cancel button
- **THEN** the row returns to view mode with no change applied

#### Scenario: Delete button arms on first click
- **WHEN** the user clicks the 🗑 icon button in a location row
- **THEN** a confirmation state is shown with a confirm 🗑 button and a ✕ cancel button; no deletion occurs yet

#### Scenario: Delete confirmed on second click
- **WHEN** the delete is armed and the user clicks the confirm button
- **THEN** the location is deleted and removed from the list

#### Scenario: No text labels on row action buttons
- **WHEN** the user views the location management page
- **THEN** row action buttons show only icon characters — no "Edit", "Save", "Delete", "Sure?", or "Cancel" text is visible

