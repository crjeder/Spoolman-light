"""Integration tests for the Location API endpoint."""

from typing import Any

import httpx

from ..conftest import URL, RED


def test_find_all_locations():
    """Test listing all locations."""
    loc1 = httpx.post(f"{URL}/api/v1/location", json={"name": "Shelf 1"}).json()
    loc2 = httpx.post(f"{URL}/api/v1/location", json={"name": "Shelf 2"}).json()

    result = httpx.get(f"{URL}/api/v1/location")
    result.raise_for_status()
    locations = result.json()
    ids = {loc["id"] for loc in locations}
    assert loc1["id"] in ids
    assert loc2["id"] in ids

    httpx.delete(f"{URL}/api/v1/location/{loc1['id']}").raise_for_status()
    httpx.delete(f"{URL}/api/v1/location/{loc2['id']}").raise_for_status()


def test_spool_count_increments(random_filament: dict[str, Any]):
    """Test that spool_count increments when a spool is assigned to the location."""
    loc = httpx.post(f"{URL}/api/v1/location", json={"name": "Counter Test"}).json()

    assert loc["spool_count"] == 0

    spool = httpx.post(
        f"{URL}/api/v1/spool",
        json={
            "filament_id": random_filament["id"],
            "initial_weight": 1000.0,
            "colors": [RED],
            "location_id": loc["id"],
        },
    ).json()

    loc_updated = httpx.get(f"{URL}/api/v1/location/{loc['id']}").json()
    assert loc_updated["spool_count"] == 1

    httpx.delete(f"{URL}/api/v1/spool/{spool['id']}").raise_for_status()
    httpx.delete(f"{URL}/api/v1/location/{loc['id']}").raise_for_status()
