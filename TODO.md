# TODO

Items to address. Move completed items to [CHANGELOG.md](CHANGELOG.md) under the appropriate release.

## Pending
- [ ] B20: color search broken. link in color header does not do anything

### Clippy Warnings
- [x] `crates/spoolman-server/src/routes/filament.rs:87` — useless `format!()`, replace with `.to_string()`
- [x] `crates/spoolman-server/src/store.rs:287` — `list_spools` has 8 args (>7); consider a filter struct
- [x] `crates/spoolman-server/src/store.rs:632` — `sort_items` takes `&mut Vec<T>`, should be `&mut [T]`

### Enhancements
- [ ] NFC / QR sticker integration — [OpenSpoolMan](https://github.com/drndos/openspoolman) or [OpenTag3D](https://opentag3d.com/) compatible; spool NFC URL already maps to `/api/v1/spool/<id>`
- [ ] use locale to format date and time. fall back to what is configured in settings. add a setting for date / time format
- [x] rename "filter" to "search"
- [ ] table headers contain filter button (partial: Color column header activates color picker filter)
- [ ] move Filament.net_weight to spool.net_weight
- [ ] add delete buttons wherever edit buttons are
- ~~remove "remaining %"~~
- [ ] location must not be empty or "none"
- [ ] don't care about time. remove from display. if it must be set then set it to 5 min past midnight
- [x] the sensitivity slider in color search is not intuitive. replace it by a selector for distinct values e. g. off = no color match (default), fine, medium and coarse
- [x] remove the color button from above the table
- [x] place a little square unicode U+25A0 in the current color if color search is not "off"
- [ ] extend search to location
- [ ] place a filter icon in location table head. user can select a location from drop down. filter table to show only entries which match the selected location
