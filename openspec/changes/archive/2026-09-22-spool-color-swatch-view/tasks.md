## 1. Colour ordering primitives

- [x] 1.1 In `crates/spoolman-client/src/utils/color.rs`: add `rgba_to_oklch(&Rgba) -> (f32, f32, f32)` returning `(lightness, chroma, hue)`, reusing the existing OkLab conversion. Normalise hue to `0..2pi` starting at red.
- [x] 1.2 Add `const ACHROMATIC_CHROMA: f32 = 0.02;` and `hue_sort_key(&[Rgba]) -> (bool, f32, f32)` returning `(is_achromatic, hue_or_0, lightness)` from the first colour, using the `#c8c8c8` fallback for an empty slice. Tuple ordering gives chromatic-then-achromatic for free.
- [x] 1.3 Add an `assert`-based unit test: a black, a white, a mid grey, a red and a blue sort to `[red, blue, black, grey, white]` ascending, and reverse exactly under descending.

## 2. Shared state

- [x] 2.1 In `state.rs`: add `default_sort: &'static str` parameter to `use_table_state`; update the `"filaments"` and `"spools"` call sites to pass `"registered"`.
- [x] 2.2 In `state.rs`: add `ViewMode { Spool, Color }` and `DefaultView(pub RwSignal<Option<ViewMode>>)` with a `default_view()` context accessor.
- [x] 2.3 In `app.rs`: load `default_view` from the settings resource into the `DefaultView` signal (`Some(Color)` only for the exact value `"color"`, `Some(Spool)` otherwise).

## 3. Material filter becomes a set

- [x] 3.1 In `spool.rs`: change `material_filter` to `RwSignal<Vec<String>>`; parse the `filter.spools.material` session value by splitting on `,` and dropping empties; persist by joining with `,`.
- [x] 3.2 Update the `filtered()` material predicate to pass when the set is empty or contains the spool's material abbreviation.
- [x] 3.3 Update the table's Material `<select>`: "All" sets an empty vec, a concrete value sets a one-element vec, and when `len() > 1` render an extra selected `"Multiple (n)"` option.
- [x] 3.4 Update `clear_filters` to empty the vec.

## 4. Swatch grid component

- [x] 4.1 New `crates/spoolman-client/src/components/swatch_grid.rs` with `<SwatchGrid items=Signal<Vec<SpoolResponse>> locations=... />`: one card per spool, linking to `/spools/{id}`, labelled with colour name (hex fallback), filament name, material, remaining weight, location; archived cards carry the archived class.
- [x] 4.2 Card fill: single colour flat; 2 to 4 colours as equal hard-stop `linear-gradient` bands; empty `colors` uses the `#c8c8c8` fallback.
- [x] 4.3 Material checkbox row bound to the shared filter set, listing distinct materials alphabetically.
- [x] 4.4 Register the module in `components/mod.rs`.

## 5. Wire the two views

- [x] 5.1 In `spool.rs`: give `SpoolList` a `mode: ViewMode` prop; use `use_table_state("spools", "registered")` for `Spool` mode and `use_table_state("colors", "hue")` for `Color` mode.
- [x] 5.2 Add the `"hue"` arm to the `sorted()` match using `hue_sort_key`, honouring `sort_asc`. Verify the existing colour-delta early return still precedes it.
- [x] 5.3 Branch the render block: `Color` mode renders the checkbox row + `<SwatchGrid>` + `<Pagination>`; `Spool` mode renders the existing table unchanged.
- [x] 5.4 Make the Color column header sortable on `"hue"`, keeping `stop_propagation` on the threshold select and picker popup so they do not trigger a sort.
- [x] 5.5 In `app.rs`: route `/colors` to `SpoolList mode=Color`, `/spools` to `SpoolList mode=Spool`, and `/` to a new `RootView` that renders the loading placeholder while `DefaultView` is `None`.

## 6. Settings and navigation

- [x] 6.1 In `settings.rs`: add a "Default view" selector (`spool` / `color`), pre-populated from the loaded settings, saved via `PUT /api/v1/setting/default_view`.
- [x] 6.2 In `layout.rs`: add the "Color" sidebar entry; update `spools_active` to `"/spools"` or (`"/"` and default is spool); add the equivalent active rule for Color.

## 7. Styling

- [x] 7.1 In `style/spoolman.css`: add `.swatch-grid` (`grid-template-columns: repeat(auto-fill, minmax(160px, 1fr))`), `.swatch-card`, `.swatch-card-fill`, `.swatch-card-meta`, `.material-checks`, and an archived-card rule. Use existing tokens so both themes work.

## 8. Docs

- [x] 8.1 Document the `default_view` setting in `README.md` alongside the other settings.
- [x] 8.2 Add "pin favourites / palette export" to `TODO.md` under Enhancements, referencing the upstream project.
- [x] 8.3 Mark the `integrate https://github.com/Disane87/spoolman-filament-swatch` TODO item as done with a one-line note on what was and was not ported.

## 9. Verification

- [x] 9.1 `cargo check -p spoolman-client --target wasm32-unknown-unknown` and `cargo clippy -p spoolman-types -p spoolman-server`.
- [x] 9.2 New `tests/e2e/tests/swatch-view.spec.ts`: `/colors` renders cards and no `table.data-table`; a card links to its spool; greys appear after saturated colours; ticking two material checkboxes filters the grid.
- [ ] 9.3 E2E: set `default_view = color`, load `/`, assert the grid renders and the URL is still `/`.
- [ ] 9.4 E2E: apply a material filter in the table, switch to `/colors`, assert it is still applied; sort the table by Registered, visit `/colors`, return and assert the table sort is unchanged.
- [ ] 9.5 Manually verify on a phone-width viewport that cards reflow to one or two columns. Note that the 200px sidebar is not yet responsive — see the `responsive-sidebar-burger` change.
