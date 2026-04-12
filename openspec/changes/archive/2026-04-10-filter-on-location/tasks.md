## 1. Client API

- [x] 1.1 Add `location_id: Option<u32>` parameter to `api::spool::list_spools` in `crates/spoolman-client/src/api/spool.rs` and append `&location_id=<id>` to the URL when `Some`

## 2. Spool List Page

- [x] 2.1 Add a `Resource` to fetch the locations list on the spool list page (reuse `api::location::list_locations`)
- [x] 2.2 Add a `location_filter: RwSignal<Option<u32>>` signal to the spool list page
- [x] 2.3 Thread `location_filter` as a dependency into the spool list `Resource` so it re-fetches on change
- [x] 2.4 Add a location filter `<select>` dropdown (with an empty "All locations" option) above or alongside the existing controls, populated from the locations resource
- [x] 2.5 Replace the raw `location_id` integer in the Location table column with the resolved location name from the locations resource

## 3. Verification

- [x] 3.1 `cargo check -p spoolman-client` passes with no errors
- [ ] 3.2 Manually verify: selecting a location filters the spool list; clearing it shows all spools
- [ ] 3.3 Manually verify: Location column shows names, not raw IDs
