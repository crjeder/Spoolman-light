## Why

Numbers, weights, and dates are currently displayed with hardcoded Rust format strings (e.g. `{:.1}g`, `%Y-%m-%d`), ignoring the user's locale. The browser's `Intl` API provides locale-aware formatting for free — using it makes values look natural to users worldwide without requiring manual locale detection.

## What Changes

- Dates (`registered`, `first_used`, `last_used`) displayed in spool and filament views are formatted with `Intl.DateTimeFormat` using the browser locale instead of the hardcoded ISO `%Y-%m-%d` string.
- Weight and density numbers displayed in tables and detail views are formatted with `Intl.NumberFormat` (correct decimal separators, thousands grouping) using the browser locale.
- A shared Rust/WASM formatting module wraps `js_sys` / `web_sys` calls to the `Intl` API and is used consistently across all display sites.
- The existing `currency_symbol` setting acts as an override: when non-empty, it is prepended as a literal prefix to currency amounts rather than using `Intl`'s currency symbol. When empty/unset, `Intl.NumberFormat` with `style: "currency"` and the browser's locale determines the symbol and placement.
- No changes to the data model or API.

## Capabilities

### New Capabilities

- `intl-formatting`: Locale-aware formatting of numbers, weights, dates, and currency amounts via the browser `Intl` API, with `currency_symbol` setting as an override for the currency symbol.

### Modified Capabilities

*(none — no existing spec-level behavior changes)*

## Impact

- **Frontend only** — `spoolman-client` crate.
- New WASM helper module (e.g. `crates/spoolman-client/src/format.rs`) calling `js_sys::Intl` / `web_sys`.
- All display sites in `pages/spool.rs` and `pages/filament.rs` that currently use `format!("{:.1}g", …)`, `format!("{:.3}", …)`, `.format("%Y-%m-%d")`, etc. are updated to call the new helpers.
- The `currency_symbol` setting is read from app-wide state (or passed through context) wherever currency amounts are displayed.
- New dependency on `js-sys` (already a transitive Leptos dep; may need explicit declaration).
