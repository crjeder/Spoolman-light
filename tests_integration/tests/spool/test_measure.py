"""Integration tests for spool weight tracking via current_weight PATCH."""

from datetime import datetime, timezone
from typing import Any

import httpx
import pytest

from ..conftest import URL, RED


@pytest.mark.parametrize("current_weight", [1000.0, 750.0, 500.0, 0.0])
def test_update_current_weight(random_filament: dict[str, Any], current_weight: float):
    """Test that PATCHing current_weight correctly updates used_weight."""
    initial_weight = 1250.0
    result = httpx.post(
        f"{URL}/api/v1/spool",
        json={
            "filament_id": random_filament["id"],
            "initial_weight": initial_weight,
            "colors": [RED],
        },
    )
    result.raise_for_status()
    spool = result.json()

    result = httpx.patch(
        f"{URL}/api/v1/spool/{spool['id']}",
        json={"current_weight": current_weight, "first_used": datetime.now(tz=timezone.utc).isoformat()},
    )
    result.raise_for_status()
    updated = result.json()

    assert updated["current_weight"] == pytest.approx(current_weight)
    assert updated["used_weight"] == pytest.approx(initial_weight - current_weight)

    httpx.delete(f"{URL}/api/v1/spool/{spool['id']}").raise_for_status()


def test_update_current_weight_sequence(random_filament: dict[str, Any]):
    """Test a sequence of current_weight updates."""
    initial_weight = 1250.0
    result = httpx.post(
        f"{URL}/api/v1/spool",
        json={
            "filament_id": random_filament["id"],
            "initial_weight": initial_weight,
            "colors": [],
        },
    )
    result.raise_for_status()
    spool = result.json()

    for reading in [1200.0, 1100.0, 900.0, 700.0]:
        result = httpx.patch(
            f"{URL}/api/v1/spool/{spool['id']}",
            json={"current_weight": reading},
        )
        result.raise_for_status()
        updated = result.json()
        assert updated["current_weight"] == pytest.approx(reading)
        assert updated["used_weight"] == pytest.approx(initial_weight - reading)

    httpx.delete(f"{URL}/api/v1/spool/{spool['id']}").raise_for_status()


def test_update_current_weight_with_net_weight(random_filament: dict[str, Any]):
    """Test remaining_filament updates correctly when filament has net_weight."""
    filament = httpx.post(
        f"{URL}/api/v1/filament",
        json={"density": 1.24, "diameter": 1.75, "net_weight": 1000.0},
    ).json()

    initial_weight = 1250.0
    result = httpx.post(
        f"{URL}/api/v1/spool",
        json={"filament_id": filament["id"], "initial_weight": initial_weight, "colors": []},
    )
    result.raise_for_status()
    spool = result.json()

    new_weight = 900.0
    result = httpx.patch(f"{URL}/api/v1/spool/{spool['id']}", json={"current_weight": new_weight})
    result.raise_for_status()
    updated = result.json()

    assert updated["used_weight"] == pytest.approx(initial_weight - new_weight)
    assert updated["remaining_filament"] is not None

    httpx.delete(f"{URL}/api/v1/spool/{spool['id']}").raise_for_status()
    httpx.delete(f"{URL}/api/v1/filament/{filament['id']}").raise_for_status()
