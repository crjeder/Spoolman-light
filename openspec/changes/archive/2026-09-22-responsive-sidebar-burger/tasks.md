## 1. Layout component

- [x] 1.1 In `crates/spoolman-client/src/components/layout.rs`: add a local `menu_open: RwSignal<bool>` (not persisted, not in context).
- [x] 1.2 Render a burger `<button class="burger" aria-label="Menu">` with `aria-expanded` bound to the signal, before the sidebar in `.app-shell`. CSS hides it at or above the breakpoint.
- [x] 1.3 Render a `.sidebar-backdrop` when open, closing the sidebar on click; mirror the existing `.color-backdrop` pattern from `pages/spool.rs`.
- [x] 1.4 Add an `open` class to `nav.sidebar` when the signal is set; keep the inner markup identical at all widths.
- [x] 1.5 Close the sidebar on `Escape` (keydown listener on the backdrop or window) and on activation of any `nav-links` entry.

## 2. Styling

- [x] 2.1 In `style/spoolman.css`: add the first `@media (max-width: 767px)` block — hide `.sidebar` off-canvas by default, show `.burger`, let `.main-content` span the full width.
- [x] 2.2 In that block, style `nav.sidebar.open` as a fixed overlay using `--sidebar-bg` / `--sidebar-fg`, with a `z-index` above `.color-popup` and `.color-backdrop`.
- [x] 2.3 Style `.sidebar-backdrop` and `.burger`; hide `.burger` at or above 768px.

## 3. E2E tests

- [x] 3.1 Resolved: `playwright.config.ts` has a single `chromium` project using `devices['Desktop Chrome']` (1280x720), i.e. above the breakpoint. The three existing sidebar tests therefore pass unchanged. Narrow coverage is added as a `test.describe` with `test.use({ viewport: { width: 375, height: 812 } })` rather than a second project, so only the narrow tests pay the extra run.
- [x] 3.2 No change needed: those three tests run at desktop width (see 3.1), where the sidebar is permanently visible.
- [x] 3.3 Added `Navigation at phone width` in `tests/e2e/tests/navigation.spec.ts`: hidden on load, burger opens as overlay (content bounding box unchanged, backdrop visible, `aria-expanded` flips), backdrop click closes, Escape closes, activating Filaments navigates and closes, open state does not survive a reload.
- [x] 3.4 Added `Navigation at desktop width` asserting `nav.sidebar` visible and `button.burger` hidden. The button is present in the DOM at all widths (markup is identical by design) and hidden by CSS, so the assertion is `toBeHidden()`.

## 4. Verification

- [x] 4.1 `cargo check -p spoolman-client --target wasm32-unknown-unknown`.
- [ ] 4.2 `./scripts/run-e2e.sh` green. Blocked: `docker-compose.test.yml`'s shell entrypoint (`/bin/sh -c ...`) cannot run against the distroless runtime image (no `/bin/sh`), pre-existing and unrelated to this change — see flagged follow-up.
- [x] 4.3 Manually verify at 375px: full-width content, burger opens and closes, dark-mode toggle and version reachable, no horizontal scroll introduced by the sidebar itself. (verified via `docker compose -f docker-compose.yml` + browser at 375x812)
- [x] 4.4 Manually verify at 1280px that the layout is visually unchanged from before this change.
- [x] 4.5 `TODO.md` "test on mobile" note updated; CHANGELOG entry added under `[1.7.7]` at archive time.
