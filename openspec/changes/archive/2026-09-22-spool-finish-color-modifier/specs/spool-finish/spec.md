## ADDED Requirements

### Requirement: Spool carries a surface finish classification
Each spool SHALL have a `finish` field of type `SurfaceFinish` with three variants: `Matte`, `Standard`, and `Gloss`. The field SHALL default to `Standard` when absent from stored data (backward-compatible deserialization).

#### Scenario: New spool defaults to Standard finish
- **WHEN** a spool is created without specifying a finish
- **THEN** the spool's finish SHALL be `Standard`

#### Scenario: Existing spool data without finish field deserializes as Standard
- **WHEN** a JSON data file containing a spool with no `finish` key is loaded
- **THEN** the spool's finish SHALL be `Standard` and no error SHALL occur

#### Scenario: Finish round-trips through JSON
- **WHEN** a spool with `finish: Matte` is serialized and then deserialized
- **THEN** the finish SHALL remain `Matte`

### Requirement: Finish is selectable in the spool add and edit form
The spool add/edit form SHALL include a `<select>` element for finish with options: **Matte**, **Standard**, **Gloss**. The selector SHALL default to **Standard**.

#### Scenario: Finish selector is present in add form
- **WHEN** the user opens the add spool form
- **THEN** a finish selector SHALL be visible with "Standard" pre-selected

#### Scenario: User can set Gloss on a new spool
- **WHEN** the user selects "Gloss" in the finish selector and submits the form
- **THEN** the created spool SHALL have `finish: Gloss`

#### Scenario: Edit form shows current finish
- **WHEN** the user opens the edit form for a spool with `finish: Matte`
- **THEN** the finish selector SHALL show "Matte"

### Requirement: Finish is visible in the spool table
The spool table SHALL display the finish value for each spool as a text badge inline with the color swatch cell.

#### Scenario: Matte badge shown
- **WHEN** a spool with `finish: Matte` appears in the spool table
- **THEN** a "Matte" badge SHALL be displayed in the color cell

#### Scenario: Standard finish shows no badge
- **WHEN** a spool with `finish: Standard` appears in the spool table
- **THEN** no finish badge SHALL be shown (Standard is the baseline and adds no visual noise)

#### Scenario: Gloss badge shown
- **WHEN** a spool with `finish: Gloss` appears in the spool table
- **THEN** a "Gloss" badge SHALL be displayed in the color cell
