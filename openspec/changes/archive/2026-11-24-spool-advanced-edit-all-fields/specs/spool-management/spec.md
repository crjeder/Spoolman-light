## MODIFIED Requirements

### Requirement: Edit spool
The system SHALL allow updating every spool field except `id`: filament_id, colors, color_name, swatch_image, finish, location_id, initial_weight, current_weight, net_weight, price, registered, first_used, last_used, comment, archived. A provided `filament_id` SHALL reference an existing filament. A provided `initial_weight` SHALL be applied regardless of the stored value. The spool edit form SHALL by default show only current weight, location, last used and comment, plus initial weight above current weight when the stored initial weight is 0, and SHALL expose all other editable fields (including a non-zero initial weight) in a collapsed "Advanced" section of the same form; the spool ID SHALL be shown read-only there. The system SHALL set `last_used` to now only when `current_weight` is changed to a different value, unless `last_used` is explicitly provided.

#### Scenario: Update current weight
- **WHEN** PATCH /api/v1/spool/<id> is called with a current_weight different from the stored value and no last_used
- **THEN** the spool's current_weight is updated and last_used is set to now

#### Scenario: Unchanged weight does not touch last used
- **WHEN** PATCH /api/v1/spool/<id> is called with a current_weight equal to the stored value and no last_used
- **THEN** last_used (and first_used) are left unchanged

#### Scenario: Quick edit form fields
- **WHEN** the user opens the edit page for a spool with a non-zero initial weight
- **THEN** only current weight, location, last used and comment are visible, and the other fields are inside a collapsed "Advanced" section

#### Scenario: Advanced fields editable
- **WHEN** the user expands "Advanced", changes the price and saves
- **THEN** the price is updated and quick-mode fields are saved with their current values

#### Scenario: Advanced exposes all fields except ID
- **WHEN** the user expands "Advanced" for a spool with a non-zero initial weight
- **THEN** filament, initial weight, net weight, price, colors, color name, finish, registered, first used and archived are editable inputs, and the spool ID is shown read-only

#### Scenario: Unset initial weight shown in quick section
- **WHEN** the user opens the edit page for a spool whose initial weight is 0
- **THEN** the initial weight input is shown above current weight outside "Advanced", and entering it also sets current weight

#### Scenario: Change filament
- **WHEN** PATCH /api/v1/spool/<id> is called with an existing filament_id
- **THEN** the spool's filament_id is updated and the response embeds the new filament

#### Scenario: Unknown filament rejected
- **WHEN** PATCH /api/v1/spool/<id> is called with a filament_id that does not exist
- **THEN** the system returns a validation error and the spool is unchanged

#### Scenario: Correct non-zero initial weight
- **WHEN** PATCH /api/v1/spool/<id> is called with an initial_weight for a spool whose stored initial_weight is non-zero
- **THEN** the spool's initial_weight is updated and current_weight is unchanged

#### Scenario: Change registered date
- **WHEN** the user changes the registered date/time in Advanced and saves
- **THEN** the spool's registered timestamp is the chosen value

#### Scenario: Archive from edit form without location
- **WHEN** the user checks "Archived" in Advanced and saves
- **THEN** the save is allowed without a location, the spool is archived and its location is cleared

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
