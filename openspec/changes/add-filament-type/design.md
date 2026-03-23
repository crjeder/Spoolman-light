## Context

### Relevant backend endpoints

- `GET /api/v1/material` → `Vec<String>` — distinct, sorted material strings across all filaments in the store. Already implemented in `crates/spoolman-server/src/routes/other.rs` → `store.find_materials()`.
- `GET /api/v1/filament?material=<value>` → filtered filament list. The `material` query param is already supported by the list handler.

### Relevant frontend code

- `crates/spoolman-client/src/api/filament.rs` — typed API client. Has `list_filaments()`, `get_filament()`, `create_filament()`, `update_filament()`, `delete_filament()`. No `list_materials()`.
- `FilamentCreate` / `FilamentEdit` (in `pages/filament.rs`) — both have a plain `<input type="text">` for the material field with no autocomplete.
- `FilamentList` — has a single free-text filter that matches against `display_name()` client-side. No material-specific filter.
- `SpoolList` — text filter matches `filament.display_name()` and `spool.color_name`. Does not match `filament.material` separately. Does not show a material column.

### HTML `<datalist>` mechanics

```html
<input list="materials-list" ... />
<datalist id="materials-list">
  <option value="PLA" />
  <option value="PETG" />
</datalist>
```
The browser renders native autocomplete without JavaScript. In Leptos this is expressed as:
```rust
view! {
  <input list="materials-list" ... />
  <datalist id="materials-list">
    {materials.get().unwrap_or_default().into_iter()
        .map(|m| view! { <option value=m /> })
        .collect_view()}
  </datalist>
}
```

## Goals / Non-Goals

**Goals:**
- Autocomplete on material input using existing `GET /api/v1/material` data.
- Material filter dropdown on filament list.
- Material matched in spool list text filter.

**Non-Goals:**
- Enforcing an enum / closed vocabulary — material remains a free-text field; datalist is advisory only.
- A new backend field or endpoint.
- Color-based or RGBA filtering (separate TODO item).

## Decisions

### Decision: `<datalist>` over a `<select>` for the material input

A `<select>` would prevent users from entering materials not yet in the store (e.g. first PLA spool). A `<datalist>` provides suggestions while still accepting arbitrary input — the right UX for an advisory list.

**Alternative considered**: client-side combo-box component. Rejected — adds complexity; native `<datalist>` is sufficient and zero-dependency.

### Decision: material filter on filament list is a `<select>` (not free text)

Unlike the material *input* on a form (where typing new values is expected), the list *filter* benefits from a constrained dropdown: users see exactly which materials exist and can pick with one click. An "All" option at the top resets the filter.

When a material is selected, the URL query param `?material=<value>` is passed to `GET /api/v1/filament` so filtering is server-side (consistent with how `offset`/`limit` work).

**Alternative considered**: client-side filter on the already-loaded list. Rejected — server-side is consistent with the existing pagination model and handles large filament counts correctly.

### Decision: spool list text filter extended to match `filament.material`

Users type "PLA" in the spool search and expect to see all PLA spools. The change adds `filament.material` to the match expression alongside `display_name()` and `color_name`. No UI change needed — just an extra `||` in the filter closure.

## Risks / Trade-offs

- **[Risk] `GET /api/v1/material` called on every filament create/edit page load** — the list is small (< 50 entries typical) so the extra request is negligible.
- **[Risk] Stale datalist during a session** — if a user creates a new material in one tab, another tab's datalist won't update. Acceptable for a single-user home app; a refresh picks up the new entry.
- **[Risk] Material filter resets pagination** — when the material dropdown changes, the page must reset to 0 to avoid showing an empty slice. Handled by resetting `ts.page` on filter change.
