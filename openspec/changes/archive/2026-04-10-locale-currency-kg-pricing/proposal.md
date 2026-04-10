## Why

Currency amounts are always displayed with the symbol as a literal prefix (e.g. `€10.00`), which is wrong for most European locales where the symbol follows the number (`10.00 €`). Additionally, price-per-gram is an impractical unit — filament is priced and sold per kilogram, and the spool detail view is missing price and net weight information that users need at a glance.

## What Changes

- Currency symbol is positioned according to the browser locale (prefix for `$`, suffix for `€`) instead of always prepending it as a literal string.
- The `price_per_gram` API field and UI column are renamed to `price_per_kg` (value multiplied by 1000); the column label changes from "Price/g" to "Price/kg".
- Spool detail view gains three new display fields: spool purchase price, price per kg, and net weight.

## Capabilities

### New Capabilities
*(none — all changes modify existing capabilities)*

### Modified Capabilities
- `intl-formatting`: Currency formatting rule changes — symbol is no longer always a prefix; locale-aware positioning is used instead. The `price_per_gram` display unit changes to `price_per_kg`.
- `spool-management`: Spool detail view gains price, price per kg, and net weight display fields. API response field `price_per_gram` renamed to `price_per_kg`.

## Impact

- `crates/spoolman-types/src/responses.rs` — rename `price_per_gram` → `price_per_kg`, multiply value by 1000
- `crates/spoolman-client/src/format.rs` — `format_currency`: position symbol by locale (suffix for most European locales, prefix for USD/GBP etc.)
- `crates/spoolman-client/src/pages/spool.rs` — update column key, label, sort key, and cell renderer; add price, price/kg, net weight to detail view
- `crates/spoolman-server/tests/spool.rs` — update test asserting `price_per_gram` → `price_per_kg` and expected value
- `openspec/specs/intl-formatting/spec.md` — update currency positioning requirement
- `openspec/specs/spool-management/spec.md` — add detail view requirements
