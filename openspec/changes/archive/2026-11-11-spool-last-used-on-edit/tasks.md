## 1. Frontend

- [x] 1.1 In `crates/spoolman-client/src/pages/spool.rs`, replace `last_used_was_none` with a `last_used_loaded` signal holding the prefilled `last_used` string
- [x] 1.2 On submit, send today's date for `last_used` when `last_used.get() == last_used_loaded.get()`; otherwise send the user's value (drop the `prune_default` call for `last_used`)

## 2. Tests

- [x] 2.1 Add a Playwright test in `tests/e2e/tests/spools.spec.ts`: edit a spool's current weight, save, assert last used equals today
- [x] 2.2 Add a Playwright test: change the last used date manually, save, assert the chosen date is kept

## 3. Verify

- [x] 3.1 `cargo check -p spoolman-types -p spoolman-server` and `cargo clippy` pass
