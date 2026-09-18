# TODO

Items to address. Move completed items to [CHANGELOG.md](CHANGELOG.md) under the appropriate release.

## Enhancements
- [ ] NFC / QR sticker integration — [OpenSpoolMan](https://github.com/drndos/openspoolman) or [OpenTag3D](https://opentag3d.com/) compatible; spool NFC URL already maps to `/api/v1/spool/<id>`
- [ ] filament create/edit + spool create: SpoolmanDB lookup — fetch https://donkie.github.io/SpoolmanDB/filaments.json, cache in localStorage (24h TTL + ETag), client-side search, auto-fill filament fields; in spool create auto-create missing filament and notify user
- [ ] filament/spool: filamentcolors.xyz color lookup — deferred: API CORS headers absent from their Django app, direct WASM fetch will be blocked; needs a server-side proxy endpoint (/api/v1/proxy/filamentcolors) before this is viable
- [x] test on mobile
- [x] multi-color search? — spool color filter matches any of a spool's colors and sorts by closest ΔE (PR #91)
- [x] add color button next to the existing color field — "+"/"−" multi-color editor in spool create/edit (max 4), server guard. Fixed a WASM panic where "+" disposed row signals instead of adding a row (non-keyed list re-render tore down previous rows' reactive scope); switched to `<For>` keyed by a stable per-row id. openspec: multi-color-spool-edit
- [x] reload database button (in settings?) — shipped in 1.7.0 (`POST /api/v1/reload` + Settings button)
- [ ] integrate https://github.com/Disane87/spoolman-filament-swatch
- [ ] There is GET /api/v1/export but no way back in. A POST import or a restore button in Settings would pair with the existing reload button

## Defects
- [~] "HTTP 500: Internal Server Error" error on edit — root cause found: `/data` volume created root-owned, non-root container user can't write; fixed in Dockerfile, not yet verified against a real Docker build. -> use the username spoolman
- [x] date selector not always defaults to today — empty first_used/last_used pickers show today; untouched default not persisted 
- [x] detail view does not show if it's spool or filament
- [x] click on column 1 in spool view should lead to spool details (not filament)
- [x] color name on spool? — already wired: create/edit form input, list column, detail row, persisted via API
- [x] drop-down fields in edit spool / filament do not show the current value
- [x] edit filament dialog: should be Edit <current filament> not "Filament"
- [x] "search filament" only in new filament dialog not in edit — SpoolmanDbSearch added to FilamentEdit in 3e1004c
- [x] new spool should lookup local filaments, too — DB search miss no longer disables the filament selector when a local filament matches; best local match is auto-selected
- [ ] date format setting not respected — could not reproduce: `date_format` is loaded in `App` into `DateFormat` context and passed to `format::format_date` at every display site (spool/filament list + show). Needs a concrete repro (which view, what setting).
- [x] enforce location — server rejects spool create without a valid location_id (422); create/update validate the location exists
- [x] Location drop-down does not show the current location in edit spool — `<option>` now gets `selected` per-item (Suspense-rendered options ignored the select's `prop:value`)
- [~] keep filters active until session end or user deactivates them. add a remove all filters button — implemented (sessionStorage persistence + "Clear filters" button, spool + filament lists); manual/Playwright verification pending. openspec: persist-spool-filters
- [x] remove the black square in the heading of the Material column after selecting a filter — dropped the `\u{25A0}` marker; the select already shows the active value
- [x] color picker: add hex code input — `#rrggbb` text field beside the swatch in spool create/edit, synced to the color signal
- [ ] repair icons in README
