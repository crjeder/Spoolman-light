"""Smoke test for convert_export.py.

Run with:  python scripts/test_convert_export.py
"""
import json
import os
import sys
import tempfile

# Allow running from any working directory.
sys.path.insert(0, os.path.dirname(__file__))
import convert_export as conv


MINIMAL_SPOOL_EXPORT = [
    {
        "id": 1,
        "registered": "2023-01-01T00:00:00",
        "first_used": None,
        "last_used": None,
        "price": None,
        "initial_weight": None,
        "spool_weight": 200.0,         # should be dropped
        "used_weight": 50.0,           # folded into current_weight
        "location": "shelf",
        "lot_nr": "ABC",               # should be dropped
        "external_id": "EXT-1",        # should be dropped
        "comment": "test spool",
        "archived": False,
        "extra": {"key": "value"},     # should be dropped
        # Filament fields (embedded flat)
        "filament.id": 10,
        "filament.registered": "2022-06-01T00:00:00",
        "filament.name": "PLA Basic",
        "filament.material": "PLA",
        "filament.density": 1.24,
        "filament.diameter": 1.75,
        "filament.weight": 1000.0,         # → spool initial_weight
        "filament.spool_weight": 250.0,    # dropped
        "filament.article_number": "ART1", # dropped
        "filament.external_id": "FIL-1",   # dropped
        "filament.color_hex": "FF0000",    # → spool colors
        "filament.multi_color_hexes": None,
        "filament.multi_color_direction": None,
        "filament.price": 19.99,           # → spool price
        "filament.settings_extruder_temp": 210,
        "filament.settings_bed_temp": 60,
        "filament.comment": None,
        "filament.extra": {},
        "filament.vendor.id": 3,
        "filament.vendor.name": "Prusament",
        "filament.vendor.registered": "2021-01-01T00:00:00",
        "filament.vendor.comment": None,
        "filament.vendor.extra": {},
    }
]

MINIMAL_FILAMENT_EXPORT = [
    {
        # Filament already in spool export — should NOT be duplicated.
        "id": 10,
        "registered": "2022-06-01T00:00:00",
        "name": "PLA Basic",
        "material": "PLA",
        "density": 1.24,
        "diameter": 1.75,
        "vendor.name": "Prusament",
        "settings_extruder_temp": 210,
        "settings_bed_temp": 60,
        "comment": None,
        "extra": {},
    },
    {
        # Orphan filament — only present in the filament export.
        "id": 99,
        "registered": "2023-03-01T00:00:00",
        "name": "PETG",
        "material": "PETG",
        "density": 1.27,
        "diameter": 1.75,
        "vendor.name": None,
        "settings_extruder_temp": 230,
        "settings_bed_temp": 75,
        "comment": None,
        "extra": {},
    },
]


def test_hex_to_rgba():
    """_hex_to_rgba converts hex strings to RGBA dicts."""
    assert conv._hex_to_rgba("FF0000") == {"r": 255, "g": 0, "b": 0, "a": 255}
    assert conv._hex_to_rgba("00ff00") == {"r": 0, "g": 255, "b": 0, "a": 255}
    assert conv._hex_to_rgba("0000FF") == {"r": 0, "g": 0, "b": 255, "a": 255}
    assert conv._hex_to_rgba("aabbcc") == {"r": 170, "g": 187, "b": 204, "a": 255}
    print("  test_hex_to_rgba: PASS")


def test_build_colors_multi():
    """_build_colors uses multi_color_hexes when provided, up to 4 entries."""
    hexes = ["FF0000", "00FF00", "0000FF", "FFFFFF", "000000"]
    colors = conv._build_colors(None, hexes)
    assert len(colors) == 4  # capped at 4
    assert colors[0] == {"r": 255, "g": 0, "b": 0, "a": 255}
    assert colors[3] == {"r": 255, "g": 255, "b": 255, "a": 255}
    print("  test_build_colors_multi: PASS")


