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
        "spool_weight": 200.0,
        "used_weight": 50.0,
        "location": "shelf",
        "lot_nr": "ABC",           # should be dropped
        "external_id": "EXT-1",    # should be dropped
        "comment": "test spool",
        "archived": False,
        "extra": {"key": "value"},
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
        "filament.color_hex": "FF0000",    # → spool color_hex
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


def test_basic_conversion():
    filaments = conv.extract_filaments_from_spools(MINIMAL_SPOOL_EXPORT)
    spools = conv.extract_spools(MINIMAL_SPOOL_EXPORT)
    store = conv.assemble_store(filaments, spools)

    # Top-level shape.
    assert store["meta"]["schema_version"] == 2
    assert isinstance(store["filaments"], list)
    assert isinstance(store["spools"], list)
    assert store["settings"] == {}

    # One filament extracted.
    assert len(store["filaments"]) == 1
    fil = store["filaments"][0]
    assert fil["id"] == 10
    assert fil["vendor"] == "Prusament"
    assert fil["material"] == "PLA"
    # Dropped fields must not appear.
    assert "article_number" not in fil
    assert "external_id" not in fil
    assert "weight" not in fil
    assert "spool_weight" not in fil
    assert "color_hex" not in fil
    assert "price" not in fil

    # One spool.
    assert len(store["spools"]) == 1
    spool = store["spools"][0]
    assert spool["id"] == 1
    assert spool["filament_id"] == 10
    # color_hex, price, initial_weight pulled from filament.
    assert spool["color_hex"] == "FF0000"
    assert spool["price"] == 19.99
    assert spool["initial_weight"] == 1000.0
    # Dropped fields must not appear.
    assert "lot_nr" not in spool
    assert "external_id" not in spool
    # extra round-trips.
    assert spool["extra"] == {"key": "value"}

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
    print("  test_orphan_filaments_from_filament_export: PASS")


def test_atomic_write():
    """write_atomic writes valid JSON and removes the .tmp file."""
    with tempfile.TemporaryDirectory() as tmpdir:
        out = os.path.join(tmpdir, "spoolman.json")
        data = {"meta": {"schema_version": 2}, "filaments": [], "spools": [], "settings": {}}
        conv.write_atomic(out, data)
        assert os.path.exists(out)
        assert not os.path.exists(out + ".tmp")
        with open(out) as fh:
            loaded = json.load(fh)
        assert loaded["meta"]["schema_version"] == 2
    print("  test_atomic_write: PASS")


def test_missing_extra_defaults_to_empty_dict():
    """Records without an 'extra' key get extra={}."""
    record = {**MINIMAL_SPOOL_EXPORT[0]}
    del record["extra"]
    del record["filament.extra"]
    spools = conv.extract_spools([record])
    assert spools[0]["extra"] == {}
    filaments = conv.extract_filaments_from_spools([record])
    assert filaments[10]["extra"] == {}
    print("  test_missing_extra_defaults_to_empty_dict: PASS")


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
    print("  test_cli_end_to_end: PASS")


if __name__ == "__main__":
    print("Running smoke tests for convert_export.py ...")
    test_basic_conversion()
    test_deduplication()
    test_orphan_filaments_from_filament_export()
    test_atomic_write()
    test_missing_extra_defaults_to_empty_dict()
    test_cli_end_to_end()
    print("All tests passed.")
