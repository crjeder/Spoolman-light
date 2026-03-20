"""Integration tests for the Spool API endpoint."""

from typing import Any

import httpx
import pytest

from ..conftest import URL, length_from_weight


def test_update_spool(random_filament: dict[str, Any]):
    """Test updating a spool in the database."""
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

    initial_weight = 1000
    remaining_weight = 750
    location = "Living Room"
    comment = "abcdefghåäö"
    archived = True
    price = 25
    color_hex = "00FF00"

    result = httpx.patch(
        f"{URL}/api/v1/spool/{spool['id']}",
        json={
            "first_used": "2023-01-01T12:00:00+02:00",
            "last_used": "2023-01-02T12:00:00+02:00",
            "remaining_weight": remaining_weight,
            "location": location,
            "comment": comment,
            "archived": archived,
            "price": price,
            "color_hex": color_hex,
        },
    )
    result.raise_for_status()

    used_weight = initial_weight - remaining_weight
    used_length = length_from_weight(
        weight=used_weight,
        density=random_filament["density"],
        diameter=random_filament["diameter"],
    )
    remaining_length = length_from_weight(
        weight=remaining_weight,
        density=random_filament["density"],
        diameter=random_filament["diameter"],
    )

    updated_spool = result.json()
    assert updated_spool["used_weight"] == pytest.approx(used_weight)
    assert updated_spool["remaining_weight"] == pytest.approx(remaining_weight)
    assert updated_spool["used_length"] == pytest.approx(used_length)
    assert updated_spool["remaining_length"] == pytest.approx(remaining_length)
    assert updated_spool["location"] == location
    assert updated_spool["comment"] == comment
    assert updated_spool["archived"] == archived
    assert updated_spool["price"] == price
    assert updated_spool["color_hex"] == color_hex
    assert updated_spool["first_used"] == "2023-01-01T10:00:00Z"
    assert updated_spool["last_used"] == "2023-01-02T10:00:00Z"

    httpx.delete(f"{URL}/api/v1/spool/{spool['id']}").raise_for_status()


def test_update_spool_not_found():
    """Test updating a spool that does not exist."""
    result = httpx.patch(
        f"{URL}/api/v1/spool/123456789",
        json={"location": "Nowhere"},
    )
    assert result.status_code == 404


def test_update_spool_filament_id_none(random_filament: dict[str, Any]):
    """Test that filament_id cannot be set to None."""
    result = httpx.post(
        f"{URL}/api/v1/spool",
        json={"filament_id": random_filament["id"]},
    )
    result.raise_for_status()
    spool = result.json()

    result = httpx.patch(
        f"{URL}/api/v1/spool/{spool['id']}",
        json={"filament_id": None},
    )
    assert result.status_code == 422

    httpx.delete(f"{URL}/api/v1/spool/{spool['id']}").raise_for_status()
