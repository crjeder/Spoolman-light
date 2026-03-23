"""Integration tests for the Filament API endpoint."""

from typing import Any

import httpx

from ..conftest import URL, assert_dicts_compatible


def test_update_filament(random_filament: dict[str, Any]):
    """Test updating a filament in the database."""
    result = httpx.patch(
        f"{URL}/api/v1/filament/{random_filament['id']}",
        json={
            "material": "PETG",
            "density": 1.27,
            "print_temp": 230,
            "comment": "updated comment",
            "net_weight": 1000.0,
        },
    )
    result.raise_for_status()

    filament = result.json()
    assert_dicts_compatible(
        filament,
        {
            "id": random_filament["id"],
            "material": "PETG",
            "density": 1.27,
            "print_temp": 230,
            "comment": "updated comment",
            "net_weight": 1000.0,
            # Fields not patched should be unchanged
            "manufacturer": random_filament["manufacturer"],
            "diameter": random_filament["diameter"],
        },
    )


def test_update_filament_not_found():
    """Test updating a filament that does not exist."""
    result = httpx.patch(
        f"{URL}/api/v1/filament/123456789",
        json={"material": "PLA"},
    )
    assert result.status_code == 404


def test_update_filament_partial(random_filament: dict[str, Any]):
    """Test that a PATCH with a single field only changes that field."""
    result = httpx.patch(
        f"{URL}/api/v1/filament/{random_filament['id']}",
        json={"material": "ASA"},
    )
    result.raise_for_status()

    filament = result.json()
    assert filament["material"] == "ASA"
    assert filament["density"] == random_filament["density"]
    assert filament["diameter"] == random_filament["diameter"]
    assert filament["manufacturer"] == random_filament["manufacturer"]
