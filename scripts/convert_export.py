"""Convert old Spoolman flat-JSON exports to the new spoolman.json storage format.

The old Spoolman stored data in a relational database and exported spools as a
flat list of objects with dot-separated keys embedding filament and vendor data
(e.g. "filament.vendor.name", "filament.color_hex").

The new Spoolman stores data in a single JSON file with this shape::

    {
        "meta": {"schema_version": 2},
        "filaments": [...],
        "spools":    [...],
        "locations": [...],
        "settings":  {}
    }

Usage
-----
From a running old Spoolman instance (fetches both exports automatically)::

    python scripts/convert_export.py --api-url http://localhost:7912 --output spoolman.json

From local export files (spool export only)::

    python scripts/convert_export.py spools_export.json --output spoolman.json

From local export files with optional filament export::

    python scripts/convert_export.py spools_export.json \\
        --filaments filaments_export.json \\
        --output spoolman.json

Then place the output file at the path configured by ``SPOOLMAN_DATA_FILE``
(or the platform default data directory) and start the new Spoolman.

Input sources
-------------
``--api-url URL``  — Base URL of the old Spoolman instance (e.g. ``http://localhost:7912``).
                     Fetches ``/api/v1/export/spools?fmt=json`` and
                     ``/api/v1/export/filaments?fmt=json`` automatically.
                     Mutually exclusive with ``spool_export``.

``spool_export``   — Path to a local spool export JSON
                     (from ``/api/v1/export/spools?fmt=json``).  Each element is
                     a flat object whose filament and vendor data are embedded as
                     dot-separated keys, e.g.::

                         {
                             "id": 1,
                             "filament.id": 5,
                             "filament.name": "PLA Basic",
                             "filament.vendor.name": "Prusament",
                             "filament.color_hex": "FF0000",
                             ...
                         }

``--filaments FILE`` — (optional, file mode only) Path to the old filament export
                       JSON (from ``/api/v1/export/filaments?fmt=json``).  Filaments
                       not already present in the spool export are added to the output.
                       Not needed with ``--api-url`` (fetched automatically).
"""

import argparse
import json
import os
import random
import sys
import urllib.error
import urllib.parse
import urllib.request


# ---------------------------------------------------------------------------
# HTTP helpers
# ---------------------------------------------------------------------------

def fetch_json(url: str) -> list | dict:
    """Fetch JSON from *url* and return the parsed object.

    Only ``http://`` and ``https://`` schemes are permitted; ``file://`` and
    other schemes are rejected to prevent unintended local file reads.
    Uses only the standard library (``urllib``).  Raises ``SystemExit`` with a
    human-readable message on scheme, HTTP, or network errors.
    """
    parsed = urllib.parse.urlparse(url)
    if parsed.scheme not in ("http", "https"):
        sys.exit(
            f"Unsupported URL scheme '{parsed.scheme}': only http and https are allowed."
        )
    print(f"  Fetching {url} …", file=sys.stderr)
    req = urllib.request.Request(url, headers={"Accept": "application/json"})
    try:
        with urllib.request.urlopen(req, timeout=30) as resp:  # nosemgrep
            body = resp.read()
    except urllib.error.HTTPError as exc:
        sys.exit(f"HTTP {exc.code} fetching {url}: {exc.reason}")
    except urllib.error.URLError as exc:
        sys.exit(f"Could not connect to {url}: {exc.reason}")
    return json.loads(body)


# ---------------------------------------------------------------------------
# Datetime helpers
# ---------------------------------------------------------------------------

