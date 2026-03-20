## Context

Spoolman's data model has three entities: Vendor → Filament → Spool. The Filament entity currently acts as a product SKU, carrying color, price, weight, article number, and external ID alongside material properties. The Vendor entity exists primarily as a named foreign key. This change strips both entities down to what a printer-focused user actually needs.

Current state after `remove-db-use-json-storage`:
- `VendorModel`: name, comment, empty_spool_weight, external_id, extra
- `FilamentModel`: vendor_id (FK), name, material, density, diameter, color_hex, multi_color_hexes, multi_color_direction, price, weight, spool_weight, article_number, external_id, extruder_temp, bed_temp, comment, extra
- `SpoolModel`: filament_id (FK), used_weight, initial_weight, spool_weight, price, color fields not present, lot_nr, location, first_used, last_used, archived, comment, extra

## Goals / Non-Goals

**Goals:**
- Filament models only material/printing properties: vendor (string), name, material, density, diameter, extruder_temp, bed_temp, comment, extra
- Spool models the physical object: filament_id, color, initial_weight, spool_weight, used_weight, price, location, first_used, last_used, archived, comment, extra
- Remove Vendor as an entity entirely (no CRUD, no storage, no API routes)
- Preserve all existing spool usage operations (use_weight, use_length, measure_spool)
- Preserve filtering/sorting capabilities on the reduced field set

**Non-Goals:**
- Providing a migration path from old data (users are already on a new JSON format from the previous change; document the break)
- Keeping any backward-compatible aliases for removed fields
- Frontend implementation (noted as impact but not in scope for backend change)

## Decisions

### 1. Vendor becomes a free-text string on Filament

**Decision:** Replace `vendor_id: int` with `vendor: Optional[str]` directly on `FilamentModel`.

**Rationale:** Vendor's only remaining value after removing `empty_spool_weight` is grouping filaments by brand name. A free-text string achieves this without a separate entity, FK lookups, join logic, or CRUD endpoints. The cost — no atomic rename across all filaments — is acceptable for a single-user personal tracker.

**Alternative considered:** Keep Vendor as a lightweight entity (name only). Rejected: adds a whole resource layer (model, store methods, API routes, dependency injection) for zero functional benefit beyond what a string provides.

### 2. Color fields move from Filament to Spool

**Decision:** `color_hex`, `multi_color_hexes`, `multi_color_direction` become fields on `SpoolModel`.

**Rationale:** Color does not affect print behavior (density, diameter, and temperatures are identical across color variants of the same material). Keeping color on Filament forces one Filament entry per color variant. Moving it to Spool means one Filament entry per material/brand, with color tracked per physical spool.

**Alternative considered:** Keep color on Filament (SKU model). Rejected per user requirements: the app is printer-focused, not catalog-focused.

### 3. Weight and spool_weight removed from Filament

**Decision:** Drop `weight` and `spool_weight` from `FilamentModel`. These become Spool-only fields.

**Rationale:** The same material comes in multiple spool sizes (250g, 500g, 1kg, 2kg). Weight is a property of the physical spool purchased, not the material. The current `filament.weight` only existed as a default for `spool.initial_weight`; without it, users set `initial_weight` directly on the spool.

### 4. Price, article_number, external_id, lot_nr removed entirely

**Decision:** Drop `price` from Filament (keep on Spool). Drop `article_number` and `external_id` from Filament. Drop `lot_nr` and `external_id` from Spool.

**Rationale:** Price is transactional — what you paid for this spool. Article number and external IDs are SKU/catalog identifiers with no value in a print-focused workflow. Lot number is a manufacturing batch identifier useful for quality control but not for personal filament tracking.

### 5. `find_by_color` moves to Spool queries

**Decision:** The color similarity search (`find_by_color`) operates on Spools instead of Filaments.

**Rationale:** Color now lives on Spool. The endpoint behavior changes: instead of returning filament types by color, it returns spools by color. This is actually more useful — it tells you which physical spools you have of a given color.

### 6. No cascading defaults for spool_weight

**Decision:** Remove the three-level cascade (Vendor → Filament → Spool) for `spool_weight`. Users set it directly on the Spool.

**Rationale:** The cascade was a convenience feature tied to Vendor, which is being removed. `spool_weight` is now a simple optional field on Spool with no inheritance.

## Risks / Trade-offs

- **Breaking change for existing users** → Data files from the previous JSON format are incompatible. Document clearly in release notes. A one-time migration script is out of scope but recommended.
- **Loss of filament-level color search** → The `/filament` color search endpoint changes semantics (now searches spools). Clients querying filaments by color will need to use the spool endpoint instead.
- **No price default propagation** → Users who add multiple spools of the same filament type must enter price each time. Acceptable trade-off for model clarity.
- **Typo-based vendor duplicates** → Without a normalized Vendor entity, "eSun" and "esun" are distinct filter values. Acceptable for personal use; a future enhancement could add vendor autocomplete from existing values.

## Migration Plan

1. Users on the previous JSON format must start fresh (new empty `spoolman.json`).
2. No automatic migration from previous JSON schema.
3. Document the break in CHANGELOG.md under the release that includes this change.

## Open Questions

- Should the `/filament/find-by-color` endpoint be removed, repurposed to search spools, or kept as-is pointing at spool data? Recommend: repurpose to search spools and update the route to `/spool/find-by-color`.
- Should `vendor` on Filament support autocomplete from existing values in the API? (Nice-to-have, not in scope for this change.)
