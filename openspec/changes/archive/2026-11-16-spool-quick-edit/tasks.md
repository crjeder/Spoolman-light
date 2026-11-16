## 1. Server

- [x] 1.1 In `SpoolStore::update_spool` (`crates/spoolman-server/src/store.rs`), compute `weight_changed` as `req.current_weight` differing from the stored `current_weight` (before assignment)
- [x] 1.2 Add server tests in `crates/spoolman-server/tests`: changed weight stamps last_used; unchanged weight leaves last_used/first_used alone; explicit last_used wins

## 2. Client edit form

- [x] 2.1 Add `current_weight_loaded` and `last_used_was_none` signals in `SpoolEdit`, set on load
- [x] 2.2 Replace the last_used submit rule: user-changed date wins; else today if weight changed; else loaded value, or `None` if spool had none
- [x] 2.3 Wrap non-quick fields (initial/net weight, price, color, color name, finish, first used, swatch search) in `<details class="advanced"><summary>"Advanced"</summary>`; keep current weight, location, last used, comment outside it
- [x] 2.4 Style the advanced section if needed (frontend CSS)

## 3. Tests and docs

- [x] 3.1 Update/add Playwright tests in `tests/e2e/tests` for quick mode visibility, advanced expansion, and last_used behavior
- [x] 3.2 `cargo check` and `cargo clippy` for server and types; client check if wasm target available
- [x] 3.3 Mark the quick edit item in TODO.md done; add CHANGELOG entry when archiving
