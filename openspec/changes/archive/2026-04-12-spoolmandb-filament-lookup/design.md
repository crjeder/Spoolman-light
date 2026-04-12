## Context

Filament and spool forms currently require all fields to be entered manually. SpoolmanDB (`https://donkie.github.io/SpoolmanDB/filaments.json`) is a CORS-safe, publicly accessible static JSON file (~800+ entries) containing real-world filament product data including manufacturer, material, physical properties, and color. The client crate already has `gloo-net` for HTTP and `web-sys` with the `Storage` feature for localStorage — no new dependencies are required.

The Leptos WASM frontend runs entirely in the browser. All integration must be client-side only; no server changes are needed or planned.

## Goals / Non-Goals

**Goals:**
- Fetch SpoolmanDB JSON from the browser and cache it in localStorage
- Provide a reusable search panel component usable in Filament Create, Filament Edit, and Spool Create
- Auto-fill filament fields from a selected SpoolmanDB entry
- In Spool Create: auto-fill color/weight fields and, if no matching filament exists, create one via the API and notify the user
- Parse SpoolmanDB composite material strings into `MaterialType` + `material_modifier`

**Non-Goals:**
- filamentcolors.xyz integration (deferred — lacks CORS headers, needs server proxy)
- Spool Edit form lookup (color already set; filament already linked)
- Syncing/writing back to SpoolmanDB
- Any server-side changes

## Decisions

### D1: Full JSON download + client-side filtering

**Decision:** Fetch the entire `filaments.json` (~1 MB) once and filter in WASM memory.

**Rationale:** SpoolmanDB has no search API — it's a static file. 1 MB is well within WASM memory budget and downloads in <1s on typical home network. Client-side filtering is instant (no round-trip) and works offline after first load.

**Alternative considered:** Fetch only on search submission, parse each time. Rejected — repeated network calls, slower UX, no offline support.

### D2: localStorage cache with 24h TTL + ETag

**Decision:** Store the parsed JSON array and response metadata in localStorage under key `spoolmandb_cache`. Structure:
```json
{ "data": [...], "etag": "\"abc123\"", "fetched_at": 1712930000000 }
```
On load: if `fetched_at` is within 24h, use cached data. Otherwise, re-fetch with `If-None-Match: <etag>`; on 304 bump `fetched_at`; on 200 replace data + etag.

**Rationale:** `web-sys::Storage` is already wired up. 24h TTL prevents stale data accumulating without hammering GitHub Pages. ETag avoids re-downloading unchanged data. Offline: if fetch fails, serve stale cache regardless of age.

**Alternative considered:** Session-only in-memory cache (lost on tab close). Rejected — would re-fetch on every browser session.

**Alternative considered:** `gloo-storage` crate. Not needed — raw `web_sys::window().unwrap().local_storage()` is sufficient and avoids an extra dependency.

### D3: Search panel as a standalone Leptos component

**Decision:** Implement `SpoolmanDbSearch` as a self-contained `#[component]` that accepts a callback:
```rust
#[component]
fn SpoolmanDbSearch(on_select: Callback<SpoolmanEntry>) -> impl IntoView
```
The parent form passes a closure that reads the selected entry and writes to its own signals.

**Rationale:** Avoids duplicating search/filter/render logic across three call sites. The callback pattern is idiomatic Leptos and keeps the component decoupled from form-specific signals.

### D4: Material string parsing

**Decision:** A pure function `parse_material(s: &str) -> (MaterialType, Option<String>)` handles SpoolmanDB's composite strings:

- Strip known suffixes in order: `-CF`, `-GF`, `-HF`, `-ESD`, `+`
- The stripped base is passed to `MaterialType::from_abbreviation()`
- The removed suffix (if any) becomes `material_modifier`

Examples:
| SpoolmanDB `material` | `MaterialType` | `material_modifier` |
|---|---|---|
| `"PLA"` | `Pla` | `None` |
| `"PLA+"` | `Pla` | `Some("+")` |
| `"PETG-CF"` | `Petg` | `Some("CF")` |
| `"ABS-GF"` | `Abs` | `Some("GF")` |
| `"NYLON-CF"` | `Other("NYLON")` | `Some("CF")` |

Unknown base strings fall through to `Other(string)` — `from_abbreviation` already handles this.

### D5: Auto-create filament in Spool Create

**Decision:** When a SpoolmanDB entry is selected in Spool Create:
1. Search `list_filaments()` result (already fetched for the filament dropdown) for a match on `manufacturer + material + diameter` (case-insensitive manufacturer, exact material enum, diameter within ±0.01mm).
2. If found → use that filament's id, set the dropdown to it.
3. If not found → call `create_filament()` with fields from the entry, set the new id in the dropdown, show a dismissable info banner: _"Filament '[Manufacturer] [Material]' was created automatically."_

**Rationale:** Keeps the spool create flow smooth — the user selected a specific product variant and shouldn't have to navigate away to create a filament first. The notification is important so the user knows a new record was created.

**Alternative considered:** Prompt the user to confirm before creating. Rejected — adds friction; the user explicitly chose the entry.

## Risks / Trade-offs

- **SpoolmanDB availability** → Mitigation: stale localStorage cache serves as fallback; search panel degrades gracefully (show last-known data or a "database unavailable" message if cache is also empty)
- **localStorage quota** → 1 MB JSON is well under the 5–10 MB browser limit; no action needed
- **Material mapping gaps** → Unknown composites fall to `Other(string)` in `material_modifier`, which is correct behaviour; no data is lost
- **Filament duplicate creation** → Match heuristic (manufacturer + material + diameter) may miss near-duplicates with different casing or whitespace. Mitigation: normalize strings (trim, lowercase) before comparison
- **SpoolmanDB JSON schema drift** → SpoolmanDB is community-maintained and may add/remove fields. Mitigation: deserialize into an explicit struct with `#[serde(default)]` on optional fields; unknown fields ignored with `#[serde(deny_unknown_fields)]` removed (i.e., use lenient deserialization)

## Open Questions

_(none — all decisions made during exploration)_
