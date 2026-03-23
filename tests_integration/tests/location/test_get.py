"""Integration tests for the Location API endpoint."""

import httpx

from ..conftest import URL, assert_dicts_compatible


def test_get_location():
    """Test getting a location by ID."""
    result = httpx.post(f"{URL}/api/v1/location", json={"name": "Dry Box"})
    result.raise_for_status()
    created = result.json()

    result = httpx.get(f"{URL}/api/v1/location/{created['id']}")
    result.raise_for_status()

    location = result.json()
    assert_dicts_compatible(location, {"id": created["id"], "name": "Dry Box", "spool_count": 0})

    httpx.delete(f"{URL}/api/v1/location/{created['id']}").raise_for_status()


def test_get_location_not_found():
    """Test getting a location that does not exist."""
    result = httpx.get(f"{URL}/api/v1/location/123456789")
    assert result.status_code == 404
