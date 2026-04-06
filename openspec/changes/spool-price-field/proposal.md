## Why

Users want to track what they paid for a spool so they can estimate print costs. The app already supports currency formatting via the `currency_symbol` setting and locale-aware number formatting, making this a natural extension.

## What Changes

- Add an optional `price` field (`Option<f32>`) to the `Spool` entity and data model
- Expose `price` in spool create/update API requests and spool response bodies
- Add a `Price` input to the spool new/edit dialog
- Derive `price_per_gram` in spool responses when both `price` and `net_weight` are set; fall back to `price / initial_weight` when `net_weight` is absent
- Display a sortable `Price/g` column in the spool list, formatted with the existing `currency_symbol` / `Intl` currency formatting

## Capabilities

### New Capabilities

- `spool-price`: Stores and displays spool purchase price; derives and displays price-per-gram in a sortable column

### Modified Capabilities

- `data-model`: Spool entity gains an optional `price` field
- `spool-management`: Create/update/list requests and responses include `price` and derived `price_per_gram`

## Impact

- `spoolman-types`: `Spool` struct and `CreateSpoolRequest` / `UpdateSpoolRequest` gain `price: Option<f32>`
- `spoolman-server`: Spool response serialization adds `price` and derived `price_per_gram`; no new endpoints
- `spoolman-client`: New `Price` field in spool create/edit form; new sortable `Price/g` table column using existing currency formatting helpers
- `intl-formatting`: No requirement changes — `currency_symbol` / Intl currency format already specifies how currency amounts are displayed
- Data file: new optional field — existing spools deserialize with `price: None` (backwards-compatible)
