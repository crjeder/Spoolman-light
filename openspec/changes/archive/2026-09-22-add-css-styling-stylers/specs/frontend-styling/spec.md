## ADDED Requirements

### Requirement: App shell layout renders correctly
The system SHALL render the app shell as a horizontal flex row with a fixed sidebar on the left and a scrollable main content area filling the remaining width.

#### Scenario: App shell is visible on load
- **WHEN** the application loads in the browser
- **THEN** a sidebar navigation is displayed on the left side of the viewport
- **THEN** the main content area occupies the rest of the viewport width

#### Scenario: Sidebar navigation links are visible and clickable
- **WHEN** the sidebar is rendered
- **THEN** navigation links ("Spools", "Filaments", "Locations") are displayed as a vertical list with readable font size and padding

### Requirement: Dark mode toggle has a visible effect
The system SHALL apply dark background and light text colors when the `.dark` class is present on the `<body>` element, and SHALL revert to light colors when it is absent.

#### Scenario: Dark mode enabled
- **WHEN** the user activates the dark mode toggle
- **THEN** the `.dark` class is added to `<body>` and the page background becomes dark and text becomes light

#### Scenario: Dark mode disabled
- **WHEN** the user deactivates the dark mode toggle
- **THEN** the `.dark` class is removed from `<body>` and the page reverts to a light background with dark text

### Requirement: Data table is styled
The system SHALL render the spool and filament list tables with visible borders, alternating row backgrounds, and readable column headers.

#### Scenario: Table header is visually distinct
- **WHEN** the data table is rendered
- **THEN** column headers (`.col-header`) have a distinct background color or font weight compared to data rows

#### Scenario: Sort button is visible
- **WHEN** a sortable column header is rendered
- **THEN** the sort button (`.sort-btn`) is visible and indicates sort direction

### Requirement: Color swatch renders as a colored square
The system SHALL render `.color-swatch` elements as small inline-block squares showing the background color set via the `style` attribute.

#### Scenario: Color swatch displays correct color
- **WHEN** a spool with a color value is displayed in the list or detail view
- **THEN** a small square with the spool's color is shown inline next to the color name

### Requirement: Buttons are visually distinct
The system SHALL style `.btn`, `.btn-primary`, and `.btn-danger` elements with appropriate background colors, padding, and border-radius so they are clearly interactive.

#### Scenario: Primary button stands out
- **WHEN** a `.btn-primary` button is rendered (e.g., "Create", "Save")
- **THEN** it has a distinct fill color (e.g., blue) that differentiates it from secondary buttons

#### Scenario: Danger button is visually distinct
- **WHEN** a `.btn-danger` button is rendered (e.g., "Delete")
- **THEN** it has a red or warning-color background

### Requirement: CSS is generated via stylers proc-macro
The system SHALL use the `stylers 0.3.2` crate `style!` macro to define component-scoped CSS, which SHALL be bundled into `spoolman-server.css` by `cargo-leptos` at build time.

#### Scenario: CSS output is non-empty after build
- **WHEN** the project is built with `cargo leptos build`
- **THEN** `target/site/pkg/spoolman-server.css` contains non-empty CSS rules
