## Why

Spool inventory is browsed visually at least as often as it is browsed by name: "which greys do I still have?" is a colour question, and a seven-column table answers it badly. `Disane87/spoolman-filament-swatch` solves this as a separate web app that reads a Spoolman instance over the REST API — which needs CORS configuration, an external host, and duplicates filtering, theming and i18n that Spoolman-light already has.

Since colours live on `Spool` in this data model (`Filament` has none), a swatch board is not a new entity — it is a second rendering of the spool list. Building it natively reuses the existing fetch, filter, sort and pagination logic and costs one component plus a stylesheet block.

## What Changes

- A colour swatch view SHALL be added at `/colors`: a card grid where each card is filled with the spool's colour(s) and labelled with colour name, filament, remaining weight and location.
- A "Color" entry SHALL be added to the sidebar so the user can swap between the swatch board and the spool table.
- A `default_view` setting (`spool` | `color`, default `spool`) SHALL be added to the Settings page and SHALL decide which view the `/` route renders.
- The swatch view SHALL default to a hue-then-lightness ordering, with achromatic spools (greys, black, white) grouped last.
- `hue` SHALL also become a sortable field in the spool table, via the Color column header.
- The material filter SHALL become multi-select: a checkbox row above the swatch grid, and a shared filter set that the table's existing Material dropdown also writes to.
- Filters SHALL be shared between the two views; sort and pagination SHALL be per-view.

## Capabilities

### New Capabilities

- `spool-color-swatch-view`: colour card grid at `/colors`, sidebar entry, `default_view` setting, hue/lightness ordering.

### Modified Capabilities

- `spool-management`: the `/` route resolves to the view named by `default_view` rather than always rendering the spool table.
- `spool-material-filter`: the material filter becomes a set of selected materials rather than a single value.

## Impact

- `crates/spoolman-client/src/pages/spool.rs` — `SpoolList` gains a `mode` prop; material filter signal becomes a set; `hue` sort arm.
- `crates/spoolman-client/src/components/swatch_grid.rs` — new: card grid and material checkbox row.
- `crates/spoolman-client/src/utils/color.rs` — new: hue / chroma / lightness extraction for sorting (reuses the existing OkLab conversion).
- `crates/spoolman-client/src/state.rs` — `DefaultView` context; `use_table_state` gains a default-sort parameter.
- `crates/spoolman-client/src/app.rs` — `/colors` route, root view resolution, `default_view` load.
- `crates/spoolman-client/src/pages/settings.rs` — "Default view" selector.
- `crates/spoolman-client/src/components/layout.rs` — sidebar "Color" entry and its active state.
- `style/spoolman.css` — `.swatch-grid`, `.swatch-card`, `.material-checks` blocks.
- No server, API or data-model changes: `default_view` uses the existing `PUT /api/v1/setting/{key}` key-value store.
- `tests/e2e/` — new swatch-view spec; `navigation.spec.ts` unaffected.
