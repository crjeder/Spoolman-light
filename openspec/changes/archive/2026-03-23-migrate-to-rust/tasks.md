## 1. Cargo Workspace Setup

- [x] 1.1 Create `Cargo.toml` workspace at repo root listing all three crates
- [x] 1.2 Create `crates/spoolman-types/` crate skeleton (`Cargo.toml`, `src/lib.rs`)
- [x] 1.3 Create `crates/spoolman-server/` crate skeleton (`Cargo.toml`, `src/main.rs`)
- [x] 1.4 Create `crates/spoolman-client/` crate skeleton (`Cargo.toml`, `src/main.rs`)
- [x] 1.5 Add `cargo-leptos` to dev toolchain and verify `cargo leptos build` runs

## 2. Shared Types Crate (`spoolman-types`)

- [x] 2.1 Define `Rgba(u8, u8, u8, u8)` with serde derives
- [x] 2.2 Define `Filament` struct (all fields per spec, no color fields)
- [x] 2.3 Define `Spool` struct (all fields per spec, colors: Vec<Rgba>)
- [x] 2.4 Define `Location` struct
- [x] 2.5 Define `DataStore` struct (meta, filaments, spools, locations, settings)
- [x] 2.6 Define request/response wrapper types (CreateSpool, UpdateSpool, SpoolResponse with derived metrics, etc.)
- [x] 2.7 Ensure all types derive `Serialize`, `Deserialize`, `Clone`, `Debug`

## 3. Backend — JSON Store (`spoolman-server`)

- [x] 3.1 Implement `JsonStore` struct with `Arc<RwLock<DataStore>>` and `PathBuf`
- [x] 3.2 Implement `load()` — read JSON file or create empty store
- [x] 3.3 Implement atomic `flush()` — write to `.tmp` then `rename`
- [x] 3.4 Implement random u32 ID generation with collision detection
- [x] 3.5 Implement filament CRUD methods on `JsonStore`
- [x] 3.6 Implement spool CRUD methods (including archive/unarchive)
- [x] 3.7 Implement location CRUD methods
- [x] 3.8 Implement derived metric calculation (used_weight, remaining_filament, remaining_pct)
- [x] 3.9 Implement `find_materials()` and `find_lot_numbers()` scan helpers

## 4. Backend — Axum Routes

- [x] 4.1 Set up Axum app with shared `JsonStore` state via `Arc`
- [x] 4.2 Implement `GET /health` and `GET /info` endpoints
- [x] 4.3 Implement `GET /api/v1/filament` with filter/sort/pagination
- [x] 4.4 Implement `POST /api/v1/filament` (create)
- [x] 4.5 Implement `GET /api/v1/filament/<id>` (show)
- [x] 4.6 Implement `PATCH /api/v1/filament/<id>` (update)
- [x] 4.7 Implement `DELETE /api/v1/filament/<id>` with referential integrity check
- [x] 4.8 Implement `GET /api/v1/filament/search?q=` proxy to SpoolmanDB
- [x] 4.9 Implement `GET /api/v1/spool` with filter/sort/pagination and allow_archived param
- [x] 4.10 Implement `POST /api/v1/spool` (create, sets current_weight = initial_weight)
- [x] 4.11 Implement `GET /api/v1/spool/<id>` (show with derived metrics)
- [x] 4.12 Implement `PATCH /api/v1/spool/<id>` (update, auto-sets last_used on weight change)
- [x] 4.13 Implement `DELETE /api/v1/spool/<id>`
- [x] 4.14 Implement `POST /api/v1/spool/<id>/clone`
- [x] 4.15 Implement `GET /api/v1/location`, `POST`, `PATCH /<id>`, `DELETE /<id>`
- [x] 4.16 Implement `GET /api/v1/material` (distinct materials from filaments)
- [x] 4.17 Implement `GET /api/v1/export` (full DataStore JSON download)
- [x] 4.18 Implement settings endpoints (`GET /api/v1/setting`, `PUT /api/v1/setting/<key>`)
- [x] 4.19 Add static file serving for WASM frontend assets

