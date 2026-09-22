## Context

`Layout` in `crates/spoolman-client/src/components/layout.rs` renders `.app-shell` as a flex row containing `<Sidebar />` and `<main class="main-content">`. The sidebar holds a header with the logo, `ul.nav-links` of `<A>` entries, and a footer with the dark-mode toggle and version string. The stylesheet fixes it at 200px and has no media queries at all.

E2E coverage: `tests/e2e/tests/navigation.spec.ts` has three tests that reach into `nav.sidebar ul.nav-links`, assert links are visible, and click `nav.sidebar a[href="/filaments"]` / `a[href="/locations"]` directly. The Page Objects under `tests/e2e/pages/` navigate exclusively via `page.goto()` and are unaffected. `playwright.config.ts` decides the viewport the three tests run at.

## Goals / Non-Goals

**Goals:**
- Recover the full viewport width for content on phone-sized screens.
- Leave the desktop layout byte-identical in appearance.
- Keep existing class names and theme tokens so the light/dark palette spec and any selector-based tests keep working.

**Non-Goals:**
- A burger on desktop. A permanently collapsed sidebar would hide navigation that currently costs nothing to show.
- A user setting for the breakpoint or for pinning the sidebar.
- A general responsive pass over tables and forms. Tables still scroll horizontally on a phone; that is a separate concern.
- Moving CSS into the `stylers` crate (see `add-css-styling-stylers`).

## Decisions

**Breakpoint at 768px.** Below it the sidebar collapses; at or above it nothing changes. 768px is the conventional tablet boundary and matches the `tablet` preset used when testing viewports, so a tablet in portrait keeps the sidebar.

**Overlay, not reflow.** When opened at narrow widths the sidebar is positioned over the content with a translucent backdrop, rather than pushing `.main-content` aside. Pushing would reflow the table or card grid on every toggle, which is visibly slow on a phone and can move the element the user was about to tap.

**Open state is a plain component signal.** It is transient UI state, not a preference: it is not persisted, and it resets on reload. The existing dark-mode signal is in context because several components read it; nothing outside `Layout` needs the sidebar state.

**Close on navigate.** Activating an entry inside the overlay closes it, so the user is not left looking at the menu after the page behind it changes.

**Escape and backdrop both dismiss.** A backdrop click is the expected gesture on touch, `Escape` is the expected key on a small laptop window. Both are a few lines; the `color-popup` in the spool list already establishes the backdrop pattern in this codebase.

**Burger is an ordinary button with an accessible name.** It carries `aria-label="Menu"` and `aria-expanded`, and the glyph is a plain three-bar character rather than an icon dependency, consistent with how the existing buttons use single characters.

**Keep the markup identical at both sizes.** Only CSS and a class on `.app-shell` (or on the sidebar) differ between states; the same `nav.sidebar > ul.nav-links > a` structure exists at every width. This is what keeps the E2E fix to "click the burger first" rather than "rewrite the selectors".

## Risks / Trade-offs

- Three E2E tests break until updated. This is the main cost of the change and the reason it ships separately from feature work: a failing nav test in a bundled PR looks like a regression in whatever else the PR touched.
- If `playwright.config.ts` runs a narrow viewport, the burger click is mandatory in those tests; if it runs a desktop viewport, the tests pass unchanged and the collapsed path needs its own narrow-viewport test. Either way one of the two paths must be covered explicitly.
- The overlay sits above page content, so a `z-index` is introduced. The existing `.color-popup` and `.color-backdrop` rules are the only other stacked elements; the sidebar must sit above them or the popup must not be reachable while the menu is open.

## Open Questions

- Does `playwright.config.ts` currently pin a viewport, and should the suite gain a second phone-width project rather than converting the existing tests? Resolve while implementing task 3.
