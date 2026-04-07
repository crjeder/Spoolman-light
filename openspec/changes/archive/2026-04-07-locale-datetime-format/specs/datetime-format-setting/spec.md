## ADDED Requirements

### Requirement: Date format is user-configurable via a setting
The Settings page SHALL display a "Date format" selector with options `short`, `medium` (default), `long`, and `full`. The selected value SHALL be persisted as the `date_format` key via `PUT /api/v1/settings/date_format`. When absent or empty, the system SHALL default to `medium`.

#### Scenario: Default date format on first load
- **WHEN** the Settings page loads and no `date_format` key has been saved
- **THEN** the "Date format" selector SHALL show `medium` as the selected value

#### Scenario: Persisted date format pre-populates the selector
- **WHEN** the Settings page loads and `date_format = "long"` has been saved
- **THEN** the "Date format" selector SHALL show `long`

#### Scenario: Saving persists the selected date format
- **WHEN** the user selects `short` and saves the Settings form
- **THEN** a `PUT /api/v1/settings/date_format` request SHALL be issued with value `"short"`

#### Scenario: Date columns reflect the selected format immediately
- **WHEN** the user saves a new `date_format` value
- **THEN** all date cells in the spool and filament tables SHALL re-render using the new style without a page reload

---

### Requirement: Time format is user-configurable via a setting
The Settings page SHALL display a "Time format" selector with options `none` (default — date only), `short`, and `medium`. The selected value SHALL be persisted as the `time_format` key via `PUT /api/v1/settings/time_format`. When absent or empty, the system SHALL default to `none` (no time component displayed).

#### Scenario: Default time format shows no time component
- **WHEN** no `time_format` key has been saved
- **THEN** date values in the UI SHALL display only the date portion, matching current behaviour

#### Scenario: Selecting "short" time appends a time component
- **WHEN** `time_format` is `"short"` and the browser locale is `en-US`
- **THEN** a registered date of 2026-03-29T14:30:00Z SHALL be displayed with a short time suffix (e.g. `"Mar 29, 2026, 2:30 PM"`)

#### Scenario: Saving persists the selected time format
- **WHEN** the user selects `medium` for time format and saves
- **THEN** a `PUT /api/v1/settings/time_format` request SHALL be issued with value `"medium"`

#### Scenario: Reverting to "none" removes time component
- **WHEN** `time_format` is changed back to `none` and saved
- **THEN** date cells SHALL revert to showing the date portion only
