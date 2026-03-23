# TODO

Items to address. Move completed items to [CHANGELOG.md](CHANGELOG.md) under the appropriate release.

## Pending

### Enhancements
- [ ] Support `.env` files for local development (`dotenvy` crate, load at startup in `spoolman-server/src/main.rs`)
- [ ] NFC / QR sticker integration — [OpenSpoolMan](https://github.com/drndos/openspoolman) or [OpenTag3D](https://opentag3d.com/) compatible; spool NFC URL already maps to `/api/v1/spool/<id>`
- [ ] Make the Spool list the default landing page
- [ ] Color search on spool list (filter by RGBA proximity)
- [ ] Add `filament_type` field to Filament (e.g. "PLA", "PETG", "ABS")
- [ ] Light theme matching the logo

### Testing
- [ ] Update integration tests (`tests_integration/`) for the Rust stack (currently written for the Python backend)
