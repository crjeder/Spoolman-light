## 1. Layout component

- [ ] 1.1 In `crates/spoolman-client/src/components/layout.rs`: add a local `menu_open: RwSignal<bool>` (not persisted, not in context).
- [ ] 1.2 Render a burger `<button class="burger" aria-label="Menu">` with `aria-expanded` bound to the signal, before the sidebar in `.app-shell`. CSS hides it at or above the breakpoint.
- [ ] 1.3 Render a `.sidebar-backdrop` when open, closing the sidebar on click; mirror the existing `.color-backdrop` pattern from `pages/spool.rs`.
- [ ] 1.4 Add an `open` class to `nav.sidebar` when the signal is set; keep the inner markup identical at all widths.
- [ ] 1.5 Close the sidebar on `Escape` (keydown listener on the backdrop or window) and on activation of any `nav-links` entry.

## 2. Styling

- [ ] 2.1 In `style/spoolman.css`: add the first `@media (max-width: 767px)` block — hide `.sidebar` off-canvas by default, show `.burger`, let `.main-content` span the full width.
- [ ] 2.2 In that block, style `nav.sidebar.open` as a fixed overlay using `--sidebar-bg` / `--sidebar-fg`, with a `z-index` above `.color-popup` and `.color-backdrop`.
- [ ] 2.3 Style `.sidebar-backdrop` and `.burger`; hide `.burger` at or above 768px.

## 3. E2E tests

- [ ] 3.1 Check whether `tests/e2e/playwright.config.ts` pins a viewport; decide between updating the three sidebar tests in place and adding a phone-width project (see the open question in `design.md`).
- [ ] 3.2 Update the three sidebar tests in `tests/e2e/tests/navigation.spec.ts` ("Sidebar nav links exist", "Clicking Filaments nav link…", "Clicking Locations nav link…") to open the burger first when running narrow.
- [ ] 3.3 Add narrow-viewport coverage: sidebar hidden on load, burger opens it, backdrop click closes it, activating an entry navigates and closes it.
- [ ] 3.4 Add a desktop-viewport assertion that no burger button is rendered.

## 4. Verification

- [ ] 4.1 `cargo check -p spoolman-client --target wasm32-unknown-unknown`.
- [ ] 4.2 `./scripts/run-e2e.sh` green.
- [ ] 4.3 Manually verify at 375px: full-width content, burger opens and closes, dark-mode toggle and version reachable, no horizontal scroll introduced by the sidebar itself.
- [ ] 4.4 Manually verify at 1280px that the layout is visually unchanged from before this change.
- [ ] 4.5 Update the "test on mobile" note in `TODO.md` to record that the sidebar is now responsive, and add the CHANGELOG entry when archiving.
