## Why

The current Python/React stack has accumulated accidental complexity: the Refine meta-framework, Ant Design, and multiple adapter packages create deep abstraction layers for what is fundamentally a CRUD app tracking 3 entity types. Migrating to a full Rust stack (Axum backend + Leptos WASM frontend) eliminates these layers, produces a single self-contained binary for Docker deployment, and establishes a shared types crate that guarantees API contract consistency between server and client at compile time.

## What Changes

- **BREAKING** Replace Python/FastAPI backend with Rust/Axum backend
- **BREAKING** Replace React/Refine/Ant Design frontend with Rust/Leptos (WASM) frontend
- **BREAKING** Redesign JSON storage format (no existing data to preserve; format is unconstrained)
- **BREAKING** Remove vendor entity entirely (vendor becomes a plain string on Filament)
- **BREAKING** Color representation changes from hex string(s) to `Vec<RGBA>` (OpenTag3D/OpenPrintTag compatible)
- **BREAKING** Color moves from Filament to Spool (Filament = material spec; Spool = physical object with color)
- **BREAKING** Spool IDs change from sequential integers to random u32 (stable across reimport)
- Replace accumulated `used_weight` tracking with `initial_weight` + `current_weight` (scale readings)
- Upgrade Location from freeform string field to first-class entity with CRUD
- Remove: WebSocket/live updates, backup endpoint, extra fields system, QR scanner/printer, kbar, PWA, drag-and-drop locations, vendor UI
- Add: `GET /filament/search` endpoint (proxies SpoolmanDB, pull-on-demand)
- Keep: all spool/filament/location CRUD, export, settings, i18n infrastructure, dark mode

## Capabilities

### New Capabilities
- `spool-management`: Full spool lifecycle — list, create, edit, clone, archive/unarchive. Weight tracked via scale readings (initial + current). Color on spool (Vec<RGBA>, OpenTag3D-compatible).
- `filament-management`: Filament catalog management — list, create, edit. Material spec only (no color). OpenTag3D/OpenPrintTag field alignment. Search/import from SpoolmanDB (pull-on-demand via backend proxy).
- `location-management`: Location as first-class entity — list, create, edit, delete. Spool edit includes location dropdown.
- `data-model`: Shared Rust types crate (`spoolman-types`) defining Spool, Filament, Location, DataStore. Random u32 IDs. JSON storage format designed for Rust/serde ergonomics.

### Modified Capabilities
<!-- No existing specs to modify — this is a full rewrite -->

## Impact

- **Entire codebase replaced**: `spoolman/` (Python) and `client/` (TypeScript/React) are superseded by a Cargo workspace
- **Docker image**: Single binary replaces Python runtime + Node build artifacts; image size drops dramatically
- **API surface**: REST endpoints preserved (minus vendor, backup, WebSocket, extra fields, QR); spool/filament/location shapes change due to model redesign
- **NFC tag compatibility**: Filament model aligns with OpenTag3D and OpenPrintTag (identical standards); spool NFC Online Data URL = `/api/v1/spool/<id>`
- **No data migration**: JSON format is unconstrained; no existing deployments with data to preserve
- **Dependencies removed**: SQLAlchemy, Alembic, Pydantic, uvicorn, React, Refine, Ant Design, TanStack Query, react-router, and ~30 other npm packages
- **New dependencies**: Axum, Leptos, Tokio, Serde, cargo-leptos toolchain
