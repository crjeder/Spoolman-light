## ADDED Requirements

### Requirement: Filament name in spool detail links to filament page
In the spool detail view, the filament name SHALL be rendered as a hyperlink that navigates to the corresponding filament detail page (`/filaments/<id>`).

#### Scenario: Filament name is a link in spool detail
- **WHEN** a user views the spool detail page (`/spools/:id`)
- **THEN** the "Filament" field value SHALL be an anchor element linking to `/filaments/<filament_id>`

#### Scenario: Clicking filament link navigates to filament detail
- **WHEN** a user clicks the filament name link in the spool detail view
- **THEN** the browser SHALL navigate to the filament detail page for that filament

### Requirement: Filament name in spool list links to filament page
In the spool list table, each row's filament name cell SHALL be rendered as a hyperlink to `/filaments/<filament_id>`.

#### Scenario: Filament name is a link in spool list row
- **WHEN** a user views the spool list page
- **THEN** each row's filament name SHALL be an anchor element linking to `/filaments/<filament_id>`

#### Scenario: Clicking filament link in list navigates to filament detail
- **WHEN** a user clicks a filament name in the spool list table
- **THEN** the browser SHALL navigate to the corresponding filament detail page