def test_material_modifier_derivation():
    """_derive_material_modifier handles all edge cases."""
    # Name starts with material prefix → strip it.
    assert conv._derive_material_modifier("PLA Basic", "PLA") == "Basic"
    assert conv._derive_material_modifier("PETG Galaxy Black", "PETG") == "Galaxy Black"
    # Name equals material exactly → None.
    assert conv._derive_material_modifier("PLA", "PLA") is None
    assert conv._derive_material_modifier("pla", "PLA") is None
    # Name has no material prefix → use full name.
    assert conv._derive_material_modifier("Galaxy Black", "PLA") == "Galaxy Black"
    # Null / empty name → None.
    assert conv._derive_material_modifier(None, "PLA") is None
    assert conv._derive_material_modifier("", "PLA") is None
    # No material → use full name.
    assert conv._derive_material_modifier("Basic", None) == "Basic"
    print("  test_material_modifier_derivation: PASS")


def test_basic_conversion():
    locations, name_to_id_map = conv.collect_locations(MINIMAL_SPOOL_EXPORT)
    filaments = conv.extract_filaments_from_spools(MINIMAL_SPOOL_EXPORT)
    spools = conv.extract_spools(MINIMAL_SPOOL_EXPORT, name_to_id_map)
    store = conv.assemble_store(filaments, spools, locations)

    # Top-level shape.
    assert store["meta"]["schema_version"] == 2
    assert isinstance(store["filaments"], list)
    assert isinstance(store["spools"], list)
    assert isinstance(store["locations"], list)
    assert store["settings"] == {}

    # One filament extracted — correct field names.
    assert len(store["filaments"]) == 1
    fil = store["filaments"][0]
    assert fil["id"] == 10
    assert fil["manufacturer"] == "Prusament"   # not "vendor"
    assert fil["material"] == "PLA"
    assert fil["material_modifier"] == "Basic"
    assert fil["print_temp"] == 210             # not "settings_extruder_temp"
    assert fil["bed_temp"] == 60                # not "settings_bed_temp"
    # Old fields must not appear.
    assert "vendor" not in fil
    assert "name" not in fil
    assert "settings_extruder_temp" not in fil
    assert "settings_bed_temp" not in fil
    assert "extra" not in fil
    assert "article_number" not in fil
    assert "external_id" not in fil
    assert "weight" not in fil
    assert "color_hex" not in fil
    assert "price" not in fil

    # One spool — correct field names and computed values.
    assert len(store["spools"]) == 1
    spool = store["spools"][0]
    assert spool["id"] == 1
    assert spool["filament_id"] == 10
    # colors as RGBA list (from filament.color_hex "FF0000").
    assert spool["colors"] == [{"r": 255, "g": 0, "b": 0, "a": 255}]
    # price pulled from filament.
    assert spool["price"] == 19.99
    # initial_weight from filament.weight.
    assert spool["initial_weight"] == 1000.0
    # current_weight = 1000 - 50.
    assert spool["current_weight"] == 950.0
    # location_id not location string.
    assert "location" not in spool
    assert spool["location_id"] == 1
    # Dropped fields.
    assert "lot_nr" not in spool
    assert "external_id" not in spool
    assert "used_weight" not in spool
    assert "spool_weight" not in spool
    assert "multi_color_direction" not in spool
    assert "extra" not in spool

    # One location collected.
    assert len(store["locations"]) == 1
    assert store["locations"][0] == {"id": 1, "name": "shelf"}

    print("  test_basic_conversion: PASS")


def test_deduplication():
    """Two spool records with the same filament.id → one filament in output."""
    records = MINIMAL_SPOOL_EXPORT + [{**MINIMAL_SPOOL_EXPORT[0], "id": 2}]
    filaments = conv.extract_filaments_from_spools(records)
    assert len(filaments) == 1
    print("  test_deduplication: PASS")


def test_orphan_filaments_from_filament_export():
    """Filaments not in the spool export are added from --filaments file."""
    filaments = conv.extract_filaments_from_spools(MINIMAL_SPOOL_EXPORT)
    for record in MINIMAL_FILAMENT_EXPORT:
        fid = record.get("id")
        if fid is not None and fid not in filaments:
            filaments[fid] = conv._extract_filament_from_filament_export(record)

    # id=10 appears in both; id=99 is orphan — total should be 2.
    assert len(filaments) == 2
    assert 99 in filaments
    assert filaments[99]["material"] == "PETG"
    # Orphan with name == material → modifier is None.
    assert filaments[99]["material_modifier"] is None
    print("  test_orphan_filaments_from_filament_export: PASS")


