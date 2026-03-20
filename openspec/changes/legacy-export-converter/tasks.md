## 1. Script Scaffold

- [ ] 1.1 Create `scripts/convert_export.py` with `argparse` CLI: positional `spool_export`, `--filaments`, `--output` (default `spoolman.json`)
- [ ] 1.2 Add usage/help docstring describing expected input format and example command

## 2. Filament Extraction

- [ ] 2.1 Parse the flat spool-export JSON and extract unique filaments keyed by `filament.id`
- [ ] 2.2 Map `filament.*` flat keys to `FilamentModel` fields (id, registered, name, material, density, diameter, settings_extruder_temp, settings_bed_temp, comment, extra)
- [ ] 2.3 Inline vendor: map `filament.vendor.name` → `vendor` string (null if absent)

## 3. Spool Extraction

- [ ] 3.1 Map flat spool keys to `SpoolModel` fields (id, registered, first_used, last_used, filament_id, used_weight, initial_weight, spool_weight, location, comment, archived, extra)
- [ ] 3.2 Copy color fields from filament to spool: `filament.color_hex` → `color_hex`, `filament.multi_color_hexes` → `multi_color_hexes`, `filament.multi_color_direction` → `multi_color_direction` (only when spool field is null/absent)
- [ ] 3.3 Copy `filament.price` → spool `price` when spool price is null/absent
- [ ] 3.4 Copy `filament.weight` → spool `initial_weight` when `initial_weight` is null/absent

## 4. Optional Filament Export Input

- [ ] 4.1 When `--filaments` is provided, parse the flat filament-export JSON
- [ ] 4.2 Add filaments from that file that are not already present (by ID) in the spool-derived filament map

## 5. Output

- [ ] 5.1 Assemble the final dict: `{meta: {schema_version: 2}, filaments: [...], spools: [...], settings: {}}`
- [ ] 5.2 Write output JSON atomically (write to `.tmp`, rename) to the `--output` path
- [ ] 5.3 Print a summary line: number of filaments and spools written, output path

## 6. Validation & Edge Cases

- [ ] 6.1 Silently drop unknown/removed fields (`lot_nr`, `external_id`, `article_number`, `filament.spool_weight` on filament, etc.) without raising errors
- [ ] 6.2 Default `extra` to `{}` when the key is absent from a record
- [ ] 6.3 Add a smoke test: run the script against a hand-crafted minimal old-format JSON and assert the output matches the expected `spoolman.json` shape
