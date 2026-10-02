## Context

`MaterialType` mirrors the OpenPrintTag `material_type_enum`; `CPE` (key 12, "Copolyester") already exists. "CoPE" is an alternate spelling of the same material, parsed via `from_abbreviation`, which does exact string matching.

## Goals / Non-Goals

**Goals:**
- "CoPE" input resolves to `Cpe`.

**Non-Goals:**
- A new enum variant (would diverge from the OpenPrintTag spec and NFC key space).
- General case-insensitive matching of all abbreviations.
- Migrating already-stored `Other("CoPE")` values.

## Decisions

- Add one match arm `"CPE" | "CoPE" | "COPE"` in `from_abbreviation`. Alternative: `to_uppercase()` the input for all materials; rejected as a behaviour change for every material beyond the request.

## Risks / Trade-offs

- Old `Other("CoPE")` records are not converted → re-save the filament to pick up `CPE`.
