## Context

The `Spool` struct in `spoolman-types` currently tracks weight (initial, current, net) but has no cost field. `SpoolResponse` already derives `used_weight` and `remaining_filament` from stored fields — `price_per_gram` follows the same pattern. The `intl-formatting` spec already defines how currency amounts are displayed using `currency_symbol` + `Intl.NumberFormat`.

## Goals / Non-Goals

**Goals:**
- Add `price: Option<f32>` to the `Spool` struct and all create/update request types
- Derive `price_per_gram: Option<f32>` in `SpoolResponse`
- Surface `price` in the create/edit spool dialog
- Add a sortable `Price/g` column to the spool list table, formatted with the existing currency helpers
- Remain backwards-compatible: existing spools without `price` deserialize to `None`

**Non-Goals:**
- Currency conversion (price is stored as a bare number; currency semantics are in `currency_symbol` setting only)
- Price history or tracking price changes over time
- Price on filament records
- Any new settings

## Decisions

### D1: Store price on Spool, not Filament

Price can vary per purchase batch (same filament, different price on re-order). Storing it on `Spool` gives per-spool purchase cost, which is the correct granularity for cost tracking.

**Alternative:** Per-filament price — rejected because users often pay different prices for the same filament type.

### D2: `price: Option<f32>` — optional, no default

Not every user wants cost tracking. Making it optional means the field can be omitted on create/update without breaking anything.

**Alternative:** `f32` with default `0.0` — rejected because `0.0` is ambiguous (free spool vs not-set).

### D3: Derive `price_per_gram` in `SpoolResponse`, not stored

Follows the same pattern as `used_weight` and `remaining_filament`. Computed at read time from `price` and the weight denominator — never redundantly stored.

Weight denominator: use `net_weight` when present (actual filament mass), otherwise `initial_weight` (best proxy when net weight unknown).

`price_per_gram = price / denominator_weight`

**Alternative:** Store `price_per_gram` — rejected, derived values are never stored.

### D4: Column label `Price/g` with currency-formatted value

Matches the app convention of showing the unit in the column header (like `Weight (g)`). The numeric value uses the existing `format_currency` helper from `intl-formatting`.

**Alternative:** `€/kg` — rejected because the app's weight unit is grams; mixing units would be confusing. Users can mentally multiply by 1000.

### D5: No data migration needed

`serde` defaults missing `price` fields to `None` on deserialization. Schema version does not need bumping for an optional additive field.

## Risks / Trade-offs

- [Float precision] `price / weight` may accumulate floating-point error → Mitigation: display with 4–6 significant digits; precision is sufficient for cost display
- [Missing net_weight] Using `initial_weight` as denominator includes spool tare, slightly overstating filament content → Mitigation: documented limitation; users should enter `net_weight` for accurate cost-per-gram; column shows `—` when `price` is absent

## Migration Plan

No migration required. Adding `price: Option<f32>` with `#[serde(default)]` is fully backwards-compatible with existing `spoolman.json` data files.
