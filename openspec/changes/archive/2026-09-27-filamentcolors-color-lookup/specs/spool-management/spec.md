## ADDED Requirements

### Requirement: Swatch image in spool list and detail view
When a spool has a `swatch_image` reference, the spool list table's color cell and the spool detail view's Colors field SHALL render the stored swatch image instead of the flat color chip. Spools without a `swatch_image` SHALL continue to render the flat color chip as before.

#### Scenario: List shows swatch image when present
- **WHEN** a spool with a `swatch_image` reference is rendered in the spool list table
- **THEN** its color cell displays the stored swatch image

#### Scenario: Detail view shows swatch image when present
- **WHEN** a spool with a `swatch_image` reference is rendered in the spool detail view
- **THEN** its Colors field displays the stored swatch image

#### Scenario: No swatch image falls back to chip
- **WHEN** a spool has no `swatch_image` reference
- **THEN** the list and detail views render the flat color chip as before this feature existed
