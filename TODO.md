# TODO

Items to address. Move completed items to [CHANGELOG.md](CHANGELOG.md) under the appropriate release.

## Pending
- [x] B20: color square (U+25A0) does not show the currently selected color. 
- [ ] B21 /api/v1/info is empty

### OpenSpec Implementation Discrepancies

Items where the actual code does NOT match the OpenSpec specification:

**Summary: 4 active OpenSpec changes checked**
- ✓ 3 fully implemented (`color-hex-display`, `sort-spools-by-color-delta`, `alternative-color-distance`)
- ⚠️ 1 design mismatch (`add-filament-type` — spec says datalist/free-text, code uses enum/select)
- ✓ 1 bonus implemented (`validate-location-required-spool` — not yet in spec, already in code)

**Details:**

1. **[color-hex-display]** ✓ IMPLEMENTED — hex formatted as `#{:02x}{:02x}{:02x}` in `spool.rs`, CSS styling in `spoolman.css`.

2. **[alternative-color-distance]** ✓ IMPLEMENTED — `ColorAlgorithm` enum (Ciede2000/OkLab/Din99d), `color_distance(a, b, algo)` dispatcher, per-algorithm thresholds via `threshold_for()`, OKLab via `oklab` crate, DIN99d via rotation-matrix formula. Settings page exposes algorithm selector; choice persisted via `put_setting` and provided globally via `ColorDistanceAlgorithm` Leptos context. Filter and sort both use the reactive context signal. Change archived to `openspec/changes/archive/2026-04-02-alternative-color-distance/`.

3. **[add-filament-type]** ✓ IMPLEMENTED — `MaterialType` enum in `spoolman-types`; `<select>` on filament create/edit driven by `MaterialType::all_known()`; material filter on filament list; spool text filter matches `abbreviation()`. Design doc explicitly chose `<select>` over `<datalist>` to enforce a closed vocabulary.

4. **[sort-spools-by-color-delta]** ✓ IMPLEMENTED — `sorted()` closure in `spool.rs` computes `min_delta()` for each spool and sorts ascending by delta when `color_level != "off"`. Uses the selected algorithm from context.

5. **[validate-location-required-spool]** ✓ BONUS IMPLEMENTATION — Not yet a formal spec, but frontend validation already exists in `SpoolCreate` and `SpoolEdit`. **ACTION: Consider archiving or formalizing as spec if feature is stable.**

### Enhancements
- [ ] NFC / QR sticker integration — [OpenSpoolMan](https://github.com/drndos/openspoolman) or [OpenTag3D](https://opentag3d.com/) compatible; spool NFC URL already maps to `/api/v1/spool/<id>`
- [~] use locale to format date and time. fall back to what is configured in settings. add a setting for date / time format — implementation complete on `feat/locale-datetime-format` (PR #49); pending manual UI verification (Docker build + browser smoke tests)
- [x] rename "filter" to "search"
- [ ] table headers contain filter button (partial: Color column header activates color picker filter)
- [x] Manual verify `remove-time-display`: detail panel shows date-only and form retains `YYYY-MM-DD` semantics
- [ ] Manual verify `format-currency-date-numbers-intl`: locale formatting is active for dates/weights/density in browser rendering
- [x] move Filament.net_weight to spool.net_weight
- [ ] add delete buttons wherever edit buttons are
- ~~remove "remaining %"~~
- [x] location must not be empty or "none"
- [x] don't care about time. remove from display. if it must be set then set it to 5 min past midnight
- [x] the sensitivity slider in color search is not intuitive. replace it by a selector for distinct values e. g. off = no color match (default), fine, medium and coarse
- [x] remove the color button from above the table
- [x] place a little square unicode U+25A0 in the current color if color search is not "off"
- [ ] extend search to location
- [ ] place a filter icon in location table head. user can select a location from drop down. filter table to show only entries which match the selected location
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
- [ ] use of deprecated function `leptos::prelude::create_effect`: This function is being removed to conform to Rust idioms. Please use `Effect::new()` instead.
- [ ] replace buttons with icons everywhere
- [ ] display spool price and price per kg in spool details 
- [ ] change the unit to price per kg in spool table
- [ ] filter on location
- [ ] default values for din99 13; 19; 25
- [ ] € defect
- [ ] € after number in price