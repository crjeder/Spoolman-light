"""Integration tests for the Filament API endpoint."""

from datetime import datetime, timezone

import httpx

from ..conftest import URL, assert_dicts_compatible


def test_add_filament():
    """Test adding a filament to the database."""
    manufacturer = "eSun"
    material = "PLA"
    density = 1.24
    diameter = 1.75
    print_temp = 210
    bed_temp = 60
    comment = "abcdefghåäö"

    result = httpx.post(
        f"{URL}/api/v1/filament",
        json={
            "manufacturer": manufacturer,
            "material": material,
            "density": density,
            "diameter": diameter,
            "print_temp": print_temp,
            "bed_temp": bed_temp,
            "comment": comment,
        },
    )
    assert result.status_code == 201

    filament = result.json()
    assert_dicts_compatible(
        filament,
        {
            "id": filament["id"],
            "registered": filament["registered"],
            "manufacturer": manufacturer,
            "material": material,
            "density": density,
            "diameter": diameter,
            "print_temp": print_temp,
            "bed_temp": bed_temp,
            "comment": comment,
        },
    )
    assert filament["net_weight"] is None
    assert filament["material_modifier"] is None

    diff = abs((datetime.now(tz=timezone.utc) - datetime.fromisoformat(filament["registered"])).total_seconds())
    assert diff < 60

    httpx.delete(f"{URL}/api/v1/filament/{filament['id']}").raise_for_status()


def test_add_filament_required_only():
    """Test adding a filament with only the required fields."""
    density = 1.24
    diameter = 1.75

    result = httpx.post(
        f"{URL}/api/v1/filament",
        json={
            "density": density,
            "diameter": diameter,
        },
    )
    assert result.status_code == 201

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
    assert filament["manufacturer"] is None
    assert filament["material"] is None
    assert filament["net_weight"] is None
    assert filament["print_temp"] is None
    assert filament["bed_temp"] is None
    assert filament["comment"] is None

    httpx.delete(f"{URL}/api/v1/filament/{filament['id']}").raise_for_status()
