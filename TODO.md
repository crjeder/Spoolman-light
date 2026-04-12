# TODO

Items to address. Move completed items to [CHANGELOG.md](CHANGELOG.md) under the appropriate release.

### Enhancements
- [ ] NFC / QR sticker integration — [OpenSpoolMan](https://github.com/drndos/openspoolman) or [OpenTag3D](https://opentag3d.com/) compatible; spool NFC URL already maps to `/api/v1/spool/<id>`
- [x] use locale to format date and time. fall back to what is configured in settings. add a setting for date / time format — implementation complete on `feat/locale-datetime-format` (PR #49); pending manual UI verification (Docker build + browser smoke tests)
- [x] rename "filter" to "search"
- [x] table headers contain filter button (partial: Color column header activates color picker filter)
- [x] Manual verify `remove-time-display`: detail panel shows date-only and form retains `YYYY-MM-DD` semantics
- [x] Manual verify `format-currency-date-numbers-intl`: locale formatting is active for dates/weights/density in browser rendering
- [x] move Filament.net_weight to spool.net_weight
- [ ] add delete buttons wherever edit buttons are
- ~~remove "remaining %"~~
- [x] location must not be empty or "none"
- [x] don't care about time. remove from display. if it must be set then set it to 5 min past midnight
- [x] the sensitivity slider in color search is not intuitive. replace it by a selector for distinct values e. g. off = no color match (default), fine, medium and coarse
- [x] remove the color button from above the table
- [x] place a little square unicode U+25A0 in the current color if color search is not "off"
- [ ] extend search to location
- [x] place a filter icon in location table head. user can select a location from drop down. filter table to show only entries which match the selected location
- [x] color search for multi color is not intuitive
- [x] pop-up color selector would be better than the selector on top of the page. color should not change when changing the threshold
- [x] add the hex value to the color display in spool details
- [x] sort spools according to delta when a color is selected
- [x] implement alternative color distance calculation oklab (or din99d). make them configurable in settings (oklab crate)#
- [x] add material column in spools. table head links to a filter (drop down) to select materials to display
- [x] make the threshold values configurable per calculation algorithm (in settings)
- [x] upgrade crate versions
- [x] spool price: implementation complete on `feat/spool-price-field` (data model, API, forms, Price/g table column); pending manual UI verification (6.3, 6.4)
- [ ] account for transparent and mate / glossy finishes in color search
- [x] remove the spool id from table.
- [x] replace the edit and delete buttons text with icons. add a view button
- [x] in spool details: link the filament name to the corresponding filament
- [ ] filament and or spool edit: add https://filamentcolors.xyz/ and / or spoolmandb  search
- [x] use of deprecated function `leptos::prelude::create_effect`: This function is being removed to conform to Rust idioms. Please use `Effect::new()` instead.
- [x] replace buttons with icons everywhere
- [x] display spool price and price per kg in spool details 
- [x] change the unit to price per kg in spool table
- [x] filter on location
- [x] default values for din99 13; 19; 25
- [x] € defect
- [x] € after number in price
- [ ] test on mobile