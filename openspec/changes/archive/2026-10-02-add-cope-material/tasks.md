## 1. Implementation

- [x] 1.1 In `crates/spoolman-types/src/models.rs` `from_abbreviation`, accept `"CoPE"` and `"COPE"` alongside `"CPE"`
- [x] 1.2 Add unit tests: `from_abbreviation("CoPE") == Cpe`; `parse_material("CoPE-CF")` returns `(Cpe, Some("CF"))` in `spoolmandb.rs`

## 2. Verify

- [x] 2.1 `cargo check -p spoolman-types -p spoolman-server` and `cargo clippy` pass
