## Context

The spool detail view (`SpoolShow`) and spool list table both display a filament's `display_name()` as plain text. The `SpoolResponse` struct already includes the full `Filament` record (with its `id`), so no API changes are needed. A route `/filaments/:id` already exists and renders `FilamentShow`.

## Goals / Non-Goals

**Goals:**
- Render the filament name as `<a href="/filaments/{id}">` in both the spool detail view and the spool list table row.

**Non-Goals:**
- Changing the filament detail page itself.
- Adding any backend or API changes.
- Linking other cross-references (e.g. location names).

## Decisions

**Single `<a>` tag at each render site, no abstraction.**
The change touches two lines in one file. Extracting a helper component for a two-occurrence pattern would be premature. A plain `<a href=...>` inline is sufficient and readable.

**Link both detail view and list table.**
The proposal targets the detail view, but the list table renders the same field with no extra cost. Linking only the detail view while leaving the list as plain text would be an inconsistency. Both sites are updated together.

## Risks / Trade-offs

- [None] — change is purely additive UI. No data model, API, or state changes. Rollback is reverting two lines.

## Open Questions

- None.
