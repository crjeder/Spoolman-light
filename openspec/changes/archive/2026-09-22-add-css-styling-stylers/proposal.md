## Superseded

This change's `stylers` proc-macro approach was never implemented. The actual styling requirement was met independently via a plain CSS file (`style/spoolman.css`, wired up through `Leptos.toml`'s `style-file` setting), added across several unrelated feature commits (multi-color editor, color search, finish badges, colour swatch view). That file already covers every selector this change lists (`.app-shell`, `.sidebar`, `.data-table`, `.color-swatch`, `body.dark`, buttons, etc.) and is verified working. No `stylers` code was ever added; adding it now would just duplicate the existing CSS for no visual benefit. Archived as superseded rather than implemented.

## Why

The Spoolman-light frontend is completely unstyled: all layout class names (`.app-shell`, `.sidebar`, `.main-content`, `.data-table`, `.color-swatch`, `.dark`) are applied in Leptos components but have no CSS rules. Using the `stylers` crate enables scoped, compile-time-checked CSS co-located with components, fixing the blank UI and establishing a styling foundation for future work.

## What Changes

- Add `stylers = "0.3.2"` dependency to `spoolman-client/Cargo.toml`
- Create scoped CSS style blocks in each Leptos component/page using `stylers::style!` macro
- Define styles for the app shell layout, sidebar navigation, main content area, data table, color swatch, and dark mode overrides
- Reference the generated CSS via `use_style!` in each component

## Capabilities

### New Capabilities
- `frontend-styling`: Scoped CSS for the Leptos WASM frontend using the `stylers` crate — app shell layout, sidebar, data table, color swatch, and dark mode

### Modified Capabilities
<!-- No existing spec-level behavior changes — this is purely presentational -->

## Impact

- **Files changed:** `crates/spoolman-client/Cargo.toml`, `crates/spoolman-client/src/app.rs`, `crates/spoolman-client/src/pages/spool.rs`, `crates/spoolman-client/src/pages/filament.rs`, `crates/spoolman-client/src/components/` (nav, table, swatch components)
- **Dependencies:** `stylers 0.3.2` added to the client crate; no server-side changes
- **APIs:** None — purely frontend/presentational
- **Build:** `stylers` uses a proc-macro; compatible with `cargo-leptos` WASM builds; no OpenSSL or Windows build issues expected
