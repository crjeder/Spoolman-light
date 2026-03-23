"""Integration tests for the Spool API endpoint."""

from typing import Any

import httpx
import pytest

from ..conftest import URL, RED


def test_update_spool_current_weight(random_filament: dict[str, Any]):
    """Test that PATCHing current_weight updates used_weight correctly."""
    result = httpx.post(
        f"{URL}/api/v1/spool",
        json={
            "filament_id": random_filament["id"],
            "initial_weight": 1250.0,
            "colors": [RED],
        },
    )
    result.raise_for_status()
    spool = result.json()

    result = httpx.patch(
        f"{URL}/api/v1/spool/{spool['id']}",
        json={"current_weight": 900.0},
    )
    result.raise_for_status()
    updated = result.json()

    assert updated["current_weight"] == pytest.approx(900.0)
    assert updated["used_weight"] == pytest.approx(350.0)  # 1250 - 900
    # Unchanged fields
    assert updated["initial_weight"] == pytest.approx(1250.0)
    assert updated["colors"] == [RED]

    httpx.delete(f"{URL}/api/v1/spool/{spool['id']}").raise_for_status()


def test_update_spool_comment_and_archived(random_filament: dict[str, Any]):
    """Test PATCHing comment and archived flag."""
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

    result = httpx.patch(
        f"{URL}/api/v1/spool/{spool['id']}",
        json={"comment": "updated comment", "archived": True},
    )
    result.raise_for_status()
    updated = result.json()

    assert updated["comment"] == "updated comment"
    assert updated["archived"] is True
    assert updated["initial_weight"] == pytest.approx(1000.0)

    httpx.delete(f"{URL}/api/v1/spool/{spool['id']}").raise_for_status()


def test_update_spool_colors(random_filament: dict[str, Any]):
    """Test PATCHing the colors array."""
    result = httpx.post(
        f"{URL}/api/v1/spool",
        json={
            "filament_id": random_filament["id"],
            "initial_weight": 1000.0,
            "colors": [RED],
        },
    )
    result.raise_for_status()
    spool = result.json()

    blue = {"r": 0, "g": 0, "b": 255, "a": 255}
    result = httpx.patch(
        f"{URL}/api/v1/spool/{spool['id']}",
        json={"colors": [blue]},
    )
    result.raise_for_status()
    updated = result.json()

    assert updated["colors"] == [blue]

    httpx.delete(f"{URL}/api/v1/spool/{spool['id']}").raise_for_status()


def test_update_spool_timestamps(random_filament: dict[str, Any]):
    """Test PATCHing first_used and last_used timestamps."""
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

    result = httpx.patch(
        f"{URL}/api/v1/spool/{spool['id']}",
        json={
            "first_used": "2023-01-01T12:00:00+02:00",
            "last_used": "2023-01-02T12:00:00+02:00",
        },
    )
    result.raise_for_status()
    updated = result.json()

    assert updated["first_used"] == "2023-01-01T10:00:00Z"
    assert updated["last_used"] == "2023-01-02T10:00:00Z"

    httpx.delete(f"{URL}/api/v1/spool/{spool['id']}").raise_for_status()


def test_update_spool_not_found():
    """Test updating a spool that does not exist."""
    result = httpx.patch(
        f"{URL}/api/v1/spool/123456789",
        json={"comment": "nowhere"},
    )
    assert result.status_code == 404


def test_update_spool_location_id(random_filament: dict[str, Any]):
    """Test assigning a spool to a location via location_id."""
    # Create a location
    loc = httpx.post(f"{URL}/api/v1/location", json={"name": "Shelf B"}).json()

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

    result = httpx.patch(
        f"{URL}/api/v1/spool/{spool['id']}",
        json={"location_id": loc["id"]},
    )
    result.raise_for_status()
    updated = result.json()

    assert updated["location_id"] == loc["id"]

    httpx.delete(f"{URL}/api/v1/spool/{spool['id']}").raise_for_status()
    httpx.delete(f"{URL}/api/v1/location/{loc['id']}").raise_for_status()
