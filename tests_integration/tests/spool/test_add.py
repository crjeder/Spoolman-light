"""Integration tests for the Spool API endpoint."""

from datetime import datetime, timezone
from typing import Any

import httpx
import pytest

from ..conftest import URL, assert_dicts_compatible, length_from_weight


def test_add_spool_with_initial_weight(random_filament: dict[str, Any]):
    """Test adding a spool with initial_weight and used_weight."""
    initial_weight = 1000
    used_weight = 250
    spool_weight = 200
    color_hex = "FF0000"
    price = 25.0
    location = "The Pantry"
    comment = "abcdefghåäö"
    archived = True

    result = httpx.post(
        f"{URL}/api/v1/spool",
        json={
            "first_used": "2023-01-02T12:00:00+01:00",
            "last_used": "2023-01-02T11:00:00Z",
            "filament_id": random_filament["id"],
            "initial_weight": initial_weight,
            "spool_weight": spool_weight,
            "used_weight": used_weight,
            "color_hex": color_hex,
            "price": price,
            "location": location,
            "comment": comment,
            "archived": archived,
        },
    )
    result.raise_for_status()

    remaining_weight = initial_weight - used_weight
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

    spool = result.json()
    assert_dicts_compatible(
        spool,
        {
            "id": spool["id"],
            "registered": spool["registered"],
            "first_used": "2023-01-02T11:00:00Z",
            "last_used": "2023-01-02T11:00:00Z",
            "filament": random_filament,
            "initial_weight": pytest.approx(initial_weight),
            "spool_weight": pytest.approx(spool_weight),
            "used_weight": pytest.approx(used_weight),
            "remaining_weight": pytest.approx(remaining_weight),
            "used_length": pytest.approx(used_length),
            "remaining_length": pytest.approx(remaining_length),
            "color_hex": color_hex,
            "price": price,
            "location": location,
            "comment": comment,
            "archived": archived,
        },
    )

    diff = abs((datetime.now(tz=timezone.utc) - datetime.fromisoformat(spool["registered"])).total_seconds())
    assert diff < 60

    httpx.delete(f"{URL}/api/v1/spool/{spool['id']}").raise_for_status()


def test_add_spool_remaining_weight(random_filament: dict[str, Any]):
    """Test adding a spool using remaining_weight (requires initial_weight)."""
    initial_weight = 1000
    remaining_weight = 750
    result = httpx.post(
        f"{URL}/api/v1/spool",
        json={
            "filament_id": random_filament["id"],
            "initial_weight": initial_weight,
            "remaining_weight": remaining_weight,
        },
    )
    result.raise_for_status()

    used_weight = initial_weight - remaining_weight
    spool = result.json()
    assert pytest.approx(spool["used_weight"]) == used_weight
    assert pytest.approx(spool["remaining_weight"]) == remaining_weight

    httpx.delete(f"{URL}/api/v1/spool/{spool['id']}").raise_for_status()


def test_add_spool_required(random_filament: dict[str, Any]):
    """Test adding a spool with only the required fields."""
    result = httpx.post(
        f"{URL}/api/v1/spool",
        json={"filament_id": random_filament["id"]},
    )
    result.raise_for_status()

    spool = result.json()
    assert_dicts_compatible(
        spool,
        {
            "id": spool["id"],
            "registered": spool["registered"],
            "filament": random_filament,
            "used_weight": 0.0,
            "archived": False,
        },
    )

    httpx.delete(f"{URL}/api/v1/spool/{spool['id']}").raise_for_status()


def test_add_spool_both_used_and_remaining_weight(random_filament: dict[str, Any]):
    """Test that specifying both used_weight and remaining_weight is rejected."""
    result = httpx.post(
        f"{URL}/api/v1/spool",
        json={
            "filament_id": random_filament["id"],
            "remaining_weight": 750,
            "used_weight": 250,
        },
    )
    assert result.status_code == 400


def test_add_spool_color_hex(random_filament: dict[str, Any]):
    """Test adding a spool with a color hex."""
    color_hex = "FF0000"
    result = httpx.post(
        f"{URL}/api/v1/spool",
        json={
            "filament_id": random_filament["id"],
            "color_hex": color_hex,
        },
    )
    result.raise_for_status()
    spool = result.json()
    assert spool["color_hex"] == color_hex
    httpx.delete(f"{URL}/api/v1/spool/{spool['id']}").raise_for_status()


def test_add_spool_multi_color(random_filament: dict[str, Any]):
    """Test adding a spool with multi color hexes."""
    multi_color_hexes = "FF0000,00FF00"
    multi_color_direction = "coaxial"
    result = httpx.post(
        f"{URL}/api/v1/spool",
        json={
            "filament_id": random_filament["id"],
            "multi_color_hexes": multi_color_hexes,
            "multi_color_direction": multi_color_direction,
        },
    )
    result.raise_for_status()
    spool = result.json()
    assert spool["multi_color_hexes"] == multi_color_hexes
    assert spool["multi_color_direction"] == multi_color_direction
    httpx.delete(f"{URL}/api/v1/spool/{spool['id']}").raise_for_status()


def test_add_spool_remaining_weight_no_initial_weight(random_filament: dict[str, Any]):
    """Test that remaining_weight without initial_weight is rejected."""
    result = httpx.post(
        f"{URL}/api/v1/spool",
        json={
            "filament_id": random_filament["id"],
            "remaining_weight": 750,
        },
    )
    assert result.status_code == 400
