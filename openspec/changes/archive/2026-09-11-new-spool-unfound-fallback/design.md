## Context

The `SpoolmanDbSearch` component currently accepts a single `on_select` callback and manages its query and results entirely internally. The Spool Create page uses it only to receive a selected entry for auto-fill. There is no way for the parent to know the current search text or whether the search returned results.

The requested behavior requires the parent to react to a "non-empty query, zero results" state: disable the filament selector and use the search text as the spool name.

## Goals / Non-Goals

**Goals:**
- `SpoolmanDbSearch` exposes its query and no-results state to the parent via an additional callback.
- Spool Create disables the filament dropdown when the search is non-empty and has no results.
- Spool Create pre-fills the spool name field with the search text in that state.
- Normal behavior (enabled filament dropdown, no pre-fill) resumes when the query is cleared or a result is selected.

**Non-Goals:**
- No changes to the Filament Create or Filament Edit pages -- those pages do not use the filament dropdown in the same way.
- No backend changes.
- No changes to auto-create behavior when a DB entry is selected.

## Decisions

### Extend `SpoolmanDbSearch` with an `on_search_state` callback

The component gains an optional `on_search_state: Option<Callback<(String, bool)>>` parameter where the tuple is `(current_query, has_results)`. The component calls this whenever the query or the results list changes. Using a single callback avoids two separate effect paths and keeps the API minimal.

Alternative considered: expose the query as a writable signal passed in from the parent. Rejected -- Leptos signal threading through component props is more verbose and makes the component harder to reuse.

Alternative considered: expose separate `on_query_change` and `on_no_results` callbacks. Rejected -- requires two handlers in the parent where one is sufficient.

### Parent derives disabled/prefill state from the callback

In `spool.rs`, a `RwSignal<(String, bool)>` stores the last search state. Two derived signals drive:
1. `filament_disabled`: `query.len() > 0 && !has_results`
2. `name_prefill`: when `filament_disabled` becomes true, the name field is pre-filled with the query; when it becomes false, the name field is not automatically cleared (the user may have edited it).

The filament dropdown gets a `disabled` attribute bound to `filament_disabled`. A helper label (e.g. "Not found in database -- filament selection disabled") is shown when disabled.

## Risks / Trade-offs

- **Name not cleared on query clear**: If the user types a query (no results, name pre-filled), then clears the query, the name field retains the pre-filled value. This is intentional -- silently erasing user input would be worse. The user can clear it manually.
- **Leptos reactivity edge**: The `on_search_state` effect fires on every keystroke. This is fine -- the callback only updates a signal and triggers no async work.
