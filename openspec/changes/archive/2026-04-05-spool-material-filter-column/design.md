## Context

The spool list table (`SpoolList` in `crates/spoolman-client/src/pages/spool.rs`) currently shows ID, Filament, Color, Remaining, Location, and Registered columns. Material type is available on each spool via `sr.filament.material` (an `Option<MaterialType>`) but is not displayed.

The color column already uses an in-header `<select>` dropdown for its level filter, providing a proven pattern to follow. All data is available client-side in the already-fetched spool list — no backend changes are needed.

## Goals / Non-Goals

**Goals:**
- Display material abbreviation (e.g. "PLA", "PETG") in a new Material column
- Material column header contains a `<select>` dropdown populated with the distinct materials present in the full (pre-filter) spool list, plus an "All" option
- Selecting a material filters the displayed spools; selecting "All" clears the filter
- Active-filter indicator (■) appears in the column header when a material filter is active

**Non-Goals:**
- Sorting by material (material is derived from filament; the filament sort covers this)
- Persisting the material filter across page loads
- Server-side material filtering

## Decisions

### 1. Client-side signal for selected material

Add a `material_filter: RwSignal<String>` (empty = all) to `SpoolList`. The `filtered()` closure gains an additional predicate: if `material_filter` is non-empty, only spools whose `filament.material.map(|m| m.abbreviation()) == Some(&material_filter)` pass.

**Alternative considered:** re-use the existing text search box for material. Rejected — a dropdown is faster and avoids ambiguous partial matches.

### 2. Dropdown options derived from the full spool list

Options are computed from `spools` (the full unfiltered resource), not from `filtered()`, so the dropdown always shows all materials in the dataset rather than shrinking as the filter is applied. Options are sorted alphabetically; blank material is omitted from the list (spools with no material always pass the filter when "All" is selected).

### 3. Column header structure mirrors the color column

The `<th>` contains:
- A label span showing "Material" with a ■ indicator when a filter is active
- A `<select>` with "All" + one `<option>` per distinct material abbreviation

No popup overlay needed (unlike color); the select is the entire interaction surface.

### 4. Material filter applied after text filter, before color filter

Filter predicates are independent — the `filtered()` closure applies text AND material AND color. Order does not matter for correctness.

## Risks / Trade-offs

- [Dropdown grows large for datasets with many material types] → Acceptable; the full `MaterialType` enum has ~42 variants but any real dataset will have far fewer. No mitigation needed.
- [Spools with `material: None` are hidden when a material filter is active] → Correct and expected — they simply don't match any material-specific selection. They remain visible when "All" is selected.
