"""Integration tests for the Spool API endpoint."""

import asyncio
import math
from datetime import datetime, timezone
from typing import Any

import httpx
import pytest

from ..conftest import URL

INITIAL_WEIGHT = 1000


@pytest.mark.parametrize("use_weight", [0, 0.05, -0.05, 1500])  # 1500 uses all filament
def test_use_spool_weight(random_filament: dict[str, Any], use_weight: float):
    """Test using a spool by weight."""
    result = httpx.post(
        f"{URL}/api/v1/spool",
        json={
            "filament_id": random_filament["id"],
            "initial_weight": INITIAL_WEIGHT,
            "used_weight": 0,
        },
    )
    result.raise_for_status()
    spool = result.json()

    result = httpx.put(
        f"{URL}/api/v1/spool/{spool['id']}/use",
        json={"use_weight": use_weight},
    )
    result.raise_for_status()

    spool = result.json()
    assert spool["used_weight"] == pytest.approx(max(use_weight, 0))
    if use_weight >= 0:
        assert spool["remaining_weight"] == pytest.approx(max(INITIAL_WEIGHT - use_weight, 0))

    diff = abs((datetime.now(tz=timezone.utc) - datetime.fromisoformat(spool["first_used"])).total_seconds())
    assert diff < 60

    diff = abs((datetime.now(tz=timezone.utc) - datetime.fromisoformat(spool["last_used"])).total_seconds())
    assert diff < 60

    httpx.delete(f"{URL}/api/v1/spool/{spool['id']}").raise_for_status()


@pytest.mark.parametrize("use_length", [0, 10, -10, 500e3])  # 500e3 uses all filament
def test_use_spool_length(random_filament: dict[str, Any], use_length: float):
    """Test using a spool by length."""
    result = httpx.post(
        f"{URL}/api/v1/spool",
        json={
            "filament_id": random_filament["id"],
            "initial_weight": INITIAL_WEIGHT,
            "used_weight": 0,
            "first_used": "2023-01-01T00:00:00Z",
        },
    )
    result.raise_for_status()
    spool = result.json()

    result = httpx.put(
        f"{URL}/api/v1/spool/{spool['id']}/use",
        json={"use_length": use_length},
    )
    result.raise_for_status()

    spool = result.json()
    use_weight = (
        random_filament["density"] * (use_length * 1e-1) * math.pi * ((random_filament["diameter"] * 1e-1 / 2) ** 2)
    )
    assert spool["used_weight"] == pytest.approx(max(use_weight, 0))

    assert spool["first_used"] == "2023-01-01T00:00:00Z"

    diff = abs((datetime.now(tz=timezone.utc) - datetime.fromisoformat(spool["last_used"])).total_seconds())
    assert diff < 60

    httpx.delete(f"{URL}/api/v1/spool/{spool['id']}").raise_for_status()


def test_use_spool_weight_and_length(random_filament: dict[str, Any]):
    """Test that specifying both weight and length is rejected."""
    result = httpx.post(
        f"{URL}/api/v1/spool",
        json={"filament_id": random_filament["id"]},
    )
    result.raise_for_status()
    spool = result.json()

    result = httpx.put(
        f"{URL}/api/v1/spool/{spool['id']}/use",
        json={"use_weight": 0.05, "use_length": 10},
    )
    assert result.status_code == 400

    httpx.delete(f"{URL}/api/v1/spool/{spool['id']}").raise_for_status()


def test_use_spool_not_found():
    """Test using a spool that does not exist."""
    result = httpx.put(
        f"{URL}/api/v1/spool/123456789/use",
        json={"use_weight": 0.05},
    )
    assert result.status_code == 404
    message = result.json()["message"].lower()
    assert "spool" in message
    assert "id" in message
    assert "123456789" in message


@pytest.mark.asyncio
async def test_use_spool_concurrent(random_filament: dict[str, Any]):
    """Test using a spool with many concurrent requests."""
    result = httpx.post(
        f"{URL}/api/v1/spool",
        json={
            "filament_id": random_filament["id"],
            "initial_weight": INITIAL_WEIGHT,
        },
    )
    result.raise_for_status()
    spool = result.json()

    requests = 100
    used_weight = 0.5
    async with httpx.AsyncClient() as client:
        await asyncio.gather(
            *(
                client.put(
                    f"{URL}/api/v1/spool/{spool['id']}/use",
                    json={"use_weight": used_weight},
                    timeout=60,
                )
                for _ in range(requests)
            ),
        )

    result = httpx.get(f"{URL}/api/v1/spool/{spool['id']}")
    result.raise_for_status()
    spool = result.json()
    assert spool["used_weight"] == pytest.approx(used_weight * requests)

    httpx.delete(f"{URL}/api/v1/spool/{spool['id']}").raise_for_status()
