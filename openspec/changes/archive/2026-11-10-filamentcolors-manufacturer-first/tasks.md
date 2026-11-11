## 1. Search component

- [x] 1.1 Add `#[prop(into, optional)] manufacturer: Signal<Option<String>>` to `FilamentColorsSearch`
- [x] 1.2 Add a pure `rank_swatches(entries, tokens, manufacturer) -> Vec<FilamentColorsSwatch>` helper (token filter, stable sort manufacturer matches first, take 10) and use it in `results`
- [x] 1.3 Add a native unit test for `rank_swatches`: manufacturer-first ordering, fallback fill to 10, bidirectional substring match, empty/None manufacturer keeps order

## 2. Wire pages

- [x] 2.1 Spool Create: pass manufacturer of the filament matching `filament_id` in `filaments_list`
- [x] 2.2 Spool Edit: pass `sr.filament.manufacturer` from the loaded spool

## 3. Verify

- [x] 3.1 `cargo test -p spoolman-client` (native) and `cargo clippy -p spoolman-types -p spoolman-server` pass; `cargo check -p spoolman-client --target wasm32-unknown-unknown` if the target is installed
- [ ] 3.2 Manual check on New Spool: pick a filament, type a color, confirm that brand's swatches appear first and change with the filament selection
