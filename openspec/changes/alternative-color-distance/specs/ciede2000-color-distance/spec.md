## MODIFIED Requirements

### Requirement: Color search threshold uses configurable algorithm scale
The spool list color search filter SHALL compare the color distance returned by the configured algorithm against the threshold for that algorithm at the selected level. The default algorithm SHALL be CIEDE2000 with its existing thresholds. A spool SHALL be included in results if any of its stored colors has a distance ≤ threshold from the target color under the active algorithm.

#### Scenario: Spool with matching color is included
- **WHEN** a spool has a color whose distance (under the active algorithm) is ≤ the threshold for the active level
- **THEN** that spool appears in the filtered list

#### Scenario: Spool with no matching color is excluded
- **WHEN** all of a spool's colors have a distance (under the active algorithm) > the threshold for the active level from the selected color
- **THEN** that spool does not appear in the filtered list

#### Scenario: No color filter shows all spools
- **WHEN** no color is selected in the color picker (level is "Off")
- **THEN** all spools are shown regardless of their colors

#### Scenario: Default algorithm (CIEDE2000) preserves existing behavior
- **WHEN** the `color_distance_algorithm` setting is absent and level is "Fine"
- **THEN** the threshold applied is 10.0 on the CIEDE2000 ΔE\*00 scale — identical to pre-change behavior
