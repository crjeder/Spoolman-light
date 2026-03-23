"""Integration tests for the Spool API endpoint."""

from typing import Any

import httpx
import pytest

from ..conftest import URL, RED


def test_get_spool(random_filament: dict[str, Any]):
    """Test getting a spool from the database."""
    result = httpx.post(
        f"{URL}/api/v1/spool",
        json={
            "filament_id": random_filament["id"],
            "initial_weight": 1250.0,
            "colors": [RED],
            "color_name": "Fire Red",
            "comment": "abcdefghåäö",
        },
    )
    result.raise_for_status()
    created = result.json()

    result = httpx.get(f"{URL}/api/v1/spool/{created['id']}")
    result.raise_for_status()
    spool = result.json()

    assert spool == created
    assert spool["initial_weight"] == pytest.approx(1250.0)
    assert spool["current_weight"] == pytest.approx(1250.0)
    assert spool["used_weight"] == pytest.approx(0.0)
    assert spool["filament"]["id"] == random_filament["id"]

    httpx.delete(f"{URL}/api/v1/spool/{spool['id']}").raise_for_status()


def test_get_spool_not_found():
    """Test getting a spool that does not exist."""
    result = httpx.get(f"{URL}/api/v1/spool/123456789")
    assert result.status_code == 404
