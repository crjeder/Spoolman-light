## Context

The spool list table has an "actions" cell per row containing an "Edit" anchor link and a "Delete" button with inline confirmation ("Sure?" / "Cancel"). The detail page (`/spools/:id`) already exists as `SpoolShow` but is not reachable from the list. The change is purely frontend — no API or data model changes.

## Goals / Non-Goals

**Goals:**
- Replace "Edit" text with a pencil (✏) icon button/link.
- Replace "Delete"/"Sure?"/"Cancel" text with trash (🗑) icon equivalents.
- Add a View icon button (👁) that navigates to `/spools/:id`.
- Keep the delete confirmation flow intact (one click to arm, second to confirm).

**Non-Goals:**
- Changing the delete confirmation mechanism (still inline in the table row).
- Adding tooltips or ARIA labels beyond basic `title` attributes.
- Changing any other page (SpoolShow, SpoolEdit, etc.).
- Any backend changes.

## Decisions

### Icon source: Unicode / CSS characters, no external icon library
Using Unicode characters (✏️, 🗑, 👁) or simple CSS-drawn icons avoids adding a dependency. The project currently has no icon library. Inline SVG is also acceptable if consistent with the existing style. HTML entities or character references keep it self-contained.

**Alternatives considered:**
- Font Awesome / Heroicons: adds external dependency and build complexity.
- Inline SVG per button: verbose but most controllable; acceptable if needed.

### Button styling: `btn-icon` utility class
A new `.btn-icon` CSS class will handle size, padding, and hover for icon-only buttons. This keeps the existing `.btn`, `.btn-danger` classes intact and avoids changes to the shared button styles.

### View button placement: leftmost in actions cell
Order: View → Edit → Delete. View is the least destructive and most common navigation action.

## Risks / Trade-offs

- [Emoji rendering differences across OS/browser] → Use consistent, widely-supported codepoints or fall back to simple text symbols (e.g., `✏`, `🗑`, `👁`). If inconsistent, switch to inline SVG.
- [Narrower actions column may clip on small screens] → Icon buttons should be more compact than text buttons, so this is an improvement not a regression.
