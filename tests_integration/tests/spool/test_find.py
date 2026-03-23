"""Integration tests for the Spool API endpoint."""

from collections.abc import Iterable
from dataclasses import dataclass
from typing import Any

import httpx
import pytest

from ..conftest import URL, RED, assert_lists_compatible


@dataclass
class Fixture:
    spools: list[dict[str, Any]]
    filament: dict[str, Any]
    location: dict[str, Any]


@pytest.fixture(scope="module")
def spools(
    random_filament_mod: dict[str, Any],
    random_empty_filament_mod: dict[str, Any],
) -> Iterable[Fixture]:
    """Add some spools to the database."""
    loc = httpx.post(f"{URL}/api/v1/location", json={"name": "Find Test Shelf"}).json()

    result = httpx.post(
        f"{URL}/api/v1/spool",
        json={"filament_id": random_filament_mod["id"], "initial_weight": 1000.0, "colors": [], "location_id": loc["id"]},
    )
    result.raise_for_status()
    spool_1 = result.json()

    result = httpx.post(
        f"{URL}/api/v1/spool",
        json={"filament_id": random_filament_mod["id"], "initial_weight": 1000.0, "colors": [RED]},
    )
    result.raise_for_status()
    spool_2 = result.json()

    result = httpx.post(
        f"{URL}/api/v1/spool",
        json={"filament_id": random_filament_mod["id"], "initial_weight": 1000.0, "colors": []},
    )
    result.raise_for_status()
    spool_3_unarchived = result.json()
    spool_3 = httpx.patch(f"{URL}/api/v1/spool/{spool_3_unarchived['id']}", json={"archived": True}).json()

    result = httpx.post(
        f"{URL}/api/v1/spool",
        json={"filament_id": random_empty_filament_mod["id"], "initial_weight": 500.0, "colors": []},
    )
    result.raise_for_status()
    spool_4 = result.json()

    yield Fixture(spools=[spool_1, spool_2, spool_3, spool_4], filament=random_filament_mod, location=loc)

    httpx.delete(f"{URL}/api/v1/spool/{spool_1['id']}").raise_for_status()
    httpx.delete(f"{URL}/api/v1/spool/{spool_2['id']}").raise_for_status()
    httpx.delete(f"{URL}/api/v1/spool/{spool_3['id']}").raise_for_status()
    httpx.delete(f"{URL}/api/v1/spool/{spool_4['id']}").raise_for_status()
    httpx.delete(f"{URL}/api/v1/location/{loc['id']}").raise_for_status()


def test_find_all_spools(spools: Fixture):
    """Test listing all spools with allow_archived=true."""
    result = httpx.get(f"{URL}/api/v1/spool", params={"allow_archived": "true"})
    result.raise_for_status()
    found = result.json()
    found_ids = {s["id"] for s in found}
    for s in spools.spools:
        assert s["id"] in found_ids


def test_find_spools_excludes_archived_by_default(spools: Fixture):
    """Test that archived spools are excluded by default."""
    result = httpx.get(f"{URL}/api/v1/spool")
    result.raise_for_status()
    found = result.json()
    archived_spool = next(s for s in spools.spools if s["archived"])
    found_ids = {s["id"] for s in found}
    assert archived_spool["id"] not in found_ids


def test_find_spools_by_filament_id(spools: Fixture):
    """Test filtering spools by filament_id."""
    result = httpx.get(
        f"{URL}/api/v1/spool",
        params={"filament_id": spools.filament["id"], "allow_archived": "true"},
    )
    result.raise_for_status()
    found = result.json()
    expected = [s for s in spools.spools if s["filament_id"] == spools.filament["id"]]
    assert_lists_compatible(found, expected)


def test_find_spools_by_location_id(spools: Fixture):
    """Test filtering spools by location_id."""
    result = httpx.get(
        f"{URL}/api/v1/spool",
        params={"location_id": spools.location["id"]},
    )
    result.raise_for_status()
    found = result.json()
    expected = [s for s in spools.spools if s.get("location_id") == spools.location["id"] and not s["archived"]]
    assert_lists_compatible(found, expected)


def test_find_spools_x_total_count(spools: Fixture):
    """Test that X-Total-Count header matches list length."""
    result = httpx.get(f"{URL}/api/v1/spool", params={"allow_archived": "true"})
    result.raise_for_status()
    items = result.json()
    total = int(result.headers["x-total-count"])
    assert total >= len(spools.spools)
    assert total == len(items)
