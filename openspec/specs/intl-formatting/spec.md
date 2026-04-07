# Spec: Intl Formatting

## Purpose

Defines how numeric values (weight, density), dates, and currency amounts are formatted in the UI using the browser's `Intl` APIs, so that displayed values respect the user's locale and regional conventions.

---

## Requirements

### Requirement: Weight values are formatted with the browser locale
All weight values displayed in the UI (spool table, spool detail, filament detail) SHALL be formatted using `Intl.NumberFormat` with `style: "decimal"` and the browser's default locale. The unit suffix `" g"` SHALL be appended as a literal string. Form input fields used for data entry are excluded — they retain plain numeric values.

#### Scenario: Weight displayed using locale decimal separator
- **WHEN** the browser locale uses a comma as the decimal separator (e.g. `de-DE`)
- **THEN** a weight of `1234.5` grams is displayed as `"1.234,5 g"` (or equivalent locale form)

#### Scenario: Weight displayed using period as decimal separator
- **WHEN** the browser locale uses a period as the decimal separator (e.g. `en-US`)
- **THEN** a weight of `1234.5` grams is displayed as `"1,234.5 g"` (or equivalent locale form)

#### Scenario: Zero-gram weight
- **WHEN** a weight value is `0.0` grams
- **THEN** it is displayed as `"0 g"` (or locale-appropriate equivalent), not as an empty string

---

### Requirement: Density values are formatted with the browser locale
Density values (g/cm³) displayed in the filament table and detail view SHALL be formatted using `Intl.NumberFormat` with `maximumFractionDigits: 3`. The unit suffix `" g/cm³"` SHALL be appended.

#### Scenario: Density formatted with three significant decimals
- **WHEN** a filament has density `1.24` g/cm³ and the browser locale is `en-US`
- **THEN** the displayed value is `"1.24 g/cm³"` (trailing zero suppressed unless significant)

#### Scenario: Density formatted with locale decimal separator
- **WHEN** the browser locale uses a comma as the decimal separator
- **THEN** a density of `1.24` is displayed with a comma separator and the `" g/cm³"` suffix

---

### Requirement: Dates are formatted with the browser locale
All read-only date values (registered, first used, last used) displayed in spool and filament views SHALL be formatted using `Intl.DateTimeFormat` with the browser's default locale and a `dateStyle` option driven by the `date_format` setting. When the `date_format` setting is absent or empty, `dateStyle: "medium"` SHALL be used, preserving current default output. When the `time_format` setting is `"short"` or `"medium"`, the corresponding `timeStyle` option SHALL be included in the `Intl.DateTimeFormat` call; when `time_format` is `"none"` or absent, the `timeStyle` option SHALL be omitted entirely. The ISO `YYYY-MM-DD` format MUST NOT be used for display.

#### Scenario: Date displayed using default format (medium, no time)
- **WHEN** no `date_format` or `time_format` keys have been saved and the browser locale is `en-US`
- **THEN** a registered date of 2026-03-29 is displayed as `"Mar 29, 2026"` (identical to pre-change behaviour)

#### Scenario: Date displayed with long style
- **WHEN** `date_format` is `"long"` and `time_format` is `"none"` and the browser locale is `en-US`
- **THEN** a registered date of 2026-03-29 is displayed as `"March 29, 2026"`

#### Scenario: Date displayed with short style
- **WHEN** `date_format` is `"short"` and the browser locale is `en-US`
- **THEN** a registered date of 2026-03-29 is displayed as `"3/29/26"` (or equivalent locale short form)

#### Scenario: Date and time displayed when time_format is short
- **WHEN** `date_format` is `"medium"` and `time_format` is `"short"` and the browser locale is `en-US`
- **THEN** a timestamp of 2026-03-29T14:30:00Z is displayed with both a medium date and a short time (e.g. `"Mar 29, 2026, 2:30 PM"`)

#### Scenario: Date displayed in non-English locale
- **WHEN** the browser locale is `fr-FR` and `date_format` is `"medium"`
- **THEN** a registered date of 2026-03-29 is displayed in a locale-natural form (e.g. `"29 mars 2026"`) and does not use the ISO format

#### Scenario: Absent optional date
- **WHEN** an optional date field (e.g. `first_used`) is `None`/null
- **THEN** the field displays as empty, not as a formatted date or placeholder string

---

### Requirement: Currency amounts respect the `currency_symbol` setting override
When a currency amount is displayed, the system SHALL consult the `currency_symbol` setting. If the setting value is non-empty, it SHALL be used as a literal prefix and the numeric part SHALL be formatted with `Intl.NumberFormat` in `style: "decimal"` with `minimumFractionDigits: 2`. If the setting is empty or absent, the system SHALL format the amount with `Intl.NumberFormat` in `style: "currency"` using the browser locale's default currency.

#### Scenario: Custom symbol overrides Intl currency
- **WHEN** `currency_symbol` is set to `"PLN "` and a price of `49.9` is displayed
- **THEN** the displayed value is `"PLN 49.90"` (symbol prefix + locale-formatted decimal)

#### Scenario: Empty symbol falls back to Intl currency format
- **WHEN** `currency_symbol` is empty and the browser locale is `de-DE`
- **THEN** the amount `49.9` is formatted by `Intl.NumberFormat` with `style: "currency"` according to the `de-DE` locale (e.g. `"49,90 €"`)

#### Scenario: Default symbol (€) acts as override
- **WHEN** `currency_symbol` is `"€"` (the default) and a price of `10.0` is displayed
- **THEN** the prefix `"€"` is used and the numeric part is locale-formatted (not necessarily `"€10.00"` in all locales — the symbol is a prefix, not Intl currency style)
