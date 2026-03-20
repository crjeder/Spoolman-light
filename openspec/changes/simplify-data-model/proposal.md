## Why

Spoolman's current data model treats Filament as a product SKU (including color, price, weight, article number), which conflates printing properties with purchasing/inventory details. For a printer-focused tracker, Filament should describe how a material prints — not what it costs or what color it is. This change restructures the model around actual use: Filament = material formula, Spool = physical instance.

## What Changes

- **BREAKING**: Remove `Vendor` as a separate entity; replace `vendor_id` on Filament with a free-text `vendor: str` field
- **BREAKING**: Move `color_hex`, `multi_color_hexes`, `multi_color_direction` from Filament → Spool
- **BREAKING**: Move `price` from Filament → Spool only (drop filament-level default)
- **BREAKING**: Remove `weight` and `spool_weight` from Filament (weight tracking belongs on the physical spool)
- **BREAKING**: Remove `article_number` and `external_id` from Filament (SKU-oriented fields)
- **BREAKING**: Remove `lot_nr` and `external_id` from Spool
- **BREAKING**: Remove `empty_spool_weight` from Vendor (eliminated with Vendor entity)
- Remove all `/vendor` API endpoints
- Remove vendor CRUD from the storage layer

## Capabilities

### New Capabilities
- `simplified-data-model`: Two-entity model where Filament captures only material/printing properties and Spool captures all physical instance attributes (color, weight, price, usage)

### Modified Capabilities
- `json-storage`: Storage layer loses Vendor entity and gains restructured Filament/Spool models

## Impact

- **Backend models**: `spoolman/storage/models.py` — remove `VendorModel`, restructure `FilamentModel` and `SpoolModel`
- **Storage layer**: `spoolman/storage/store.py` — remove all vendor methods; update filament/spool create/update/find signatures
- **API routes**: Remove `spoolman/api/v1/vendor.py`; update `filament.py` and `spool.py` request/response models
- **API models**: `spoolman/api/v1/models.py` — update `Filament` and `Spool` response models, remove `Vendor`
- **Extra fields**: `spoolman/api/v1/field.py` — remove vendor entity type
- **Prometheus**: `spoolman/prometheus/metrics.py` — remove vendor metrics if any
- **Frontend**: All vendor-related pages, components, and API calls must be removed; filament and spool forms updated
- **Integration tests**: Update fixtures and test cases to reflect new model
