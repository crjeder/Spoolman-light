## 1. Parser

- [x] 1.1 Add `crates/spoolman-client/src/slicer_profile.rs` with `ProfileData` and `parse_orca(&str) -> Result<ProfileData, String>` (value access, nil handling, modifier heuristic, bed-type mapping, validation); register in `lib.rs`
- [x] 1.2 Unit tests: Polymaker HT-PLA example (inline JSON), nil/missing values, unknown bed type fallback, invalid JSON, non-profile object

## 2. UI

- [x] 2.1 In `FilamentCreate` add "Import profile" button + hidden `.json` file input; read with `gloo_file::futures::read_as_text`
- [x] 2.2 Apply `ProfileData` to the form signals (skip diameter when `ds.uniform`, keep values for absent fields); show parse errors via the `error` signal
- [x] 2.3 Reset the file input value after import so the same file can be re-selected

## 3. Verification

- [x] 3.1 `cargo test -p spoolman-client` (parser tests) and `cargo clippy -p spoolman-types -p spoolman-server`
- [x] 3.2 Add Playwright test in `tests/e2e/tests/` uploading a profile fixture and asserting the pre-filled fields
- [x] 3.3 Add entry to `CHANGELOG.md` / `TODO.md` per project conventions on archive
