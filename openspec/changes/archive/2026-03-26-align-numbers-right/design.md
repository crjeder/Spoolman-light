## Context

The spool and filament list views display numeric values (IDs, weights, percentages, density) in plain `<td>` cells with no alignment override. All cells currently inherit `text-align: left` from the base stylesheet, making numeric columns visually inconsistent with data-table conventions.

## Goals / Non-Goals

**Goals:**
- Right-align numeric `<th>` and `<td>` cells in the Spools and Filaments list tables
- Keep the fix contained to CSS + Leptos view markup — no logic or data-model changes

**Non-Goals:**
- Monospace/tabular numerals (separate concern)
- Aligning numeric fields on detail/show pages (`<dl>` layout)
- Any column not containing a numeric value

## Decisions

### Add a `.num` CSS utility class

**Decision:** Add `.num { text-align: right }` to `spoolman.css` and apply it to matching `<th>` / `<td>` elements by hand.

**Rationale over alternatives:**
- *Inline `style` attributes* — clutters RSX markup and can't be toggled globally
- *CSS `:nth-child` selectors* — brittle; breaks when columns are reordered or toggled
- *`data-type="number"` attribute* — more semantic but requires a parallel CSS rule anyway; `.num` is idiomatic and brief

Columns to mark `.num`:

| Table    | Column        | Element type                |
|----------|---------------|-----------------------------|
| Spools   | ID            | `<ColHeader>` + `<td>`      |
| Spools   | Remaining%    | `<ColHeader>` + `<td>`      |
| Spools   | Remaining (g) | `<ColHeader>` + `<td>`      |
| Filaments| Diameter      | plain `<th>` + `<td>`       |
| Filaments| Net weight    | plain `<th>` + `<td>`       |
| Filaments| Density       | `<ColHeader>` + `<td>`      |

`ColHeader` renders a `<th>` element; adding `class="num"` to it propagates through the existing `.col-header` styles unchanged.

## Risks / Trade-offs

- [Risk] Forgetting to add `.num` to both the header *and* the data cell → header and data misalign visually. Mitigation: tasks checklist covers every cell pair explicitly.
- [Risk] Future column additions omit `.num` → regresses silently. Mitigation: the convention is documented here; the class name is self-explanatory.
