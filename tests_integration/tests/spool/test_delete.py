"""Integration tests for the Spool API endpoint."""

from typing import Any

import httpx

from ..conftest import URL, RED


def test_delete_spool(random_filament: dict[str, Any]):
    """Test deleting a spool from the database."""
    result = httpx.post(
        f"{URL}/api/v1/spool",
        json={
            "filament_id": random_filament["id"],
            "initial_weight": 1000.0,
            "colors": [],
        },
    )
    result.raise_for_status()
    spool = result.json()

    result = httpx.delete(f"{URL}/api/v1/spool/{spool['id']}")
    assert result.status_code == 204

    result = httpx.get(f"{URL}/api/v1/spool/{spool['id']}")
    assert result.status_code == 404


def test_delete_spool_not_found():
    """Test deleting a spool that does not exist."""
    result = httpx.delete(f"{URL}/api/v1/spool/123456789")
    assert result.status_code == 404


def test_delete_spool_decrements_location_spool_count(random_filament: dict[str, Any]):
    """Test that deleting a spool decrements the location's spool_count."""
    loc = httpx.post(f"{URL}/api/v1/location", json={"name": "Temp Shelf"}).json()

    result = httpx.post(
        f"{URL}/api/v1/spool",
        json={
            "filament_id": random_filament["id"],
            "initial_weight": 1000.0,
            "colors": [RED],
            "location_id": loc["id"],
        },
    )
    result.raise_for_status()
    spool = result.json()

    loc_with_spool = httpx.get(f"{URL}/api/v1/location/{loc['id']}").json()
    assert loc_with_spool["spool_count"] == 1

    httpx.delete(f"{URL}/api/v1/spool/{spool['id']}").raise_for_status()

    loc_empty = httpx.get(f"{URL}/api/v1/location/{loc['id']}").json()
    assert loc_empty["spool_count"] == 0

    httpx.delete(f"{URL}/api/v1/location/{loc['id']}").raise_for_status()
