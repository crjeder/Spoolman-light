## MODIFIED Requirements

### Requirement: Spool entity structure
A Spool SHALL have: id (random u32), filament_id (u32), location_id (Option<u32>), colors (Vec<Rgba>, len 1–4), color_name (Option<String>), initial_weight (f32, grams), current_weight (f32, grams), net_weight (Option<f32>, grams), price (Option<f32>), registered (DateTime UTC), first_used (Option<DateTime>), last_used (Option<DateTime>), comment (Option<String>), archived (bool).

#### Scenario: Spool created with required fields
- **WHEN** a spool is created with filament_id, colors, initial_weight, and current_weight
- **THEN** the system stores the spool with a unique random u32 id and registered timestamp

#### Scenario: Spool id collision is handled
- **WHEN** a randomly generated u32 id already exists in the store
- **THEN** the system generates a new random u32 until a unique id is found

#### Scenario: Spool stores its own net weight
- **WHEN** a spool is created with net_weight provided
- **THEN** the spool stores the net_weight independently of its filament

#### Scenario: Spool net weight is optional
- **WHEN** a spool is created without net_weight
- **THEN** net_weight is stored as None and weight percentage metrics are omitted from the response

#### Scenario: Spool price is optional
- **WHEN** a spool is created without price
- **THEN** price is stored as None and price_per_gram is omitted from the response

#### Scenario: Existing spools without price deserialize correctly
- **WHEN** the server reads a spoolman.json that has spool entries without a price field
- **THEN** those spools are loaded with price: None and no error occurs
