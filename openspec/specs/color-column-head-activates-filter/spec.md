# Spec: color-column-head-activates-filter

## Purpose

The "Color" column header in the spool list table is interactive: clicking it focuses the color picker input in the page header, activating the color filter.

## Requirements

### Requirement: Color column header activates color filter
The spool list table's "Color" column header SHALL be interactive. Clicking it SHALL focus the color picker input in the page header, activating the color filter.

#### Scenario: Click color column header focuses color picker
- **WHEN** the user clicks the "Color" column header in the spool table
- **THEN** the color picker input receives focus (browser color popup opens or input is focused)

#### Scenario: Color column header indicates interactivity
- **WHEN** the user hovers over the "Color" column header
- **THEN** the cursor changes to a pointer and a visual hover style is applied to signal it is clickable
