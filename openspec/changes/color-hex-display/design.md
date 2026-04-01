## Context

The spool detail view renders each color as a colored `<span class="color-swatch">`. No text representation of the color is shown. The RGBA values are already available in the `colors` Vec on the spool struct; the hex string can be derived from them without any new data fetching.

## Goals / Non-Goals

**Goals:**
- Show the hex RGB value (e.g., `#ff6a00`) as a text label next to each swatch in the spool detail view.

**Non-Goals:**
- Showing hex in the spool table row (only the detail view).
- Showing the alpha channel in the hex label (alpha is displayed separately via the alpha % in the edit form; hex convention for filament color is RGB-only).
- Adding a copy-to-clipboard action (keep it simple).

## Decisions

**Derive hex inline from RGBA struct fields**

The `Rgba` struct exposes `.r`, `.g`, `.b` fields as `u8`. Format them with `format!("#{:02x}{:02x}{:02x}", c.r, c.g, c.b)` directly in the template — no helper function needed.

Alternative considered: adding a `to_hex()` method on `Rgba`. Rejected — overkill for a single call site.

**Render as plain text after the swatch**

The hex string is rendered as plain text inside the same `<dd>` element, immediately after the swatch span. No additional wrapper element needed; the existing CSS can style it via an adjacent-sibling or `.color-hex` class on a `<span>`.

## Risks / Trade-offs

- [Minimal risk] Only touches the read-only detail view template; no data model or API changes.
