## 1. Types

- [x] 1.1 `models.rs`: make `print_temp`/`bed_temp` legacy (`#[serde(default, skip_serializing)]`), add `min_chamber_temp`/`max_chamber_temp`, bump `StoreMeta` default to schema 3
- [x] 1.2 `requests.rs`: remove `print_temp`/`bed_temp` from create and update requests, add chamber min/max

## 2. Server

- [x] 2.1 `store.rs`: create/update handle chamber min/max, drop single temps
- [x] 2.2 `store.rs`: validate min <= max per pair (422 on violation)
- [x] 2.3 `store.rs` `migrate()`: v2 to v3, fill min/max from legacy single values where `None`, clear legacy fields
- [x] 2.4 Tests: migration (both scenarios), chamber round-trip, min > max rejected; update `tests/common` fixture usage if needed

## 3. Client

- [x] 3.1 `pages/filament.rs`: create/edit forms with min/max inputs for print, bed, chamber; fill from SpoolmanDB and slicer profile as min = max
- [x] 3.2 `pages/filament.rs`: detail view shows ranges
- [x] 3.3 `pages/spool.rs`: auto-created filament sets min/max from SpoolmanDB entry, drop single temps

## 4. Data and tooling

- [x] 4.1 `scripts/convert_export.py` and `scripts/test_convert_export.py`: emit min/max pairs
- [x] 4.2 `assets/spoolman.json` fixture and `scripts/expand_testdata.py`: use min/max
- [x] 4.3 e2e `filament-import-profile.spec.ts`: expect min/max inputs

## 5. Docs and verification

- [x] 5.1 README field list, CHANGELOG (breaking), TODO.md
- [x] 5.2 `cargo check` and `cargo clippy` for types and server, `cargo test -p spoolman-server`
