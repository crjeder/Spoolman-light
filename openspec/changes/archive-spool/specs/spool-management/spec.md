## ADDED Requirements

### Requirement: Archive button in spool UI
The spool list row actions cell and the spool detail page header SHALL each provide an icon-only Archive button for non-archived spools and an icon-only Unarchive button for archived spools. Activating it SHALL call the update endpoint with `archived` set to the opposite of the current value and refresh the view. Unarchiving SHALL NOT require confirmation.

#### Scenario: Archive an empty spool
- **WHEN** the user clicks Archive on a non-archived spool that is empty
- **THEN** the spool is archived immediately and, unless "Show archived" is on, disappears from the list

#### Scenario: Unarchive a spool
- **WHEN** the user clicks Unarchive on an archived spool
- **THEN** the spool is restored to non-archived without confirmation

### Requirement: Warn when archiving a non-empty spool
A spool is empty when its remaining filament (net weight minus used weight) is 0 or less; when net weight is unknown, it is empty when `current_weight` is 0 or less. When the user clicks Archive on a spool that is not empty, the UI SHALL NOT archive immediately. It SHALL show a warning stating the spool still has filament weight and require a second click on a confirm button; a Cancel button SHALL abort. The row actions cell SHALL reserve width so the confirm state does not shift neighbouring buttons.

#### Scenario: Warning on non-empty spool
- **WHEN** the user clicks Archive on a spool with remaining filament > 0
- **THEN** a warning is shown with Confirm and Cancel buttons and the spool is not yet archived

#### Scenario: Warning falls back to current weight
- **WHEN** the user clicks Archive on a spool with no net weight and `current_weight` > 0
- **THEN** a warning is shown with Confirm and Cancel buttons and the spool is not yet archived

#### Scenario: Confirm archive
- **WHEN** the warning is shown and the user clicks Confirm
- **THEN** the spool is archived

#### Scenario: Cancel archive
- **WHEN** the warning is shown and the user clicks Cancel
- **THEN** the spool remains non-archived and the warning is dismissed
