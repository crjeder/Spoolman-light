# Spec: color-search-selector

## Purpose

The color filter controls use a named level selector (Off / Fine / Medium / Coarse) instead of a raw numeric threshold slider. The level drives a CIEDE2000 distance threshold for color matching. The color picker is only shown when the level is active.
## Requirements
### Requirement: Color search level selector replaces threshold slider
The color filter controls SHALL include a `<select>` element with four named levels: **Off**, **Fine**, **Medium**, and **Coarse**. The selector SHALL default to **Off**. The raw numeric range slider SHALL be removed.

Each level maps to a CIEDE2000 threshold:
- Off — filter disabled (no color matching performed)
- Fine — threshold ≤ 10.0
- Medium — threshold ≤ 30.0
- Coarse — threshold ≤ 60.0

When a spool's `finish` is not `Standard`, the system SHALL apply an HSV modifier to the spool's stored color before computing the distance:
- Matte: S×0.85, V×1.10
- Standard: S×1.00, V×1.00 (identity)
- Gloss: S×1.15, V×0.95

V SHALL be clamped to [0, 1] after applying the multiplier. The modifier is applied to the spool's color, not to the user's search target.

#### Scenario: Default state is Off
- **WHEN** the spool list page loads
- **THEN** the color level selector SHALL show "Off" and no color filtering SHALL be applied

#### Scenario: Selecting Fine filters by close colour match
- **WHEN** the user selects a colour with the picker AND sets the level to "Fine"
- **THEN** only spools whose finish-adjusted colour is within a CIEDE2000 distance of 10 from the selected colour SHALL be shown

#### Scenario: Selecting Medium broadens the match
- **WHEN** the user selects a colour with the picker AND sets the level to "Medium"
- **THEN** only spools whose finish-adjusted colour is within a CIEDE2000 distance of 30 from the selected colour SHALL be shown

#### Scenario: Selecting Coarse gives the widest match
- **WHEN** the user selects a colour with the picker AND sets the level to "Coarse"
- **THEN** only spools whose finish-adjusted colour is within a CIEDE2000 distance of 60 from the selected colour SHALL be shown

#### Scenario: Switching to Off disables colour filter
- **WHEN** the user changes the level selector to "Off"
- **THEN** colour filtering SHALL be disabled and all spools satisfying other active filters SHALL be shown

#### Scenario: Matte spool matched via desaturated effective color
- **WHEN** a spool with `finish: Matte` and stored color `#ff0000` is in the list AND the user searches for a muted red
- **THEN** the effective comparison color SHALL be the HSV-modified version of `#ff0000` (S×0.85, V×1.10)

#### Scenario: Standard spool uses stored color unchanged
- **WHEN** a spool with `finish: Standard` is compared during color search
- **THEN** the stored color SHALL be used as-is (no HSV modification applied)

### Requirement: Color picker is hidden when level is Off
The `<input type="color">` and its clear button SHALL only be shown when the level is not "Off".

#### Scenario: Picker hidden at Off
- **WHEN** the level selector is set to "Off"
- **THEN** the colour picker input SHALL NOT be visible

#### Scenario: Picker shown when level is active
- **WHEN** the level selector is set to any level other than "Off"
- **THEN** the colour picker input SHALL be visible and interactive

### Requirement: Active color level implicitly enables delta sort
When the color level selector is set to any value other than Off, the spool list sort order SHALL switch to color-delta sort mode as defined in the `color-delta-sort` capability. This is implicit — no additional user action is required.

#### Scenario: Switching level from Off to Fine activates delta sort
- **WHEN** the user changes the color level selector from "Off" to "Fine" (or Medium or Coarse)
- **THEN** the spool list SHALL immediately re-sort by ascending ΔE*00 from the currently selected color

#### Scenario: Switching level back to Off deactivates delta sort
- **WHEN** the user changes the color level selector back to "Off"
- **THEN** the spool list SHALL immediately revert to column-based sort order

### Requirement: Color level and picked color persist for the session
The color level selector value and the picked color SHALL be stored in `sessionStorage` and restored when the spool list is re-created or reloaded within the same tab session. When a stored level other than "Off" is restored, the color filter and implicit color-delta sort SHALL apply on first render without user action. The "Clear filters" control SHALL reset the level to "Off" (and, per `color-search-selector`, hide the picker and revert to column sort) and remove the stored values.

#### Scenario: Active color level restored after navigation
- **WHEN** the user sets the level to "Medium" with a picked color, navigates away and back
- **THEN** the level selector shows "Medium", the picker shows the stored color, and the list is filtered and delta-sorted accordingly

#### Scenario: Clear filters resets the color level
- **WHEN** a color level other than "Off" is active and the user clicks "Clear filters"
- **THEN** the level selector returns to "Off", the picker is hidden, and the list reverts to column-based sort

