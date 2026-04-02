## Why

When a user performs a color search, matching spools are shown but remain in their default order — making it hard to tell which spool is the closest match at a glance. Sorting by ascending color delta when a color filter is active surfaces the best matches first, turning the color search into a ranked result list rather than a mere filter.

## What Changes

- When a color level is active (Fine / Medium / Coarse), the spool list is sorted by ascending color distance (ΔE*00) from the selected color — closest match first.
- When the color level is Off, the spool list reverts to its default sort order (spool ID ascending).
- The sort-by-delta behavior is automatic and implicit — no extra UI controls are added.
- Each spool's minimum delta across all its stored colors is used for ranking.

## Capabilities

### New Capabilities
- `color-delta-sort`: When a color filter is active, spools are ranked by ascending minimum ΔE*00 from the selected color. When no color is active, default sort order is restored.

### Modified Capabilities
- `ciede2000-color-distance`: The color distance function result is now also used for sort ordering in addition to filtering; no requirement changes — implementation detail only.
- `color-search-selector`: Sort order now depends on whether a color level is active. The selector's state implicitly controls sort mode as well as filtering.

## Impact

- `crates/spoolman-client/src/pages/spool_list.rs` — sorting logic reads the active color and computes min-delta per spool before rendering the table
- No API changes — sorting is client-side, same as filtering
- No new dependencies
