## 1. Implementation

- [x] 1.1 In `crates/spoolman-client/src/pages/spool.rs`, update the spool detail view (`SpoolShow`) — replace `{sr.filament.display_name()}` with `<a href=format!("/filaments/{}", sr.filament.id)>{sr.filament.display_name()}</a>`
- [x] 1.2 In the same file, update the spool list table row — replace `<td>{name}</td>` with `<td><a href=format!("/filaments/{}", sr.filament.id)>{name}</a></td>`

## 2. Verification

- [x] 2.1 `cargo check -p spoolman-client --target wasm32-unknown-unknown` passes with no errors
- [ ] 2.2 Manually verify: spool detail page shows filament name as a clickable link that navigates to the correct filament detail page
- [ ] 2.3 Manually verify: spool list table shows filament names as clickable links
