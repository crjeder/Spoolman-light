## Context

`SpoolList` in `crates/spoolman-client/src/pages/spool.rs` already owns everything a swatch board needs: a `LocalResource` of `SpoolResponse`, a `filtered()` closure over the text / material / location / colour-distance filters, a `sorted()` closure, and `page_items()` feeding `<Pagination>`. `SpoolResponse` carries `spool.colors: Vec<Rgba>`, `spool.color_name`, `filament.display_name()`, `material`, `remaining_filament`, `location_id` and `archived` — the full contents of a card.

Settings are a server-global key-value store (`GET /api/v1/setting`, `PUT /api/v1/setting/{key}`) loaded once by `App` into context signals; `date_format` is the closest template. Table sort/page live in `localStorage` under `table.<namespace>.*`; filters live in `sessionStorage` under `filter.spools.*`.

## Goals / Non-Goals

**Goals:**
- A colour-first browse view that reuses the existing data, filter and pagination path verbatim.
- One server setting that decides the landing view, so a fresh browser lands where the user wants.
- An ordering that makes the grid read as a palette rather than as a shuffled table.

**Non-Goals:**
- Carousel / swipe view — a grid is better on desktop and adequate on mobile.
- Swatch view for filaments — filaments have no colours in this data model; aggregating their spools' colours is a different feature.
- Pin favourites / palette export — parked in `TODO.md`.
- Consuming or embedding the upstream app, or adding CORS configuration.
- Per-browser view preference. The setting is server-global, matching every other setting.

## Decisions

**Two routes, not a mode toggle.**
`/spools` renders the table, `/colors` renders the grid. The sidebar entry is then an ordinary `<A href="/colors">` like every other entry, the view is bookmarkable and shareable, and `default_view` has exactly one job: what `/` resolves to. A mode signal on a single route would need its own storage layer and would give the two views the same URL.

**One component, two render branches.**
`SpoolList` takes a `mode: ViewMode` prop and both routes point at it. Resources, filters, sort and pagination stay in one place and cannot drift between views. Only the render block branches. Extracting a shared `use_spool_list_state()` would be more code for the same result. The card markup itself lives in `components/swatch_grid.rs` so `spool.rs` does not grow much.

**Root view resolution without a redirect.**
`default_view` arrives asynchronously from the settings resource, so `/` cannot be mapped statically. The `/` route renders a `RootView` component reading `DefaultView(RwSignal<Option<ViewMode>>)` from context: while the signal is `None` it renders the existing `"Loading…"` placeholder, then it renders the chosen view. This preserves the existing `spool-management` guarantee that `/` renders a list directly rather than redirecting, and avoids rendering the table for one frame before swapping to the grid.

**Filters shared, ordering per-view.**
The filter signals and their `filter.spools.*` session keys are shared: switching views keeps the set you are looking at, which is the point of having two views. Sort and pagination are per-view — `use_table_state("spools")` for the table, `use_table_state("colors")` for the grid — so the grid defaulting to `hue` does not silently re-sort the table, and each view remembers its own page. This requires a `default_sort` parameter on `use_table_state` (existing call sites pass `"registered"`).

**Hue ordering: chromatic by hue then lightness, achromatic last.**
Hue is unstable at low chroma, so sorting naively scatters greys, black and white through every hue band. Colours are converted with the OkLab conversion already in `utils/color.rs`; `hue = atan2(b, a)` normalised to `0..2π` starting at red, `chroma = hypot(a, b)`, `lightness = L`. Spools with `chroma < 0.02` form an achromatic block sorted by ascending lightness and placed after all chromatic spools. Chromatic spools sort by ascending hue, then ascending lightness. `sort_asc = false` reverses the whole ordering, greys included. A multi-colour spool sorts by its first colour — that is the colour the user named it by. A spool with no colours uses the same `#c8c8c8` fallback the table already paints, so it lands in the achromatic block.

The threshold is a hardcoded constant, not a setting. There are already nine colour-threshold settings; a tenth knob for "how grey is grey" earns its place only if `0.02` proves wrong in use.

**Colour-delta sort still wins.**
`hue` is an ordinary column sort field, so the existing `color-delta-sort` precedence is unchanged: when a colour level is active with a valid hex, both views sort by ascending ΔE from the picked colour and ignore the sort field. Setting the level to Off returns the grid to hue order.

**Multi-colour card fill: hard-stop linear gradient.**
One colour is a flat fill; two to four colours are equal-width bands with hard stops, left to right. A conic gradient reads as a pie chart, and blended stops invent colours the spool does not have.

**Material filter: one shared set, with a "Multiple" option in the table.**
`material_filter` becomes `RwSignal<Vec<String>>`, serialised comma-separated under the existing `filter.spools.material` key — a legacy single value reads back as a one-element set, and an empty string as "no filter". The grid renders it as a checkbox row; the table's existing `<select>` writes a one-element set (or an empty one for "All"). When the set holds more than one material the table's dropdown shows an extra selected `"Multiple (n)"` option, so an active filter is never displayed as inactive; picking a concrete material replaces the set.

## Risks / Trade-offs

- The table's Material dropdown cannot express a multi-material selection. Mitigated by the `"Multiple (n)"` option rather than by silently showing "All".
- `"Loading…"` flashes at `/` for one settings round-trip. The alternative — assuming `spool` and swapping — flashes a whole table instead.
- Card grids are heavier to render than table rows. At home-use scale (tens to low hundreds of spools, paginated at 25) this needs no virtualisation.
- `style/spoolman.css` grows by three blocks. The unimplemented `add-css-styling-stylers` change proposes moving all CSS into the `stylers` crate and will have to carry these too.

## Open Questions

- None blocking. If `0.02` OkLab chroma misclassifies real filaments (pearlescent greys, very dark colours), the constant moves or becomes a setting later.
