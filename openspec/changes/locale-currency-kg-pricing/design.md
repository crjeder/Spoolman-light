## Context

Currency formatting in `format.rs` uses a `symbol_override` string that is always prepended as a literal prefix (`format!("{}{}", symbol, number)`). This is wrong for European locales where the symbol follows the value (`10.00 €`). The browser's `Intl.NumberFormat.formatToParts()` API can determine correct symbol placement without bundling locale data.

The API response field `price_per_gram` is computed as `spool.price / net_weight_grams`. Filament is sold by the kilogram, so `price_per_kg` (`price / net_weight_kg = price_per_gram * 1000`) is the natural unit for display and comparison.

The spool detail view (`SpoolShow`) currently shows weight metrics but omits price, price-per-kg, and net weight — information the user needs to assess the cost of remaining filament.

## Goals / Non-Goals

**Goals:**
- Currency symbol position (prefix vs. suffix) follows the browser locale automatically.
- The `price_per_gram` API field is renamed `price_per_kg` with value multiplied by 1000.
- Spool detail view shows: purchase price, price per kg, net weight.

**Non-Goals:**
- No currency code (ISO 4217) lookup — the user's `currency_symbol` setting remains a free-form display string.
- No change to the settings page or to how `currency_symbol` is stored.
- No change to form inputs — they remain plain numeric fields.
- No change to the filament detail view.

## Decisions

### Decision: Use `Intl.NumberFormat.formatToParts()` with USD as a position probe

**Problem:** We have a user-supplied display symbol (e.g. `€`), not an ISO currency code. `Intl.NumberFormat` requires a currency code to use `style: "currency"`.

**Chosen approach:** Format the number with `style: "currency", currency: "USD"` using `formatToParts()`, then replace the `currency` part with the user's symbol. This correctly inherits locale-specific spacing (e.g. `fr-FR` adds a non-breaking space before `€`) and symbol placement without hardcoding any locale table.

```js
export function sm_format_currency(amount, symbol) {
    const parts = new Intl.NumberFormat(undefined, {
        style: 'currency',
        currency: 'USD',
        minimumFractionDigits: 2,
        maximumFractionDigits: 2,
    }).formatToParts(amount);
    return parts.map(p => p.type === 'currency' ? symbol : p.value).join('');
}
```

When `symbol` is empty the Rust wrapper returns the plain locale-decimal string (existing behaviour, no symbol).

**Alternative considered:** Detect position by checking if the `$` appears before or after the numeric digits in the formatted string, then manually prepend/append the user symbol. Rejected — fragile, doesn't handle locale-specific spacing.

**Alternative considered:** Require users to set an ISO currency code instead of a symbol. Rejected — breaking change to existing settings; many users have stored free-form symbols.

### Decision: Rename `price_per_gram` → `price_per_kg` in `SpoolResponse`

**Rationale:** The field is exposed in the JSON API. Multiplying by 1000 in `SpoolResponse::new()` and renaming the field is a single-source change. All consumers (client, tests) update their references. The API has no versioning contract yet, so this is acceptable.

### Decision: Show net_weight, price, price_per_kg in `SpoolShow` read-only view

**Rationale:** These three fields are already present in the `SpoolResponse` — displaying them is purely additive UI. Net weight is shown formatted as grams (same as other weights). Price and price-per-kg use `format_currency`. Fields with `None` values display as `"—"` to be consistent with other optional fields in the detail grid.

## Risks / Trade-offs

- **`formatToParts` availability** — supported in all modern browsers (Chrome 57+, Firefox 52+, Safari 10.1+). Not a concern for a self-hosted tool.
- **USD as position probe** — if a locale positions USD differently from other currencies (rare), the symbol placement will follow USD convention, not the local currency's convention. Acceptable given the free-form symbol approach.
- **API field rename is breaking** — any external client parsing `price_per_gram` will break. Currently no documented external consumers; acceptable at this stage.
- **Intl-formatting spec update** — the existing spec's "Default symbol (€) acts as override" scenario states the symbol is a prefix. That scenario must be updated to reflect locale-aware positioning.
