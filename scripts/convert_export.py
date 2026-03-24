"""Convert old Spoolman flat-JSON exports to the new spoolman.json storage format.

The old Spoolman stored data in a relational database and exported spools as a
flat list of objects with dot-separated keys embedding filament and vendor data
(e.g. "filament.vendor.name", "filament.color_hex").

The new Spoolman stores data in a single JSON file with this shape::

    {
        "meta": {"schema_version": 2},
        "filaments": [...],
        "spools":    [...],
        "settings":  {}
    }

Usage
-----
Minimal (spool export only)::

    python scripts/convert_export.py spools_export.json --output spoolman.json

With optional filament export (covers filaments that have no associated spools)::

    python scripts/convert_export.py spools_export.json \\
        --filaments filaments_export.json \\
        --output spoolman.json

Then place the output file at the path configured by ``SPOOLMAN_DATA_FILE``
(or the platform default data directory) and start the new Spoolman.

Expected input format
---------------------
``spool_export``  — JSON array produced by the old ``/api/v1/export/spools?fmt=json``
                    endpoint.  Each element is a flat object whose filament and
                    vendor data are embedded as dot-separated keys, e.g.::

                        {
                            "id": 1,
                            "filament.id": 5,
                            "filament.name": "PLA",
                            "filament.vendor.name": "Prusament",
                            "filament.color_hex": "FF0000",
                            ...
                        }

``--filaments``   — (optional) JSON array produced by the old
                    ``/api/v1/export/filaments?fmt=json`` endpoint.  Each element
                    is a flat filament object with the same dot-separated vendor keys.
"""

import argparse
import json
import os
import sys


# ---------------------------------------------------------------------------
# Filament extraction helpers
# ---------------------------------------------------------------------------

def _extract_filament(record: dict) -> dict:
    """Build a new-format filament dict from a flat spool-export record.

    Silently drops removed fields (article_number, external_id, spool_weight,
    weight) and inlines the vendor name as a plain string.
    """
    return {
        "id": record.get("filament.id"),
        "registered": record.get("filament.registered"),
        "name": record.get("filament.name"),
        # Vendor entity removed — inline name as a string (null if absent).
        "vendor": record.get("filament.vendor.name"),
        "material": record.get("filament.material"),
        "density": record.get("filament.density"),
        "diameter": record.get("filament.diameter"),
        "settings_extruder_temp": record.get("filament.settings_extruder_temp"),
        "settings_bed_temp": record.get("filament.settings_bed_temp"),
        "comment": record.get("filament.comment"),
        "extra": record.get("filament.extra") or {},
        # Dropped: filament.weight, filament.spool_weight,
        #          filament.article_number, filament.external_id,
        #          filament.color_hex, filament.multi_color_hexes,
        #          filament.multi_color_direction, filament.price
        #          (those move to spool or are removed entirely)
    }


def _extract_filament_from_filament_export(record: dict) -> dict:
    """Build a new-format filament dict from a flat filament-export record.

    The filament export uses the same key convention as the spool export but
    without the ``filament.`` prefix for top-level fields.
    """
    return {
        "id": record.get("id"),
        "registered": record.get("registered"),
        "name": record.get("name"),
        "vendor": record.get("vendor.name"),
        "material": record.get("material"),
        "density": record.get("density"),
        "diameter": record.get("diameter"),
        "settings_extruder_temp": record.get("settings_extruder_temp"),
        "settings_bed_temp": record.get("settings_bed_temp"),
        "comment": record.get("comment"),
        "extra": record.get("extra") or {},
    }


def extract_filaments_from_spools(spool_records: list) -> dict:
    """Return a dict of {filament_id: filament_dict} deduplicated by filament.id.

    The first occurrence wins when multiple spool records share the same
    filament.id (minor field drift between records is harmless).
    """
    filaments: dict = {}
    for record in spool_records:
        fid = record.get("filament.id")
        if fid is not None and fid not in filaments:
            filaments[fid] = _extract_filament(record)
    return filaments


# ---------------------------------------------------------------------------
# Spool extraction helpers
# ---------------------------------------------------------------------------

