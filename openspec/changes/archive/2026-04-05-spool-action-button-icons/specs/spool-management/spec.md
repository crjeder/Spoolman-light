## MODIFIED Requirements

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
