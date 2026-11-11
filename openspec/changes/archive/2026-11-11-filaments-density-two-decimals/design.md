## Context

`format::format_density` calls `sm_format_decimal(value, 0, 3)` (min 0, max 3 fraction digits). It is used in the filaments table cell and the filament detail view.

## Goals / Non-Goals

**Goals:** density shown as `x.xx` (2 fraction digits, rounded) in the filaments table.

**Non-Goals:** changing form input precision, stored values, or other formatters (`format_weight`, `format_mm`).

## Decisions

- Change `format_density` to `sm_format_decimal(value, 2, 2)` rather than adding a table-only formatter. The detail view shares the function and the existing spec treats both views together; one consistent format avoids a second function for a single use.
- Rounding is done by `Intl.NumberFormat` (half-expand), so no Rust-side rounding.

## Risks / Trade-offs

- Detail view loses the third digit (`1.245` shows `1.25`). Accepted; the edit form still shows the exact stored value.
