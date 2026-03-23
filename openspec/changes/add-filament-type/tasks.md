## 1. API client

- [x] 1.1 Add `list_materials() -> Result<Vec<String>, ApiError>` to `crates/spoolman-client/src/api/filament.rs` calling `GET /api/v1/material`

## 2. Material select on filament forms

> Implementation: replaced datalist/autocomplete with a full `<select>` driven by `MaterialType::all_known()` — no async fetch needed since all variants are compiled in.

- [x] 2.1 `MaterialSelect` component in `pages/filament.rs` renders `<select>` from `MaterialType::all_known()`
- [x] 2.2 `FilamentCreate` uses `MaterialSelect` wired to `material: RwSignal<String>`
- [x] 2.3 `FilamentEdit` uses the same `MaterialSelect` component

## 3. Material filter on filament list

- [x] 3.1 `FilamentList` has `material_filter: RwSignal<String>`
- [x] 3.2 `<select>` dropdown in page-actions bar: first option "All materials", then one per `MaterialType::all_known()`
- [x] 3.3 `material_filter` change resets `ts.page` to 0
- [x] 3.4 `list_filaments(material: Option<&str>)` appends `?material=<value>` when set
- [x] 3.5 Resource re-fetches when `material_filter` changes

## 4. Material in spool list text filter

- [x] 4.1 `SpoolList` filtered closure includes `filament.material.abbreviation()` in the match

## 5. Update CHANGELOG and TODO

- [x] 5.1 Added `MaterialType` enum entry to `CHANGELOG.md` under `[Unreleased] → Added`
- [x] 5.2 Added material filter and spool filter entries to `CHANGELOG.md`
- [x] 5.3 Remove "Add `filament_type` field to Filament" from `TODO.md` (covered by this change)
