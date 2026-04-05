## 1. Remove ID column from spool list

- [x] 1.1 Delete the `<ColHeader label="ID" field="id" ...>` element from the spool list table header in `crates/spoolman-client/src/pages/spool.rs`
- [x] 1.2 Delete the corresponding `<td>` cell that renders the spool ID link (`<td class="num"><a href=...>{id}</a></td>`) from each table row

## 2. Verify

- [x] 2.1 Run `cargo check -p spoolman-types && cargo check -p spoolman-server` to confirm no compilation errors in non-WASM crates
- [x] 2.2 Confirm the sort-by-id path (`"id" =>` branch in the sort match) still compiles (it can remain for URL-based sorting; it just won't be exposed in the UI)
