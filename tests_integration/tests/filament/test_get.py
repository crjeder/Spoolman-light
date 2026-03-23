"""Integration tests for the Filament API endpoint."""

import httpx

from ..conftest import URL, assert_dicts_compatible


def test_get_filament():
    """Test getting a filament from the database."""
    result = httpx.post(
        f"{URL}/api/v1/filament",
        json={
            "manufacturer": "eSun",
            "material": "PLA",
            "density": 1.24,
            "diameter": 1.75,
            "print_temp": 210,
            "bed_temp": 60,
            "comment": "abcdefghåäö",
        },
    )
    result.raise_for_status()
    added = result.json()

    result = httpx.get(f"{URL}/api/v1/filament/{added['id']}")
    result.raise_for_status()

    filament = result.json()
    assert_dicts_compatible(
        filament,
        {
            "id": added["id"],
            "registered": added["registered"],
            "manufacturer": "eSun",
            "material": "PLA",
            "density": 1.24,
            "diameter": 1.75,
            "print_temp": 210,
            "bed_temp": 60,
            "comment": "abcdefghåäö",
        },
    )

    httpx.delete(f"{URL}/api/v1/filament/{filament['id']}").raise_for_status()


def test_get_filament_not_found():
    """Test getting a filament that does not exist."""
    result = httpx.get(f"{URL}/api/v1/filament/123456789")
    assert result.status_code == 404
