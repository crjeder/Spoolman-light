## MODIFIED Requirements

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
