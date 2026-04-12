# TODO

Items to address. Move completed items to [CHANGELOG.md](CHANGELOG.md) under the appropriate release.

### Enhancements
- [ ] NFC / QR sticker integration — [OpenSpoolMan](https://github.com/drndos/openspoolman) or [OpenTag3D](https://opentag3d.com/) compatible; spool NFC URL already maps to `/api/v1/spool/<id>`
- [ ] extend search to location
- [ ] filament create/edit + spool create: SpoolmanDB lookup — fetch https://donkie.github.io/SpoolmanDB/filaments.json, cache in localStorage (24h TTL + ETag), client-side search, auto-fill filament fields; in spool create auto-create missing filament and notify user
- [ ] filament/spool: filamentcolors.xyz color lookup — deferred: API CORS headers absent from their Django app, direct WASM fetch will be blocked; needs a server-side proxy endpoint (/api/v1/proxy/filamentcolors) before this is viable
- [ ] test on mobile