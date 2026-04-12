## Context

The server-side `GET /api/v1/spool?location_id=<id>` filter is already implemented in `SpoolFilter` and `store.list_spools()`. The gap is entirely in the frontend:

- `api::spool::list_spools` only accepts `allow_archived: bool` — no `location_id` parameter
- The spool list page has no location filter UI
- The Location column in the spool table displays the raw `location_id` integer instead of the location name

Locations are already loaded on the spool detail page; the spool list page does not fetch them.

## Goals / Non-Goals

**Goals:**
- Expose the existing server-side `location_id` filter through the frontend
- Add a location filter dropdown to the spool list page
- Display location names in the Location column (resolving from the fetched locations list)

**Non-Goals:**
- Server-side changes (none needed)
- Multi-location filter (single selection only)
- URL persistence of the filter state

## Decisions

**Decision: fetch locations on spool list page load**

The spool list page will call `api::location::list_locations()` (already exists) in a Leptos `Resource`. The dropdown is populated from this list. Cost is one extra request on page load; the list is typically small.

Alternatively, we could fetch locations lazily (only when the dropdown is opened), but this complicates the UX with a loading state in the middle of interaction. Eager fetch on load is simpler and consistent with how the spool detail page works.

**Decision: pass `location_id` to `list_spools` and re-fetch on change**

`list_spools` will gain a `location_id: Option<u32>` parameter. The spool list resource will take `location_id` as a signal dependency so it re-fetches automatically when the filter changes. This is the same pattern used by `allow_archived` toggling (if present) and avoids client-side filtering of a stale full list.

**Decision: resolve location name in the table column**

Instead of showing a raw integer, the Location column will look up the name from the fetched locations list. When the locations resource is loading or errored, fall back to the id string. This reuses the already-fetched locations resource — no extra requests.

## Risks / Trade-offs

- [Risk] Locations list and spool list are fetched in parallel; the Location column may briefly show raw IDs on slow connections → Mitigation: Suspense wraps both; in practice both requests resolve quickly from the same origin.
- [Risk] Filter resets on page navigation → Acceptable for now; not in scope.
