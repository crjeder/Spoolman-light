## MODIFIED Requirements

### Requirement: Color column header activates color filter
The spool list table's "Color" column header SHALL be interactive. Clicking it SHALL focus the color picker input in the page header, activating the color filter. When a color level other than "Off" is selected, the header SHALL display a filled square indicator (■, U+25A0) to signal that a color filter is active.

#### Scenario: Click color column header focuses color picker
- **WHEN** the user clicks the "Color" column header in the spool table
- **THEN** the color picker input receives focus (browser color popup opens or input is focused)

#### Scenario: Color column header indicates interactivity
- **WHEN** the user hovers over the "Color" column header
- **THEN** the cursor changes to a pointer and a visual hover style is applied to signal it is clickable

#### Scenario: Active filter indicator shown
- **WHEN** the color level selector is set to "Fine", "Medium", or "Coarse"
- **THEN** the "Color" column header SHALL display a ■ character alongside the label "Color"

#### Scenario: Active filter indicator hidden when Off
- **WHEN** the color level selector is set to "Off"
- **THEN** the "Color" column header SHALL display only the label "Color" with no ■ indicator

## REMOVED Requirements

### Requirement: Standalone toolbar color-picker button
**Reason**: The Color column header already activates the picker; a separate toolbar button is redundant and clutters the UI.
**Migration**: Use the Color column header click to open the colour picker.
