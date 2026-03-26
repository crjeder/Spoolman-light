## Requirements

### Requirement: Numeric table columns are right-aligned
The application SHALL right-align all table header and data cells that display numeric values in the Spools and Filaments list views. A `.num` CSS utility class with `text-align: right` SHALL be added to the stylesheet and applied to the relevant `<th>` and `<td>` elements.

#### Scenario: Spools list ID column header is right-aligned
- **WHEN** the Spools list page is rendered
- **THEN** the "ID" column header cell SHALL have `text-align: right`

#### Scenario: Spools list ID column data cells are right-aligned
- **WHEN** spool rows are displayed
- **THEN** each ID data cell SHALL have `text-align: right`

#### Scenario: Spools list Remaining% column is right-aligned
- **WHEN** the Spools list page is rendered
- **THEN** the "Remaining%" column header and all its data cells SHALL have `text-align: right`

#### Scenario: Spools list Remaining weight column is right-aligned
- **WHEN** the Spools list page is rendered
- **THEN** the "Remaining (g)" column header and all its data cells SHALL have `text-align: right`

#### Scenario: Filaments list Diameter column is right-aligned
- **WHEN** the Filaments list page is rendered
- **THEN** the "Diameter" column header and all its data cells SHALL have `text-align: right`

#### Scenario: Filaments list Net weight column is right-aligned
- **WHEN** the Filaments list page is rendered
- **THEN** the "Net weight" column header and all its data cells SHALL have `text-align: right`

#### Scenario: Filaments list Density column is right-aligned
- **WHEN** the Filaments list page is rendered
- **THEN** the "Density" column header and all its data cells SHALL have `text-align: right`

#### Scenario: Text columns remain left-aligned
- **WHEN** either list page is rendered
- **THEN** text columns (Filament name, Manufacturer, Material, Color name, Location, Registered, Actions) SHALL NOT have `text-align: right`
