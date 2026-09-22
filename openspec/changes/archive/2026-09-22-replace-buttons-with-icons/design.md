## Context

The `.btn-icon` CSS class and the icon set for row actions are already established by the spool table (completed in a previous pass). The spool list uses: 👁 (`\u{1F441}`) View, ✏ (`\u{270F}`) Edit, 🗑 (`\u{1F5D1}`) Delete, ✕ (`\u{2715}`) Cancel. Confirm-delete buttons in the spool table already use `btn-icon btn-danger` for the destructive action and plain `btn-icon` for the cancel.

The remaining pages — filament list, location list, spool detail view, and pagination — still use text labels on `btn` / `btn-danger` elements.

No new CSS classes are needed; all required styling is already present in `style/spoolman.css`.

## Goals / Non-Goals

**Goals:**
- Apply `btn-icon` + Unicode glyphs consistently to all row-level actions across filament, location, and spool detail pages
- Apply icon-only style to pagination prev/next buttons
- Add `title` attributes to every icon button for accessibility (tooltip on hover, screen-reader label)
- Match the pattern already in place in the spool list (same icons, same class combinations)

**Non-Goals:**
- Form submit buttons ("Create", "Save", "Add") — these are primary form CTAs; text labels are intentional and stay as `.btn` / `.btn-primary`
- Introducing any icon library or SVG system — Unicode emoji/symbols only, consistent with existing code
- Changing button behaviour or confirm-delete flow — only the visual presentation changes

## Decisions

### Icon mapping (consistent across all pages)

| Action | Icon | Unicode | Class |
|--------|------|---------|-------|
| Edit | ✏ | `\u{270F}` | `btn btn-icon` |
| Delete (trigger) | 🗑 | `\u{1F5D1}` | `btn btn-icon btn-danger` |
| Delete (confirm "Sure?") | 🗑 | `\u{1F5D1}` | `btn btn-icon btn-danger` |
| Cancel / dismiss | ✕ | `\u{2715}` | `btn btn-icon` |
| Save (inline row edit) | 💾 | `\u{1F4BE}` | `btn btn-icon` |
| Clone | ⧉ | `\u{29C9}` | `btn btn-icon` |
| Pagination prev | ‹ | `\u{2039}` | `btn btn-icon` |
| Pagination next | › | `\u{203A}` | `btn btn-icon` |

The spool detail view buttons (Edit, Clone, Delete, Cancel) move from `.btn` text buttons to `.btn-icon` icon buttons, matching the spool table row.

### Rationale: icon-only (not icon + text)

The spool table already established icon-only as the pattern. Mixing icon+text on detail pages while the list uses icon-only would be inconsistent. `title` attributes cover discoverability.

### Rationale: 💾 for Save (location inline edit)

The location page has an inline row-edit mode with a distinct "Save" action. Using 💾 distinguishes it from the ✕ cancel and makes the save intent clear without text.

## Risks / Trade-offs

- **Emoji rendering varies by OS/font** — Unicode emoji used throughout the existing codebase already; this is an accepted constraint for the project.
- **Icon discoverability** — mitigated by `title` attributes (tooltips) on every icon button, consistent with the already-shipped spool table.

## Migration Plan

No data migration. Changes are purely presentational Leptos RSX edits. No server-side changes. No new dependencies.

Rollback: revert the RSX edits — no state or schema involved.
