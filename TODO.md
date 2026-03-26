# TODO

Items to address. Move completed items to [CHANGELOG.md](CHANGELOG.md) under the appropriate release.

## Pending

### Bugs (found via Playwright Docker test, 2026-03-24)

### B7 Color's alpha value is not used anywhere
- the color picker should allow to select an alpha value. if that's not possible add an extra selector elsewhere

### B14 no date in Spool view
- does not show date information (creation, last use)
- edit of date is not possible

#### ~~B15 delete buttons broken~~ (Fixed)
- **Root cause:** `refetch()` unreliable on `create_resource(|| (), ...)` in Leptos 0.6; no confirmation dialog; missing delete buttons on spool/filament list and filament detail; 404 on deleted entity detail pages.
- **Fix applied:** Version counter trigger for reactive refetch; inline two-step "Sure?"/"Cancel" confirm on all delete buttons; delete added to spool list, filament list, filament detail; 404 redirects to list in SpoolShow and FilamentShow.

### Enhancements
- [ ] NFC / QR sticker integration — [OpenSpoolMan](https://github.com/drndos/openspoolman) or [OpenTag3D](https://opentag3d.com/) compatible; spool NFC URL already maps to `/api/v1/spool/<id>`
