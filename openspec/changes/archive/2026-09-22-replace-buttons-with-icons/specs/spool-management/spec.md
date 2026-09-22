## ADDED Requirements

### Requirement: Spool detail view action icon buttons
The spool detail view SHALL display its action buttons (Edit, Clone, Delete) as icon-only buttons using the standard icon set, consistent with the spool list row actions. The Delete button SHALL use a two-step inline confirmation. All icon buttons SHALL carry `title` attributes. Form submit buttons on the create and edit forms ("Create", "Save", "Cancel") are NOT icon buttons and SHALL remain as labelled `.btn` elements.

#### Scenario: Edit button navigates to spool edit page from detail view
- **WHEN** the user clicks the ✏ icon button on the spool detail view
- **THEN** the browser navigates to `/spools/:id/edit`

#### Scenario: Clone button triggers clone action
- **WHEN** the user clicks the ⧉ icon button on the spool detail view
- **THEN** a new spool is created as a clone and the user is navigated to the new spool's page

#### Scenario: Delete button arms on first click in detail view
- **WHEN** the user clicks the 🗑 icon button on the spool detail view
- **THEN** a confirmation state appears with a confirm 🗑 button and a ✕ cancel button; no deletion occurs yet

#### Scenario: Delete confirmed from detail view
- **WHEN** the delete is armed on the detail view and the user clicks the confirm button
- **THEN** the spool is deleted and the browser navigates to the spool list

#### Scenario: No text labels on detail view action buttons
- **WHEN** the user views the spool detail page
- **THEN** the action buttons display only icon characters — no "Edit", "Clone", "Delete", or "Cancel" text is visible
