# list-filter-persistence Specification

## Purpose
TBD - created by archiving change persist-spool-filters. Update Purpose after archive.
## Requirements
### Requirement: List filters persist for the browser tab session
The spool list and filament list SHALL store their active filter values in `sessionStorage`. On page load or when a list component is re-created (e.g. after navigating to a detail page and back), each filter SHALL initialise from its stored value. Stored values SHALL NOT be read from or written to `localStorage`, so filters do not persist across separate tab sessions.

Persisted spool list filters: text search, material, location, color level, picked color, show-archived.
Persisted filament list filters: text search.

#### Scenario: Filter survives navigation within the app
- **WHEN** the user sets the material filter to "PLA" on the spool list, opens a spool detail page, then returns to the spool list
- **THEN** the material filter is still set to "PLA" and the list is filtered accordingly

#### Scenario: Filter survives a tab reload
- **WHEN** the user has an active text search and color level filter and reloads the page in the same tab
- **THEN** both filters are restored to their previous values and the list reflects them

#### Scenario: Filters do not leak to a new tab session
- **WHEN** the user closes the tab and opens the application in a new tab
- **THEN** all list filters are at their default (inactive) state

#### Scenario: Stored filter value is applied on first render
- **WHEN** `sessionStorage` holds a stored location filter and the spool list mounts
- **THEN** the location dropdown shows the stored location and the list is filtered before any user interaction

### Requirement: Clear filters control resets every filter
The spool list and filament list headers SHALL each contain a "Clear filters" button. The button SHALL be visible only when at least one filter on that list is active (non-default). Clicking it SHALL reset every filter on that list to its default value and clear the corresponding `sessionStorage` entries.

#### Scenario: Button hidden when no filter active
- **WHEN** the spool list loads with all filters at their defaults
- **THEN** the "Clear filters" button is not shown

#### Scenario: Button appears when a filter becomes active
- **WHEN** the user types into the text search or selects a material, location, or color level
- **THEN** the "Clear filters" button becomes visible

#### Scenario: Clicking clears all filters at once
- **WHEN** the user has an active text search, material filter, and color level, and clicks "Clear filters"
- **THEN** the text search is emptied, the material filter returns to "All", the color level returns to "Off", and the full list is shown

#### Scenario: Cleared state does not return after navigation
- **WHEN** the user clicks "Clear filters" and then navigates away and back
- **THEN** all filters remain at their defaults


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
