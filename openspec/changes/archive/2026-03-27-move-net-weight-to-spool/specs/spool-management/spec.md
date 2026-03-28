## MODIFIED Requirements

### Requirement: Create spool
The system SHALL allow creating a new spool by providing filament_id, colors, initial_weight, and optionally location_id, color_name, net_weight, comment.

#### Scenario: Spool created successfully
- **WHEN** POST /api/v1/spool is called with valid filament_id and initial_weight
- **THEN** a new spool is created with a random u32 id, current_weight set to initial_weight, and registered set to now

#### Scenario: Spool created with net_weight
- **WHEN** POST /api/v1/spool is called with net_weight provided
- **THEN** the spool stores the net_weight and weight percentage metrics are included in the response

#### Scenario: Filament not found
- **WHEN** POST /api/v1/spool is called with a non-existent filament_id
- **THEN** the system returns 404

### Requirement: Edit spool
The system SHALL allow updating any mutable spool field: colors, color_name, location_id, current_weight, net_weight, first_used, last_used, comment.

#### Scenario: Update current weight
- **WHEN** PATCH /api/v1/spool/<id> is called with a new current_weight
- **THEN** the spool's current_weight is updated and last_used is set to now

#### Scenario: Update net weight
- **WHEN** PATCH /api/v1/spool/<id> is called with a new net_weight
- **THEN** the spool's net_weight is updated and weight percentage metrics reflect the new value

#### Scenario: Spool not found
- **WHEN** PATCH /api/v1/spool/<id> is called with a non-existent id
- **THEN** the system returns 404

### Requirement: Derive weight metrics
The system SHALL derive and return weight metrics in spool responses. Metrics are computed from stored fields, never stored redundantly. net_weight is read from the spool, not from the filament.

#### Scenario: Used weight derived
- **WHEN** a spool is retrieved
- **THEN** used_weight = initial_weight - current_weight is included in the response

#### Scenario: Remaining filament derived when net_weight known
- **WHEN** a spool is retrieved and the spool has a net_weight
- **THEN** remaining_filament = spool.net_weight - used_weight is included in the response

#### Scenario: Remaining percentage derived
- **WHEN** a spool is retrieved and the spool has a net_weight
- **THEN** remaining_pct = remaining_filament / spool.net_weight × 100 is included in the response

#### Scenario: No metrics when net_weight absent
- **WHEN** a spool is retrieved and spool.net_weight is None
- **THEN** remaining_filament and remaining_pct are omitted (None) from the response

### Requirement: Clone spool
The system SHALL allow cloning an existing spool to create a new spool pre-filled with the same filament_id, colors, color_name, net_weight, and initial_weight.

#### Scenario: Clone creates new spool
- **WHEN** POST /api/v1/spool/<id>/clone is called
- **THEN** a new spool is created with a new random id, same filament, color, and net_weight; current_weight set to initial_weight; registered set to now
