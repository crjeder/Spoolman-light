## ADDED Requirements

### Requirement: Icon-only row action buttons
Row-level action buttons (edit, delete, save, cancel, confirm) throughout the application SHALL use icon characters with no visible text labels. Each icon button SHALL carry a `title` attribute containing a human-readable label for tooltip and accessibility. Icon buttons SHALL use the `.btn-icon` CSS class. Destructive actions SHALL additionally use `.btn-danger`.

#### Scenario: Icon button shows no text label
- **WHEN** the user views any list page (spools, filaments, locations)
- **THEN** row action buttons display only an icon character; no text such as "Edit", "Delete", "Save", or "Cancel" is visible

#### Scenario: Icon button title attribute present
- **WHEN** a user hovers over a row action icon button
- **THEN** the browser tooltip shows the human-readable action label (e.g. "Edit", "Delete", "Save changes", "Cancel")

#### Scenario: Destructive icon button styled differently
- **WHEN** a delete or confirm-delete icon button is rendered
- **THEN** it uses the `.btn-icon.btn-danger` class combination and displays in the danger color

### Requirement: Standard icon set for actions
The application SHALL use a consistent set of Unicode characters for action icons across all pages.

| Action | Icon | Unicode |
|--------|------|---------|
| Edit | ✏ | U+270F |
| Delete (trigger and confirm) | 🗑 | U+1F5D1 |
| Cancel / dismiss | ✕ | U+2715 |
| Save (inline row edit) | 💾 | U+1F4BE |
| Clone | ⧉ | U+29C9 |
| Pagination previous | ‹ | U+2039 |
| Pagination next | › | U+203A |

#### Scenario: Edit button uses pencil icon
- **WHEN** an edit action button is rendered on any page
- **THEN** it displays ✏ (U+270F)

#### Scenario: Delete button uses wastebasket icon
- **WHEN** a delete trigger or confirm button is rendered on any page
- **THEN** it displays 🗑 (U+1F5D1)

#### Scenario: Cancel button uses multiplication sign
- **WHEN** a cancel action button is rendered on any page
- **THEN** it displays ✕ (U+2715)

### Requirement: Icon-only pagination buttons
The pagination component SHALL use icon characters for the previous and next page buttons, with no text labels.

#### Scenario: Previous page button shows icon
- **WHEN** the pagination component is rendered and there is a previous page
- **THEN** the previous button displays ‹ (U+2039) with `title="Previous page"`

#### Scenario: Next page button shows icon
- **WHEN** the pagination component is rendered and there is a next page
- **THEN** the next button displays › (U+203A) with `title="Next page"`
