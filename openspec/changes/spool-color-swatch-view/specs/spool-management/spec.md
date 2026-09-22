## MODIFIED Requirements

### Requirement: Spool list UI
The frontend SHALL provide a spool list page with sortable columns, server-side filtering, pagination, and column visibility toggle. Archived spools SHALL be togglable via a button. The spool table SHALL be rendered at `"/spools"`. The `"/"` route SHALL render a list view directly, without a redirect: the spool table when `default_view` is `spool` (the default), or the colour swatch grid when `default_view` is `color`. The Spools navigation link SHALL appear active when the current path is `"/spools"`, or when the current path is `"/"` and `default_view` is `spool`.

#### Scenario: Default list shows active spools
- **WHEN** the spool list page loads
- **THEN** only non-archived spools are shown with sensible default sort (by registered date, descending)

#### Scenario: Table state is optionally persisted
- **WHEN** the user changes sort or filter and returns to the page
- **THEN** the previous state is restored from localStorage if persistence is enabled

#### Scenario: Root path renders a list without redirect
- **WHEN** the user navigates to `"/"`
- **THEN** the view named by `default_view` is displayed without a redirect and the URL remains `"/"`

#### Scenario: Root path renders the spool table by default
- **WHEN** the user navigates to `"/"` and no `default_view` has been saved
- **THEN** the spool table is displayed, unchanged from previous behaviour

#### Scenario: Nav link is active at root path
- **WHEN** the current path is `"/"` and `default_view` is `spool`
- **THEN** the Spools navigation link is highlighted as active

#### Scenario: Nav link is not active at root path when the colour view is default
- **WHEN** the current path is `"/"` and `default_view` is `color`
- **THEN** the Spools navigation link is not highlighted as active
