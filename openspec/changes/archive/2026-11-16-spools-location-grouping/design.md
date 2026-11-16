## Context

`SpoolList` (`crates/spoolman-client/src/pages/spool.rs`) already filters client-side (`filtered`), sorts (`sorted`, sort field `"location"` set by the Location header button) and paginates (`page_items`). Location names come from the `locations` resource. Filters: text search (`ts.filter`), material set (`material_filter`), colour (`color_level`/`color_pick`), location dropdown (`location_filter`), show-archived.

## Goals / Non-Goals

**Goals:**
- Grouped, collapsed-by-default view per location when the table is sorted by Location.
- Group row shows count and the active filter values.

**Non-Goals:**
- No server/API change, no persistence of expanded state, no grouping in the colour swatch view, no grouping by other columns.

## Decisions

- **Grouping is a function of the sort field, not a new toggle.** Grouped iff `mode == Spool && ts.sort_field == "location"`. The header click already sets that field; a second click flips `sort_asc` (group order). Sorting by another column leaves grouped mode. Alternative: a separate "group by" control - rejected, more state and UI for the same effect, and the request ties it to the header click.
- **Groups built from `sorted()`.** Spools are already location-sorted (None last); consecutive spools with equal `location_id` form a group. Within a group, order is the existing sort order, so no extra sorting code. Group label from the `locations` resource (fallback to the id, as the flat table does); `None` -> `No location`.
- **Expanded state:** `RwSignal<HashSet<Option<u32>>>`, in memory only. Row click toggles. Collapsed shows `+ Name: n entries`, expanded `- Name: n entries`. "1 entries" is avoided: `1 entry`.
- **Group row cells:** label in the Location cell, all other cells empty except per active filter: Filament = search text (if non-empty), Material = selected materials joined by `, ` (when the set differs from "all"), Color = a swatch of `color_pick` (when `color_level != off`). The location filter needs no extra display (the label is the location). Show-archived has no column, so it is not rendered. Row click handler on the whole `<tr>`; no action buttons in group rows.
- **Pagination:** in grouped mode the page unit is a group (`page_size` groups per page); `total` counts groups. Expanded spools render under their group without counting against the page. Alternative: paginate spools and split groups across pages - rejected, a group's count would be misleading.
- **Counts** are computed from the filtered list, so they reflect all active filters.
- **Implementation size:** one helper `group_by_location(items) -> Vec<(Option<u32>, Vec<SpoolResponse>)>` plus a branch in `<tbody>`; the per-spool row view is extracted into a closure so flat and expanded rows share it.

## Risks / Trade-offs

- [Surprise: sorting by Location now also collapses rows] -> It is the requested behaviour; sorting by another column restores the flat table.
- [Row view extraction touches existing markup] -> Keep the extraction mechanical; e2e flat-table tests guard regressions.
- [Group-level pagination makes `total` differ between modes] -> `Pagination` takes a signal, so `total` is derived per mode.
