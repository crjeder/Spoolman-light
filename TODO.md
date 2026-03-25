# TODO

Items to address. Move completed items to [CHANGELOG.md](CHANGELOG.md) under the appropriate release.

## Pending

### Bugs (found via Playwright Docker test, 2026-03-24)

#### B4 — No CSS: app is completely unstyled
- **Root cause:** `assets/spoolman-light-logo.png` is the only asset; no `.css` file exists in the project. The built `target/site/pkg/spoolman-server.css` is empty. All layout class names (`.app-shell`, `.sidebar`, `.main-content`, `.data-table`, `.page-header`, `.color-swatch`, `.dark`, etc.) have no rules.
- **Impact:** Dark mode toggle adds `.dark` class to `<body>` but has no visual effect. No layout, spacing, or color styling.
- **Fix:** Create `assets/style.css` (cargo-leptos copies it to `target/site/`) and reference it from `index.html` as `/style.css`. Key classes to define: `.app-shell` (flex row), `.sidebar` (fixed left nav), `.main-content` (flex-grow), `.data-table`, `.color-swatch` (inline-block colored square), `.dark` body overrides.
  - Or use the `style-file` option in `Leptos.toml` to point to a CSS source file that cargo-leptos will compile and embed into `spoolman-server.css`.

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
- [ ] Add CSS Styles using  [stylers = "0.3.2"](https://github.com/abishekatp/stylers)
