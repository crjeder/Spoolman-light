"""Integration tests for the Spool API endpoint."""

from typing import Any

import httpx
import pytest

from ..conftest import URL


def test_get_spool(random_filament: dict[str, Any]):
    """Test getting a spool from the database."""
    initial_weight = 1000
    spool_weight = 200
    location = "The Pantry"
    comment = "abcdefghåäö"
    price = 25
    color_hex = "FF0000"
    archived = True
    result = httpx.post(
        f"{URL}/api/v1/spool",
        json={
            "filament_id": random_filament["id"],
            "initial_weight": initial_weight,
            "spool_weight": spool_weight,
            "color_hex": color_hex,
            "location": location,
            "comment": comment,
            "price": price,
            "archived": archived,
        },
    )
    result.raise_for_status()
    spool = result.json()

    result = httpx.get(f"{URL}/api/v1/spool/{spool['id']}")
    result.raise_for_status()

    assert result.json() == spool

    httpx.delete(f"{URL}/api/v1/spool/{spool['id']}").raise_for_status()


def test_get_spool_weights(random_filament: dict[str, Any]):
    """Test getting a spool with explicit weights set."""
    initial_weight = 1255
    spool_weight = 246
    remaining_weight = 750
    result = httpx.post(
        f"{URL}/api/v1/spool",
        json={
            "filament_id": random_filament["id"],
            "initial_weight": initial_weight,
            "spool_weight": spool_weight,
            "remaining_weight": remaining_weight,
        },
    )
    result.raise_for_status()
    spool = result.json()

    result = httpx.get(f"{URL}/api/v1/spool/{spool['id']}")
    result.raise_for_status()
    result_spool = result.json()

    assert result_spool == spool
    assert result_spool["initial_weight"] == pytest.approx(initial_weight)
    assert result_spool["spool_weight"] == pytest.approx(spool_weight)
    assert result_spool["remaining_weight"] == pytest.approx(remaining_weight)

    httpx.delete(f"{URL}/api/v1/spool/{spool['id']}").raise_for_status()


def test_get_spool_not_found():
    """Test getting a spool that does not exist."""
    result = httpx.get(f"{URL}/api/v1/spool/123456789")

    assert result.status_code == 404
    message = result.json()["message"].lower()
    assert "spool" in message
    assert "id" in message
    assert "123456789" in message
