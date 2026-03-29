# TODO

Items to address. Move completed items to [CHANGELOG.md](CHANGELOG.md) under the appropriate release.

## Pending

### Bugs (found via Playwright Docker test, 2026-03-27)
~~1. Settings page: currency symbol renders as `â,¬` instead of `€` — UTF-8 double-encoding in settings page~~
~~2. Spool detail (`/spools/<id>`): assigned Location not displayed — Location field missing from detail view~~
~~3. /api/v1/info is empty~~
~~4. delete button in locations is broken~~ (see #9)
~~5. "sure?" button (after delete) has no effect~~
~~7. HTTP 404: Not Found on save after edit~~
~~8. save in edit location broken~~

### Bugs (found via Playwright E2E test run, 2026-03-28)
~~9. **E2E: location delete UI doesn't update after confirm** — fixed: `deleteLocation` helper now waits for the deleted row to become detached instead of `waitForLoadState('networkidle')`.~~
~~10. **E2E: spool/filament add|delete tests always see 25 rows** — Fixed: rewritten to use `X-Total-Count` response header via `getSpoolCount()`/`getFilamentCount()` instead of DOM row count.~~

### Infrastructure fixes (2026-03-28)
- ~~`docker-compose.test.yml` `command:` string was split by Docker Compose into individual words when combined with `entrypoint: ["/bin/sh", "-c"]`, causing `cp: missing file operand`. Fixed by moving the full command into the entrypoint array.~~

### Enhancements
- [ ] NFC / QR sticker integration — [OpenSpoolMan](https://github.com/drndos/openspoolman) or [OpenTag3D](https://opentag3d.com/) compatible; spool NFC URL already maps to `/api/v1/spool/<id>`
- [ ] use locale to format date and time. fall back to what is configured in settings. add a setting for date / time format
- ~~"clear search" button~~ (done: × button clears the text search input)
- [ ] rename "filter" to "search"
- [ ] table headers contain filter button (partial: Color column header activates color picker filter)
- [ ] move Filament.net_weight to spool.net_weight
- [ ] add delete buttons wherever edit buttons are
- ~~remove "remaining %"~~
- [ ] location must not be empty or "none"
- [ ] don't care about time. remove from display. if it must be set then set it to 5 min past midnight
