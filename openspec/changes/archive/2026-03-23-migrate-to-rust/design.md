## Context

Spoolman is a self-hosted filament tracking tool with a Python/FastAPI backend and a React/Refine/Ant Design frontend. The backend was recently simplified from SQLAlchemy/Alembic to a JSON file store, leaving no database migration concerns. There are no existing deployments with data to preserve. The JSON storage format is unconstrained.

The current frontend uses the Refine meta-framework which adds deep abstraction layers (data providers, resource system, hook wrappers) that obscure what the app actually does. The goal is a leaner, more transparent stack.

## Goals / Non-Goals

**Goals:**
- Single Rust binary serving both API and frontend WASM
- Shared types crate ensuring compile-time API contract consistency
- Data model aligned with OpenTag3D / OpenPrintTag NFC tag standards
- Idiomatic Leptos frontend — no translated React patterns
- Significantly smaller Docker image

**Non-Goals:**
- Preserving the existing JSON data format
- Backward compatibility with the Python API responses
- Feature parity with removed features (vendor, WebSocket, extra fields, QR, kbar, PWA)
- Supporting multiple database backends

## Decisions

### D1: Cargo workspace with shared types crate

```
spoolman/                   ← workspace root
  Cargo.toml
  crates/
    spoolman-types/         ← shared: Spool, Filament, Location, DataStore
    spoolman-server/        ← Axum backend
    spoolman-client/        ← Leptos WASM frontend
```

**Why**: Both server and client compile against the same `spoolman-types` crate. Type mismatches between API and UI are compile errors, not runtime bugs. Eliminates the TypeScript ↔ Python model drift that existed before.

**Alternative considered**: Separate repos with code generation (OpenAPI → types). Rejected: adds tooling, a generation step, and still allows drift between the schema and the generator output.

---

### D2: Axum for the backend

Axum is the standard choice in the Rust async ecosystem. It composes cleanly with Tower middleware, has first-class WebSocket support (retained for future use), and integrates naturally with Tokio.

**Alternative considered**: Actix-web. Rejected: different async model (actor-based), more complex for this use case.

---

### D3: Leptos for the frontend

Leptos uses fine-grained reactivity (signals, not virtual DOM diffing), compiles to WASM, and has first-class server function support. It avoids the overhead of a virtual DOM for a mostly-static CRUD UI.

**Alternative considered**: Dioxus. Rejected: virtual DOM model, less mature server integration. Yew also rejected: stagnating ecosystem.

**Table strategy**: A shared `use_table_state(namespace: &str) -> TableState` function provides reactive sort/filter/pagination state with localStorage persistence. Each list page renders its own columns. No generic table wrapper — three pages, three render functions, one shared state function.

---

### D4: Color on Spool, not Filament

Filament = material specification (manufacturer, material, density, temps). Spool = physical object with color. One Filament entry covers all color variants of that material; each Spool carries its own `Vec<Rgba>` (1–4 colors) and `color_name`.

**Why**: Reduces catalog duplication ("eSun PLA Red", "eSun PLA Blue", "eSun PLA Black" → one "eSun PLA" entry). Colors on physical spools naturally belongs to the spool instance.

**NFC alignment**: OpenTag3D/OpenPrintTag tags are physically attached to spools. The color fields in the tag spec describe the specific spool's color — which matches our model. The tag's `Online Data URL` field maps to `/api/v1/spool/<id>` (without `https://` prefix, per spec).

---

### D5: Random u32 spool IDs

All entities (Spool, Filament, Location) use random u32 IDs. Collision detection on insert: generate random u32, check store, retry if collision (negligible probability at realistic scale).

