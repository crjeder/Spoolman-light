## MODIFIED Requirements

### Requirement: Density values are formatted with the browser locale
Density values (g/cm³) displayed in the filament table and detail view SHALL be formatted using `Intl.NumberFormat` with `minimumFractionDigits: 2` and `maximumFractionDigits: 2`, rounding to the second decimal digit. The unit suffix `" g/cm³"` SHALL be appended.

#### Scenario: Density padded to two decimals
- **WHEN** a filament has density `1.2` g/cm³ and the browser locale is `en-US`
- **THEN** the displayed value is `"1.20 g/cm³"`

#### Scenario: Density rounded to two decimals
- **WHEN** a filament has density `1.246` g/cm³ and the browser locale is `en-US`
- **THEN** the displayed value is `"1.25 g/cm³"`

#### Scenario: Density formatted with locale decimal separator
- **WHEN** the browser locale uses a comma as the decimal separator
- **THEN** a density of `1.24` is displayed as `"1,24 g/cm³"`
