## MODIFIED Requirements

### Requirement: Colour swatch view renders spools as a card grid
The frontend SHALL provide a colour swatch view at the `/colors` route, rendering the spool list as a grid of cards instead of a table. Each card SHALL display the spool's colour(s) as its fill, and SHALL be labelled with the colour name (falling back to the first colour's `#rrggbb` value when `color_name` is absent), the filament display name, the material abbreviation, the remaining weight and the location name. Each card SHALL link to `/spools/{id}`. Archived spools SHALL be visually de-emphasised in the same way archived table rows are. When a spool has a `swatch_image` reference, its card SHALL use the stored swatch image as its fill instead of the flat colour fill.

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

#### Scenario: Card uses swatch image when present
- **WHEN** a spool has a `swatch_image` reference
- **THEN** its card is filled with the stored swatch image instead of the flat colour fill

#### Scenario: Card without swatch image uses flat colour fill
- **WHEN** a spool has no `swatch_image` reference
- **THEN** its card renders the flat colour fill as before this feature existed
