## Why

The Rust rewrite has no automated test coverage for the full application stack. Without a test suite, regressions in the API or UI are caught only by manual inspection. A Docker + Playwright setup enables reproducible, CI-friendly end-to-end tests that validate both the Axum backend and the Leptos frontend together.

## What Changes

- Add a `Dockerfile` that builds the full release image (server + WASM assets) via `cargo leptos build --release`
- Add a `docker-compose.test.yml` that launches the app container with test fixture data
- Add a Playwright test project (TypeScript) under `tests/e2e/` with test cases covering core user journeys
- Add an npm workspace (or standalone `package.json`) in `tests/e2e/` for Playwright dependencies
- Add a shell script `scripts/run-e2e.sh` that starts Docker, waits for the server, runs Playwright, and tears down

## Capabilities

### New Capabilities

- `e2e-test-harness`: Docker-based test harness that builds and runs the app in a container with seed data, serving as the target for Playwright tests
- `playwright-tests`: Playwright test suite covering spool CRUD, filament CRUD, location CRUD, and basic UI navigation

### Modified Capabilities

<!-- None — this change adds new infrastructure only, no existing spec requirements change -->

## Impact

- New dev dependency: Node.js / npm (Playwright only needed in test environments)
- `Dockerfile` builds via multi-stage: Rust/cargo-leptos in builder, minimal runtime image
- `assets/spoolman.json` used as seed data for tests (already exists)
- No changes to production code paths
- CI can run `scripts/run-e2e.sh` on Linux/macOS; Windows users run via WSL or Docker Desktop
