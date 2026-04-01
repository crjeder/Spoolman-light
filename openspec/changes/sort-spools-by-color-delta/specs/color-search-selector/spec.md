# Specification: color-search-selector (delta)

## ADDED Requirements

### Requirement: Active color level implicitly enables delta sort
When the color level selector is set to any value other than Off, the spool list sort order SHALL switch to color-delta sort mode as defined in the `color-delta-sort` capability. This is implicit — no additional user action is required.

#### Scenario: Switching level from Off to Fine activates delta sort
- **WHEN** the user changes the color level selector from "Off" to "Fine" (or Medium or Coarse)
- **THEN** the spool list SHALL immediately re-sort by ascending ΔE*00 from the currently selected color

#### Scenario: Switching level back to Off deactivates delta sort
- **WHEN** the user changes the color level selector back to "Off"
- **THEN** the spool list SHALL immediately revert to column-based sort order
