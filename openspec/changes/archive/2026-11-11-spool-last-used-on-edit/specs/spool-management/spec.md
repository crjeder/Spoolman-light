## MODIFIED Requirements

### Requirement: Edit spool
The system SHALL allow updating any mutable spool field: colors, color_name, location_id, current_weight, net_weight, first_used, last_used, comment. Saving the spool edit form SHALL set `last_used` to the current date unless the user changed the `last_used` field.

#### Scenario: Update current weight
- **WHEN** PATCH /api/v1/spool/<id> is called with a new current_weight
- **THEN** the spool's current_weight is updated and last_used is set to now

#### Scenario: Edit form refreshes last used
- **WHEN** the user opens the edit form for a spool whose `last_used` is an earlier date, changes the current weight, and saves without touching the `last_used` field
- **THEN** the spool's `last_used` becomes today's date

#### Scenario: Edit form keeps manually chosen last used
- **WHEN** the user changes the `last_used` field to another date and saves
- **THEN** the spool's `last_used` is the chosen date