def _to_rfc3339(value: str | None) -> str | None:
    """Normalise an old-Spoolman datetime string to RFC 3339 / ISO 8601.

    Old Spoolman stores datetimes as ``"YYYY-MM-DD HH:MM:SS"`` (space
    separator, no timezone).  ``chrono::DateTime<Utc>`` rejects that format;
    it needs ``"YYYY-MM-DDTHH:MM:SSZ"``.  Strings already containing ``T``
    or a ``+`` offset are returned unchanged.
    """
    if not value:
        return value
    s = value.strip()
    # Already RFC 3339 / ISO 8601 (has T separator or explicit offset/Z)
    if "T" in s or "+" in s or s.endswith("Z"):
        return s
    # "YYYY-MM-DD HH:MM:SS[.ffffff]" → "YYYY-MM-DDTHH:MM:SS[.ffffff]Z"
    if " " in s:
        s = s.replace(" ", "T", 1)
        if not s.endswith("Z"):
            s += "Z"
        return s
    return value


# ---------------------------------------------------------------------------
# Color helpers
# ---------------------------------------------------------------------------

def _hex_to_rgba(hex_str: str) -> dict | None:
    """Convert a hex color string (with or without '#') to an RGBA dict with alpha=255.

    Accepts 6-character (#RRGGBB) and 3-character (#RGB) forms.
    Returns ``None`` for strings that cannot be parsed so callers can skip them.
    """
    s = hex_str.lstrip("#").strip()
    if len(s) == 3:
        s = s[0]*2 + s[1]*2 + s[2]*2  # expand shorthand
    if len(s) != 6:
        return None
    try:
        r = int(s[0:2], 16)
        g = int(s[2:4], 16)
        b = int(s[4:6], 16)
    except ValueError:
        return None
    return {"r": r, "g": g, "b": b, "a": 255}


def _build_colors(color_hex, multi_color_hexes) -> list:
    """Return a ``colors`` list from old hex-string color fields.

    Priority: multi_color_hexes (up to 4 entries) → color_hex → empty list.
    """
    if multi_color_hexes:
        return [c for h in multi_color_hexes[:4] if h for c in [_hex_to_rgba(h)] if c]
    if color_hex:
        rgba = _hex_to_rgba(color_hex)
        return [rgba] if rgba else []
    return []


# ---------------------------------------------------------------------------
# Material modifier helper
# ---------------------------------------------------------------------------

def _derive_material_modifier(name: str | None, material: str | None) -> str | None:
    """Derive ``material_modifier`` from the old free-text filament name.

    Strips a leading material-type prefix (case-insensitive) and surrounding
    whitespace.  Returns ``None`` when the name is absent or equal to the
    material string alone.
    """
    if not name:
        return None
    name = name.strip()
    if not name:
        return None
    if material:
        mat = material.strip()
        if name.lower() == mat.lower():
            return None
        if name.lower().startswith(mat.lower()):
            remainder = name[len(mat):].strip()
            return remainder if remainder else None
    return name


# ---------------------------------------------------------------------------
# Filament extraction helpers
# ---------------------------------------------------------------------------

def _extract_filament(record: dict) -> dict:
    """Build a new-format filament dict from a flat spool-export record.

    Field mapping applied:
    - filament.vendor.name  → manufacturer
    - filament.settings_extruder_temp → print_temp
    - filament.settings_bed_temp      → bed_temp
    - filament.name + filament.material → material_modifier (derived)
    Dropped: name, extra, article_number, external_id, spool_weight, weight,
             color_hex, multi_color_hexes, multi_color_direction, price.
    """
    name = record.get("filament.name")
    material = record.get("filament.material")
    return {
        "id": record.get("filament.id"),
        "registered": _to_rfc3339(record.get("filament.registered")),
        "manufacturer": record.get("filament.vendor.name"),
        "material": material,
        "material_modifier": _derive_material_modifier(name, material),
        "density": record.get("filament.density"),
        "diameter": record.get("filament.diameter"),
        "print_temp": record.get("filament.settings_extruder_temp"),
        "bed_temp": record.get("filament.settings_bed_temp"),
        "comment": record.get("filament.comment"),
    }


