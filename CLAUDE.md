# Spoolman

Self-hosted web service for tracking 3D printing filament spools. Python (FastAPI) backend being replaced by a full Rust rewrite (Axum + Leptos WASM) — see `crates/` and `openspec/changes/migrate-to-rust/`.

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

## OpenSpec

The Rust rewrite proposal lives at `openspec/changes/rust-rewrite/`. Design is complete — do not re-explore architecture decisions; implement from the spec.

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
- **New stack:** Rust (Axum + Leptos WASM), cargo-leptos build system
- **Package managers:** `pdm` or `uv` (Python), `cargo` (Rust)

## Workflow

Use git worktrees for feature work to keep changes isolated from the current workspace. Before starting any non-trivial implementation, create a worktree on a new branch rather than working directly on the checked-out branch.
For every non-trivial implementation check crates.io if there is already a crate implementing the functionality.

After every change, update [CHANGELOG.md](CHANGELOG.md):
- Put entries under a new version 
- Follow [Keep a Changelog](https://keepachangelog.com/en/1.1.0/) format — write for humans, not diffs
- Use [TODO.md](TODO.md) to track pending work
- Never push to the upstream repository unless specifically instructed

## Gotchas

- **Rust rewrite in progress** — do not add features to `spoolman/` (Python); it will be removed after the Rust stack is verified.
- **cargo-leptos blocked on Windows** — `cargo leptos build` fails because `openssl-sys` needs OpenSSL dev headers. Build in WSL, Linux, or Docker (`docker build .`).
- **Semgrep path-traversal false positive** — the "Path Traversal with Actix" rule fires on any `std::fs` op whose path originates from a function parameter, even after `canonicalize()`. `// nosemgrep` and `.semgrepignore` are ignored by the MCP hook (`semgrep mcp -k post-tool-cli-scan`). Scope suppressions carefully; don't restructure valid path code to avoid them.
- **Do not add `leptos` to `spoolman-server/Cargo.toml`** — Leptos is a client-only dependency. The server crate must not depend on it.
- **No unit tests** — only Docker-based integration tests exist. Running `pdm run itest` builds Docker images first.
- **JSON file storage** — data stored in `spoolman.json` in platform user-data dir; no DB env vars needed.
- **JsonStore uses threading.RLock** — concurrent writes are serialized; don't bypass the store's flush mechanism.
- **uvloop disabled on Windows/armv7l** — don't assume uvloop is available in cross-platform code.
- **Shell scripts must use LF line endings** — `entrypoint.sh` and other scripts must be LF (not CRLF) or Docker containers fail on Linux. Verify when editing on Windows.
- **NFC tag URL format** — Spool Online Data URL is `<host>/api/v1/spool/<id>` without `https://` (OpenTag3D / OpenPrintTag spec).
- **Random u32 IDs** — Rust data model uses random u32 IDs with collision check on insert (not sequential).
