## MODIFIED Requirements

### Requirement: Currency amounts respect the `currency_symbol` setting override
When a currency amount is displayed, the system SHALL consult the `currency_symbol` setting. If the setting value is non-empty, it SHALL be used as a display symbol and positioned according to the browser locale using `Intl.NumberFormat.formatToParts()` with `currency: "USD"` as a position probe: the `currency` part SHALL be replaced with the user's symbol, preserving locale-specific spacing and placement (prefix or suffix). The numeric part SHALL be formatted with `minimumFractionDigits: 2` and `maximumFractionDigits: 2`. If the setting is empty or absent, the system SHALL format the amount with `Intl.NumberFormat` in `style: "decimal"` with 2 fractional digits (no symbol).

#### Scenario: Symbol positioned as suffix in European locale
- **WHEN** `currency_symbol` is `"€"` and the browser locale is `de-DE` and a price of `10.0` is displayed
- **THEN** the displayed value is `"10,00 €"` (symbol suffix with non-breaking space, locale decimal)

#### Scenario: Symbol positioned as prefix in US locale
- **WHEN** `currency_symbol` is `"$"` and the browser locale is `en-US` and a price of `10.0` is displayed
- **THEN** the displayed value is `"$10.00"` (symbol prefix, no space)

#### Scenario: Custom symbol overrides USD probe, position follows locale
- **WHEN** `currency_symbol` is `"PLN"` and the browser locale is `pl-PL` and a price of `49.9` is displayed
- **THEN** the `PLN` symbol is placed at the position determined by the locale (suffix for `pl-PL`), with locale-formatted decimal

#### Scenario: Empty symbol omits symbol entirely
- **WHEN** `currency_symbol` is empty or absent and a price of `49.9` is displayed
- **THEN** the displayed value is a locale-formatted decimal with 2 fractional digits and no currency symbol

## MODIFIED Requirements

### Requirement: Price column uses per-kilogram unit
The spool list table's price column SHALL display the price per kilogram (`price_per_kg`), not per gram. The column header SHALL read `"Price/kg"`. The API field providing this value SHALL be named `price_per_kg` and its value SHALL equal `spool.price / net_weight_kg` (i.e. `spool.price / net_weight_grams * 1000`).

#### Scenario: Price per kg displayed in spool list
- **WHEN** a spool has `price = 20.0` and `net_weight = 1000 g` and the browser locale is `en-US` and `currency_symbol` is `"$"`
- **THEN** the Price/kg cell displays `"$20.00"`

#### Scenario: Price per kg is null when no price set
- **WHEN** a spool has no price set
- **THEN** the `price_per_kg` API field is `null` and the cell is empty
