## Why

Leptos has deprecated `leptos::prelude::create_effect` in favour of `Effect::new()`. Keeping deprecated API calls produces compiler warnings and risks breakage on future Leptos version bumps.

## What Changes

- Replace all 11 call-sites of `create_effect(move |_| { … })` with `Effect::new(move |_| { … })` across the `spoolman-client` crate.
- Update import statements where `create_effect` is imported from `leptos::prelude` to instead use `leptos::prelude::Effect`.

Affected files:
- `crates/spoolman-client/src/app.rs` (1 call-site)
- `crates/spoolman-client/src/state.rs` (4 call-sites)
- `crates/spoolman-client/src/components/layout.rs` (1 call-site)
- `crates/spoolman-client/src/pages/filament.rs` (1 call-site)
- `crates/spoolman-client/src/pages/settings.rs` (2 call-sites)
- `crates/spoolman-client/src/pages/spool.rs` (2 call-sites)

## Capabilities

### New Capabilities

_(none — this is a pure code-hygiene change with no new user-facing behaviour)_

### Modified Capabilities

_(none — no spec-level requirements change; this is an internal implementation update)_

## Impact

- **Code**: `spoolman-client` crate only; no server or types crate changes.
- **API**: No API surface change.
- **Dependencies**: No `Cargo.toml` changes; `Effect` is already exported from `leptos::prelude` in the current Leptos version in use.
- **Build**: Eliminates deprecation warnings; no behaviour change expected.
