## ADDED Requirements

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
