## ADDED Requirements

### Requirement: Store spool purchase price
A Spool MAY have an optional `price: Option<f32>` field representing the purchase price in the user's preferred currency. The field is optional; its absence means the user has not recorded a price for that spool.

#### Scenario: Spool created with price
- **WHEN** POST /api/v1/spool is called with a `price` value
- **THEN** the spool is stored with the given price

#### Scenario: Spool created without price
- **WHEN** POST /api/v1/spool is called without a `price` field
- **THEN** the spool is stored with `price: None`

#### Scenario: Spool price updated
- **WHEN** PATCH /api/v1/spool/<id> is called with a new `price` value
- **THEN** the spool's price is updated to the new value

#### Scenario: Spool price cleared
- **WHEN** PATCH /api/v1/spool/<id> is called with `price: null`
- **THEN** the spool's price is set to None

### Requirement: Derive price per gram in spool response
When a spool has a `price`, the system SHALL derive and return `price_per_gram` in the `SpoolResponse`. The denominator SHALL be `net_weight` when set, otherwise `initial_weight`. The derived value is never stored.

#### Scenario: Price per gram derived with net_weight
- **WHEN** a spool is retrieved and the spool has both `price` and `net_weight`
- **THEN** `price_per_gram = price / net_weight` is included in the response

#### Scenario: Price per gram derived with initial_weight fallback
- **WHEN** a spool is retrieved and the spool has `price` but no `net_weight`
- **THEN** `price_per_gram = price / initial_weight` is included in the response

#### Scenario: No price_per_gram when price absent
- **WHEN** a spool is retrieved and `price` is None
- **THEN** `price_per_gram` is None in the response

### Requirement: Display price field in spool create/edit dialog
The spool create and edit dialogs SHALL include an optional numeric `Price` input field. An empty input is valid and means "no price recorded".

#### Scenario: User enters a price
- **WHEN** the user fills in the Price field and submits the spool form
- **THEN** the spool is created or updated with the entered price

#### Scenario: User leaves price empty
- **WHEN** the user leaves the Price field empty and submits the spool form
- **THEN** the spool is created or updated with `price: null`

### Requirement: Display price-per-gram as a sortable column in the spool list
The spool list table SHALL include a `Price/g` column that shows the derived `price_per_gram` for each spool, formatted as a currency amount using the `currency_symbol` setting and locale-aware `Intl.NumberFormat`. The column SHALL be sortable. Spools without a price SHALL display `—` in the column.

#### Scenario: Price per gram displayed
- **WHEN** a spool has `price_per_gram` in the response
- **THEN** the `Price/g` column shows the value formatted as a currency amount

#### Scenario: No price shows dash
- **WHEN** a spool has no price
- **THEN** the `Price/g` column shows `—`

#### Scenario: Column is sortable
- **WHEN** the user clicks the `Price/g` column header
- **THEN** the spool list is sorted by `price_per_gram` ascending or descending
