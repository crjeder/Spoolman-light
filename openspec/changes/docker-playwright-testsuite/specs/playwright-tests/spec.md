## ADDED Requirements

### Requirement: Playwright project configuration exists
A `tests/e2e/` directory SHALL contain a `package.json`, `playwright.config.ts`, and TypeScript test files. `npx playwright test` run from `tests/e2e/` SHALL execute all tests against `http://localhost:8000`.

#### Scenario: Playwright installs and runs
- **WHEN** `npm install` is run in `tests/e2e/` followed by `npx playwright test`
- **THEN** Playwright discovers and runs all `*.spec.ts` files under `tests/e2e/`

#### Scenario: Global timeout is sufficient for WASM load
- **WHEN** the browser navigates to the app root
- **THEN** the page reaches `networkidle` within 30 seconds (WASM bundle load)

### Requirement: Navigation between sections works
The app SHALL allow navigating to Spools, Filaments, and Locations sections via the main navigation.

#### Scenario: Spools page loads
- **WHEN** the user navigates to the root URL `/`
- **THEN** a page containing a spools table or heading is visible

#### Scenario: Filaments page loads
- **WHEN** the user navigates to `/filaments` or clicks the Filaments nav link
- **THEN** a page containing a filaments table or heading is visible

#### Scenario: Locations page loads
- **WHEN** the user navigates to `/locations` or clicks the Locations nav link
- **THEN** a page containing a locations table or heading is visible

### Requirement: Spool CRUD flows are testable
The Playwright suite SHALL cover creating, reading, and deleting a spool.

#### Scenario: Spool list is populated from fixture data
- **WHEN** the Spools page loads after the test harness starts with fixture data
- **THEN** at least one spool row is visible in the table

#### Scenario: Add spool dialog opens
- **WHEN** the user clicks the "Add Spool" or equivalent button on the Spools page
- **THEN** a dialog or form for creating a spool becomes visible

#### Scenario: New spool appears after creation
- **WHEN** the user fills in required fields in the add-spool form and submits
- **THEN** the new spool appears in the spool table

#### Scenario: Spool can be deleted
- **WHEN** the user selects a spool and triggers the delete action and confirms
- **THEN** the spool no longer appears in the spool table

### Requirement: Filament CRUD flows are testable
The Playwright suite SHALL cover creating, reading, and deleting a filament.

#### Scenario: Filament list is populated from fixture data
- **WHEN** the Filaments page loads after the test harness starts with fixture data
- **THEN** at least one filament row is visible in the table

#### Scenario: New filament appears after creation
- **WHEN** the user fills in required fields in the add-filament form and submits
- **THEN** the new filament appears in the filament table

#### Scenario: Filament can be deleted
- **WHEN** the user selects a filament and triggers the delete action and confirms
- **THEN** the filament no longer appears in the filament table

### Requirement: Location CRUD flows are testable
The Playwright suite SHALL cover creating, reading, and deleting a location.

#### Scenario: Location list is populated from fixture data
- **WHEN** the Locations page loads after the test harness starts with fixture data
- **THEN** at least one location row is visible in the table

#### Scenario: New location appears after creation
- **WHEN** the user fills in required fields in the add-location form and submits
- **THEN** the new location appears in the location table

#### Scenario: Location can be deleted
- **WHEN** the user selects a location and triggers the delete action and confirms
- **THEN** the location no longer appears in the location table

### Requirement: Page-Object Model organises test code
Each major page (Spools, Filaments, Locations) SHALL have a corresponding POM class under `tests/e2e/pages/`. Test files SHALL import and use these classes rather than embedding raw locators.

#### Scenario: POM encapsulates navigation
- **WHEN** a test calls `page.goto()` through a POM method
- **THEN** the POM waits for the page to be ready before returning

#### Scenario: POM exposes typed action methods
- **WHEN** a test calls a POM method such as `addSpool(data)` or `deleteSpool(name)`
- **THEN** the action is performed and the POM waits for the UI to reflect the change
