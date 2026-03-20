## 1. Restructure Storage Models

- [x] 1.1 Remove `VendorModel` from `spoolman/storage/models.py`
- [x] 1.2 Update `FilamentModel`: replace `vendor_id: int` with `vendor: Optional[str]`; remove `weight`, `spool_weight`, `price`, `color_hex`, `multi_color_hexes`, `multi_color_direction`, `article_number`, `external_id`
- [x] 1.3 Update `SpoolModel`: add `color_hex`, `multi_color_hexes`, `multi_color_direction`; remove `lot_nr`
- [x] 1.4 Update `DataStore`: remove `vendors: list[VendorModel]`; bump `schema_version` to 2 in default meta

## 2. Update Storage Layer

- [x] 2.1 Remove all vendor methods from `spoolman/storage/store.py` (`create_vendor`, `get_vendor`, `find_vendors`, `update_vendor`, `delete_vendor`, `clear_extra_field_vendors`, `_next_vendor_id`)
- [x] 2.2 Update `create_filament`: remove `vendor_id`, `weight`, `spool_weight`, `price`, `color_hex`, `multi_color_hexes`, `multi_color_direction`, `article_number`, `external_id` parameters; add `vendor: Optional[str]`
- [x] 2.3 Update `find_filaments`: remove `vendor_id`, `vendor_name` filter parameters; add `vendor` string filter
- [x] 2.4 Update `update_filament`: remove handling for removed fields and vendor inheritance logic
- [x] 2.5 Update `create_spool`: add `color_hex`, `multi_color_hexes`, `multi_color_direction` parameters; remove `lot_nr`; remove filament `spool_weight` inheritance (no longer on filament)
- [x] 2.6 Update `find_spools`: remove `vendor_id`, `vendor_name` filter parameters (filament no longer has vendor FK); add `vendor` string filter via filament lookup
- [x] 2.7 Update `update_spool`: handle new color fields; remove lot_nr handling
- [x] 2.8 Move `find_by_color` from filament-based to spool-based search
- [x] 2.9 Update `_sort_filaments`: remove `vendor.*` sort fields; add `vendor` string sort
- [x] 2.10 Update `_sort_spools`: remove `filament.vendor.*` cross-entity sort; update for new spool fields

## 3. Update API Route Handlers

- [x] 3.1 Delete `spoolman/api/v1/vendor.py`
- [x] 3.2 Remove vendor router registration from `spoolman/api/v1/router.py`
- [x] 3.3 Update `spoolman/api/v1/filament.py`: update `FilamentParameters` and `FilamentUpdateParameters` to reflect new field set; update `_filament_to_api` helper
- [x] 3.4 Update `spoolman/api/v1/spool.py`: update `SpoolParameters` and `SpoolUpdateParameters` to add color fields and remove `lot_nr`; move `/find-by-color` endpoint to spool router
- [x] 3.5 Remove `/filament/find-by-color` endpoint if it exists; add `/spool/find-by-color` endpoint
- [x] 3.6 Update `spoolman/api/v1/field.py`: remove vendor entity type from extra fields management

## 4. Update API Response Models

- [x] 4.1 Update `Filament` response model in `spoolman/api/v1/models.py`: remove `vendor` object/id fields; add `vendor: Optional[str]`; remove color, price, weight, article_number, external_id fields
- [x] 4.2 Update `Spool` response model in `spoolman/api/v1/models.py`: add color fields; remove `lot_nr`
- [x] 4.3 Remove `Vendor` response model from `spoolman/api/v1/models.py`
- [x] 4.4 Update `Spool.from_db` and `Filament.from_db` class methods to reflect new fields

## 5. Update Prometheus Metrics

- [x] 5.1 Update `spoolman/prometheus/metrics.py`: remove any vendor count metrics; ensure filament/spool metrics use updated store methods

## 6. Update Integration Tests

- [x] 6.1 Remove all vendor-related test cases from integration tests
- [x] 6.2 Update filament test fixtures to use new field set (remove color, price, weight; add vendor string)
- [x] 6.3 Update spool test fixtures to include color fields, remove lot_nr
- [x] 6.4 Add tests for color filtering on spools
- [x] 6.5 Run full integration test suite and fix any remaining failures

## 7. Documentation and Changelog

- [x] 7.1 Update `CHANGELOG.md` with breaking changes: removed Vendor entity, restructured Filament/Spool fields
- [x] 7.2 Update `README.md` to remove vendor documentation and update field listings
- [x] 7.3 Update `TODO.md` to reflect completed and pending work
