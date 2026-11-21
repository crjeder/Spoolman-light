## MODIFIED Requirements

### Requirement: Edit spool
The system SHALL allow updating any mutable spool field: colors, color_name, location_id, current_weight, net_weight, first_used, last_used, comment. The spool edit form SHALL by default show only current weight, location, last used and comment, and SHALL expose all other editable fields in a collapsed "Advanced" section of the same form. The system SHALL set `last_used` to now only when `current_weight` is changed to a different value, unless `last_used` is explicitly provided.

#### Scenario: Update current weight
- **WHEN** PATCH /api/v1/spool/<id> is called with a current_weight different from the stored value and no last_used
- **THEN** the spool's current_weight is updated and last_used is set to now

#### Scenario: Unchanged weight does not touch last used
- **WHEN** PATCH /api/v1/spool/<id> is called with a current_weight equal to the stored value and no last_used
- **THEN** last_used (and first_used) are left unchanged

#### Scenario: Quick edit form fields
- **WHEN** the user opens the edit page for a spool
- **THEN** only current weight, location, last used and comment are visible, and the other fields are inside a collapsed "Advanced" section

#### Scenario: Advanced fields editable
- **WHEN** the user expands "Advanced", changes the price and saves
- **THEN** the price is updated and quick-mode fields are saved with their current values

#### Scenario: Weight change refreshes last used
- **WHEN** the user changes the current weight in the edit form and saves without touching the `last_used` field
- **THEN** the spool's `last_used` becomes today's date

#### Scenario: Save without weight change keeps last used
- **WHEN** the user edits only the location or comment and saves without touching the weight or `last_used`
- **THEN** the spool's `last_used` is unchanged (still unset if it was unset)

#### Scenario: Edit form keeps manually chosen last used
- **WHEN** the user changes the `last_used` field to another date and saves
- **THEN** the spool's `last_used` is the chosen date

#### Scenario: Update net weight
- **WHEN** PATCH /api/v1/spool/<id> is called with a new net_weight
- **THEN** the spool's net_weight is updated and weight percentage metrics reflect the new value

#### Scenario: Spool not found
- **WHEN** PATCH /api/v1/spool/<id> is called with a non-existent id
- **THEN** the system returns 404
