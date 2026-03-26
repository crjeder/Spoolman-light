## Why

Numeric table columns (ID, Remaining %, Remaining weight, Diameter, Net weight, Density) are currently left-aligned like text columns, making values harder to scan and compare. Right-aligning numbers is a standard data table convention that improves readability at a glance.

## What Changes

- Add a `.num` CSS utility class with `text-align: right` to `spoolman.css`
- Apply `.num` to the header `<th>` / `<ColHeader>` and data `<td>` cells for each numeric column in the Spools and Filaments list views

**Spools table numeric columns:** ID, Remaining%, Remaining (g)
**Filaments table numeric columns:** Diameter, Net weight, Density

## Capabilities

### New Capabilities

- `numeric-column-alignment`: CSS utility class `.num` that right-aligns table cells containing numeric values; applied to header and body cells in spool and filament list views

### Modified Capabilities

<!-- No spec-level requirement changes -->

## Impact

- `style/spoolman.css` — one new rule (`.num { text-align: right }`)
- `crates/spoolman-client/src/pages/spool.rs` — `class="num"` on numeric `<th>` and `<td>` elements
- `crates/spoolman-client/src/pages/filament.rs` — same
- No API, data model, or backend changes
