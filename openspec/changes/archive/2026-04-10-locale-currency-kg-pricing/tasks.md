## 1. API: Rename price_per_gram → price_per_kg

- [x] 1.1 In `crates/spoolman-types/src/responses.rs`: rename field `price_per_gram` to `price_per_kg` and multiply the computed value by `1000.0` (divide by kg, not grams)
- [x] 1.2 In `crates/spoolman-server/tests/spool.rs`: update the test `spool_price_returns_price_per_gram` — rename it, update field name to `price_per_kg`, and update expected value (×1000)

## 2. Client: Locale-aware currency formatting

- [x] 2.1 In `crates/spoolman-client/src/format.rs`: add JS function `sm_format_currency(amount, symbol)` using `Intl.NumberFormat.formatToParts()` with `currency: "USD"` as locale probe; replace the `currency` part with the user symbol
- [x] 2.2 In `format.rs`: update `format_currency(amount, symbol_override)` to call `sm_format_currency`; when symbol is empty, fall back to `sm_format_decimal(amount, 2, 2)`

## 3. Client: Spool list — update price column

- [x] 3.1 In `crates/spoolman-client/src/pages/spool.rs`: update the sort-field key from `"price_per_gram"` to `"price_per_kg"` (in the initial column list and the sort match arm)
- [x] 3.2 Update the column header label from `"Price/g"` to `"Price/kg"`
- [x] 3.3 Update the cell renderer to read `sr.price_per_kg` instead of `sr.price_per_gram`

## 4. Client: Spool detail — add price, price/kg, net weight rows

- [x] 4.1 In `SpoolShow` in `spool.rs`: add `"Net weight"` row — display `format_weight(nw)` when `spool.net_weight` is `Some`, else `"—"`
- [x] 4.2 Add `"Price"` row — display `format_currency(price, &cur_sym)` when `spool.price` is `Some`, else `"—"` (read `currency_symbol` reactive signal)
- [x] 4.3 Add `"Price/kg"` row — display `format_currency(ppkg, &cur_sym)` when `sr.price_per_kg` is `Some`, else `"—"`
- [x] 4.4 Ensure `currency_symbol` signal is accessible inside the `SpoolShow` component (add `let cur_sym = currency_symbol();` at the top)

## 5. Spec sync

- [x] 5.1 Apply delta to `openspec/specs/intl-formatting/spec.md`: replace the "Currency amounts respect the `currency_symbol` setting override" requirement with the locale-aware positioning version from the delta spec; add the "Price column uses per-kilogram unit" requirement
- [x] 5.2 Apply delta to `openspec/specs/spool-management/spec.md`: add the "Spool detail view displays price, price per kg, and net weight" requirement; update the "Derive weight metrics" requirement to use `price_per_kg`

## 6. Verification

- [x] 6.1 Run `cargo check -p spoolman-types -p spoolman-server` — no errors
- [x] 6.2 Run `cargo clippy -p spoolman-types -p spoolman-server` — no new warnings
- [ ] 6.3 Manually verify in browser: spool list shows "Price/kg" column with correct value; `€` appears after the number in a `de-DE` locale context; `$` appears before in `en-US`
- [ ] 6.4 Manually verify spool detail view shows Net weight, Price, and Price/kg rows
