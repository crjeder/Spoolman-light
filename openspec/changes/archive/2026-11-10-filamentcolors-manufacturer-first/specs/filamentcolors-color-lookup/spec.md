## MODIFIED Requirements

### Requirement: filamentcolors.xyz search panel on spool create/edit
The system SHALL display an inline "Search filamentcolors.xyz" panel on the Spool Create and Spool Edit forms, alongside the existing SpoolmanDB search panel. The panel SHALL contain a text input and, as the user types, SHALL filter the cached swatch list client-side and display up to 10 matching results, each showing a thumbnail, manufacturer, and color name. The panel is optional: the form SHALL remain fully usable without ever opening or using it.

The filter SHALL split the query on whitespace into tokens and match a swatch when every token is a case-insensitive substring of the swatch's manufacturer or color name (checked independently per token, so a query combining both fields, e.g. `<manufacturer> <color>`, matches swatches where each word is found in either field).

When the spool's filament has a non-empty manufacturer (on Spool Create: the currently selected filament; on Spool Edit: the spool's filament), the system SHALL list matching swatches whose manufacturer matches the filament's manufacturer before all other matching swatches. A swatch manufacturer matches when, compared case-insensitively after trimming, either name contains the other. Within each group the original swatch order SHALL be preserved, and the 10-result limit SHALL apply after ordering, so other manufacturers' swatches fill remaining slots. When no filament is selected or its manufacturer is empty, results SHALL be listed in the original swatch order.

#### Scenario: Panel is present on Spool Create
- **WHEN** the user navigates to the New Spool page
- **THEN** a "Search filamentcolors.xyz" panel is visible

#### Scenario: Panel is present on Spool Edit
- **WHEN** the user navigates to the Edit Spool page
- **THEN** a "Search filamentcolors.xyz" panel is visible

#### Scenario: Typing filters results
- **WHEN** the user types a query into the panel's search input
- **THEN** up to 10 matching swatches are listed, each with a thumbnail, manufacturer, and color name

#### Scenario: Combined manufacturer and color query matches
- **WHEN** the user types `Prusament Galaxy Black` (manufacturer and color name as a single query)
- **THEN** swatches whose manufacturer contains "Prusament" and whose color name contains "Galaxy" and "Black" are listed

#### Scenario: Filament manufacturer's swatches listed first
- **WHEN** the selected filament's manufacturer is `Polymaker` and the user types `Black`
- **THEN** all matching Polymaker swatches are listed before matching swatches from any other manufacturer

#### Scenario: Other manufacturers fill remaining slots
- **WHEN** the selected filament's manufacturer has 3 swatches matching the query and other manufacturers have 20
- **THEN** the 3 manufacturer swatches are listed first, followed by 7 swatches from other manufacturers

#### Scenario: Ranking follows filament selection
- **WHEN** the user changes the selected filament on the New Spool page to one with a different manufacturer
- **THEN** the listed results are re-ordered to put the new manufacturer's swatches first

#### Scenario: No manufacturer keeps original order
- **WHEN** the selected filament has no manufacturer
- **THEN** matching swatches are listed in the original swatch order

#### Scenario: No matches shows empty state
- **WHEN** the query matches no cached swatches
- **THEN** a "No results" message is displayed

#### Scenario: Form usable without the panel
- **WHEN** the user fills in and submits the spool form without ever interacting with the filamentcolors.xyz panel
- **THEN** the spool is created or updated exactly as before this feature existed
