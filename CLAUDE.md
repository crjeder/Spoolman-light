# Spoolman

Self-hosted web service for tracking 3D printing filament spools. Built with Rust: Axum backend + Leptos WASM frontend — see `crates/` and `openspec/changes/rust-rewrite/`.

## Commands

### Rust
```bash
# Type-check entire workspace (no build needed)
cargo check -p spoolman-types
cargo check -p spoolman-server
# cargo check -p spoolman-client  # requires wasm32 target

# Lint
cargo clippy -p spoolman-types -p spoolman-server

# Full release build (use WSL/Linux/Docker — blocked on Windows by OpenSSL)
# cargo leptos build --release
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

## Key Environment Variables

| Variable | Default | Purpose |
|----------|---------|---------|
| `SPOOLMAN_DATA_FILE` | `<data_dir>/spoolman.json` | Path to JSON data file |
| `SPOOLMAN_HOST` | 0.0.0.0 | Bind host |
| `SPOOLMAN_PORT` | 8000 | Bind port |
| `LEPTOS_SITE_ROOT` | `target/site` | Path to compiled WASM/static assets |

## Stack Details

- **Stack:** Rust (Axum + Leptos WASM), cargo-leptos build system
- **Package manager:** `cargo`

## Testing

### E2E (Docker + Playwright)

Run the full end-to-end suite (requires Docker and Node.js; use WSL on Windows):

```bash
./scripts/run-e2e.sh
```

This builds the Docker image, starts the container with `assets/spoolman.json` as fixture data, waits for the server, runs Playwright, and tears everything down.

To run Playwright against an already-running container:

```bash
# In one terminal — start the test container:
docker compose -f docker-compose.test.yml up --build

# In another terminal — run tests:
cd tests/e2e
npm ci
npx playwright test

# When done:
docker compose -f docker-compose.test.yml down
```

Test files live in `tests/e2e/tests/`. Page-Object Models are in `tests/e2e/pages/`.

- Test data: `assets/spoolman.json` (fixture, read-only bind-mount)
- The test container does NOT touch the development `spoolman_data` volume

## Workflow

- Use git worktrees for feature work to keep changes isolated from the current workspace. Before starting any non-trivial implementation, create a worktree on a new branch rather than working directly on the checked-out branch. Place worktrees in .worktrees in the project directory
- For every non-trivial implementation check crates.io if there is already a crate implementing the functionality. Use the `crates-mcp` MCP server (tools: `crates_search`, `crates_get`, `crates_get_versions`, `crates_get_dependencies`) — it has direct API access and is more reliable than context7 for Rust crates.
- use openspec to plan changes and new features


when archiving the change, update [CHANGELOG.md](CHANGELOG.md):
- Put entries under a new version
- Follow [Keep a Changelog](https://keepachangelog.com/en/1.1.0/) format — write for humans, not diffs
- Use [TODO.md](TODO.md) to track pending work
- Never push to the upstream repository unless specifically instructed
- update this Claude.md with important learnings

## Gotchas

- **cargo-leptos blocked on Windows** — `cargo leptos build` fails because `openssl-sys` needs OpenSSL dev headers. Build in WSL, Linux, or Docker (`docker build .`).
- **Semgrep path-traversal false positive** — the "Path Traversal with Actix" rule fires on any `std::fs` op whose path originates from a function parameter, even after `canonicalize()`. `// nosemgrep` and `.semgrepignore` are ignored by the MCP hook (`semgrep mcp -k post-tool-cli-scan`). Scope suppressions carefully; don't restructure valid path code to avoid them.
- **Do not add `leptos` to `spoolman-server/Cargo.toml`** — Leptos is a client-only dependency. The server crate must not depend on it.
- **JSON file storage** — data stored in `spoolman.json` in platform user-data dir; no DB env vars needed.
- **NFC tag URL format** — Spool Online Data URL is `<host>/api/v1/spool/<id>` without `https://` (OpenTag3D / OpenPrintTag spec).
- **Random u32 IDs** — Rust data model uses random u32 IDs with collision check on insert (not sequential).
