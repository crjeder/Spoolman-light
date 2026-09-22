## Why

`style/spoolman.css` contains zero `@media` rules, and `.sidebar` is a hard `width: 200px; min-width: 200px` flex child of `.app-shell`. On a 375px phone the navigation therefore consumes 53% of the viewport width and cannot be dismissed, leaving 175px for tables and forms. "Test on mobile" is ticked in `TODO.md`, but the layout was never made responsive — the app is usable on a phone only because the main content scrolls horizontally.

A burger toggle that collapses the sidebar below a breakpoint recovers the full viewport width on phones and changes nothing on desktop.

## What Changes

- The stylesheet SHALL gain its first breakpoint. Below it, the sidebar SHALL be hidden by default and opened by a burger button; at or above it, the sidebar SHALL render exactly as it does today with no burger button.
- The open sidebar SHALL overlay the content with a dismissible backdrop rather than reflowing it, so the narrow layout does not reflow twice per toggle.
- The sidebar SHALL close when a navigation entry is activated, when the backdrop is clicked, and when `Escape` is pressed.
- Existing class names (`.sidebar`, `.nav-links`, `.app-shell`, `.main-content`) and the `--sidebar-bg` / `--sidebar-fg` tokens SHALL be preserved.
- The three sidebar tests in `tests/e2e/tests/navigation.spec.ts` SHALL be updated to open the burger first when running at a narrow viewport.

## Capabilities

### New Capabilities

- `responsive-navigation`: breakpoint-driven sidebar collapse, burger toggle, overlay and dismissal behaviour.

### Modified Capabilities

*(none)* — `light-theme-palette` only constrains the `--sidebar-bg` token, which is unchanged.

## Impact

- `crates/spoolman-client/src/components/layout.rs` — burger button, open/closed signal, backdrop, close-on-navigate.
- `style/spoolman.css` — first `@media` block; overlay and burger rules.
- `tests/e2e/tests/navigation.spec.ts` — three tests assert `nav.sidebar ul.nav-links a` is visible and click it directly; they need a burger click first at narrow widths, or an explicit desktop viewport.
- No server, API, data-model or settings changes.
- Conflicts with the unimplemented `add-css-styling-stylers` change, which proposes moving all CSS into the `stylers` crate; whichever lands second carries these rules across.
