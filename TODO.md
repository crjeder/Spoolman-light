# TODO

Items to address. Move completed items to [CHANGELOG.md](CHANGELOG.md) under the appropriate release.

## Pending

### Bugs (found via Playwright Docker test, 2026-03-24)

#### B1 — Pagination "Next →" button renders raw Leptos code as text
- **File:** `crates/spoolman-client/src/components/pagination.rs:22`
- **Root cause:** `>=` inside `disabled=move || page.get() + 1 >= total_pages()` — the Leptos `view!` macro parses the `>` in `>=` as the closing `>` of the `<button>` tag, so everything after it becomes the button's text content.
- **Fix:** Extract to a derived signal before the `view!` block:
  ```rust
  let next_disabled = Signal::derive(move || page.get() + 1 >= total_pages());
  ```
  Then use `disabled=next_disabled` in the button attribute.
- **Affects:** All three list pages (Spools, Filaments, Locations) that use `<Pagination>`.

#### B2 — Spool create: `filament_id` stays 0, causing 404 on submit
- **File:** `crates/spoolman-client/src/pages/spool.rs:233`
- **Root cause:** `let filament_id = create_rw_signal(0u32)` is never updated when the dropdown auto-selects the first loaded filament (signal only updates `on:change`). POST sends `filament_id: 0` → server returns 404 (no filament with that ID).
- **Fix:** After `filaments` resource resolves, initialize the signal to the first filament's ID. Use `create_effect` watching `filaments`:
  ```rust
  create_effect(move |_| {
      if let Some(Ok(fs)) = filaments.get() {
          if let Some(first) = fs.first() {
              filament_id.set(first.id);
          }
      }
  });
  ```

#### B3 — No color picker on spool create/edit
- **File:** `crates/spoolman-client/src/pages/spool.rs:282` (create), and the Edit component further down in same file
- **Root cause:** `SpoolCreate` has a `color_name` text input but no `<input type="color">` for the hex value. `CreateSpool { colors: vec![] }` is always sent with an empty colors array.
- **Fix:** Add a color signal and `<input type="color">` field. Parse the hex value into `Rgba` before submitting:
  ```rust
  let color_hex = create_rw_signal(String::from("#000000"));
  // in form:
  // <input type="color" prop:value=color_hex on:input=move |ev| color_hex.set(event_target_value(&ev)) />
  // in on_submit:
  // colors: hex_to_rgba(color_hex.get().as_str()).map(|c| vec![c]).unwrap_or_default(),
  ```
  `hex_to_rgba` already exists at `crates/spoolman-client/src/utils/color.rs`.

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
