# Spoolman

Self-hosted web service for tracking 3D printing filament spools. Python (FastAPI) backend + React/TypeScript frontend. **A full Rust rewrite (Axum + Leptos WASM) is in progress** — see `crates/` and `openspec/changes/migrate-to-rust/`.

## Commands

### Backend
```bash
# Install deps (use pdm or uv)
pdm install

# Run dev server (default: http://localhost:8000)
pdm run app
# or directly:
uvicorn spoolman.main:app --reload

# Lint
ruff check .

# Format
black .

# Unit tests (none currently — integration tests only)
```

### Frontend
```bash
cd client

npm install

npm run dev        # Dev server (Vite, proxies API to localhost:8000)
npm run build      # TypeScript check + production build → client/dist/
npm run check-i18n # Verify translation keys are consistent
```

### Rust (new stack)
```bash
# Type-check entire workspace (no build needed)
cargo check -p spoolman-types
cargo check -p spoolman-server
# cargo check -p spoolman-client  # requires wasm32 target

# Full release build (use WSL/Linux/Docker — blocked on Windows by OpenSSL)
# cargo leptos build --release
```

### Integration Tests (Docker required)
```bash
# Only sqlite target has a compose file
python tests_integration/run.py sqlite
```

## Rust Workspace Layout

```
crates/
  spoolman-types/   # Shared types: Spool, Filament, Location, DataStore, requests, responses
  spoolman-server/  # Axum backend — routes, JsonStore (Arc<RwLock>), config, backup stub
  spoolman-client/  # Leptos WASM frontend — pages, components, API wrappers, table state
Cargo.toml          # Workspace root
Leptos.toml         # cargo-leptos build config
```

## Architecture (Python stack — being replaced)

```
spoolman/           # Python backend (FastAPI, no ORM)
  api/v1/           # FastAPI route handlers
  storage/          # JSON file storage (JsonStore, models)
  main.py           # App entry point, FastAPI app setup
  env.py            # All environment variable parsing
  settings.py       # Runtime settings
  ws.py             # WebSocket support

client/             # React 19 + TypeScript frontend (Vite + Refine)
  src/
    pages/          # Route-level components
    components/     # Shared UI components
    utils/          # Helpers
  public/           # Static assets

tests_integration/  # Docker-based integration tests (pytest)
```

## Key Environment Variables

| Variable | Default | Purpose |
|----------|---------|---------|
| `SPOOLMAN_DATA_FILE` | `<data_dir>/spoolman.json` | Path to JSON data file |
| `SPOOLMAN_DIR_DATA` | platform default | Data directory |
| `SPOOLMAN_DIR_LOGS` | platform default | Logs directory |
| `SPOOLMAN_DIR_BACKUPS` | platform default | Backups directory |
| `SPOOLMAN_CORS_ORIGIN` | FALSE | CORS origin (set to frontend URL if needed) |
| `SPOOLMAN_DEBUG_MODE` | FALSE | Enable debug mode |
| `SPOOLMAN_LOGGING_LEVEL` | INFO | Log level |
| `SPOOLMAN_BASE_PATH` | "" | URL base path prefix |
| `SPOOLMAN_HOST` | 0.0.0.0 | Bind host (Docker entrypoint) |
| `SPOOLMAN_PORT` | 8000 | Bind port (Docker entrypoint) |
| `SPOOLMAN_AUTOMATIC_BACKUP` | TRUE | Auto DB backup |

## Stack Details

- **Backend:** Python 3.9–3.12, FastAPI 0.115, JSON file storage (no ORM), Pydantic v2, uvicorn
- **Frontend:** React 19, TypeScript, Vite 7, Refine framework, Ant Design, TanStack Query, react-router 7, i18next, Zustand
- **Package managers:** `pdm` or `uv` (Python), `npm` (frontend, Node 20.x required)

## Workflow

After every change, update [CHANGELOG.md](CHANGELOG.md):
- Put entries under a new version 
- Follow [Keep a Changelog](https://keepachangelog.com/en/1.1.0/) format — write for humans, not diffs
- Use [TODO.md](TODO.md) to track pending work
- Never push to the upstream repository unless specifically instructed

## Gotchas

- **Rust rewrite in progress** — do not add features to `spoolman/` (Python) or `client/` (React); those will be removed after the Rust stack is verified.
- **cargo-leptos blocked on Windows** — `cargo leptos build` fails because `openssl-sys` needs OpenSSL dev headers. Build in WSL, Linux, or Docker (`docker build .`).
- **Semgrep path-traversal false positive** — the "Path Traversal with Actix" rule fires on any `std::fs` op whose path originates from a function parameter, even after `canonicalize()`. `// nosemgrep` and `.semgrepignore` are ignored by the MCP hook (`semgrep mcp -k post-tool-cli-scan`). Scope suppressions carefully; don't restructure valid path code to avoid them.
- **Do not add `leptos` to `spoolman-server/Cargo.toml`** — Leptos is a client-only dependency. The server crate must not depend on it.
- **Frontend framework is Refine** — data fetching, CRUD, and routing follow Refine conventions, not plain React patterns.
- **No unit tests** — only Docker-based integration tests exist. Running `pdm run itest` builds Docker images first.
- **JSON file storage** — data stored in `spoolman.json` in platform user-data dir; no DB env vars needed.
- **JsonStore uses threading.RLock** — concurrent writes are serialized; don't bypass the store's flush mechanism.
- **uvloop disabled on Windows/armv7l** — don't assume uvloop is available in cross-platform code.
- **Shell scripts must use LF line endings** — `entrypoint.sh` and other scripts must be LF (not CRLF) or Docker containers fail on Linux. Verify when editing on Windows.
- **i18n required for UI strings** — all user-visible frontend text must go through i18next (`t()` calls); run `npm run check-i18n` to verify.
