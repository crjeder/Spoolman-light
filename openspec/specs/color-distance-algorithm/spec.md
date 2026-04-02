# Specification: Color Distance Algorithm

### Requirement: Color distance algorithm setting
The system SHALL store a user-selected color distance algorithm under the settings key `color_distance_algorithm`. Valid values are `ciede2000`, `oklab`, and `din99d`. When the key is absent the system SHALL default to `ciede2000`. The value SHALL be persisted via the existing `PUT /api/v1/settings/{key}` endpoint.

#### Scenario: Default algorithm when setting is absent
- **WHEN** the `color_distance_algorithm` setting key has never been saved
- **THEN** the system uses CIEDE2000 for all color distance computations

#### Scenario: Algorithm persists across reloads
- **WHEN** the user saves `color_distance_algorithm = oklab` on the Settings page
- **THEN** after a full page reload the spool list color filter uses OKLab distance

### Requirement: Algorithm selector on Settings page
The Settings page SHALL display a labeled `<select>` element for the color distance algorithm with options "CIEDE2000 (default)", "OKLab", and "DIN99d". Submitting the Settings form SHALL persist the selected value. The selector SHALL be pre-populated with the current persisted value on load.

#### Scenario: Selector shows persisted algorithm
- **WHEN** the Settings page loads and `color_distance_algorithm = oklab` is persisted
- **THEN** the selector displays "OKLab" as the selected option

#### Scenario: Saving a new algorithm updates the selector and filter
- **WHEN** the user changes the selector to "DIN99d" and saves
- **THEN** the selector shows "DIN99d" and the spool list immediately applies DIN99d distance without a page reload

### Requirement: OKLab color distance
The system SHALL support computing perceptual color difference in the OKLab color space (ΔE_ok) as the Euclidean distance between two OKLab values derived from sRGB inputs. Alpha SHALL be ignored.

#### Scenario: Identical colors return zero distance
- **WHEN** both input colors are the same sRGB value and OKLab is the active algorithm
- **THEN** the returned ΔE_ok is 0.0

#### Scenario: Clearly different colors return non-zero distance
- **WHEN** comparing pure red (#ff0000) and pure blue (#0000ff) using OKLab
- **THEN** the returned ΔE_ok is greater than 0.1

### Requirement: DIN99d color distance
The system SHALL support computing perceptual color difference using the DIN99d metric (DIN 6176:2001) as a closed-form transform applied to CIE L\*a\*b\* values derived from sRGB inputs. Alpha SHALL be ignored.

#### Scenario: Identical colors return zero distance
- **WHEN** both input colors are the same sRGB value and DIN99d is the active algorithm
- **THEN** the returned DIN99d ΔE is 0.0

#### Scenario: Clearly different colors return high distance
- **WHEN** comparing pure red (#ff0000) and pure blue (#0000ff) using DIN99d
- **THEN** the returned DIN99d ΔE is greater than 25.0

### Requirement: Per-algorithm thresholds for color search levels
The system SHALL map the color search level labels (Same, Close, Ballpark) to numeric thresholds that are user-configurable and persisted in the settings store. When no user value has been saved for a given level/algorithm combination, the system SHALL fall back to the following defaults:

| Level    | CIEDE2000 | OKLab | DIN99d |
|----------|-----------|-------|--------|
| Same     | 10.0      | 0.10  | 10.0   |
| Close    | 20.0      | 0.20  | 20.0   |
| Ballpark | 35.0      | 0.35  | 35.0   |

#### Scenario: OKLab Same threshold defaults to 0.10 when not configured
- **WHEN** OKLab is the active algorithm, the level is "Same", and no `color_threshold_oklab_same` setting has been saved
- **THEN** spools SHALL be included only if any stored color has ΔE_ok ≤ 0.10 from the selected color

#### Scenario: DIN99d Ballpark threshold defaults to 35.0 when not configured
- **WHEN** DIN99d is the active algorithm, the level is "Ballpark", and no `color_threshold_din99d_ballpark` setting has been saved
- **THEN** spools SHALL be included only if any stored color has DIN99d ΔE ≤ 35.0 from the selected color

#### Scenario: User-configured threshold overrides default
- **WHEN** `color_threshold_ciede2000_same = 5` has been persisted and CIEDE2000 Same is active
- **THEN** spools SHALL be included only if any stored color has CIEDE2000 ΔE ≤ 5.0 from the selected color
