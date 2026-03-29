# TODO

Items to address. Move completed items to [CHANGELOG.md](CHANGELOG.md) under the appropriate release.

## Pending

### Bugs (found via Playwright Docker test, 2026-03-27)
~~1. Settings page: currency symbol renders as `â,¬` instead of `€` — UTF-8 double-encoding in settings page~~
2. Spool detail (`/spools/<id>`): assigned Location not displayed — Location field missing from detail view
~~3. /api/v1/info is empty~~
4. delete button in locations is broken
~~5. "sure?" button (after delete) has no effect~~
7. HTTP 404: Not Found on save after edit
8. save in edit location broken

### Bugs (found via Playwright E2E test run, 2026-03-28)
9. **E2E: location delete UI doesn't update after confirm** — Playwright `deleteLocation` test: after clicking "Sure?", `waitForLoadState('networkidle')` resolves before the Leptos reactive list refresh propagates to the DOM. Server-side deletion succeeds (row count confirmed via API), but `this.rows.count()` still returns the pre-delete count. Error: expected 16, received 17. Root cause: `waitForLoadState('networkidle')` returns immediately when the network was already idle before `spawn_local` starts the DELETE fetch. Fix: wait for the deleted row to become detached, or ensure the version signal is incremented synchronously before the async fetch.
10. **E2E: spool/filament add|delete tests always see 25 rows** — Fixture has 200 spools and 40 filaments; default page size is 25. Adding a new spool/filament does not change the visible row count on page 1 (25 items before and after). Deleting from page 1 pulls in the next item, keeping count at 25. Tests `add spool appears in list` / `delete spool removes it from list` / `add filament appears in list` / `delete filament removes it from list` all fail with expected ±1. Fix: either reduce fixture to < 25 items per entity, or rewrite tests to check total count via `X-Total-Count` header instead of DOM row count.

### Infrastructure fixes (2026-03-28)
- ~~`docker-compose.test.yml` `command:` string was split by Docker Compose into individual words when combined with `entrypoint: ["/bin/sh", "-c"]`, causing `cp: missing file operand`. Fixed by moving the full command into the entrypoint array.~~

### Enhancements
- [ ] NFC / QR sticker integration — [OpenSpoolMan](https://github.com/drndos/openspoolman) or [OpenTag3D](https://opentag3d.com/) compatible; spool NFC URL already maps to `/api/v1/spool/<id>`
- [ ] use locale to format date and time. fall back to what is configured in settings. add a setting for date / time format
- [ ] "clear search" button
- [ ] rename "filter" to "search"
- [ ] table headers contain filter button
- [ ] move Filament.net_weight to spool.net_weight
- [ ] add delete buttons wherever edit buttons are
- [ ] remove "remaining %"
- [ ] location must not be empty or "none"
- [ ] don't care about time. remove from display. if it must be set then set it to 5 min past midnight
