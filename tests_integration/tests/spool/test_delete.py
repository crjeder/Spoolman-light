"""Integration tests for the Spool API endpoint."""

from typing import Any

import httpx

from ..conftest import URL


def test_delete_spool(random_filament: dict[str, Any]):
    """Test deleting a spool from the database."""
    result = httpx.post(
        f"{URL}/api/v1/spool",
        json={
            "filament_id": random_filament["id"],
            "initial_weight": 1000,
            "location": "The Pantry",
        },
    )
    result.raise_for_status()
    spool = result.json()

    httpx.delete(f"{URL}/api/v1/spool/{spool['id']}").raise_for_status()

    result = httpx.get(f"{URL}/api/v1/spool/{spool['id']}")
    assert result.status_code == 404


def test_delete_spool_not_found():
    """Test deleting a spool that does not exist."""
    result = httpx.delete(f"{URL}/api/v1/spool/123456789")

    assert result.status_code == 404
    message = result.json()["message"].lower()
    assert "spool" in message
    assert "id" in message
    assert "123456789" in message
