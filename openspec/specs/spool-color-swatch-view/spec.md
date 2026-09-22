# Spec: Spool Color Swatch View

## Purpose

Provide a colour-first alternative to the spool table: a card grid at `/colors` where each spool is represented by its colour, with a user-configurable default view and shared filters between the two views.

## Requirements

### Requirement: Colour swatch view renders spools as a card grid
The frontend SHALL provide a colour swatch view at the `/colors` route, rendering the spool list as a grid of cards instead of a table. Each card SHALL display the spool's colour(s) as its fill, and SHALL be labelled with the colour name (falling back to the first colour's `#rrggbb` value when `color_name` is absent), the filament display name, the material abbreviation, the remaining weight and the location name. Each card SHALL link to `/spools/{id}`. Archived spools SHALL be visually de-emphasised in the same way archived table rows are.

#### Scenario: Grid renders one card per spool
- **WHEN** the user navigates to `/colors` and the spool list contains 12 spools within the current page size
- **THEN** 12 colour cards are displayed in a grid, and no `table.data-table` is rendered

#### Scenario: Card shows colour name when present
- **WHEN** a spool has `color_name = "Galaxy Black"`
- **THEN** its card displays "Galaxy Black"

#### Scenario: Card falls back to hex when colour name is absent
- **WHEN** a spool has no `color_name` and its first colour is `#1A1A1A`
- **THEN** its card displays `#1A1A1A`

#### Scenario: Card links to the spool detail page
- **WHEN** the user clicks a colour card for the spool with id 42
- **THEN** the application navigates to `/spools/42`

#### Scenario: Archived spool card is de-emphasised
- **WHEN** archived spools are shown and one of them is rendered as a card
- **THEN** that card carries the archived styling used for archived table rows

### Requirement: Multi-colour spools render as hard-stop bands
A card for a spool with a single colour SHALL be filled with that colour. A card for a spool with two to four colours SHALL be filled with equal-width bands of those colours, left to right, with hard stops and no blending. A spool with no stored colours SHALL be filled with the same neutral fallback colour used by the spool table.

#### Scenario: Single colour fills the card
- **WHEN** a spool has exactly one colour
- **THEN** its card is filled with that colour alone

#### Scenario: Two colours split the card in half
- **WHEN** a spool has two colours
- **THEN** its card shows both colours as equal halves with a hard boundary between them

#### Scenario: Four colours split the card in quarters
- **WHEN** a spool has four colours
- **THEN** its card shows four equal bands in stored order

#### Scenario: Colourless spool uses the fallback fill
- **WHEN** a spool has an empty `colors` array
- **THEN** its card is filled with the neutral fallback colour rather than rendering empty

### Requirement: Sidebar offers a Color entry to swap views
The sidebar SHALL contain a "Color" navigation entry pointing at `/colors`, listed alongside the existing entries. The Color entry SHALL appear active when the current path is `/colors`, or when the current path is `/` and `default_view` is `color`. The Spools entry SHALL appear active when the current path is `/spools`, or when the current path is `/` and `default_view` is `spool`.

#### Scenario: Color entry present in the sidebar
- **WHEN** any page is rendered
- **THEN** the sidebar contains a link with text "Color" and href `/colors`

#### Scenario: Color entry active on its own route
- **WHEN** the current path is `/colors`
- **THEN** the Color navigation entry is highlighted as active and the Spools entry is not

#### Scenario: Root path highlights the configured default view
- **WHEN** the current path is `/` and `default_view` is `color`
- **THEN** the Color navigation entry is highlighted as active and the Spools entry is not

### Requirement: Default view is user-configurable via a setting
The Settings page SHALL display a "Default view" selector with options `spool` (default) and `color`. The selected value SHALL be persisted as the `default_view` key via `PUT /api/v1/setting/default_view`. When the key is absent, empty or unrecognised, the system SHALL default to `spool`. The setting is server-global, shared by every browser connecting to the instance.

#### Scenario: Default on first load
- **WHEN** the Settings page loads and no `default_view` key has been saved
- **THEN** the "Default view" selector shows `spool`

#### Scenario: Persisted value pre-populates the selector
- **WHEN** the Settings page loads and `default_view = "color"` has been saved
- **THEN** the "Default view" selector shows `color`

#### Scenario: Saving persists the selected view
- **WHEN** the user selects `color` and saves the Settings form
- **THEN** a `PUT /api/v1/setting/default_view` request is issued with value `"color"`

#### Scenario: Unrecognised stored value falls back to the table
- **WHEN** `default_view` holds a value that is neither `spool` nor `color`
- **THEN** the `/` route renders the spool table

### Requirement: Root route renders the configured default view without redirecting
The `/` route SHALL render the view named by `default_view`, without issuing a redirect and without changing the URL. While the setting has not yet loaded, the `/` route SHALL render the existing loading placeholder rather than rendering one view and replacing it with the other.

#### Scenario: Root renders the swatch grid when configured
- **WHEN** `default_view` is `color` and the user navigates to `/`
- **THEN** the colour card grid is displayed, the URL remains `/`, and no redirect occurs

#### Scenario: Root renders the spool table by default
- **WHEN** no `default_view` has been saved and the user navigates to `/`
- **THEN** the spool table is displayed at `/`

