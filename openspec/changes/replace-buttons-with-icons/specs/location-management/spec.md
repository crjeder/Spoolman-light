## ADDED Requirements

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
