"""Integration tests for the Filament API endpoint."""

from datetime import datetime, timezone

import httpx

from ..conftest import URL, assert_dicts_compatible


def test_add_filament():
    """Test adding a filament to the database."""
    name = "Filament X"
    vendor = "eSun"
    material = "PLA"
    density = 1.25
    diameter = 1.75
    settings_extruder_temp = 200
    settings_bed_temp = 60
    comment = "abcdefghåäö"
    result = httpx.post(
        f"{URL}/api/v1/filament",
        json={
            "name": name,
            "vendor": vendor,
            "material": material,
            "density": density,
            "diameter": diameter,
            "settings_extruder_temp": settings_extruder_temp,
            "settings_bed_temp": settings_bed_temp,
            "comment": comment,
        },
    )
    result.raise_for_status()

    filament = result.json()
    assert_dicts_compatible(
        filament,
        {
            "id": filament["id"],
            "registered": filament["registered"],
            "name": name,
            "vendor": vendor,
            "material": material,
            "density": density,
            "diameter": diameter,
            "settings_extruder_temp": settings_extruder_temp,
            "settings_bed_temp": settings_bed_temp,
            "comment": comment,
        },
    )

    diff = abs((datetime.now(tz=timezone.utc) - datetime.fromisoformat(filament["registered"])).total_seconds())
    assert diff < 60

    httpx.delete(f"{URL}/api/v1/filament/{filament['id']}").raise_for_status()


def test_add_filament_required():
    """Test adding a filament with only the required fields to the database."""
    density = 1.25
    diameter = 1.75
    result = httpx.post(
        f"{URL}/api/v1/filament",
        json={
            "density": density,
            "diameter": diameter,
        },
    )
    result.raise_for_status()

    filament = result.json()
    assert_dicts_compatible(
        filament,
        {
            "id": filament["id"],
            "registered": filament["registered"],
            "density": density,
            "diameter": diameter,
        },
    )

    httpx.delete(f"{URL}/api/v1/filament/{filament['id']}").raise_for_status()
