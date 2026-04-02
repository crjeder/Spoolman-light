# Specification: Color Threshold Settings

### Requirement: Color search level thresholds are user-configurable
The Settings page SHALL display three numeric input fields for the active color distance algorithm's thresholds — one for each search level (Same, Close, Ballpark). Each field SHALL be pre-populated with the persisted value for that level/algorithm combination, or the hardcoded default when no value has been saved. Saving the Settings form SHALL persist each threshold under its key `color_threshold_{algo}_{level}` (e.g. `color_threshold_ciede2000_same`) via `PUT /api/v1/settings/{key}`.

#### Scenario: Fields pre-populated with defaults on first load
- **WHEN** the Settings page loads and no threshold keys have been saved
- **THEN** the Same, Close, and Ballpark fields SHALL show the hardcoded default values for the active algorithm (CIEDE2000: 10 / 20 / 35; OKLab: 0.10 / 0.20 / 0.35; DIN99d: 10 / 20 / 35)

#### Scenario: Fields pre-populated with persisted values after save
- **WHEN** the Settings page loads and `color_threshold_ciede2000_same = 8` has been saved
- **THEN** the Same field SHALL show 8 when CIEDE2000 is the active algorithm

#### Scenario: Saving persists new threshold values
- **WHEN** the user changes the Same field to 5 and saves with CIEDE2000 active
- **THEN** a `PUT /api/v1/settings/color_threshold_ciede2000_same` request SHALL be issued with value `"5"`

#### Scenario: Threshold fields update when algorithm changes
- **WHEN** the user changes the algorithm selector from CIEDE2000 to OKLab on the Settings page
- **THEN** the threshold fields SHALL immediately show the OKLab values (persisted or default)

### Requirement: Persisted thresholds applied immediately to color filter
After saving, the spool list color filter SHALL use the updated threshold values without requiring a page reload.

#### Scenario: Filter re-evaluates after threshold save
- **WHEN** the user saves a reduced Same threshold (e.g. from 10 to 5) and returns to the Spools page with the Same level active
- **THEN** only spools within the new tighter threshold distance SHALL be shown

#### Scenario: Default thresholds applied when keys are absent
- **WHEN** no threshold keys have been persisted and the Same level is active with CIEDE2000
- **THEN** the filter uses threshold 10.0, identical to pre-change behaviour
