## 1. Data model

- [x] 1.1 Add `swatch_image: Option<String>` to `Spool` in `crates/spoolman-types/src/models.rs` with serde default, and to the create/update request types
- [x] 1.2 Add `swatch_image` to any spool response DTOs / OpenAPI-ish doc comments alongside existing fields (e.g. `price`)

## 2. Server: swatch image storage

- [x] 2.1 Add a `swatch_images/` directory alongside the JSON data file (created lazily), and a dedupe lookup (source URL -> stored id) persisted on disk or in the JSON store
- [x] 2.2 Implement `POST /api/v1/swatch-image` accepting `{ url }`: downloads the image if not already stored, generates the on-disk filename/id server-side (never from client input directly), saves the bytes, returns `{ id }`
- [x] 2.3 Implement `GET /api/v1/swatch-image/{id}` serving the stored image bytes with correct content type
- [x] 2.4 Wire both routes into `crates/spoolman-server/src/routes/mod.rs`

## 3. Client: filamentcolors.xyz data layer

- [x] 3.1 Create `crates/spoolman-client/src/filamentcolors.rs` mirroring `spoolmandb.rs`: `FilamentColorsSwatch` struct (manufacturer, color name, `hex_color`, image URL, id), `load_filamentcolors()` with 24h localStorage cache under `filamentcolors_cache`, native stub returns `Err`
- [x] 3.2 Implement client-side case-insensitive filter over cached swatches (manufacturer + color name), capped at 10 results

## 4. Client: search UI component

- [x] 4.1 Create `crates/spoolman-client/src/components/filamentcolors_search.rs`: `FilamentColorsSearch` component with text input, result list (thumbnail + manufacturer + color name), `on_select: Callback<FilamentColorsSwatch>`
- [x] 4.2 On select, call `POST /api/v1/swatch-image` with the swatch's image URL and store the returned image id in the form's local state (fire-and-forget; does not block the form)

## 5. Wire into spool create/edit forms

- [x] 5.1 Add `<FilamentColorsSearch>` alongside `<SpoolmanDbSearch>` in the spool create form (`crates/spoolman-client/src/pages/spool.rs`)
- [x] 5.2 Add it to the spool edit form as well
- [x] 5.3 On select, set the color field(s) from `hex_color` and keep fields editable afterward
- [x] 5.4 Include `swatch_image` in the create/update API payload when set

## 6. Shared swatch rendering

- [x] 6.1 Extract a small shared color-rendering helper/component used by the spool list table, spool detail view, and `/colors` swatch grid: renders the stored swatch image (`<img>`) when `swatch_image` is present, otherwise the existing flat CSS-background chip
- [x] 6.2 Update spool list table color cell (`crates/spoolman-client/src/pages/spool.rs`) to use it
- [x] 6.3 Update spool detail view Colors field (same file) to use it
- [x] 6.4 Update `/colors` swatch grid card fill (`crates/spoolman-client/src/components/swatch_grid.rs`, `fill_style()` / card render) to use it

## 7. Verification

- [x] 7.1 `cargo check -p spoolman-types -p spoolman-server` passes (also verified `cargo check -p spoolman-client` native and `--target wasm32-unknown-unknown`, and `cargo clippy -p spoolman-types -p spoolman-server`)
- [ ] 7.2 Manually verify: search a swatch on spool create, confirm hex color and image apply, confirm image renders in list/detail/grid
- [ ] 7.3 Manually verify: a spool without any filamentcolors.xyz interaction renders unchanged (flat chip, no regressions)
- [ ] 7.4 Manually verify: selecting a second, different swatch's image dedupes correctly (same URL twice does not re-download)
