# TODO

Items to address. Move completed items to [CHANGELOG.md](CHANGELOG.md) under the appropriate release.

## Pending

### Clippy Warnings
- [x] `crates/spoolman-server/src/routes/filament.rs:87` — useless `format!()`, replace with `.to_string()`
- [x] `crates/spoolman-server/src/store.rs:287` — `list_spools` has 8 args (>7); consider a filter struct
- [x] `crates/spoolman-server/src/store.rs:632` — `sort_items` takes `&mut Vec<T>`, should be `&mut [T]`

### Enhancements
- [ ] NFC / QR sticker integration — [OpenSpoolMan](https://github.com/drndos/openspoolman) or [OpenTag3D](https://opentag3d.com/) compatible; spool NFC URL already maps to `/api/v1/spool/<id>`
- [ ] use locale to format date and time. fall back to what is configured in settings. add a setting for date / time format
- [ ] rename "filter" to "search"
- [ ] table headers contain filter button (partial: Color column header activates color picker filter)
- [ ] move Filament.net_weight to spool.net_weight
- [ ] add delete buttons wherever edit buttons are
- ~~remove "remaining %"~~
- [ ] location must not be empty or "none"
- [ ] don't care about time. remove from display. if it must be set then set it to 5 min past midnight
