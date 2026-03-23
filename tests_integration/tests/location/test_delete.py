"""Integration tests for the Location API endpoint."""

from typing import Any

import httpx

from ..conftest import URL, RED


def test_delete_location():
    """Test deleting an empty location."""
    result = httpx.post(f"{URL}/api/v1/location", json={"name": "Temp"})
    result.raise_for_status()
    location = result.json()

    result = httpx.delete(f"{URL}/api/v1/location/{location['id']}")
    assert result.status_code == 204

    result = httpx.get(f"{URL}/api/v1/location/{location['id']}")
    assert result.status_code == 404


def test_delete_location_not_found():
    """Test deleting a location that does not exist."""
    result = httpx.delete(f"{URL}/api/v1/location/123456789")
    assert result.status_code == 404


def test_delete_location_with_spools_fails(random_filament: dict[str, Any]):
    """Test that deleting a location that has spools is rejected."""
    loc = httpx.post(f"{URL}/api/v1/location", json={"name": "Occupied"}).json()

    spool = httpx.post(
        f"{URL}/api/v1/spool",
        json={
            "filament_id": random_filament["id"],
            "initial_weight": 1000.0,
            "colors": [RED],
            "location_id": loc["id"],
        },
    ).json()

    result = httpx.delete(f"{URL}/api/v1/location/{loc['id']}")
    assert result.status_code in (409, 400)

    # Clean up
    httpx.delete(f"{URL}/api/v1/spool/{spool['id']}").raise_for_status()
    httpx.delete(f"{URL}/api/v1/location/{loc['id']}").raise_for_status()