**Why**: Stable across export/reimport. Sequential IDs reassign on reimport, breaking any NFC tags written with the old ID. u32 fits safely in JSON Number (max 2^32 ≈ 4B, well within JavaScript's safe integer range of 2^53).

**Alternative considered**: UUID (u128). Rejected: unnecessarily large for a personal tool; ugly in URLs and NFC tag URLs.

---

### D6: Weight model — scale readings only

Store `initial_weight` (total weight at spool creation) and `current_weight` (latest scale reading). Both are the full spool weight including empty spool tare.

**Why**: The tare cancels out in `used = initial_weight - current_weight`, so `spool_weight` is never required for usage tracking. `remaining = filament.net_weight - used` gives filament-remaining when the manufacturer net weight is known.

**Removed**: The three-mode weight entry (used/remaining/measured) from the original. Users enter the current scale reading, the system computes everything else.

---

### D7: SpoolmanDB — pull-on-demand via backend proxy

When creating a filament, the user searches SpoolmanDB from the UI. The frontend calls `GET /api/v1/filament/search?q=...`, which the backend proxies to `donkie.github.io/SpoolmanDB/`. No local cache, no background scheduler.

**Why**: Eliminates the scheduler, the cache file, and `hishel` dependency. Simpler backend, same UX. SpoolmanDB is served over GitHub Pages — fast enough for interactive search.

**Split on import**: A SpoolmanDB result is a color-specific entry. During import, material specs (manufacturer, material, density, etc.) populate the Filament form; color fields populate the Spool form. If a matching Filament already exists, the user can reuse it.

---

### D8: Location as first-class entity

Location is upgraded from a freeform string on Spool to a `Location { id: u32, name: String }` entity with full CRUD. Spools reference locations via `location_id: Option<u32>`.

**Why**: Enables create/edit/delete UI. Rename cascades automatically (edit the Location entity, all spools update). Removes the `/other/location` rename endpoint.

---

### D9: Single binary deployment

`spoolman-server` embeds the compiled WASM frontend assets (via `include_dir!` or served from `dist/`) and serves them as static files alongside the API.

```dockerfile
FROM debian:bookworm-slim
COPY spoolman /spoolman
EXPOSE 8000
CMD ["/spoolman"]
```

No Python runtime, no Node.js, no multi-stage build complexity.

---

### D10: OpenTag3D / OpenPrintTag field alignment

Filament model fields map directly to OpenTag3D core + extended spec (the two standards are byte-for-byte identical). RGBA colors use 4×u8 in sRGB + alpha. Diameter stored as f32 mm (default 1.75). Required fields per spec: manufacturer, material, colors[0], diameter, net_weight, density, print_temp, bed_temp.

## Risks / Trade-offs

**[Leptos ecosystem immaturity]** → No production-ready table or form library comparable to Ant Design. The table state function covers logic; column rendering is per-page (~50–100 lines each). Accept and build.

**[WASM bundle size]** → Initial WASM bundles can be large (500KB–2MB before compression). Mitigated by `wasm-opt` (run automatically by `cargo-leptos`) and HTTP compression (gzip/brotli). Acceptable for a self-hosted LAN tool.

**[JS interop for future features]** → If QR scanning is re-added later, it requires `web-sys` bindings to the browser camera API. Scoped out for now; the architecture supports adding it.

**[SpoolmanDB live proxy]** → If SpoolmanDB is unreachable, filament import fails. Fallback is manual entry — acceptable. No data loss risk.

**[NFC tag URL stability]** → Random u32 IDs must never be reassigned. Export/reimport preserves IDs. Any tooling that regenerates IDs would invalidate written NFC tags. Document clearly.

## Migration Plan

1. Create Cargo workspace alongside existing `spoolman/` and `client/` directories
2. Implement `spoolman-types` crate (data model, serde derives)
3. Implement `spoolman-server` (Axum routes, JSON store port)
4. Implement `spoolman-client` (Leptos UI)
5. Build and test the new stack standalone
6. Replace Docker image entrypoint to use new binary
7. Archive / remove `spoolman/` and `client/` directories

**Rollback**: The Python stack remains intact until step 6. Rolling back before that point is a no-op.

## Open Questions

- None. All design decisions resolved during exploration.
