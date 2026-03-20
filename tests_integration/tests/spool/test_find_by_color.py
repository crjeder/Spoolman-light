"""Integration tests for the spool find-by-color endpoint."""

from typing import Any

import httpx
import pytest

from ..conftest import URL


def test_find_spools_by_color(random_filament: dict[str, Any]):
    """Test finding spools by color similarity."""
    result = httpx.post(
        f"{URL}/api/v1/spool",
        json={
            "filament_id": random_filament["id"],
            "color_hex": "FF0000",
        },
    )
    result.raise_for_status()
    red_spool = result.json()

    result = httpx.post(
        f"{URL}/api/v1/spool",
        json={
            "filament_id": random_filament["id"],
            "color_hex": "0000FF",
        },
    )
    result.raise_for_status()
    blue_spool = result.json()

    # Search for red — should find red spool, not blue
    result = httpx.get(f"{URL}/api/v1/spool/find-by-color", params={"color": "FF0000", "threshold": 20})
    result.raise_for_status()
    found = result.json()
    found_ids = [s["id"] for s in found]
    assert red_spool["id"] in found_ids
    assert blue_spool["id"] not in found_ids

    httpx.delete(f"{URL}/api/v1/spool/{red_spool['id']}").raise_for_status()
    httpx.delete(f"{URL}/api/v1/spool/{blue_spool['id']}").raise_for_status()


def test_find_spools_by_color_no_match(random_filament: dict[str, Any]):
    """Test that color search returns empty when no spools match."""
    result = httpx.post(
        f"{URL}/api/v1/spool",
        json={
            "filament_id": random_filament["id"],
            "color_hex": "00FF00",
        },
    )
    result.raise_for_status()
    spool = result.json()

    # Search for red with a very low threshold — should not find green
    result = httpx.get(f"{URL}/api/v1/spool/find-by-color", params={"color": "FF0000", "threshold": 5})
    result.raise_for_status()
    found = result.json()
    found_ids = [s["id"] for s in found]
    assert spool["id"] not in found_ids

    httpx.delete(f"{URL}/api/v1/spool/{spool['id']}").raise_for_status()


def test_find_spools_by_color_excludes_colorless(random_filament: dict[str, Any]):
    """Test that spools without a color are excluded from color search."""
    result = httpx.post(
        f"{URL}/api/v1/spool",
        json={"filament_id": random_filament["id"]},
    )
    result.raise_for_status()
    spool = result.json()

    result = httpx.get(f"{URL}/api/v1/spool/find-by-color", params={"color": "FF0000", "threshold": 100})
    result.raise_for_status()
    found = result.json()
    found_ids = [s["id"] for s in found]
    assert spool["id"] not in found_ids

    httpx.delete(f"{URL}/api/v1/spool/{spool['id']}").raise_for_status()


@pytest.mark.parametrize("color", ["FF0000", "00FF00", "0000FF", "FFFFFF", "000000"])
def test_find_spools_by_color_various(random_filament: dict[str, Any], color: str):
    """Test color search with various colors."""
    result = httpx.post(
        f"{URL}/api/v1/spool",
        json={
            "filament_id": random_filament["id"],
            "color_hex": color,
        },
    )
    result.raise_for_status()
    spool = result.json()

    result = httpx.get(f"{URL}/api/v1/spool/find-by-color", params={"color": color, "threshold": 1})
    result.raise_for_status()
    found = result.json()
    found_ids = [s["id"] for s in found]
    assert spool["id"] in found_ids

    httpx.delete(f"{URL}/api/v1/spool/{spool['id']}").raise_for_status()
