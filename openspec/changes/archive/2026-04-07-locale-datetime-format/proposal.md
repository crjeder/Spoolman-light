## Why

Date values in the spool and filament tables are currently formatted using a hardcoded `dateStyle: "medium"` style. Users who prefer a different format (short, long, or full) or want to include the time component have no way to change this. Adding a `date_format` setting lets each user choose their preferred style while keeping the locale-aware `Intl.DateTimeFormat` approach.

## What Changes

- Add a `date_format` setting key (stored server-side like other settings) with the allowed values `"short"`, `"medium"` (default), `"long"`, and `"full"`.
- Add a `time_format` setting key with allowed values `"none"` (default — date only, current behaviour), `"short"`, and `"medium"`.
- The `format_date` helper in `format.rs` reads the active format signals and passes the selected `dateStyle` / `timeStyle` options to `Intl.DateTimeFormat`; with defaults it produces the exact same output as today.
- The Settings page gains two new `<select>` controls: "Date format" and "Time format".
- On save, the new keys are persisted alongside existing settings.
- App-level context signals for `date_format` and `time_format` are added (parallel to `currency_symbol`) so that the table re-renders reactively without a page reload after saving.

## Capabilities

### New Capabilities

- `datetime-format-setting`: User-configurable date and time display format stored as a server setting; controls `Intl.DateTimeFormat` `dateStyle`/`timeStyle` options used throughout the UI.

### Modified Capabilities

- `intl-formatting`: Date formatting requirements change — `dateStyle` is now driven by the `date_format` setting (default `"medium"`) and an optional `timeStyle` driven by `time_format` (default `"none"` / omitted). Existing locale-first behaviour is preserved; the setting is an override, not a replacement.

## Impact

- `crates/spoolman-client/src/format.rs` — `sm_format_date_medium` JS shim and `format_date` Rust wrapper replaced by a parameterised version that accepts `dateStyle` and optional `timeStyle` strings.
- `crates/spoolman-client/src/state.rs` — two new context signals: `date_format: RwSignal<String>` and `time_format: RwSignal<String>`.
- `crates/spoolman-client/src/app.rs` — seed the new signals from persisted settings on startup (same pattern as `currency_symbol`).
- `crates/spoolman-client/src/pages/settings.rs` — two new `<select>` controls; two additional `put_setting` calls on submit.
- `crates/spoolman-client/src/pages/spool.rs` and `filament.rs` — pass the context signals into `format_date`.
- `openspec/specs/intl-formatting/spec.md` — delta required: date format requirement updated.
- No server-side changes beyond storing two new string keys — the existing `PUT /api/v1/settings/{key}` endpoint handles them.
