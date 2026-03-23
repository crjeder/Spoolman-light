"""Integration tests for the Filament API endpoint."""

import httpx

from ..conftest import URL


def test_delete_filament():
    """Test deleting a filament from the database."""
    result = httpx.post(
        f"{URL}/api/v1/filament",
        json={
            "manufacturer": "eSun",
            "material": "PLA",
            "density": 1.24,
            "diameter": 1.75,
        },
    )
    result.raise_for_status()
    added = result.json()

    result = httpx.delete(f"{URL}/api/v1/filament/{added['id']}")
    assert result.status_code == 204

    result = httpx.get(f"{URL}/api/v1/filament/{added['id']}")
    assert result.status_code == 404


def test_delete_filament_not_found():
    """Test deleting a filament that does not exist."""
    result = httpx.delete(f"{URL}/api/v1/filament/123456789")
    assert result.status_code == 404
