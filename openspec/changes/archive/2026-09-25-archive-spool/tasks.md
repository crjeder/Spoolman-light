## 1. List page

- [x] 1.1 Add `confirm_archive: RwSignal<Option<u32>>` and `on_archive(id, archived)` handler calling `api::update_spool` then refetching the list resource
- [x] 1.2 Add Archive/Unarchive icon button to the row actions cell; warn + Confirm/Cancel when the spool is not empty (shared `is_empty` helper: `remaining_filament <= 0`, fallback `current_weight <= 0`)
- [x] 1.3 Widen the reserved actions-cell width so confirm state does not shift buttons

## 2. Detail page

- [x] 2.1 Add Archive/Unarchive button and `confirm_archive` signal to `SpoolShow` header with the same warning flow and `is_empty` helper, refetching `spool` after update

## 3. Tests and docs

- [~] 3.1 Playwright test: archive empty spool (no warning), archive non-empty spool (warning, cancel, confirm), unarchive
- [x] 3.2 `cargo check -p spoolman-client --target wasm32-unknown-unknown` and clippy pass
- [x] 3.3 Add TODO.md entry/update CHANGELOG.md on archive