#### Scenario: No view flash before the setting resolves
- **WHEN** the user navigates to `/` and the settings request has not yet completed
- **THEN** a loading placeholder is shown, and neither view is rendered until the setting resolves

### Requirement: Swatch view is ordered by hue then lightness with greys last
The swatch view SHALL default to sorting by the `hue` field, ascending. Under this ordering, spools whose first colour has an OkLab chroma below the achromatic threshold SHALL form a block ordered by ascending lightness, placed after all chromatic spools. Chromatic spools SHALL be ordered by ascending hue angle (normalised to `0..2pi` starting at red), then by ascending lightness within equal hue. A spool's sort key SHALL be derived from its first stored colour; a spool with no colours SHALL use the neutral fallback colour and therefore fall in the achromatic block. Reversing the sort direction SHALL reverse the whole ordering, including the position of the achromatic block.

#### Scenario: Grid defaults to hue order
- **WHEN** the swatch view is opened for the first time and no colour filter level is active
- **THEN** the cards are ordered by ascending hue with the achromatic block last

#### Scenario: Greys, black and white group together
- **WHEN** the spool list contains black, white, three greys and several saturated colours
- **THEN** the black, white and grey cards appear contiguously at the end of the grid, ordered dark to light, rather than interleaved with the saturated colours

#### Scenario: Equal hues break ties by lightness
- **WHEN** two spools share the same hue angle but differ in lightness
- **THEN** the darker spool is ordered first

#### Scenario: Multi-colour spool sorts by its first colour
- **WHEN** a spool stores red followed by grey
- **THEN** it is ordered among the chromatic spools by red's hue, not placed in the achromatic block

#### Scenario: Descending direction reverses the grey block too
- **WHEN** the hue sort direction is set to descending
- **THEN** the achromatic block appears first, ordered light to dark, followed by the chromatic spools in descending hue order

#### Scenario: Active colour filter still takes precedence
- **WHEN** a colour level is active with a valid hex colour while the swatch view is displayed
- **THEN** the cards are ordered by ascending colour distance from the picked colour, as specified by `color-delta-sort`, and not by hue

#### Scenario: Turning the colour level off restores hue order
- **WHEN** the colour level is set to Off in the swatch view
- **THEN** the cards return to hue order

### Requirement: Hue is a sortable field in the spool table
The Color column header in the spool table SHALL be sortable by the `hue` field, using the same ordering rules as the swatch view. Activating it SHALL toggle direction on repeat activation, consistent with the other sortable column headers.

#### Scenario: Sorting the table by hue
- **WHEN** the user activates sorting on the Color column header
- **THEN** the table rows are ordered by ascending hue with the achromatic rows last

#### Scenario: Repeat activation reverses hue direction
- **WHEN** the Color column is already the active sort field ascending and the user activates it again
- **THEN** the table rows are ordered by descending hue

#### Scenario: Colour filter controls remain reachable in the header
- **WHEN** the Color column header is made sortable
- **THEN** the colour threshold selector and colour picker popup in that header continue to operate without triggering a sort

### Requirement: Sort and pagination are per-view, filters are shared
The swatch view and the spool table SHALL keep independent sort field, sort direction, page and page size state, persisted under separate `localStorage` namespaces. The text, material, location, colour and show-archived filters SHALL be shared between the two views, persisted under the existing spool filter session keys, so that switching views preserves the active filter set.

#### Scenario: Grid sort does not disturb table sort
- **WHEN** the table is sorted by Registered descending and the user opens the swatch view, which sorts by hue
- **THEN** returning to the table shows it still sorted by Registered descending

#### Scenario: Filters survive a view swap
- **WHEN** a material filter and a location filter are active in the table and the user opens the swatch view
- **THEN** the same material and location filters are active in the swatch view, restricting the cards shown

#### Scenario: Each view remembers its own page
- **WHEN** the user is on page 3 of the table and switches to the swatch view
- **THEN** the swatch view shows its own remembered page rather than page 3

#### Scenario: Clear filters applies to both views
- **WHEN** filters are cleared from either view
- **THEN** the other view also shows an unfiltered list

### Requirement: Swatch view offers a material checkbox row
The swatch view SHALL display a row of material checkboxes above the card grid, one per distinct material abbreviation present in the unfiltered spool list, sorted alphabetically. Ticking checkboxes SHALL restrict the grid to spools whose filament material is among the ticked set. With no checkbox ticked, spools of every material SHALL be shown. The checkbox row SHALL reflect the shared material filter set, including selections made from the table's Material dropdown.

#### Scenario: Checkbox row lists distinct materials
- **WHEN** the spool list contains materials PLA, PETG and PLA
- **THEN** the checkbox row shows PETG and PLA once each, in alphabetical order

#### Scenario: Ticking two materials shows both
- **WHEN** the user ticks PLA and PETG
- **THEN** the grid shows PLA and PETG spools and hides spools of other materials

#### Scenario: No tick means no material filter
- **WHEN** no material checkbox is ticked
- **THEN** the grid shows spools of every material, including spools whose filament has no material set

#### Scenario: Table dropdown selection is reflected in the checkbox row
- **WHEN** the user selects PETG in the table's Material dropdown and then opens the swatch view
- **THEN** the PETG checkbox is ticked and no other material checkbox is ticked
