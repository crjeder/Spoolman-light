## Why

The CHANGELOG explicitly states users must export data before upgrading to the new JSON-storage build, then re-import it afterward — but no tool exists to perform that conversion. Users who exported via the old `/api/v1/export/spools?fmt=json` endpoint have flat, dot-separated JSON that cannot be loaded directly into the new `spoolman.json` format.

## What Changes

- **New**: A standalone Python script (`scripts/convert_export.py`) that reads one or more old-format export files and writes a valid `spoolman.json` data file.
- The script accepts the old flat spool-export JSON (and optionally filament/vendor exports) as input.
- It reconstructs the new storage structure: separate `filaments` and `spools` arrays with foreign-key references, vendor names inlined as strings, and color/price fields moved from filament to spool.
- Dropped fields (`article_number`, `external_id` on filament; `lot_nr`, `external_id` on spool; `weight`/`spool_weight` on filament) are silently ignored during conversion.
- The output is a valid `spoolman.json` with `meta.schema_version: 2`.

## Capabilities

### New Capabilities

- `legacy-export-converter`: CLI script that converts the old flat-JSON export format into the new `spoolman.json` storage format.

### Modified Capabilities

<!-- none -->

## Impact

- New file: `scripts/convert_export.py` (no changes to existing application code)
- No API changes; no frontend changes
- Depends only on the stdlib + the existing `spoolman.storage.models` Pydantic models (or can be self-contained)
