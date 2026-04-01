## ADDED Requirements

### Requirement: Hex label alongside color swatch in detail view
The spool detail view SHALL display the hex color code (e.g., `#ff6a00`) as a text label immediately after each color swatch in the "Colors" field.

#### Scenario: Single color swatch shows hex label
- **WHEN** a spool detail view is rendered and the spool has one color
- **THEN** the Colors field shows the swatch followed by the hex code in `#rrggbb` lowercase format

#### Scenario: Multiple color swatches each show their hex label
- **WHEN** a spool detail view is rendered and the spool has multiple colors
- **THEN** each swatch is followed by its own hex code in `#rrggbb` lowercase format

#### Scenario: Hex label reflects RGBA RGB channels only
- **WHEN** a color has a non-255 alpha value
- **THEN** the hex label still shows only the RGB channels (no alpha channel in the hex string)
