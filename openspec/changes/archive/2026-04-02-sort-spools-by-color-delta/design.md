## Context

The spool list page (`crates/spoolman-client/src/pages/spool.rs`) has two reactive closures in sequence:
1. `filtered()` — applies text search and color distance threshold to produce a `Vec<SpoolResponse>`
2. `sorted()` — sorts `filtered()` by a user-chosen column (`sort_field` / `sort_asc` signals from `TableState`)

Color filtering already computes `color_distance(c, &target)` per spool color, but only for the `any(…≤ thresh)` predicate. That distance value is discarded — it is never used for ordering.

## Goals / Non-Goals

**Goals:**
- When a color level is active (not "off"), spools are sorted by their minimum ΔE*00 from the selected color, ascending (best match first).
- When the level is "off" or no valid hex is selected, the existing column-based sort is used unchanged.
- The sort-by-delta mode overrides any currently selected column sort — the user does not need to do anything extra.

**Non-Goals:**
- No new UI controls (no "sort by delta" toggle).
- No server-side changes — this is purely client-side reactive state.
- No change to how filtering works — `filtered()` is unchanged.
- Does not affect other sort columns or the `TableState` signals.

## Decisions

### Compute min-delta inside `sorted()`, not `filtered()`

The `filtered()` closure already iterates spool colors for the threshold predicate. We could cache the min-delta there, but `filtered()` returns a plain `Vec<SpoolResponse>` — adding a parallel delta vec or a wrapper type would change its shape.

Instead, `sorted()` captures the same `color_pick` and `color_level` signals it already needs (they're in scope), detects the active-color condition, and computes `min_delta` inline per spool using the same `hex_to_rgba` + `color_distance` calls. The computation is O(n × colors_per_spool) and runs only when the reactive signal changes — acceptable for typical spool counts (< 1000).

**Alternative considered:** a third `sorted_by_delta()` closure chained after `sorted()`. Rejected — adds an extra reactive layer and a second full allocation of the vec for no benefit.

### Use minimum delta across all spool colors

A spool may have multiple colors (e.g., a gradient filament). The sort key is `min(color_distance(c, target) for c in spool.colors)` — the same criterion already used for filtering inclusion. This keeps filtering and sorting semantically consistent.

### Delta-sort overrides column sort silently

When color search is active the column headers still show and remain clickable, but their sort has no effect. This matches the existing behavior where color filtering already re-orders results implicitly. A future improvement could disable or visually mark the column headers when delta-sort is active, but that is out of scope.

## Risks / Trade-offs

- **Stale sort field indicator** — The active `ColHeader` sort indicator (▲/▼) may still show on a column while delta-sort is in effect, which could confuse users. Mitigation: acceptable for now; the color level selector visually communicates that color mode is active.
- **Re-computation cost** — `sorted()` recomputes min-delta for every spool on each reactive update (color pick change, level change, data reload). For large collections this is proportional to O(n × k) where k = colors per spool. Mitigation: client-side WASM is fast enough for spool counts in the hundreds; memoisation can be added later if needed.

## Migration Plan

No migration needed — this is a pure client-side behavioral change with no data model or API impact. Deploy by rebuilding and redeploying the WASM client.
