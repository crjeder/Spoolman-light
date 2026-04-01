# TODO

Items to address. Move completed items to [CHANGELOG.md](CHANGELOG.md) under the appropriate release.

## Pending
- [x] B20: color square (U+25A0) does not show the currently selected color. 
- [ ] B21 /api/v1/info is empty

### Enhancements
- [ ] NFC / QR sticker integration — [OpenSpoolMan](https://github.com/drndos/openspoolman) or [OpenTag3D](https://opentag3d.com/) compatible; spool NFC URL already maps to `/api/v1/spool/<id>`
- [ ] use locale to format date and time. fall back to what is configured in settings. add a setting for date / time format
- [x] rename "filter" to "search"
- [ ] table headers contain filter button (partial: Color column header activates color picker filter)
- [ ] move Filament.net_weight to spool.net_weight
- [ ] add delete buttons wherever edit buttons are
- ~~remove "remaining %"~~
- [x] location must not be empty or "none"
- [x] don't care about time. remove from display. if it must be set then set it to 5 min past midnight
- [x] the sensitivity slider in color search is not intuitive. replace it by a selector for distinct values e. g. off = no color match (default), fine, medium and coarse
- [x] remove the color button from above the table
- [x] place a little square unicode U+25A0 in the current color if color search is not "off"
- [ ] extend search to location
- [ ] place a filter icon in location table head. user can select a location from drop down. filter table to show only entries which match the selected location
- [ ] color search for multi color is not intuitive
- [x] pop-up color selector would be better than the selector on top of the page. color should not change when changing the threshold
- [ ] add the hex value to the color display in spool details
- [x] sort spools according to delta when a color is selected
- [ ] implement alternative color distance calculation oklab (or din99d). make them configurable in settings (oklab crate)#
- [ ] handling of alpha value in color search needs to be better
- [ ] take surface finish into account for color search
- [ ] add material column in spools. table head links to a filter (drop down) to select materials to display

