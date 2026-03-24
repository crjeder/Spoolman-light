# TODO

Items to address. Move completed items to [CHANGELOG.md](CHANGELOG.md) under the appropriate release.

## Pending

### In Progress
- [ ] Complete Rust rewrite (`openspec/changes/migrate-to-rust/`) — code written, pending build verification
  - [ ] Install `cargo-leptos` on Windows (blocked: OpenSSL dev headers missing — build in WSL/Linux/Docker instead)
  - [ ] `cargo leptos build --release` first successful build
  - [ ] Verify single binary serves API + WASM frontend (task 12.2)
  - [ ] Update `docker-compose.yml` for new binary entrypoint (task 12.3)
  - [ ] Verify `SPOOLMAN_DATA_FILE` env var mounts correctly in container (task 12.4)
- [ ] Legacy export converter (`openspec/changes/legacy-export-converter/`) — script to convert old Spoolman JSON exports to new format
  - [ ] Create `scripts/convert_export.py` CLI (tasks 1–6)

### Enhancements
- [ ] NFC / QR sticker integration — [OpenSpoolMan](https://github.com/drndos/openspoolman) or [OpenTag3D](https://opentag3d.com/) compatible; spool NFC URL already maps to `/api/v1/spool/<id>`
- [ ] Make the Spool list the default landing page
- [ ] Color search on spool list (filter by RGBA proximity)
- [ ] Light theme matching the logo
