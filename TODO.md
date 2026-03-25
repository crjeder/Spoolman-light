# TODO

Items to address. Move completed items to [CHANGELOG.md](CHANGELOG.md) under the appropriate release.

## Pending

### Bugs (found via Playwright Docker test, 2026-03-24)

#### ~~B5 Delete Button in Locations~~ (Fixed)
- **Root cause:** `disabled=move || count > 0` inside `view!` — the `>` in `count > 0` was parsed as the closing `>` of the `<button` opening tag, so `on:click=...>"Delete"` became raw text content of the button (same class of bug as Pagination "Next →").
- **Fix applied:** Extracted to `let delete_disabled = count > 0;` before the `view!` block; used `disabled=delete_disabled`. Also added `.btn:disabled` CSS rule so disabled buttons stay visually distinct from unstyled pagination buttons.

#### ~~B6 sorting in list view~~ (Fixed)
- **Root cause:** `sort_field`/`sort_asc` signals were tracked by `ColHeader` for UI display only — no sort was ever applied to the data. `Remaining (g)` and `Location` used plain `<th>` with no sort button.
- **Fix applied:** Added `sorted` closure after `filtered` that reads `ts.sort_field`/`ts.sort_asc` and sorts by the active field using numeric comparison (not string). `page_items` now consumes `sorted()` instead of `filtered()`. Replaced plain `<th>` for Remaining (g) and Location with `ColHeader` using fields `remaining_weight` and `location`. `None` values sort last regardless of direction.

### B7 Color's alpha value is not used anywhere
- the color picker should allow to select an alpha value. if that's not possible add an extra selector elsewhere

### B8 server error in spool edit
- "HTTP 500: Internal Server Error" when saving changes

### B9 jumps to spool details after edit spool
- should jump to spool list

### B10 "Filter" actually means "search"
- the text box at the top displays "Filter" but it's free-text search. change the label to "search"

#### ~~B11 Filament view — sort~~ (Fixed)
- **Fix applied:** Same pattern as B6 — added `sorted` closure, wired `page_items` to `sorted()`, added `ColHeader` for Density column.

#### ~~B17 Help~~ (Fixed)
- **Root cause:** Link pointed to `/api/v1/setting` (returns empty map by default); NFC URL used `&lt;id&gt;` string literal which Leptos double-escapes, showing raw `&lt;id&gt;` instead of `<id>`; no `/info` endpoint existed.
- **Fix applied:** Added `GET /api/v1/info` returning `{ version, data_file }`; updated Help page link to `/api/v1/info`; fixed NFC URL string to `<id>`.

### B12 Create does not work
- when crating any new instance I get a "HTTP 500: Internal Server Error"

### B13 Pagination broken
- prev and next buttons are allways greyed out

### B14 no date in Spool view
- does not show date information (creation, last use)
- edit of date is not possible

### B15 delete buttons broken
- when the delete button of a location is pressed, it does not disappear until reload. before removal add a "Sure?" dialog. same for the other entities
- when deleting a spool / filament in detail view "HTTP 404: Not Found" is shown. (because the element was just deleted) jump to list view instead.

### B16 Location must not be "none"
- Spool edit / create dialogs allow save, despite no Location is selected

#### ~~B4 — No CSS: app is completely unstyled~~ (Fixed in feat/add-css-styling-stylers)
- **Fix applied:** Added `stylers = "0.3.2"` for scoped component CSS and `style-file = "style/spoolman.css"` in `Leptos.toml` for global CSS (variables, reset, dark mode, buttons, shared page classes). All major components now have `style!` blocks.
- **Pending:** Visual verification requires Docker/WSL build (cargo-leptos blocked on Windows).

### Docker / Build Notes (context for resuming)
- Test image: `spoolman-light:test` (built from current branch `feat/color-search-spool-list`)
- Running container: `spoolman-test` on `localhost:8000`
- Two Dockerfile patches were made during testing (not yet committed):
  1. `cp target/site/pkg/spoolman-server.wasm target/site/pkg/spoolman-server_bg.wasm` — aliases the renamed WASM file so the JS loader finds it
  2. `printf '...' > target/site/index.html` — generates the CSR bootstrap HTML that cargo-leptos 0.3.x omits when a server binary is present
- `assets/index.html` was tried and removed (cargo-leptos 0.3.5 rejects it: "path reserved for Leptos")
- `public/` directory was created and removed — real assets dir is `assets/`
- Stop/clean test environment: `docker stop spoolman-test && docker rm spoolman-test`

### Enhancements
- [ ] NFC / QR sticker integration — [OpenSpoolMan](https://github.com/drndos/openspoolman) or [OpenTag3D](https://opentag3d.com/) compatible; spool NFC URL already maps to `/api/v1/spool/<id>`
- [ ] Make the Spool list the default landing page
- [ ] Light theme matching the logo
- [x] Add CSS Styles using  [stylers = "0.3.2"](https://github.com/abishekatp/stylers)
- [ ] align numbers in columns on right
- [ ] make Diameter column optional. configured in settings "Same diameter for all Filaments / Spools". add a default diameter setting, too. in filament creation / edit only display the diameter input if above setting is false.

### Data Model
- Explore move of Net weight from filament to spool
