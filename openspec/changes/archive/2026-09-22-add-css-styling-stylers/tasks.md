## 1. Dependency Setup

- [x] 1.1 Add `stylers = "0.3.2"` to `crates/spoolman-client/Cargo.toml` under `[dependencies]`
- [x] 1.2 Verify `Leptos.toml` does not need `style-file` changes for `stylers` output (cargo-leptos auto-detects `stylers` when the crate is present)

## 2. Layout and Navigation Styles

- [x] 2.1 Add `style!` block to `crates/spoolman-client/src/components/layout.rs` with CSS for `.app-shell` (flex row, full height), `.sidebar` (fixed width, vertical flex), `.sidebar-header`, `.nav-links`, `.sidebar-footer`, `.logo`, `.dark-toggle`, `.version`
- [x] 2.2 Add `use_style!` call in the `AppShell` component and the `Sidebar` component in `layout.rs`
- [x] 2.3 Define dark mode overrides using `:global(body.dark)` selector within the `style!` block for background and text color variables

## 3. Global Tokens and Button Styles

- [x] 3.1 Add a `style!` block in `layout.rs` (or a new `crates/spoolman-client/src/styles.rs` module) defining CSS custom properties on `:root` (colors, spacing, font) and light/dark values
- [x] 3.2 Define `.btn`, `.btn-primary`, and `.btn-danger` styles (padding, border-radius, background, color, cursor)

## 4. Data Table Styles

- [x] 4.1 Add `style!` block to `crates/spoolman-client/src/components/table.rs` with CSS for `.col-header`, `.sort-btn`, `.col-filter`
- [x] 4.2 Add `style!` block to `crates/spoolman-client/src/pages/spool.rs` for `.data-table` (border-collapse, width 100%), `tr`, `th`, `td` (padding, border), `.archived` (muted color), `.actions` (right-aligned), `.color-swatch` (inline-block, width/height 16px, border-radius 2px, vertical-align middle)

## 5. Page-Level Styles

- [x] 5.1 Add `style!` block to `crates/spoolman-client/src/pages/spool.rs` for `.page`, `.page-header`, `.page-actions`, `.spool-list`, `.spool-show`, `.spool-create`, `.spool-edit`, `.detail-grid`, `.error`, `.color-filter`
- [x] 5.2 Add `style!` block to `crates/spoolman-client/src/pages/filament.rs` for `.filament-list`, `.filament-show`, `.filament-create`, `.filament-edit` and shared `.page`/`.page-header` classes
- [x] 5.3 Add `style!` blocks to `crates/spoolman-client/src/pages/location.rs` and `pages/settings.rs` for their respective page classes

## 6. Pagination Styles

- [x] 6.1 Add `style!` block to `crates/spoolman-client/src/components/pagination.rs` for pagination container and button styles

## 7. Verification

- [x] 7.1 Build the project in WSL/Linux/Docker (`cargo leptos build --release`) and confirm `target/site/pkg/spoolman-server.css` is non-empty
- [x] 7.2 Run the app in Docker and visually verify: sidebar is visible, table has borders, buttons are styled, dark mode toggle changes appearance, color swatches show correct colors
- [x] 7.3 Update `TODO.md` to mark B4 as resolved and add an entry to `CHANGELOG.md`
