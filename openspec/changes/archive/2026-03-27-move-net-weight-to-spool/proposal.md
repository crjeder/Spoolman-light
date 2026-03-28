## Why

`Filament.net_weight` is a property of an individual spool purchase, not the filament specification — the same material can be sold in 250g, 750g, 1kg, and 2.85kg formats. Keeping it on `Filament` forces users to create separate filament entries for each package size and makes derived weight metrics unavailable when a filament has mixed-size spools.

## What Changes

- **BREAKING** Remove `net_weight: Option<f32>` from the `Filament` struct
- **BREAKING** Add `net_weight: Option<f32>` to the `Spool` struct
- Update spool weight derivation to read `spool.net_weight` instead of `filament.net_weight`
- Update `CreateSpool` and `UpdateSpool` request types to include `net_weight`
- Remove `net_weight` from `CreateFilament` and `UpdateFilament` request types
- Update JSON data migration: move `net_weight` from each filament into each spool that references it (best-effort; spools without a matching filament entry get `None`)
- Update frontend filament form to remove `net_weight` field; add to spool form

## Capabilities

### New Capabilities
<!-- none -->

### Modified Capabilities
- `data-model`: Filament entity loses `net_weight`; Spool entity gains `net_weight`
- `spool-management`: Weight derivation reads `spool.net_weight` instead of `filament.net_weight`; `CreateSpool` and `UpdateSpool` include `net_weight`
- `filament-management`: `net_weight` removed from filament create/edit endpoints and forms

## Impact

- **`spoolman-types`**: `Filament` and `Spool` structs, `CreateFilament`, `UpdateFilament`, `CreateSpool`, `UpdateSpool` request types
- **`spoolman-server`**: spool response weight derivation; JSON store migration (schema_version bump)
- **`spoolman-client`**: filament form (remove field), spool form (add field), weight display logic
- **Data migration**: existing `spoolman.json` files need a one-time migration on server start