def _extract_filament_from_filament_export(record: dict) -> dict:
    """Build a new-format filament dict from a flat filament-export record.

    The filament export uses the same key convention as the spool export but
    without the ``filament.`` prefix for top-level fields.
    """
    name = record.get("name")
    material = record.get("material")
    return {
        "id": record.get("id"),
        "registered": _to_rfc3339(record.get("registered")),
        "manufacturer": record.get("vendor.name"),
        "material": material,
        "material_modifier": _derive_material_modifier(name, material),
        "density": record.get("density"),
        "diameter": record.get("diameter"),
        "print_temp": record.get("settings_extruder_temp"),
        "bed_temp": record.get("settings_bed_temp"),
        "comment": record.get("comment"),
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
# Nested API helpers (for archived spools from /api/v1/spool?allow_archived=true)
# ---------------------------------------------------------------------------

def _extract_filament_from_api(fil: dict) -> dict:
    """Build a new-format filament dict from a nested /api/v1/spool filament object."""
    name = fil.get("name")
    material = fil.get("material")
    return {
        "id": fil.get("id"),
        "registered": fil.get("registered"),  # already RFC 3339
        "manufacturer": (fil.get("vendor") or {}).get("name"),
        "material": material,
        "material_modifier": _derive_material_modifier(name, material),
        "density": fil.get("density"),
        "diameter": fil.get("diameter"),
        "print_temp": fil.get("settings_extruder_temp"),
        "bed_temp": fil.get("settings_bed_temp"),
        "comment": fil.get("comment"),
    }


def _extract_spool_from_api(record: dict, name_to_id_map: dict) -> dict:
    """Build a new-format spool dict from a nested /api/v1/spool record.

    Used for archived spools that the flat export endpoint omits.
    Dates are already RFC 3339.  remaining_weight is available directly.
    """
    fil = record.get("filament") or {}
    color_hex = record.get("color_hex") or fil.get("color_hex")
    multi_color_hexes = record.get("multi_color_hexes") or fil.get("multi_color_hexes")
    colors = _build_colors(color_hex, multi_color_hexes)

    net_weight = record.get("initial_weight") or fil.get("weight")
    spool_body = record.get("spool_weight") or fil.get("spool_weight") or 0
    initial_weight = (net_weight or 0) + spool_body

    used_weight = record.get("used_weight") or 0
    remaining = (net_weight or 0) - used_weight
    current_weight = remaining + spool_body

    loc_name = record.get("location")
    location_id = name_to_id_map.get(loc_name) if loc_name else None

    return {
        "id": record["id"],
        "registered": record.get("registered"),
        "first_used": record.get("first_used"),
        "last_used": record.get("last_used"),
        "filament_id": fil.get("id"),
        "location_id": location_id,
        "colors": colors,
        "color_name": record.get("color_name"),
        "initial_weight": initial_weight,
        "current_weight": current_weight,
        "net_weight": net_weight,
        "price": record.get("price"),
        "comment": record.get("comment"),
        "archived": record.get("archived", True),
    }


# ---------------------------------------------------------------------------
# Location collection helpers
# ---------------------------------------------------------------------------

def collect_locations(spool_records: list) -> tuple[list, dict]:
    """Collect unique non-null location strings and assign synthetic IDs.

    Returns:
        (locations_list, name_to_id_map) where locations_list is a list of
        ``{id, name}`` dicts and name_to_id_map maps each location name to
        its assigned integer ID.
    """
    seen: dict = {}  # name → id
    next_id = 1
    for record in spool_records:
        loc = record.get("location")
        if loc and loc not in seen:
            seen[loc] = next_id
            next_id += 1
    locations = [{"id": id_, "name": name} for name, id_ in seen.items()]
    return locations, seen


# ---------------------------------------------------------------------------
# Spool extraction helpers
# ---------------------------------------------------------------------------

def extract_spools(spool_records: list, name_to_id_map: dict) -> list:
    """Convert old flat spool records to new-format spool dicts.

    Field changes applied:
    - color_hex / multi_color_hexes → colors (Vec<Rgba> objects)
    - location (string) → location_id (int reference)
    - initial_weight / filament.weight → net_weight (net filament weight)
    - spool_weight (per-spool, fallback filament.spool_weight) → used to build
      gross readings: initial_weight = net_weight + spool_body,
                      current_weight = (net_weight - used_weight) + spool_body
    - price: copied from filament.price when the spool price is null/absent
    Dropped: used_weight, spool_weight, multi_color_direction, extra,
             lot_nr, external_id.
    """
    spools = []
    for record in spool_records:
        # Color fields moved from filament to spool; fall through when null.
        color_hex = record.get("color_hex") or record.get("filament.color_hex")
        multi_color_hexes = (
            record.get("multi_color_hexes") or record.get("filament.multi_color_hexes")
        )
        colors = _build_colors(color_hex, multi_color_hexes)

        # price moved from filament to spool.
        price = record.get("price")
        if price is None:
            price = record.get("filament.price")

        # net_weight: the old Spoolman's initial_weight / filament.weight is the
        # net filament weight (no spool body tare).
        # spool_weight: per-spool empty-spool body weight (falls back to
        # filament.spool_weight when not overridden at the spool level).
        # initial_weight in the new format is the gross scale reading:
        #   net_weight + spool_body.
        net_weight = record.get("initial_weight") or record.get("filament.weight")
        spool_body = record.get("spool_weight") or record.get("filament.spool_weight") or 0
        initial_weight = (net_weight or 0) + spool_body

        # current_weight (new format) = gross scale reading = remaining + spool_body.
        # Derive remaining first: prefer used_weight arithmetic (most accurate
        # from old Spoolman), fall back to remaining_weight field if present.
        used_weight = record.get("used_weight") or 0
        remaining = (net_weight or 0) - used_weight
        current_weight = remaining + spool_body

        # location string → location_id reference.
        loc_name = record.get("location")
        location_id = name_to_id_map.get(loc_name) if loc_name else None

        spools.append({
            "id": record.get("id"),
            "registered": _to_rfc3339(record.get("registered")),
            "first_used": _to_rfc3339(record.get("first_used")),
            "last_used": _to_rfc3339(record.get("last_used")),
            "filament_id": record.get("filament.id"),
            "location_id": location_id,
            "colors": colors,
            "color_name": record.get("color_name"),
            "initial_weight": initial_weight,
            "current_weight": current_weight,
            "net_weight": net_weight,
            "price": price,
            "comment": record.get("comment"),
            "archived": record.get("archived", False),
            # Dropped: used_weight, spool_weight, multi_color_direction,
            #          extra, lot_nr, external_id
        })
    return spools


# ---------------------------------------------------------------------------
# Output assembly
# ---------------------------------------------------------------------------

def _new_id(used: set) -> int:
    """Return a pseudorandom u32 in 1..=2³²-1 not already in *used*.

    Mirrors the Rust implementation: ``rng.random_range(1..=u32::MAX)`` with a
    collision-check loop.  Uses Python's Mersenne Twister (``random.randint``)
    to match the pseudorandom (non-cryptographic) intent of the spec.
    """
    while True:
        v = random.randint(1, 0xFFFF_FFFF)
        if v not in used:
            used.add(v)
            return v


def randomize_ids(store: dict) -> dict:
    """Replace all sequential IDs with pseudorandom u32 values.

    Remaps filament IDs, location IDs, and spool IDs, updating every
    cross-reference (spool.filament_id, spool.location_id) in place.
    Returns the mutated store dict.
    """
    used: set = set()

    fil_map: dict = {}
    for fil in store["filaments"]:
        new = _new_id(used)
        fil_map[fil["id"]] = new
        fil["id"] = new

    loc_map: dict = {}
    for loc in store["locations"]:
        new = _new_id(used)
        loc_map[loc["id"]] = new
        loc["id"] = new

    for spool in store["spools"]:
        spool["id"] = _new_id(used)
        fid = spool.get("filament_id")
        if fid is not None:
            spool["filament_id"] = fil_map.get(fid, fid)
        lid = spool.get("location_id")
        if lid is not None:
            spool["location_id"] = loc_map.get(lid, lid)

    return store


def assemble_store(filaments: dict, spools: list, locations: list) -> dict:
    """Assemble the final spoolman.json structure."""
    return {
        "meta": {"schema_version": 2},
        "filaments": list(filaments.values()),
        "spools": spools,
        "locations": locations,
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
    source = parser.add_mutually_exclusive_group(required=True)
    source.add_argument(
        "spool_export",
        nargs="?",
        help="Path to the old spool export JSON (from /api/v1/export/spools?fmt=json).",
    )
    source.add_argument(
        "--api-url",
        metavar="URL",
        help=(
            "Base URL of the old Spoolman instance "
            "(e.g. http://localhost:7912).  "
            "Fetches /api/v1/export/spools?fmt=json and "
            "/api/v1/export/filaments?fmt=json automatically."
        ),
    )
    parser.add_argument(
        "--filaments",
        metavar="FILE",
        help=(
            "Optional path to the old filament export JSON "
            "(from /api/v1/export/filaments?fmt=json).  "
            "Only used in file mode; ignored when --api-url is given."
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

    filament_records: list = []

    archived_api_records: list = []

    if args.api_url:
        # --- API mode: fetch both exports from the running instance -----------
        base = args.api_url.rstrip("/")
        spool_records = fetch_json(f"{base}/api/v1/export/spools?fmt=json")
        filament_records = fetch_json(f"{base}/api/v1/export/filaments?fmt=json")
        # The export endpoint silently drops archived spools; fetch them
        # separately from the spool API (nested format).
        all_api_spools = fetch_json(f"{base}/api/v1/spool?allow_archived=true")
        archived_api_records = [s for s in all_api_spools if s.get("archived")]
        print(
            f"  Found {len(archived_api_records)} archived spool(s) via API "
            f"(not included in export).",
            file=__import__("sys").stderr,
        )
    else:
        # --- File mode: load from local JSON files ----------------------------
        with open(args.spool_export, encoding="utf-8") as fh:
            spool_records = json.load(fh)
        if args.filaments:
            with open(args.filaments, encoding="utf-8") as fh:
                filament_records = json.load(fh)

    # Extract filaments deduplicated from spool records.
    filaments = extract_filaments_from_spools(spool_records)

    # Collect locations from export spools; also seed from archived API records
    # so their location strings get assigned IDs.
    archived_loc_seeds = [{"location": r.get("location")} for r in archived_api_records]
    locations, name_to_id_map = collect_locations(spool_records + archived_loc_seeds)

    # Extract active spools (needs location map).
    spools = extract_spools(spool_records, name_to_id_map)

    # Append archived spools (nested API format); deduplicate by id.
    existing_ids = {s["id"] for s in spools}
    for record in archived_api_records:
        if record["id"] not in existing_ids:
            spools.append(_extract_spool_from_api(record, name_to_id_map))
            # Ensure the archived spool's filament is in the filaments dict.
            fil = record.get("filament") or {}
            fid = fil.get("id")
            if fid is not None and fid not in filaments:
                filaments[fid] = _extract_filament_from_api(fil)

    # Merge filament export (adds filaments with no spools).
    for record in filament_records:
        fid = record.get("id")
        if fid is not None and fid not in filaments:
            filaments[fid] = _extract_filament_from_filament_export(record)

    # Assemble, assign pseudorandom IDs, and write.
    store = assemble_store(filaments, spools, locations)
    randomize_ids(store)
    write_atomic(args.output, store)

    print(
        f"Wrote {len(filaments)} filament(s), {len(spools)} spool(s), "
        f"and {len(locations)} location(s) to {args.output}"
    )


if __name__ == "__main__":
    main()
