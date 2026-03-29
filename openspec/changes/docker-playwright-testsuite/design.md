## Context

The Rust rewrite has a working `Dockerfile` (multi-stage: `cargo leptos build --release` → Debian slim runtime). There is no test harness. The CLAUDE.md mandates Docker for running the code and Playwright for front-end tests. `assets/spoolman.json` already contains representative test fixture data.

## Goals / Non-Goals

**Goals:**
- A `docker-compose.test.yml` that builds the app image and mounts fixture data for a clean test run
- A Playwright TypeScript project under `tests/e2e/` with tests for core CRUD flows (spools, filaments, locations) and navigation
- A `scripts/run-e2e.sh` entry-point: build → start container → wait for health → run Playwright → teardown
- Tests runnable on Linux/macOS and via Docker Desktop / WSL on Windows

**Non-Goals:**
- Unit tests or Rust `#[test]` coverage (separate concern)
- CI pipeline integration (GitHub Actions wiring not in scope here)
- Visual regression / screenshot diffing
- Authentication testing (app has no auth)

## Decisions

### D1 — Playwright TypeScript over Python/JS
Playwright's TypeScript bindings are the most complete and best-documented. The project has no existing Node toolchain to conflict with, and TS gives type-safe page-object models. **Alternative:** `playwright-pytest` (Python) was considered but adds a Python venv alongside Rust, and the MCP Playwright tool used in development is also Node-based.

### D2 — Standalone `tests/e2e/package.json` (not a monorepo workspace)
Keeps test dependencies fully isolated from any future JS tooling in the main project. A simple `npm install` in `tests/e2e/` is all that's needed. **Alternative:** root-level package.json workspace — rejected because the Rust project has no existing npm root and adding one creates confusion.

### D3 — `docker-compose.test.yml` mounts fixture JSON via bind-mount, not volume copy
`assets/spoolman.json` is bind-mounted into `/data/spoolman.json` read-only, then the container writes to a tmpfs or named volume overlay. This keeps fixture data in source control and avoids polluting the development `spoolman_data` volume. **Alternative:** `COPY` fixture into the image at build time — rejected because it couples test data to the image layer.

### D4 — Health-check via HTTP poll in `run-e2e.sh`
`scripts/run-e2e.sh` polls `http://localhost:8000/api/v1/info` with a retry loop (max 60 s) before launching Playwright. This is simpler than a Docker health-check dependency and works without additional tools. **Alternative:** `docker compose wait` / `depends_on: condition: service_healthy` — viable but requires the Dockerfile to declare a `HEALTHCHECK`, which adds image complexity.

### D5 — Page-Object Model (POM) pattern for Playwright tests
Each major page (SpoolsPage, FilamentsPage, LocationsPage) gets a lightweight POM class. This keeps test logic readable and makes locator changes easy to maintain. Tests follow: navigate → assert initial state → perform action → assert result.

## Risks / Trade-offs

- **Slow CI build** — `cargo leptos build --release` takes several minutes in Docker. Mitigation: use Docker layer caching (`--cache-from`) in CI; the Dockerfile's `COPY . .` is already the last expensive step.
- **Flaky tests on slow machines** — Playwright's default timeout may be too short if the WASM bundle is large. Mitigation: set `timeout: 30_000` globally in `playwright.config.ts` and use `waitForLoadState('networkidle')` after navigation.
- **Fixture data drift** — If `assets/spoolman.json` is modified, tests that assert specific counts or names break. Mitigation: tests assert structural properties (e.g., "at least one spool row") rather than exact fixture values where possible; brittle assertions are explicitly documented.
- **Windows build blocked** — `cargo leptos build` fails on Windows due to `openssl-sys`. Mitigation: `run-e2e.sh` must be run in WSL or Docker Desktop Dev Containers; this is already a known project constraint documented in CLAUDE.md.

## Open Questions

- Should `tests/e2e/` include a `Dockerfile` for running Playwright in a container (useful for headless CI without installing Node locally)? Likely yes but deferred to tasks.
- Exact fixture assertions: decide during implementation whether to pin specific IDs from `assets/spoolman.json` or keep tests fixture-agnostic.
