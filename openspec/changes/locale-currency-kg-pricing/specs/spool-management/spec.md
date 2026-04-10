## ADDED Requirements

### Requirement: Spool detail view displays price, price per kg, and net weight
The spool detail view (`SpoolShow`) SHALL display the following read-only fields when present:
- **Net weight**: the spool's `net_weight` in grams, formatted as a locale weight string. When absent, display `"—"`.
- **Price**: the spool's purchase `price`, formatted as a locale currency amount using the `currency_symbol` setting. When absent, display `"—"`.
- **Price/kg**: the derived `price_per_kg` value from the API response, formatted as a locale currency amount using the `currency_symbol` setting. When absent, display `"—"`.

These fields SHALL appear in the detail grid alongside existing weight and date fields.

#### Scenario: Net weight shown when present
- **WHEN** a spool has `net_weight = 1000` g
- **THEN** the detail view shows `"Net weight"` row with value `"1,000 g"` (locale-formatted)

#### Scenario: Net weight shows dash when absent
- **WHEN** a spool has no `net_weight`
- **THEN** the detail view shows `"Net weight"` row with value `"—"`

#### Scenario: Price shown with locale currency symbol
- **WHEN** a spool has `price = 24.99` and `currency_symbol` is `"€"` and the browser locale is `de-DE`
- **THEN** the detail view shows `"Price"` row with value `"24,99 €"`

#### Scenario: Price shown as dash when absent
- **WHEN** a spool has no price
- **THEN** the detail view shows `"Price"` row with value `"—"`

#### Scenario: Price per kg shown with locale currency symbol
- **WHEN** a spool has `price = 20.0` and `net_weight = 1000 g` and `currency_symbol` is `"$"` and browser locale is `en-US`
- **THEN** the detail view shows `"Price/kg"` row with value `"$20.00"`

#### Scenario: Price per kg shown as dash when absent
- **WHEN** a spool has no price or no net weight
- **THEN** the detail view shows `"Price/kg"` row with value `"—"`

## MODIFIED Requirements

### Requirement: Derive weight metrics
The system SHALL derive and return weight metrics in spool responses. Metrics are computed from stored fields, never stored redundantly. `net_weight` is read from the spool, not from the filament. The derived price metric SHALL be named `price_per_kg` (not `price_per_gram`) and SHALL equal `spool.price / (net_weight_grams / 1000.0)`, using `net_weight` when available and falling back to `initial_weight`.

#### Scenario: Used weight derived
- **WHEN** a spool is retrieved
- **THEN** `used_weight = initial_weight - current_weight` is included in the response

#### Scenario: Remaining filament derived when net_weight known
- **WHEN** a spool is retrieved and the spool has a `net_weight`
- **THEN** `remaining_filament = spool.net_weight - used_weight` is included in the response

#### Scenario: No remaining_filament when net_weight absent
- **WHEN** a spool is retrieved and `spool.net_weight` is `None`
- **THEN** `remaining_filament` is omitted (`None`) from the response

#### Scenario: Price per kg derived when price and weight known
- **WHEN** a spool has `price = 20.0` and `net_weight = 1000 g`
- **THEN** `price_per_kg = 20.0` is included in the response

#### Scenario: Price per kg uses initial_weight fallback
- **WHEN** a spool has `price = 20.0` and no `net_weight` and `initial_weight = 500 g`
- **THEN** `price_per_kg = 40.0` (20.0 / 0.5 kg) is included in the response

#### Scenario: Price per kg absent when no price set
- **WHEN** a spool has no price
- **THEN** `price_per_kg` is `null` in the response
