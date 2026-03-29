## Context

The Spoolman Light frontend (`spoolman-client`) is a Leptos WASM application compiled to WebAssembly and run in the browser. All display formatting currently uses Rust's `format!` macro with hardcoded patterns:

- Weights: `format!("{:.0}g", w)`, `format!("{:.1}g", w)`
- Density: `format!("{:.3}", d)`
- Dates: `.format("%Y-%m-%d")` (chrono)
- No currency amounts are currently displayed, but the `currency_symbol` setting (`€` by default) exists in the data store for future use.

The browser exposes the JavaScript `Intl` API (part of the ECMAScript standard) which handles locale-aware number and date formatting. WASM modules running in the browser can call JS APIs via `js-sys` and `web_sys` bindings.

## Goals / Non-Goals

**Goals:**
- Wrap `Intl.NumberFormat` and `Intl.DateTimeFormat` in a thin Rust helper module.
- Replace all hardcoded display-formatting calls in `pages/spool.rs` and `pages/filament.rs` with calls to the new helpers.
- Honour the `currency_symbol` setting as a literal prefix override when a currency amount is displayed.
- Keep the change purely in `spoolman-client`; no server, API, or data model changes.

**Non-Goals:**
- Adding price/cost fields to the data model (separate change).
- Server-side locale detection or Accept-Language header handling.
- Formatting values inside `<input>` elements (form inputs must remain plain machine-format numbers for parse-back).
- Changing how weight values are stored or computed.

## Decisions

### 1. Call `Intl` directly via `js_sys` rather than a Rust locale crate

**Decision:** Use `js_sys::Intl::NumberFormat` and `js_sys::Intl::DateTimeFormat` (or their `web_sys` equivalents) by constructing JS objects through `wasm-bindgen`.

**Rationale:** The browser already has a correct, maintained, locale-database–backed `Intl` implementation. Pure-Rust locale crates (`pure-rust-locales`, `icu`) add significant binary size to the WASM bundle and require bundling their own CLDR data. Since this app already runs in a browser, using the native `Intl` API is zero-cost in terms of bundle size and always up-to-date.

**Alternative considered:** `num-format` crate — locale data is Rust-side, avoids JS interop overhead, but adds ~200 KB WASM and still doesn't handle dates.

### 2. A thin `format.rs` module with free functions

**Decision:** Create `crates/spoolman-client/src/format.rs` exposing:
- `pub fn format_weight(grams: f32) -> String` — uses `Intl.NumberFormat` with `style: "decimal"`, `maximumFractionDigits: 1`, appends `" g"`.
- `pub fn format_density(g_per_cm3: f32) -> String` — 3 decimal places, appends `" g/cm³"`.
- `pub fn format_date(date: NaiveDate) -> String` — uses `Intl.DateTimeFormat` with `dateStyle: "medium"` (locale default short date, e.g. "29 Mar 2026" or "Mar 29, 2026").
- `pub fn format_currency(amount: f64, symbol_override: &str) -> String` — if `symbol_override` is non-empty, formats the number with `Intl.NumberFormat(style:"decimal", minimumFractionDigits:2)` and prepends the override symbol; otherwise uses `Intl.NumberFormat(style:"currency", currency: detected_or_default)`.

**Rationale:** A single module localises the JS interop risk, is easy to unit-test, and keeps call sites clean (`format::format_weight(w)` vs repeated inline `format!` calls).

### 3. Currency symbol override takes precedence unconditionally

**Decision:** If `currency_symbol` is non-empty (even a single space), skip `Intl` currency style entirely and use the raw string as a prefix.

**Rationale:** Users who set a custom symbol have opted out of locale-driven placement and symbol choice. Trying to merge the two (e.g. replace Intl's symbol but keep its placement) would be fragile and unexpected.

### 4. Date style: `"medium"` not `"short"` or `"long"`

**Decision:** Use `dateStyle: "medium"` for `Intl.DateTimeFormat`.

**Rationale:** `"short"` often produces fully-numeric ambiguous dates (e.g. `3/29/26`). `"medium"` gives an unambiguous human-readable form (`Mar 29, 2026`, `29 mars 2026`) without being overly verbose.

### 5. Locale: use browser default (`undefined`)

**Decision:** Pass `undefined` as the locale argument to `Intl` constructors, letting the browser use `navigator.language`.

**Rationale:** Spoolman Light has no per-user locale setting; using the browser's own locale is the least-surprise approach.

## Risks / Trade-offs

- **JS interop verbosity** — constructing `Intl` options objects via `js_sys::Object` and `Reflect::set` is boilerplate-heavy in Rust. Mitigation: keep it inside `format.rs` so the mess is isolated; add a helper macro if the pattern repeats more than three times.
- **`js-sys` version pinning** — `js-sys` must match the `wasm-bindgen` version used by Leptos. Mitigation: add it as a workspace dependency inheriting the version already pulled in by Leptos (check `Cargo.lock`).
- **Date formatting differs between browsers/locales** — `"medium"` style varies. Mitigation: this is intentional and desirable; tests should assert the format is non-empty and parseable rather than matching an exact string.
- **No price fields yet** — the currency formatter is written but has no call sites today. Mitigation: include at least one smoke call (or document it) so it's exercised before the price feature lands.
