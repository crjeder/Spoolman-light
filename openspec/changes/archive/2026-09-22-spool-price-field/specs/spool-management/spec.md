## MODIFIED Requirements

### Requirement: Create spool
The system SHALL allow creating a new spool by providing filament_id, colors, initial_weight, and optionally location_id, color_name, net_weight, price, comment.

#### Scenario: Spool created successfully
- **WHEN** POST /api/v1/spool is called with valid filament_id and initial_weight
- **THEN** a new spool is created with a random u32 id, current_weight set to initial_weight, and registered set to now

#### Scenario: Spool created with net_weight
- **WHEN** POST /api/v1/spool is called with net_weight provided
- **THEN** the spool stores the net_weight and weight percentage metrics are included in the response

#### Scenario: Spool created with price
- **WHEN** POST /api/v1/spool is called with a price value
- **THEN** the spool stores the price and price_per_gram is derived in the response

#### Scenario: Filament not found
- **WHEN** POST /api/v1/spool is called with a non-existent filament_id
- **THEN** the system returns 404

### Requirement: Edit spool
The system SHALL allow updating any mutable spool field: colors, color_name, location_id, current_weight, net_weight, price, first_used, last_used, comment.

#### Scenario: Update current weight
- **WHEN** PATCH /api/v1/spool/<id> is called with a new current_weight
- **THEN** the spool's current_weight is updated and last_used is set to now

#### Scenario: Update net weight
- **WHEN** PATCH /api/v1/spool/<id> is called with a new net_weight
- **THEN** the spool's net_weight is updated and weight percentage metrics reflect the new value

#### Scenario: Update price
- **WHEN** PATCH /api/v1/spool/<id> is called with a new price value
- **THEN** the spool's price is updated and price_per_gram is recalculated in the response

#### Scenario: Clear price
- **WHEN** PATCH /api/v1/spool/<id> is called with price set to null
- **THEN** the spool's price is set to None and price_per_gram is omitted from the response

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

#### Scenario: No remaining_filament when net_weight absent
- **WHEN** a spool is retrieved and spool.net_weight is None
- **THEN** remaining_filament is omitted (None) from the response

#### Scenario: price_per_gram derived when price and weight are set
- **WHEN** a spool is retrieved and the spool has a price
- **THEN** price_per_gram = price / (net_weight if set, else initial_weight) is included in the response

#### Scenario: No price_per_gram when price absent
- **WHEN** a spool is retrieved and spool.price is None
- **THEN** price_per_gram is omitted (None) from the response
