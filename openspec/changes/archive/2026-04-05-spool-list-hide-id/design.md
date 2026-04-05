## Context

The spools list in `spool.rs` renders a table with a sortable "ID" column. IDs are random u32 values with no semantic meaning to users — they exist solely for internal routing. The column adds noise and occupies horizontal space.

## Goals / Non-Goals

**Goals:**
- Remove the ID column from the spool list table (header + data cell)
- Preserve the existing detail-page link on the filament name or another column if currently absent

**Non-Goals:**
- Removing ID from the detail page (`/spools/:id`)
- Changing routing or URL structure
- Reordering remaining columns

## Decisions

**Delete header and cell, keep link target elsewhere**
The ID cell currently renders `<a href="/spools/{id}">{id}</a>`. After removal the link still exists implicitly via the "Edit" / detail navigation buttons already present per row. No additional linking is required.

**No alternative layout needed**
The remaining columns (filament, color, weight, location, actions) fill the space naturally; no column span adjustments are necessary.

## Risks / Trade-offs

- [Minimal risk] Users who relied on visually scanning the ID number lose that affordance — acceptable given IDs are random and meaningless to end users.
