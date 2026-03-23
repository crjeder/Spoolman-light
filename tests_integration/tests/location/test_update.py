"""Integration tests for the Location API endpoint."""

import httpx

from ..conftest import URL


def test_update_location():
    """Test updating a location name."""
    result = httpx.post(f"{URL}/api/v1/location", json={"name": "Old Name"})
    result.raise_for_status()
    location = result.json()

    result = httpx.patch(f"{URL}/api/v1/location/{location['id']}", json={"name": "New Name"})
    result.raise_for_status()
    updated = result.json()

    assert updated["name"] == "New Name"
    assert updated["id"] == location["id"]
    assert updated["spool_count"] == 0

    httpx.delete(f"{URL}/api/v1/location/{location['id']}").raise_for_status()


def test_update_location_not_found():
    """Test updating a location that does not exist."""
    result = httpx.patch(f"{URL}/api/v1/location/123456789", json={"name": "Nowhere"})
    assert result.status_code == 404
