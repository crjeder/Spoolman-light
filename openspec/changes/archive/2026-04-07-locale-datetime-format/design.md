## Context

All displayed date values pass through `format_date` in `format.rs`, which calls a thin `wasm_bindgen` JS shim (`sm_format_date_medium`) hardcoded to `dateStyle: "medium"`. There is no time component. Other per-user preferences (currency symbol, diameter defaults, color algorithm) are stored as string key/value pairs via `PUT /api/v1/settings/{key}`, loaded at app startup into Leptos context signals, and read reactively from any component.

The change adds two new setting keys — `date_format` and `time_format` — following the exact same pattern as `currency_symbol` to keep the implementation consistent and minimal.

## Goals / Non-Goals

**Goals:**
- Let users choose a date style (`short` / `medium` / `long` / `full`) persisted as `date_format`.
- Let users optionally add a time component (`none` / `short` / `medium`) persisted as `time_format`.
- Reactive update: changing and saving the setting reflects immediately in every date cell without a page reload.
- Preserve the current `dateStyle: "medium"` / no-time default for existing users who have not set the keys.

**Non-Goals:**
- Custom format strings (e.g. `"MM/DD/YYYY"`) — the `Intl.DateTimeFormat` `dateStyle`/`timeStyle` enum covers the common cases without the localisation complexity of arbitrary patterns.
- Formatting of `<input type="date">` / `<input type="datetime-local">` form fields — those remain machine-format as today.
- Server-side validation of the setting values — the server stores any string; the client enforces the allowed set via `<select>`.
- Time-zone selection — all displayed times remain UTC (current behaviour via `timeZone: 'UTC'` in the shim).

## Decisions

### 1 — Parameterise the JS shim rather than add a new one

**Decision:** Replace `sm_format_date_medium(timestamp_ms)` with `sm_format_date(timestamp_ms, date_style, time_style)` where `time_style` is the empty string to omit the `timeStyle` option entirely.

**Alternatives considered:**
- *Add a second shim* (`sm_format_datetime_short`, etc.) per combination — combinatorial explosion; one parameterised shim is cleaner.
- *Use `web_sys::js_sys`* to call `Intl.DateTimeFormat` directly from Rust — more idiomatic WASM, but significantly more boilerplate for no user-visible benefit at this scale.

**Rationale:** The inline-JS shim approach already works and is tested. A single parameterised shim keeps the diff small and the call sites identical in shape.

### 2 — Two context signals, seeded in `app.rs`

**Decision:** Add `date_format: RwSignal<String>` and `time_format: RwSignal<String>` to the existing app-level context (alongside `currency_symbol`). Seed them from fetched settings in `app.rs` with defaults `"medium"` and `"none"` respectively.

**Alternatives considered:**
- *Derive from `LocalResource`* inside each page — would require threading the resource through the component tree or re-fetching per page; context signals are already the established pattern.
- *Store as a struct* (`DateTimeFormatSettings`) — premature abstraction for two strings.

**Rationale:** Matches the existing pattern exactly; no new abstractions needed.

### 3 — `format_date` reads signals directly

**Decision:** Change `format_date(dt: DateTime<Utc>)` to `format_date(dt: DateTime<Utc>, date_style: &str, time_style: &str)` and pass the signal values at each call site.

**Alternatives considered:**
- *Access context signals inside `format_date`*  — functions in `format.rs` have no Leptos context access (they're not components or effects); this would require a Leptos dependency in `format.rs`, coupling the formatting utility to the reactive runtime.
- *Return a closure / reactive wrapper* — over-engineered; call sites already read signals from context.

**Rationale:** Keeps `format.rs` as a pure utility with no Leptos dependency; call sites are already inside reactive closures (`move ||`) so reading two extra signals is trivial.

## Risks / Trade-offs

- **[Existing call sites]** — Each `format::format_date(...)` call must be updated to pass two extra arguments. There are currently ~5 call sites (spool table cell, spool detail, spool first/last used, filament table cell). Missing one will cause a compile error, so the risk is caught at build time, not runtime.
- **[Signal not yet in context]** — If `format_date` is called before `app.rs` seeds the signals (e.g. in a test harness), `use_context` would return `None`. Mitigation: provide a `get_date_format_or_default()` accessor that returns `("medium", "none")` when the context is absent.
- **[`time_style: "none"` passed to Intl]** — Passing `timeStyle: "none"` to `Intl.DateTimeFormat` would throw a `RangeError`; the shim must omit the `timeStyle` key entirely when the value is `"none"` / empty string. The parameterised shim handles this with a conditional.

## Migration Plan

1. Existing users have no `date_format` / `time_format` keys stored → signals default to `"medium"` / `"none"` → output is identical to today. No data migration needed.
2. Deploy: standard Docker image rebuild; no server changes beyond the existing settings store.
3. Rollback: revert the client build; old code omits the new keys and the server retains the stored values harmlessly.
