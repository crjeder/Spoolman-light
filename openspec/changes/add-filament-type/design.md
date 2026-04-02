## Context

### Relevant backend endpoints

- `GET /api/v1/filament?material=<value>` → filtered filament list. The `material` query param is already supported by the list handler.
- No changes to backend required — all variants are compiled into the client.

### Relevant frontend code

- `crates/spoolman-client/src/api/filament.rs` — typed API client. Has `list_filaments()`, `get_filament()`, `create_filament()`, `update_filament()`, `delete_filament()`.
- `FilamentCreate` / `FilamentEdit` (in `pages/filament.rs`) — both had a plain `<input type="text">` for the material field.
- `FilamentList` — had a single free-text filter matching `display_name()` client-side.
- `SpoolList` — text filter matched `filament.display_name()` and `spool.color_name`. Did not match `filament.material` separately.

### `MaterialType` enum

Defined in `crates/spoolman-types/src/` (shared between server and client):

```rust
pub enum MaterialType {
    Pla, Petg, Abs, Asa, Tpu, Nylon, Pc, Pva, Hips,
    Wood, Metal, CarbonFiber, Other,
    // ...
}

impl MaterialType {
    pub fn all_known() -> &'static [MaterialType] { ... }
    pub fn abbreviation(&self) -> &'static str { ... }  // "PLA", "PETG", etc.
}
```

### `MaterialSelect` Leptos component

```rust
view! {
    <select on:change=...>
        {MaterialType::all_known().iter()
            .map(|m| view! { <option value=m.abbreviation()>{m.abbreviation()}</option> })
            .collect_view()}
    </select>
}
```

No async fetch needed — all variants are compiled in.

## Goals / Non-Goals

**Goals:**
- Closed `<select>` on material input using `MaterialType` enum — prevents inconsistent spellings.
- Material filter dropdown on filament list (same enum).
- Material matched in spool list text filter via `abbreviation()`.

**Non-Goals:**
- Free-text / arbitrary material entry (intentionally removed).
- A new backend field, enum, or endpoint.
- Color-based or RGBA filtering (separate TODO item).

## Decisions

### Decision: `<select>` over `<datalist>` for the material input

A `<datalist>` would still allow arbitrary free-text, which is the root cause of inconsistent spellings. A `<select>` enforces the closed vocabulary defined by `MaterialType::all_known()`. First-time users pick from the list; unknown materials fall back to `Other`.

**Alternative considered**: `<datalist>` with suggestions only. Rejected — does not prevent the inconsistency problem.

### Decision: `MaterialType` enum in `spoolman-types` (shared crate)

Placing the enum in the shared types crate lets both the server (for validation/filtering) and the client (for the select) use the same definition without duplication.

**Alternative considered**: enum only in the client. Rejected — server should be able to validate and filter by the same set.

### Decision: material filter on filament list is a `<select>` driven by `MaterialType::all_known()`

Consistent with the create/edit form. Users see exactly which materials exist and can pick with one click. An "All materials" option at the top resets the filter.

When a material is selected, `?material=<value>` is passed to `GET /api/v1/filament` for server-side filtering (consistent with how `offset`/`limit` work).

### Decision: spool list text filter extended to match `filament.material.abbreviation()`

Users type "PLA" in the spool search and expect to see all PLA spools. Adds `filament.material.abbreviation()` to the match expression alongside `display_name()` and `color_name`. No UI change needed — just an extra `||` in the filter closure.

## Risks / Trade-offs

- **[Risk] Closed vocabulary may not cover all materials** — `Other` is the escape hatch; users can open an issue to add new variants. Acceptable for a home-use app.
- **[Risk] Material filter resets pagination** — when the material dropdown changes, page must reset to 0 to avoid showing an empty slice. Handled by resetting `ts.page` on filter change.
