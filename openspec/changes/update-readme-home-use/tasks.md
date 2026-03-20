## 1. Rewrite Intro and Positioning

- [ ] 1.1 Replace the opening tagline and first paragraph to describe the fork as a lightweight, home-use filament tracker for one or two printers and tens of spools
- [ ] 1.2 Remove the OctoPrint/Moonraker integration mention from the intro paragraph

## 2. Update Features List

- [ ] 2.1 Rewrite the Features section to only list what the codebase currently supports: JSON file storage, REST API, web client (view/create/edit/delete filaments and spools)
- [ ] 2.2 Remove Prometheus monitoring, WebSocket real-time updates, SpoolmanDB community database, Weblate translations, QR-code label printing, and multi-printer simultaneous management from the feature list
- [ ] 2.3 Remove vendor management from the feature list
- [ ] 2.4 Update the storage bullet to accurately describe `SPOOLMAN_DATA_FILE` and the single JSON file

## 3. Update or Remove Integration Section

- [ ] 3.1 Remove the "Spoolman integrates with" bulleted list (Moonraker, OctoPrint, OctoEverywhere, Homeassistant)
- [ ] 3.2 Add a single sentence noting that any Spoolman-compatible REST API client may connect

## 4. Update Environment Variable Table

- [ ] 4.1 Remove all `SPOOLMAN_DB_*` rows from the env var table
- [ ] 4.2 Add `SPOOLMAN_DATA_FILE` row with its default path and purpose
- [ ] 4.3 Verify remaining rows (`SPOOLMAN_HOST`, `SPOOLMAN_PORT`, `SPOOLMAN_DIR_*`, `SPOOLMAN_CORS_ORIGIN`, `SPOOLMAN_DEBUG_MODE`, `SPOOLMAN_LOGGING_LEVEL`, `SPOOLMAN_BASE_PATH`, `SPOOLMAN_AUTOMATIC_BACKUP`) are still accurate

## 5. Remove Stale Sections

- [ ] 5.1 Remove the Installation wiki link (or replace with brief local dev instructions reference)
- [ ] 5.2 Remove the Prometheus/monitoring references if present anywhere else in the README