def extract_spools(spool_records: list) -> list:
    """Convert old flat spool records to new-format spool dicts.

    Field changes applied:
    - color_hex / multi_color_hexes / multi_color_direction: copied from
      ``filament.*`` when the spool's own value is null/absent.
    - price: copied from ``filament.price`` when the spool price is null/absent.
    - initial_weight: populated from ``filament.weight`` when null/absent.
    - lot_nr, external_id: silently dropped.
    """
    spools = []
    for record in spool_records:
        # Color fields moved from filament to spool; fall through when null.
        color_hex = record.get("color_hex") or record.get("filament.color_hex")
        multi_color_hexes = (
            record.get("multi_color_hexes") or record.get("filament.multi_color_hexes")
        )
        multi_color_direction = (
            record.get("multi_color_direction")
            or record.get("filament.multi_color_direction")
        )

        # price moved from filament to spool.
        price = record.get("price")
        if price is None:
            price = record.get("filament.price")

        # initial_weight: use spool value when present, else filament.weight.
        initial_weight = record.get("initial_weight")
        if initial_weight is None:
            initial_weight = record.get("filament.weight")

        spools.append({
            "id": record.get("id"),
            "registered": record.get("registered"),
            "first_used": record.get("first_used"),
            "last_used": record.get("last_used"),
            "filament_id": record.get("filament.id"),
            "used_weight": record.get("used_weight"),
            "initial_weight": initial_weight,
            "spool_weight": record.get("spool_weight"),
            "location": record.get("location"),
            "comment": record.get("comment"),
            "archived": record.get("archived", False),
            "extra": record.get("extra") or {},
            "color_hex": color_hex,
            "multi_color_hexes": multi_color_hexes,
            "multi_color_direction": multi_color_direction,
            "price": price,
            # Dropped: lot_nr, external_id
        })
    return spools


# ---------------------------------------------------------------------------
# Output assembly
# ---------------------------------------------------------------------------

def assemble_store(filaments: dict, spools: list) -> dict:
    """Assemble the final spoolman.json structure."""
    return {
        "meta": {"schema_version": 2},
        "filaments": list(filaments.values()),
        "spools": spools,
        "settings": {},
    }


def write_atomic(path: str, data: dict) -> None:
    """Write *data* as JSON to *path* atomically via a .tmp intermediate."""
    tmp_path = path + ".tmp"
    with open(tmp_path, "w", encoding="utf-8") as fh:
        json.dump(data, fh, indent=2, ensure_ascii=False)
    os.replace(tmp_path, path)


# ---------------------------------------------------------------------------
# CLI
# ---------------------------------------------------------------------------

def build_parser() -> argparse.ArgumentParser:
    parser = argparse.ArgumentParser(
        prog="convert_export.py",
        description=__doc__,
        formatter_class=argparse.RawDescriptionHelpFormatter,
    )
    parser.add_argument(
        "spool_export",
        help="Path to the old spool export JSON (from /api/v1/export/spools?fmt=json).",
    )
    parser.add_argument(
        "--filaments",
        metavar="FILE",
        help=(
            "Optional path to the old filament export JSON "
            "(from /api/v1/export/filaments?fmt=json).  "
            "Filaments not already present in the spool export are added to the output."
        ),
    )
    parser.add_argument(
        "--output",
        metavar="FILE",
        default="spoolman.json",
        help="Output path for the converted spoolman.json (default: %(default)s).",
    )
    return parser


def main(argv=None) -> None:
    parser = build_parser()
    args = parser.parse_args(argv)

    # Load spool export.
    with open(args.spool_export, encoding="utf-8") as fh:
        spool_records: list = json.load(fh)

    # Extract filaments deduplicated from spool records.
    filaments = extract_filaments_from_spools(spool_records)

    # Extract spools.
    spools = extract_spools(spool_records)

    # Merge optional filament export (adds filaments with no spools).
    if args.filaments:
        with open(args.filaments, encoding="utf-8") as fh:
            filament_records: list = json.load(fh)
        for record in filament_records:
            fid = record.get("id")
            if fid is not None and fid not in filaments:
                filaments[fid] = _extract_filament_from_filament_export(record)

    # Assemble and write.
    store = assemble_store(filaments, spools)
    write_atomic(args.output, store)

    print(
        f"Wrote {len(filaments)} filament(s) and {len(spools)} spool(s) "
        f"to {args.output}"
    )


if __name__ == "__main__":
    main()