## 5. Backend — Environment & Config

- [x] 5.1 Parse env vars: `SPOOLMAN_DATA_FILE`, `SPOOLMAN_HOST`, `SPOOLMAN_PORT`, `SPOOLMAN_BASE_PATH`, `SPOOLMAN_DEBUG_MODE`, `SPOOLMAN_LOGGING_LEVEL`, `SPOOLMAN_CORS_ORIGIN`
- [x] 5.2 Implement CORS middleware (conditional, from env)
- [x] 5.3 Implement GZip middleware
- [x] 5.4 Implement structured logging (tracing crate)
- [x] 5.5 Implement automatic backup scheduler (daily, 5 rotating copies) — optional, behind env flag

## 6. Frontend — Shell & Routing (`spoolman-client`)

- [x] 6.1 Set up Leptos app entry point and `cargo-leptos` build config
- [x] 6.2 Implement top-level layout: sidebar navigation, header, footer with version
- [x] 6.3 Implement dark mode toggle with CSS variable switching
- [x] 6.4 Implement routing (spools, filaments, locations, settings, home, help)
- [x] 6.5 Set up i18n infrastructure (leptos-i18n or equivalent), English translations only
- [x] 6.6 Implement API client layer (typed fetch wrappers using spoolman-types)

## 7. Frontend — Table Infrastructure

- [x] 7.1 Implement `use_table_state(namespace: &str) -> TableState` with reactive signals for sort, filter, pagination
- [x] 7.2 Implement localStorage persistence for table state keyed by namespace
- [x] 7.3 Implement shared `<Pagination>` component
- [x] 7.4 Implement shared column header component with sort indicator and filter input

## 8. Frontend — Spool Pages

- [x] 8.1 Implement spool list page with columns: id, filament name, color swatch, remaining %, remaining weight, location, registered, comment
- [x] 8.2 Implement column visibility toggle on spool list
- [x] 8.3 Implement show archived toggle on spool list
- [x] 8.4 Implement spool create form: filament selector, color picker (Vec<Rgba>), color name, initial weight, location dropdown, comment
- [x] 8.5 Implement spool edit form (same fields as create)
- [x] 8.6 Implement spool show page (read-only detail view with derived metrics)
- [x] 8.7 Implement spool clone action
- [x] 8.8 Implement archive/unarchive action with confirmation dialog for non-empty spools
- [x] 8.9 Implement delete action with confirmation dialog

## 9. Frontend — Filament Pages

- [x] 9.1 Implement filament list page with columns: manufacturer, material, modifier, diameter, net_weight, density, registered
- [x] 9.2 Implement filament create form: all spec fields; include SpoolmanDB search (nice to have)
- [x] 9.3 Implement filament edit form
- [x] 9.4 Implement filament show page
- [x] 9.5 Implement SpoolmanDB search modal: search field, results list, select to pre-fill both filament and spool forms (nice to have)

## 10. Frontend — Location Pages

- [x] 10.1 Implement location list page showing name and spool count per location
- [x] 10.2 Implement location create/edit inline or modal
- [x] 10.3 Implement location delete with confirmation dialog
- [x] 10.4 Implement location dropdown component (reused in spool create/edit)

## 11. Frontend — Settings & Home

- [x] 11.1 Implement settings page (currency symbol, other key-value settings)
- [x] 11.2 Implement home/dashboard page (total spools, total filaments, recently used spools)
- [x] 11.3 Implement help page (static content, links to documentation)

## 12. Docker & Deployment

- [x] 12.1 Write `Dockerfile` using multi-stage build: Rust build stage → minimal runtime image
- [x] 12.2 Verify single binary serves both API and static WASM assets
- [x] 12.3 Update `docker-compose.yml` (if present) for new binary entrypoint
- [x] 12.4 Verify `SPOOLMAN_DATA_FILE` env var mounts correctly in container
- [x] 12.5 Update `README.md` with new stack, build instructions, and env var reference
