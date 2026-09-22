## 1. Docker Test Harness

- [x] 1.1 Create `docker-compose.test.yml` at repo root: spoolman service using local `Dockerfile`, bind-mount `./assets/spoolman.json` to `/data/spoolman.json`, expose port 8000, no named volume (use tmpfs or anonymous volume for `/data`)
- [ ] 1.2 Verify `docker compose -f docker-compose.test.yml up --build -d` builds and starts the container
- [ ] 1.3 Verify `GET http://localhost:8000/api/v1/info` returns HTTP 200 after startup
- [ ] 1.4 Verify fixture data is visible via `GET /api/v1/spool`, `/api/v1/filament`, `/api/v1/location`
- [ ] 1.5 Verify `docker compose -f docker-compose.test.yml down` removes all test containers and does not touch `spoolman_data` volume

## 2. Run Script

- [x] 2.1 Create `scripts/run-e2e.sh` — build image, start container, poll `http://localhost:8000/api/v1/info` up to 60 s
- [x] 2.2 Add Playwright test invocation step to `run-e2e.sh` (runs `npm ci && npx playwright test` in `tests/e2e/`)
- [x] 2.3 Add teardown step to `run-e2e.sh` (`docker compose … down`) in a `trap` so it runs on success and failure
- [x] 2.4 Ensure `run-e2e.sh` exits non-zero when Playwright exits non-zero
- [x] 2.5 Make `scripts/run-e2e.sh` executable (`chmod +x`)

## 3. Playwright Project Setup

- [x] 3.1 Create `tests/e2e/package.json` with `@playwright/test` dev dependency (pin a specific version)
- [x] 3.2 Create `tests/e2e/playwright.config.ts`: base URL `http://localhost:8000`, global timeout 30 000 ms, one project (Chromium headless), reporter `list`
- [x] 3.3 Create `tests/e2e/tsconfig.json` extending `@playwright/test` defaults
- [x] 3.4 Add `tests/e2e/node_modules/` and `tests/e2e/test-results/` to `.gitignore`
- [x] 3.5 Run `npm install` in `tests/e2e/` and commit `package-lock.json`

## 4. Page-Object Models

- [x] 4.1 Create `tests/e2e/pages/SpoolsPage.ts` — `goto()`, `waitForReady()`, `addSpool(data)`, `deleteSpool(name)`, `getSpoolRows()`
- [x] 4.2 Create `tests/e2e/pages/FilamentsPage.ts` — `goto()`, `waitForReady()`, `addFilament(data)`, `deleteFilament(name)`, `getFilamentRows()`
- [x] 4.3 Create `tests/e2e/pages/LocationsPage.ts` — `goto()`, `waitForReady()`, `addLocation(name)`, `deleteLocation(name)`, `getLocationRows()`
- [x] 4.4 Create `tests/e2e/pages/index.ts` re-exporting all three POMs

## 5. Test Files

- [x] 5.1 Create `tests/e2e/tests/navigation.spec.ts` — assert Spools, Filaments, Locations pages each load without errors
- [x] 5.2 Create `tests/e2e/tests/spools.spec.ts` — fixture data visible; add spool appears in list; delete spool removes it
- [x] 5.3 Create `tests/e2e/tests/filaments.spec.ts` — fixture data visible; add filament appears in list; delete filament removes it
- [x] 5.4 Create `tests/e2e/tests/locations.spec.ts` — fixture data visible; add location appears in list; delete location removes it

## 6. Verification

- [ ] 6.1 Run `scripts/run-e2e.sh` end-to-end and confirm exit code 0 with all tests green
- [ ] 6.2 Intentionally break one test and confirm `run-e2e.sh` exits non-zero
- [ ] 6.3 Confirm `spoolman_data` dev volume is untouched after the test run
- [x] 6.4 Update `CLAUDE.md` testing section with instructions for running the e2e suite
