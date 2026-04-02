## MODIFIED Requirements

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
