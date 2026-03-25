# TODO

Items to address. Move completed items to [CHANGELOG.md](CHANGELOG.md) under the appropriate release.

## Pending

### Bugs (found via Playwright Docker test, 2026-03-24)

#### ~~B5 Delete Button in Locations~~ (Fixed)
- **Root cause:** `disabled=move || count > 0` inside `view!` — the `>` in `count > 0` was parsed as the closing `>` of the `<button` opening tag, so `on:click=...>"Delete"` became raw text content of the button (same class of bug as Pagination "Next →").
- **Fix applied:** Extracted to `let delete_disabled = count > 0;` before the `view!` block; used `disabled=delete_disabled`. Also added `.btn:disabled` CSS rule so disabled buttons stay visually distinct from unstyled pagination buttons.

### B6 Spools view
- sorting on remainig (g) not possible
- sorting on location not possible

### B7 Color's alpha value is not used anywhere
- the color picker should allow to select an alpha value. if that's not possible add an extra selector elsewhere

### B8 server error in spool edit
- "HTTP 500: Internal Server Error" when saving changes

### B9 jumps to spool details after edit spool
- should jump to spool list

### B10 Sort in list view does not work
- clicking on the column heading must sort the table according to the values in the column. numbers must be sorted numerically and not treated as a string e. g. 1, 2, 9, 10 and not 10, 1, 2, ..

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
