"""Integration tests for the Filament API endpoint."""

from collections.abc import Iterable
from dataclasses import dataclass
from typing import Any

import httpx
import pytest

from ..conftest import URL, assert_lists_compatible


@dataclass
class Fixture:
    filaments: list[dict[str, Any]]


@pytest.fixture(scope="module")
def filaments() -> Iterable[Fixture]:
    """Add some filaments to the database."""
    result = httpx.post(
        f"{URL}/api/v1/filament",
        json={"manufacturer": "eSun", "material": "PLA", "density": 1.24, "diameter": 1.75},
    )
    result.raise_for_status()
    filament_1 = result.json()

    result = httpx.post(
        f"{URL}/api/v1/filament",
        json={"manufacturer": "eSun", "material": "ABS", "density": 1.05, "diameter": 1.75},
    )
    result.raise_for_status()
    filament_2 = result.json()

    result = httpx.post(
        f"{URL}/api/v1/filament",
        json={"manufacturer": "Polymaker", "material": "PLA", "density": 1.24, "diameter": 1.75},
    )
    result.raise_for_status()
    filament_3 = result.json()

    result = httpx.post(
        f"{URL}/api/v1/filament",
        json={"material": "PETG", "density": 1.27, "diameter": 1.75},
    )
    result.raise_for_status()
    filament_4 = result.json()

    yield Fixture(filaments=[filament_1, filament_2, filament_3, filament_4])

    httpx.delete(f"{URL}/api/v1/filament/{filament_1['id']}").raise_for_status()
    httpx.delete(f"{URL}/api/v1/filament/{filament_2['id']}").raise_for_status()
    httpx.delete(f"{URL}/api/v1/filament/{filament_3['id']}").raise_for_status()
    httpx.delete(f"{URL}/api/v1/filament/{filament_4['id']}").raise_for_status()


def test_find_all_filaments(filaments: Fixture):
    """Test finding all filaments."""
    result = httpx.get(f"{URL}/api/v1/filament")
    result.raise_for_status()
    found = result.json()
    assert_lists_compatible(found, filaments.filaments)


def test_find_filaments_by_material(filaments: Fixture):
    """Test filtering filaments by material."""
    result = httpx.get(f"{URL}/api/v1/filament", params={"material": "PLA"})
    result.raise_for_status()
    found = result.json()
    expected = [f for f in filaments.filaments if f.get("material") == "PLA"]
    assert_lists_compatible(found, expected)


def test_find_filaments_x_total_count(filaments: Fixture):
    """Test that X-Total-Count header matches list length."""
    result = httpx.get(f"{URL}/api/v1/filament")
    result.raise_for_status()
    items = result.json()
    total = int(result.headers["x-total-count"])
    assert total == len(items)


def test_find_filaments_pagination(filaments: Fixture):
    """Test filament listing with limit and offset."""
    result = httpx.get(f"{URL}/api/v1/filament", params={"limit": 2, "offset": 0})
    result.raise_for_status()
    page1 = result.json()
    assert len(page1) == 2

    result = httpx.get(f"{URL}/api/v1/filament", params={"limit": 2, "offset": 2})
    result.raise_for_status()
    page2 = result.json()
    assert len(page2) == 2

    ids_p1 = {f["id"] for f in page1}
    ids_p2 = {f["id"] for f in page2}
    assert not ids_p1 & ids_p2


def test_find_filaments_sort(filaments: Fixture):
    """Test that sort and order params are accepted."""
    result = httpx.get(f"{URL}/api/v1/filament", params={"sort": "id", "order": "asc"})
    result.raise_for_status()
    items = result.json()
    ids = [f["id"] for f in items]
    assert ids == sorted(ids)
