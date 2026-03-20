"""Integration tests for the Filament API endpoint."""

from typing import Any

import httpx

from ..conftest import URL, assert_dicts_compatible


def test_update_filament(random_filament: dict[str, Any]):
    """Test updating a filament in the database."""
    new_name = "Updated Filament"
    new_vendor = "Polymaker"
    new_material = "PETG"
    new_density = 1.27
    new_extruder_temp = 230
    new_comment = "updated comment"

    result = httpx.patch(
        f"{URL}/api/v1/filament/{random_filament['id']}",
        json={
            "name": new_name,
            "vendor": new_vendor,
            "material": new_material,
            "density": new_density,
            "settings_extruder_temp": new_extruder_temp,
            "comment": new_comment,
        },
    )
    result.raise_for_status()

    filament = result.json()
    assert_dicts_compatible(
        filament,
        {
            "id": random_filament["id"],
            "name": new_name,
            "vendor": new_vendor,
            "material": new_material,
            "density": new_density,
            "settings_extruder_temp": new_extruder_temp,
            "comment": new_comment,
        },
    )


def test_update_filament_not_found():
    """Test updating a filament that does not exist."""
    result = httpx.patch(
        f"{URL}/api/v1/filament/123456789",
        json={"name": "test"},
    )
    assert result.status_code == 404


def test_update_filament_density_none(random_filament: dict[str, Any]):
    """Test that density cannot be set to None."""
    result = httpx.patch(
        f"{URL}/api/v1/filament/{random_filament['id']}",
        json={"density": None},
    )
    assert result.status_code == 422


def test_update_filament_diameter_none(random_filament: dict[str, Any]):
    """Test that diameter cannot be set to None."""
    result = httpx.patch(
        f"{URL}/api/v1/filament/{random_filament['id']}",
        json={"diameter": None},
    )
    assert result.status_code == 422
