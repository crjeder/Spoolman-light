"""Integration tests for the Filament API endpoint."""

import httpx

from ..conftest import URL, assert_dicts_compatible


def test_get_filament():
    """Test getting a filament from the database."""
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
    added_filament = result.json()

    result = httpx.get(f"{URL}/api/v1/filament/{added_filament['id']}")
    result.raise_for_status()

    filament = result.json()
    assert_dicts_compatible(
        filament,
        {
            "id": added_filament["id"],
            "registered": added_filament["registered"],
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

    httpx.delete(f"{URL}/api/v1/filament/{filament['id']}").raise_for_status()


def test_get_filament_not_found():
    """Test getting a filament that does not exist."""
    result = httpx.get(f"{URL}/api/v1/filament/123456789")

    assert result.status_code == 404
    message = result.json()["message"].lower()
    assert "filament" in message
    assert "id" in message
    assert "123456789" in message
