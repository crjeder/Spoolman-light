## Context

The spool list page has a color picker in the page-header actions area and a plain `<th>"Color"</th>` in the table. Users don't associate the column with the filter because there's no visual or interactive link between them. The fix is a DOM `NodeRef` on the color picker input that the column header click handler can call `.focus()` on.

## Goals / Non-Goals

**Goals:**
- Clicking the "Color" `<th>` focuses the color picker input (activating the browser's color-picker popup or at minimum scrolling to it).
- The column header gets a hover cursor and style indicating it's interactive.

**Non-Goals:**
- Making the color column sortable (it has no meaningful sort key).
- Changing color filter behavior or position in the UI.
- Any backend or API changes.

## Decisions

**NodeRef focus vs. scrollIntoView**
Use a Leptos `NodeRef<Input>` on the `<input type="color">` element. On column header click, call `node_ref.get().map(|el| el.focus())`. This triggers the browser's native color picker and is the minimal, zero-dependency solution. Alternative (scrollIntoView) is weaker — the input is always visible in the header, so focus is sufficient and more useful.

**Plain `<th>` with `on:click` vs. wrapping in `<button>`**
Use `role="button"` + `tabindex="0"` + `on:click` on the `<th>` rather than nesting a `<button>`, because nesting interactive elements inside `<th>` is valid but a nested `<button>` inside a sortable `ColHeader` could create accessibility confusion. Since this column is not sortable (no `ColHeader`), a simple styled `<th>` with click/key handlers is clean and sufficient.

**CSS**
Add `cursor: pointer` and a hover underline to `th.color-head` to signal interactivity without overloading existing `.data-table th` styles.

## Risks / Trade-offs

- [Browser variation] Some browsers open the color picker on focus, others require a click directly on the input → the header click focuses the input, which is the best we can do without simulating a click on the input (which could be jarring). Acceptable: the user can press Enter or click the now-focused input.
- [Scope creep] Making the column header interactive may prompt requests to make it sortable. Decline — sorting color swatches has no useful ordering without additional UX work.
