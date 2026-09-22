## ADDED Requirements

### Requirement: Sidebar collapses below the narrow-viewport breakpoint
The stylesheet SHALL define a breakpoint at 768px viewport width. At or above the breakpoint the sidebar SHALL be displayed permanently as a 200px column and no burger button SHALL be rendered. Below the breakpoint the sidebar SHALL be hidden by default and the main content SHALL occupy the full viewport width.

#### Scenario: Desktop layout is unchanged
- **WHEN** the viewport is 1280px wide
- **THEN** the sidebar is visible as a 200px column and no burger button is shown

#### Scenario: Tablet portrait keeps the sidebar
- **WHEN** the viewport is 768px wide
- **THEN** the sidebar is visible and no burger button is shown

#### Scenario: Phone hides the sidebar
- **WHEN** the viewport is 375px wide
- **THEN** the sidebar is not visible, a burger button is shown, and the main content spans the full viewport width

#### Scenario: Resizing across the breakpoint restores the sidebar
- **WHEN** the viewport is resized from 375px to 1280px
- **THEN** the sidebar becomes visible and the burger button disappears, without a page reload

### Requirement: Burger button opens the sidebar as an overlay
Below the breakpoint, activating the burger button SHALL open the sidebar over the main content, together with a backdrop covering the content. Opening the sidebar SHALL NOT reflow or resize the main content. The button SHALL expose an accessible name and SHALL reflect its state via `aria-expanded`.

#### Scenario: Burger opens the navigation
- **WHEN** the viewport is 375px wide and the user activates the burger button
- **THEN** the sidebar navigation entries become visible over the content, with a backdrop behind them

#### Scenario: Content is not reflowed
- **WHEN** the sidebar is opened at 375px width
- **THEN** the main content keeps its position and width, overlaid rather than pushed aside

#### Scenario: Burger reports its state
- **WHEN** the sidebar is closed and then opened
- **THEN** the burger button reports `aria-expanded="false"` and then `aria-expanded="true"`

#### Scenario: Sidebar overlays other stacked elements
- **WHEN** the sidebar is open and page content contains a colour picker popup
- **THEN** the sidebar and its backdrop are drawn above that popup

### Requirement: Open sidebar is dismissible three ways
An open sidebar SHALL close when the backdrop is clicked, when the `Escape` key is pressed, and when a navigation entry is activated. The open state SHALL NOT be persisted: after a reload the sidebar SHALL be closed again at narrow widths.

#### Scenario: Backdrop click closes
- **WHEN** the sidebar is open and the user clicks the backdrop
- **THEN** the sidebar and backdrop are hidden and the content is interactive again

#### Scenario: Escape closes
- **WHEN** the sidebar is open and the user presses `Escape`
- **THEN** the sidebar closes

#### Scenario: Navigating closes
- **WHEN** the sidebar is open and the user activates the Filaments entry
- **THEN** the application navigates to `/filaments` and the sidebar closes

#### Scenario: Open state does not survive a reload
- **WHEN** the sidebar is open and the page is reloaded at 375px width
- **THEN** the sidebar is closed

### Requirement: Navigation markup and theming are preserved
The collapsed and expanded layouts SHALL use the same navigation markup: a `nav.sidebar` containing `ul.nav-links` with one anchor per destination, at every viewport width. The `--sidebar-bg` and `--sidebar-fg` tokens SHALL continue to colour the sidebar in both light and dark themes, and the dark-mode toggle and version string SHALL remain reachable in the collapsed layout.

#### Scenario: Selectors are stable across viewports
- **WHEN** the page is rendered at 375px and at 1280px
- **THEN** `nav.sidebar ul.nav-links a` matches the same set of destinations in both cases

#### Scenario: Theme tokens still apply to the overlay
- **WHEN** the sidebar is opened at 375px width in dark mode
- **THEN** its background resolves to the dark `--sidebar-bg` token

#### Scenario: Footer controls remain available
- **WHEN** the sidebar is opened at 375px width
- **THEN** the dark-mode toggle and the version string are visible within it
