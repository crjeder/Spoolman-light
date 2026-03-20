"""Integration tests for the Filament API endpoint."""

import httpx

from ..conftest import URL


def test_delete_filament():
    """Test deleting a filament from the database."""
    result = httpx.post(
        f"{URL}/api/v1/filament",
        json={
            "name": "Filament X",
            "vendor": "eSun",
            "material": "PLA",
            "density": 1.25,
            "diameter": 1.75,
            "comment": "abcdefghåäö",
        },
    )
    result.raise_for_status()
    added_filament = result.json()

    httpx.delete(f"{URL}/api/v1/filament/{added_filament['id']}").raise_for_status()

    result = httpx.get(f"{URL}/api/v1/filament/{added_filament['id']}")
    assert result.status_code == 404


def test_delete_filament_not_found():
    """Test deleting a filament that does not exist."""
    result = httpx.delete(f"{URL}/api/v1/filament/123456789")

    assert result.status_code == 404
    message = result.json()["message"].lower()
    assert "filament" in message
    assert "id" in message
    assert "123456789" in message
