# filament-management Specification

## Purpose
Defines filament listing, CRUD, material lookup, and SpoolmanDB import behaviour.
## Requirements
### Requirement: List filaments
The system SHALL provide an endpoint to list filaments with server-side filtering, sorting, and pagination.

#### Scenario: List all filaments
- **WHEN** GET /api/v1/filament is called
- **THEN** all filaments are returned with total count in X-Total-Count header

#### Scenario: Filter by material
- **WHEN** GET /api/v1/filament?material=PLA is called
- **THEN** only filaments with material "PLA" are returned

### Requirement: Create filament
The system SHALL allow creating a filament with material specification fields. Color fields SHALL NOT be accepted on the filament create endpoint. net_weight SHALL NOT be accepted on the filament create endpoint.

#### Scenario: Filament created with required fields
- **WHEN** POST /api/v1/filament is called with density and diameter
- **THEN** a new filament is created with a random u32 id and registered set to now

#### Scenario: net_weight field ignored on filament create
- **WHEN** POST /api/v1/filament is called with a net_weight field
- **THEN** the field is ignored (not stored on the filament)

### Requirement: Edit filament
The system SHALL allow updating any filament field: manufacturer, material, material_modifier, diameter, density, print_temp, bed_temp, spool_weight, min/max temps, comment. net_weight SHALL NOT be accepted on filament create or edit endpoints.

#### Scenario: Update filament specs
- **WHEN** PATCH /api/v1/filament/<id> is called with updated fields
- **THEN** the filament is updated; all spools referencing it reflect the new values on next read

#### Scenario: net_weight field ignored on filament edit
- **WHEN** PATCH /api/v1/filament/<id> is called with a net_weight field
- **THEN** the field is ignored (not stored on the filament)

### Requirement: Delete filament
The system SHALL allow deleting a filament. Deletion SHALL be rejected if any spool references the filament.

#### Scenario: Delete unused filament
- **WHEN** DELETE /api/v1/filament/<id> is called and no spool references it
- **THEN** the filament is removed and returns 204

#### Scenario: Delete referenced filament rejected
- **WHEN** DELETE /api/v1/filament/<id> is called and at least one spool references it
- **THEN** the system returns 409 with a message indicating which spools prevent deletion

### Requirement: List materials
The system SHALL provide an endpoint returning the distinct set of material strings across all filaments, for use in filter dropdowns.

#### Scenario: Materials derived from filaments
- **WHEN** GET /api/v1/material is called
- **THEN** a deduplicated list of material strings is returned, sorted alphabetically

### Requirement: Search SpoolmanDB
The system SHALL provide a proxy endpoint that searches the SpoolmanDB filament catalog and returns matching entries. The search is pull-on-demand; no local cache is maintained.

#### Scenario: Search returns matches
- **WHEN** GET /api/v1/filament/search?q=eSun+PLA is called
- **THEN** the backend fetches from SpoolmanDB and returns matching filament entries with all spec fields

#### Scenario: Search fails gracefully
- **WHEN** SpoolmanDB is unreachable
- **THEN** the endpoint returns 503 and the frontend falls back to manual entry

### Requirement: Filament import from SpoolmanDB
The frontend SHALL allow searching SpoolmanDB when creating a filament or spool. Material spec fields SHALL populate the filament form; color fields and net_weight from the search result SHALL populate the spool form.

#### Scenario: Import populates filament and spool forms
- **WHEN** user searches SpoolmanDB and selects a result (e.g., "eSun PLA Blue")
- **THEN** filament form is pre-filled with manufacturer, material, density, diameter, temps; spool form is pre-filled with colors, color_name, and net_weight

#### Scenario: Reuse existing filament on import
- **WHEN** a SpoolmanDB search result matches an existing filament (same manufacturer + material + modifier)
- **THEN** the user is offered the option to link the new spool to the existing filament instead of creating a duplicate

#### Scenario: Manual fallback
- **WHEN** the search returns no results or the user skips search
- **THEN** all filament and spool fields are available for manual entry

### Requirement: Filament list search input
The frontend SHALL provide a text search input on the filament list page labeled "Search" (placeholder text "Search…"). The input SHALL filter the displayed filament rows client-side. A clear ("×") button SHALL appear inside the input when it has a value; clicking it SHALL empty the input and reset the list.

#### Scenario: Search filters filaments
- **WHEN** the user types in the search input
- **THEN** only filaments whose display name contains the typed text (case-insensitive) are shown

#### Scenario: Clear button appears with input
- **WHEN** the search input contains at least one character
- **THEN** a "×" clear button is visible inside the input

#### Scenario: Clear button hidden when empty
- **WHEN** the search input is empty
- **THEN** no clear button is shown

#### Scenario: Clear button resets list
- **WHEN** the user clicks the "×" clear button
- **THEN** the search input is emptied and all filaments are shown

### Requirement: Filament list row action icon buttons
The filament list page SHALL display row-level actions as icon-only buttons using the standard icon set. Each row SHALL have an Edit icon button (✏) and a Delete icon button (🗑). The Delete button SHALL use a two-step inline confirmation: the first click arms it, showing a confirm (🗑) and a cancel (✕) button; the second click on the confirm button executes the deletion. All icon buttons SHALL carry `title` attributes.

#### Scenario: Edit button navigates to filament edit page
- **WHEN** the user clicks the ✏ icon button in a filament row
- **THEN** the browser navigates to `/filaments/:id/edit` for that filament

#### Scenario: Delete button arms on first click
- **WHEN** the user clicks the 🗑 icon button in a filament row
- **THEN** a confirmation state is shown with a confirm 🗑 button and a ✕ cancel button; no deletion occurs yet

#### Scenario: Delete confirmed on second click
- **WHEN** the delete is armed and the user clicks the confirm button
- **THEN** the filament is deleted and removed from the list

#### Scenario: Delete cancelled
- **WHEN** the delete is armed and the user clicks ✕
- **THEN** the row returns to its normal state with no deletion

#### Scenario: No text labels on row action buttons
- **WHEN** the user views the filament list
- **THEN** row action buttons show only icon characters — no "Edit", "Delete", "Sure?", or "Cancel" text is visible

