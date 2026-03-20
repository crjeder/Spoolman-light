## Context

The old Spoolman used SQLAlchemy with a relational schema (vendors → filaments → spools). Users could export data via `/api/v1/export/spools?fmt=json`, which called `dump_as_json` → `flatten_pydantic_object`, producing a flat list of objects with dot-separated keys (e.g. `"filament.vendor.name"`, `"filament.color_hex"`).

The new storage is a single `spoolman.json` file with this shape:
```json
{
  "meta": {"schema_version": 2},
  "filaments": [...],
  "spools": [...],
  "settings": {}
}
```

The breaking changes relevant to conversion are:
- `Vendor` entity removed — `filament.vendor.name` → `filament.vendor` (string)
- `color_hex`, `multi_color_hexes`, `multi_color_direction` moved from Filament → Spool
- `price` moved from Filament → Spool
- `weight`, `spool_weight`, `article_number`, `external_id` removed from Filament
- `lot_nr`, `external_id` removed from Spool

## Goals / Non-Goals

**Goals:**
- Convert the old flat spool-export JSON into a valid `spoolman.json` (the only required input, since spools embed all filament/vendor data)
- Optionally accept a flat filament-export JSON to cover filaments that have no associated spools
- Produce deterministic, reproducible output given the same input
- Self-contained script requiring no installed packages beyond the Python stdlib

**Non-Goals:**
- Migrating directly from the SQLite/PostgreSQL database (tool for the JSON export only)
- Round-tripping removed fields (`lot_nr`, `article_number`, etc.)
- Supporting the CSV export format as input

## Decisions

### Decision: Spool export as primary input

The spool export embeds full filament and vendor data under `filament.*` keys, so it contains everything needed to reconstruct both `filaments` and `spools` arrays. Taking only the spool export minimizes user burden to a single file.

**Alternative considered**: require separate spool + filament + vendor exports. Rejected — more files, more complexity, and the data is redundant.

### Decision: Standalone script, no app dependency

The script lives in `scripts/convert_export.py` and uses only Python stdlib (`json`, `argparse`, `datetime`). It does NOT import from `spoolman.*`.

**Alternative considered**: integrate into the app as an API endpoint. Rejected — this is a one-shot migration tool, not a runtime feature. Keeping it separate avoids bloating the app and lets users run it without standing up the server.

### Decision: Filaments deduplicated by ID from spool export

Multiple spools may reference the same filament. The script deduplicates filaments by `filament.id`, using the data from the first occurrence encountered.

**Alternative considered**: fail on inconsistent duplicates. Rejected — minor field drift between spool records for the same filament would block migrations unnecessarily.

### Decision: color/price fall-through strategy

Old filaments had `color_hex` and `price`; old spools did not (they were `null`). The converter copies `filament.color_hex` → spool's `color_hex` and `filament.price` → spool's `price` when the spool's own value is null.

**Alternative considered**: leave them null. Rejected — users would lose color and price data on every spool.

### Decision: `initial_weight` from old `filament.weight`

Old filaments had a `weight` field (net filament weight). New spools have `initial_weight`. The converter maps `filament.weight` → spool `initial_weight` when `initial_weight` is not already set on the spool.

## Risks / Trade-offs

- **[Risk] Old export field names may differ across Spoolman versions** → Mitigation: script warns and skips unrecognized fields rather than crashing; documents expected input schema in its help text.
- **[Risk] Filaments with no spools are lost** if only the spool export is provided → Mitigation: accept an optional `--filaments` flag pointing to the old filament export JSON.
- **[Risk] Extra fields were stored differently** in old versions → Mitigation: the `extra` dict is passed through as-is; if absent, defaults to `{}`.

## Migration Plan

1. User runs old Spoolman, exports `/api/v1/export/spools?fmt=json` → `spools_export.json`
2. (Optional) exports `/api/v1/export/filaments?fmt=json` → `filaments_export.json`
3. User upgrades to new Spoolman
4. User runs: `python scripts/convert_export.py spools_export.json --filaments filaments_export.json --output spoolman.json`
5. User places `spoolman.json` at the path configured by `SPOOLMAN_DATA_FILE` (or the default data dir)
6. Starts new Spoolman — data loads from the JSON file
