## ADDED Requirements

### Requirement: Location is required on spool create form
The spool create form SHALL require a location to be selected before submission is allowed. The submit button SHALL be disabled when no location is selected.

#### Scenario: Submit disabled without location
- **WHEN** the spool create form is open and no location is selected
- **THEN** the submit button is disabled and the form cannot be submitted

#### Scenario: Submit enabled after location selected
- **WHEN** the user selects a location in the spool create form
- **THEN** the submit button becomes enabled

### Requirement: Location is required on spool edit form
The spool edit form SHALL require a location to be selected before submission is allowed. The submit button SHALL be disabled when no location is selected.

#### Scenario: Submit disabled without location in edit form
- **WHEN** the spool edit form is open and no location is selected
- **THEN** the submit button is disabled and the form cannot be submitted

#### Scenario: Submit enabled after location selected in edit form
- **WHEN** the user selects a location in the spool edit form
- **THEN** the submit button becomes enabled
