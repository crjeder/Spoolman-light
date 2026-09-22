## 1. Implementation

- [x] 1.1 In `SpoolCreate` (`crates/spoolman-client/src/pages/spool.rs`): disable the submit button when `location_id` is `None` or unset.
- [x] 1.2 In `SpoolEdit` (`crates/spoolman-client/src/pages/spool.rs`): disable the submit button when `location_id` is `None` or unset.

## 2. Spec

- [x] 2.1 Create `openspec/changes/validate-location-required-spool/specs/spool-management/spec.md` with the formal requirement and scenarios.

## 3. Verification

- [ ] 3.1 Manually verify: spool create form — submit is disabled until a location is selected.
- [ ] 3.2 Manually verify: spool edit form — submit is disabled if location is cleared / not set.
- [ ] 3.3 Manually verify: selecting a location re-enables the submit button.
