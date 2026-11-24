## ADDED Requirements

### Requirement: Filters reset when the user changes tab or menu item
Selecting any sidebar menu item SHALL remove all persisted list filter values from `sessionStorage` before the target page is shown, so every list opens with all filters at their defaults. Filters SHALL NOT be shared between the Spools and Color tabs. Reloading the page or returning from a detail page SHALL NOT reset filters.

#### Scenario: Color tab filter does not leak into Spools tab
- **WHEN** the user selects material "PLA" on the Color tab and then selects Spools in the menu
- **THEN** the Spools list (including grouped by location) shows the Material filter at "All" and spools of every material

#### Scenario: Round trip through another menu item resets
- **WHEN** the user sets a text search and location filter on Spools, selects Filaments, then selects Spools
- **THEN** the Spools list shows no active filters

#### Scenario: Detail page round trip keeps filters
- **WHEN** the user sets material "PETG", opens a spool detail page, and returns via the browser back button
- **THEN** the material filter is still "PETG"
