"""Integration tests for the Spool API endpoint."""

from datetime import datetime, timezone
from typing import Any

import httpx
import pytest

from ..conftest import URL, RED, assert_dicts_compatible


def test_add_spool_basic(random_filament: dict[str, Any]):
    """Test adding a spool with colors and initial_weight."""
    initial_weight = 1250.0
    color_name = "Galaxy Black"
    comment = "abcdefghåäö"

    result = httpx.post(
        f"{URL}/api/v1/spool",
        json={
            "filament_id": random_filament["id"],
            "initial_weight": initial_weight,
            "colors": [RED],
            "color_name": color_name,
            "comment": comment,
        },
    )
    assert result.status_code == 201

    spool = result.json()
    assert_dicts_compatible(
        spool,
        {
            "id": spool["id"],
            "registered": spool["registered"],
            "filament_id": random_filament["id"],
            "initial_weight": pytest.approx(initial_weight),
            "current_weight": pytest.approx(initial_weight),
            "used_weight": pytest.approx(0.0),
            "colors": [RED],
            "color_name": color_name,
            "comment": comment,
            "archived": False,
        },
    )
    # Embedded filament
    assert spool["filament"]["id"] == random_filament["id"]

    diff = abs((datetime.now(tz=timezone.utc) - datetime.fromisoformat(spool["registered"])).total_seconds())
    assert diff < 60

    httpx.delete(f"{URL}/api/v1/spool/{spool['id']}").raise_for_status()


def test_add_spool_remaining_filament(random_filament: dict[str, Any]):
    """Test that remaining_filament and remaining_pct are computed when filament has net_weight."""
    filament_with_net_weight = httpx.post(
        f"{URL}/api/v1/filament",
        json={"density": 1.24, "diameter": 1.75, "net_weight": 1000.0},
    ).json()

    initial_weight = 1250.0  # spool tare + filament
    result = httpx.post(
        f"{URL}/api/v1/spool",
        json={
            "filament_id": filament_with_net_weight["id"],
            "initial_weight": initial_weight,
            "colors": [RED],
        },
    )
    result.raise_for_status()
    spool = result.json()

    assert spool["remaining_filament"] is not None
    assert spool["remaining_pct"] is not None
    assert spool["used_weight"] == pytest.approx(0.0)

    httpx.delete(f"{URL}/api/v1/spool/{spool['id']}").raise_for_status()
    httpx.delete(f"{URL}/api/v1/filament/{filament_with_net_weight['id']}").raise_for_status()


def test_add_spool_no_net_weight(random_empty_filament: dict[str, Any]):
    """Test that remaining_filament is None when filament has no net_weight."""
    result = httpx.post(
        f"{URL}/api/v1/spool",
        json={
            "filament_id": random_empty_filament["id"],
            "initial_weight": 1000.0,
            "colors": [],
        },
    )
    result.raise_for_status()
    spool = result.json()

    assert spool["remaining_filament"] is None
    assert spool["remaining_pct"] is None

    httpx.delete(f"{URL}/api/v1/spool/{spool['id']}").raise_for_status()


def test_add_spool_with_timestamps(random_filament: dict[str, Any]):
    """Test adding a spool with first_used and last_used."""
    result = httpx.post(
        f"{URL}/api/v1/spool",
        json={
            "filament_id": random_filament["id"],
            "initial_weight": 1000.0,
            "colors": [],
            "first_used": "2023-01-02T12:00:00+01:00",
            "last_used": "2023-01-02T11:00:00Z",
        },
    )
    assert result.status_code == 201
    spool = result.json()

    assert spool["first_used"] == "2023-01-02T11:00:00Z"
    assert spool["last_used"] == "2023-01-02T11:00:00Z"

    httpx.delete(f"{URL}/api/v1/spool/{spool['id']}").raise_for_status()
