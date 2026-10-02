## Why

Copolyester filament is commonly labelled "CoPE" (SpoolmanDB, vendor spools, imports). The material enum only recognises the OpenPrintTag abbreviation `CPE`, so "CoPE" is stored as `Other("CoPE")`: it gets no spec key or full name and does not group with `CPE` in the material filter.

## What Changes

- `MaterialType::from_abbreviation` maps `CoPE` (case-insensitive) to `MaterialType::Cpe`.
- Canonical serialized form stays `CPE`; no new enum variant, no new OpenPrintTag key.

## Capabilities

### New Capabilities

### Modified Capabilities
- `spoolmandb-lookup`: SpoolmanDB material strings spelled "CoPE" resolve to the `CPE` material type.

## Impact

- `crates/spoolman-types/src/models.rs` (`from_abbreviation` only).
- Existing stored data and JSON format unchanged; previously stored `Other("CoPE")` values stay as-is.
