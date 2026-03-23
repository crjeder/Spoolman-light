"""Integration tests for the Location API endpoint."""

import httpx

from ..conftest import URL, assert_dicts_compatible


def test_add_location():
    """Test adding a location."""
    result = httpx.post(f"{URL}/api/v1/location", json={"name": "Shelf A"})
    assert result.status_code == 201

    location = result.json()
    assert_dicts_compatible(
        location,
        {
            "id": location["id"],
            "name": "Shelf A",
            "spool_count": 0,
        },
    )

    httpx.delete(f"{URL}/api/v1/location/{location['id']}").raise_for_status()


def test_add_location_empty_name():
    """Test that an empty name is rejected."""
    result = httpx.post(f"{URL}/api/v1/location", json={"name": ""})
    assert result.status_code in (400, 422)
