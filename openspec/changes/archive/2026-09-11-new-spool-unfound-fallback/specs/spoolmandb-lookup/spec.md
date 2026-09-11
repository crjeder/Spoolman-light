## MODIFIED Requirements

### Requirement: SpoolmanDB search panel appears on filament and spool create/edit forms
The system SHALL display an inline "Search filament database" panel above the form fields on the Filament Create, Filament Edit, and Spool Create pages. The panel SHALL contain a text input. The panel MAY be collapsed/hidden by default and expanded by the user. The component SHALL accept an optional `on_search_state` callback; when provided, the component SHALL invoke it with `(current_query: String, has_results: bool)` whenever the query or the result list changes.

#### Scenario: Panel is present on Filament Create
- **WHEN** the user navigates to the New Filament page
- **THEN** a SpoolmanDB search panel is visible above the form fields

#### Scenario: Panel is present on Filament Edit
- **WHEN** the user navigates to the Edit Filament page
- **THEN** a SpoolmanDB search panel is visible above the form fields

#### Scenario: Panel is present on Spool Create
- **WHEN** the user navigates to the New Spool page
- **THEN** a SpoolmanDB search panel is visible above the form fields

#### Scenario: on_search_state called with empty query
- **WHEN** the search input is empty
- **THEN** `on_search_state` is called with `("", false)` if provided

#### Scenario: on_search_state called with results
- **WHEN** the search input is non-empty and at least one entry matches
- **THEN** `on_search_state` is called with `(query, true)` if provided

#### Scenario: on_search_state called with no results
- **WHEN** the search input is non-empty and no entries match
- **THEN** `on_search_state` is called with `(query, false)` if provided

#### Scenario: on_search_state not provided
- **WHEN** no `on_search_state` callback is passed to the component
- **THEN** the component behaves identically to its previous behavior with no errors
