## MODIFIED Requirements

### Requirement: Spool list UI
The frontend SHALL provide a spool list page with sortable columns, server-side filtering, pagination, and column visibility toggle. Archived spools SHALL be togglable via a button. The spool list page SHALL be the default landing page of the application, rendered at both `"/"` and `"/spools"`. The `"/"` route SHALL render the spool list component directly without a redirect. The Spools navigation link SHALL appear active when the current path is either `"/"` or `"/spools"`.

#### Scenario: Default list shows active spools
- **WHEN** the spool list page loads
- **THEN** only non-archived spools are shown with sensible default sort (by registered date, descending)

#### Scenario: Table state is optionally persisted
- **WHEN** the user changes sort or filter and returns to the page
- **THEN** the previous state is restored from localStorage if persistence is enabled

#### Scenario: Root path renders spool list
- **WHEN** the user navigates to `"/"`
- **THEN** the spool list is displayed without a redirect

#### Scenario: Nav link is active at root path
- **WHEN** the current path is `"/"`
- **THEN** the Spools navigation link is highlighted as active
