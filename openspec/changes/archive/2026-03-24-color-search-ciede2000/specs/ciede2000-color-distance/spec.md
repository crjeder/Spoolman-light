## ADDED Requirements

### Requirement: CIEDE2000 color distance computation
The system SHALL compute perceptual color difference between two sRGB colors using the CIEDE2000 (ΔE\*00) metric as specified in CIE 142-2001. The function SHALL accept two sRGB colors (as `Rgba` structs, alpha ignored) and return a non-negative `f32` ΔE\*00 value. A return value of 0.0 SHALL indicate identical colors. Values below 1.0 SHALL be considered imperceptible differences; values above 25.0 SHALL be considered clearly distinct colors.

#### Scenario: Identical colors return zero distance
- **WHEN** both input colors are the same sRGB value
- **THEN** the returned ΔE\*00 is 0.0

#### Scenario: Clearly different colors return high distance
- **WHEN** comparing pure red (#ff0000) and pure blue (#0000ff)
- **THEN** the returned ΔE\*00 is greater than 25.0

#### Scenario: Perceptually similar colors score low despite RGB difference
- **WHEN** comparing two colors that are perceptually near-identical but differ in raw RGB values
- **THEN** the returned ΔE\*00 is less than 5.0

### Requirement: sRGB to CIE L*a*b* conversion
The system SHALL convert sRGB values to CIE L\*a\*b\* (D65 illuminant) using the exact IEC 61966-2-1 inverse EOTF for linearisation, followed by the ITU-R BT.709 matrix for XYZ conversion, followed by the CIE L\*a\*b\* cube-root transform. Gamma approximations (e.g., γ ≈ 2.2) SHALL NOT be used.

#### Scenario: Black maps to Lab origin
- **WHEN** converting #000000 to Lab
- **THEN** L\* = 0, a\* = 0, b\* = 0

#### Scenario: White maps to Lab white point
- **WHEN** converting #ffffff to Lab
- **THEN** L\* ≈ 100, a\* ≈ 0, b\* ≈ 0 (within float rounding tolerance)

### Requirement: Color search threshold uses ΔE*00 scale
The spool list color search filter SHALL compare `color_distance()` (ΔE\*00) against the configured threshold. The default threshold SHALL be 10.0, representing "acceptably similar" colors on the 0–100 ΔE scale. A spool SHALL be included in results if any of its stored colors has ΔE\*00 ≤ threshold from the target color.

#### Scenario: Spool with matching color is included
- **WHEN** a spool has a color with ΔE\*00 ≤ threshold from the selected color
- **THEN** that spool appears in the filtered list

#### Scenario: Spool with no matching color is excluded
- **WHEN** all of a spool's colors have ΔE\*00 > threshold from the selected color
- **THEN** that spool does not appear in the filtered list

#### Scenario: No color filter shows all spools
- **WHEN** no color is selected in the color picker
- **THEN** all spools are shown regardless of their colors
