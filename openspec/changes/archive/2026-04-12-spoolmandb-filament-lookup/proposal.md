## Why

Entering filament data manually is tedious and error-prone. SpoolmanDB is a community-maintained, CORS-safe static JSON database (~800+ entries) of real-world filament products with accurate physical properties — making it possible to look up and auto-fill filament details directly from the browser without any server-side proxy.

## What Changes

- Add an inline "Search filament database" panel to the **Filament Create** form — user types a query, results filter from locally-cached SpoolmanDB JSON, selecting an entry auto-fills manufacturer, material, diameter, density, print temp, and bed temp
- Add the same search panel to the **Filament Edit** form — same behaviour, overwrites current values (user can still modify after)
- Add the same search panel to the **Spool Create** form — selecting an entry auto-fills color, color name, and net weight; if no matching filament exists in the local database (matched on manufacturer + material + diameter), one is created automatically via the API and the user is notified
- Add a client-side SpoolmanDB cache utility (localStorage, 24h TTL, ETag-based conditional re-fetch, offline fallback)
- Add material string parsing to split SpoolmanDB composite materials (e.g. `"PLA+"`, `"PETG-CF"`) into a base `MaterialType` + `material_modifier`

## Capabilities

### New Capabilities

- `spoolmandb-lookup`: Search and auto-fill filament/spool forms from SpoolmanDB; includes caching, material mapping, and auto-create-filament flow in spool create

### Modified Capabilities

_(none — no existing spec requirements change)_

## Impact

- **Frontend only** — all changes are in `crates/spoolman-client/`
- New utility module for SpoolmanDB fetch + localStorage cache
- `pages/filament.rs` — `FilamentCreate` and `FilamentEdit` components gain a search panel
- `pages/spool.rs` — `SpoolCreate` component gains a search panel with auto-filament-create side effect
- New dependency: `gloo-storage` (or `web-sys` localStorage API) for localStorage access in WASM; `gloo-net` already in use for HTTP
- External data source: `https://donkie.github.io/SpoolmanDB/filaments.json` (GitHub Pages, read-only, no auth)
