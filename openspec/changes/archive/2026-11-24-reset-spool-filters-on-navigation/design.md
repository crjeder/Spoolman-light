## Context

`/spools` and `/colors` both mount `SpoolList`, which reads/writes `filter.spools.{material,location,color_pick,color_level,show_archived}` in `sessionStorage`. The text filter is namespaced per table (`filter.spools.text` / `filter.colors.text`), the rest are not. Switching tabs remounts the component, which re-reads the shared keys, so the Color tab's material filter appears on the Spools tab.

## Goals / Non-Goals

**Goals:** any menu/tab change opens the target list with default filters.
**Non-Goals:** per-tab filter storage; changing reload or detail-page-return persistence.

## Decisions

- **Clear on nav click, not on mount.** The nav `<ul class="nav-links">` already has an `on:click` handler (closes the mobile menu). It additionally removes every `sessionStorage` key starting with `filter.`. Clicking happens before the router navigates, so the new `SpoolList`/`FilamentList` initialises from defaults. Clearing on mount would also wipe filters on detail-page returns, breaking the existing "survives navigation" requirement.
- **Prefix sweep (`filter.`) over a key list.** New filters are covered automatically.
- **Alternative rejected:** separate keys per mode. Fixes the leak but leaves stale filters when moving Spools -> Filaments -> Spools, which the user also wants reset.

## Risks / Trade-offs

- Clicking the already-active menu item clears storage while the mounted component keeps its in-memory signals; the UI is unchanged until the next remount. Acceptable; use "Clear filters" for an in-place reset.
