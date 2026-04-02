# Spec: color-delta-sort

## Purpose

When a color filter level is active and a valid hex color is selected, the spool list is sorted by ascending CIEDE2000 (ΔE*00) distance from the selected color rather than by the normal column-based sort order. This surfaces the closest color matches at the top of the table automatically.

## Requirements

### Requirement: Spools sorted by ascending color delta when color filter is active
When a color level (Fine, Medium, or Coarse) is active and a valid hex color is selected, the spool list SHALL be sorted by ascending minimum ΔE*00 distance from the selected color. The sort key for each spool SHALL be the minimum `color_distance` value across all of its stored colors. Spools with smaller delta (closer color match) SHALL appear first.

#### Scenario: Color active — closest match appears first
- **WHEN** the color level is set to Fine, Medium, or Coarse and a valid hex color is selected
- **THEN** the spool with the smallest minimum ΔE*00 distance from the selected color SHALL be the first row in the table

#### Scenario: Tie-breaking order is stable
- **WHEN** two spools have equal minimum delta
- **THEN** their relative order SHALL be consistent across renders (implementation may use stable sort)

#### Scenario: Spool with multiple colors uses minimum delta
- **WHEN** a spool has more than one stored color
- **THEN** its sort key SHALL be the minimum ΔE*00 across all its colors, not the first color only

### Requirement: Default sort order restored when color filter is off
When the color level is set to Off, the spool list SHALL revert to column-based sort order controlled by the sort field and sort direction signals.

#### Scenario: Level set to Off — column sort applies
- **WHEN** the color level selector is set to "Off"
- **THEN** the spool list SHALL be ordered by the currently selected sort column and direction, not by color delta

#### Scenario: Invalid or empty hex with active level falls back to column sort
- **WHEN** the color level is active but the hex value cannot be parsed as a valid color
- **THEN** the spool list SHALL use column-based sort order as if no color were selected