def test_atomic_write():
    """write_atomic writes valid JSON and removes the .tmp file."""
    with tempfile.TemporaryDirectory() as tmpdir:
        out = os.path.join(tmpdir, "spoolman.json")
        data = {
            "meta": {"schema_version": 2},
            "filaments": [],
            "spools": [],
            "locations": [],
            "settings": {},
        }
        conv.write_atomic(out, data)
        assert os.path.exists(out)
        assert not os.path.exists(out + ".tmp")
        with open(out) as fh:
            loaded = json.load(fh)
        assert loaded["meta"]["schema_version"] == 2
        assert loaded["locations"] == []
    print("  test_atomic_write: PASS")


def test_no_location_produces_empty_locations():
    """When no spool has a location, locations array is empty and location_id is null."""
    records = [{**MINIMAL_SPOOL_EXPORT[0], "location": None}]
    locations, name_to_id_map = conv.collect_locations(records)
    spools = conv.extract_spools(records, name_to_id_map)
    assert locations == []
    assert spools[0]["location_id"] is None
    print("  test_no_location_produces_empty_locations: PASS")


def test_multi_color_hexes():
    """Multi-color spools produce multiple RGBA entries, capped at 4."""
    records = [{
        **MINIMAL_SPOOL_EXPORT[0],
        "filament.color_hex": None,
        "filament.multi_color_hexes": ["FF0000", "00FF00", "0000FF"],
    }]
    _, name_to_id_map = conv.collect_locations(records)
    spools = conv.extract_spools(records, name_to_id_map)
    assert spools[0]["colors"] == [
        {"r": 255, "g": 0, "b": 0, "a": 255},
        {"r": 0, "g": 255, "b": 0, "a": 255},
        {"r": 0, "g": 0, "b": 255, "a": 255},
    ]
    print("  test_multi_color_hexes: PASS")


def test_no_color_produces_empty_colors():
    """Spools with no color produce colors: []."""
    records = [{
        **MINIMAL_SPOOL_EXPORT[0],
        "color_hex": None,
        "filament.color_hex": None,
        "multi_color_hexes": None,
        "filament.multi_color_hexes": None,
    }]
    _, name_to_id_map = conv.collect_locations(records)
    spools = conv.extract_spools(records, name_to_id_map)
    assert spools[0]["colors"] == []
    print("  test_no_color_produces_empty_colors: PASS")


def test_current_weight_computation():
    """current_weight = initial_weight - used_weight."""
    records = [{**MINIMAL_SPOOL_EXPORT[0], "initial_weight": 1200.0, "used_weight": 300.0}]
    _, name_to_id_map = conv.collect_locations(records)
    spools = conv.extract_spools(records, name_to_id_map)
    assert spools[0]["initial_weight"] == 1200.0
    assert spools[0]["current_weight"] == 900.0
    print("  test_current_weight_computation: PASS")


def test_cli_end_to_end():
    """Full CLI round-trip via main()."""
    with tempfile.TemporaryDirectory() as tmpdir:
        spool_path = os.path.join(tmpdir, "spools.json")
        out_path = os.path.join(tmpdir, "spoolman.json")
        with open(spool_path, "w") as fh:
            json.dump(MINIMAL_SPOOL_EXPORT, fh)
        conv.main([spool_path, "--output", out_path])
        with open(out_path) as fh:
            store = json.load(fh)
        assert store["meta"]["schema_version"] == 2
        assert len(store["filaments"]) == 1
        assert len(store["spools"]) == 1
        assert len(store["locations"]) == 1
        assert store["locations"][0]["name"] == "shelf"
    print("  test_cli_end_to_end: PASS")


if __name__ == "__main__":
    print("Running smoke tests for convert_export.py ...")
    test_hex_to_rgba()
    test_build_colors_multi()
    test_material_modifier_derivation()
    test_basic_conversion()
    test_deduplication()
    test_orphan_filaments_from_filament_export()
    test_atomic_write()
    test_no_location_produces_empty_locations()
    test_multi_color_hexes()
    test_no_color_produces_empty_colors()
    test_current_weight_computation()
    test_cli_end_to_end()
    print("All tests passed.")
