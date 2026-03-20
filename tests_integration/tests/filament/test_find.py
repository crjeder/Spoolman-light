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
        json={
            "name": "Filament X",
            "vendor": "eSun",
            "material": "PLA",
            "density": 1.25,
            "diameter": 1.75,
            "settings_extruder_temp": 210,
        },
    )
    result.raise_for_status()
    filament_1 = result.json()

    result = httpx.post(
        f"{URL}/api/v1/filament",
        json={
            "name": "Filament Y",
            "vendor": "eSun",
            "material": "ABS",
            "density": 1.25,
            "diameter": 1.75,
        },
    )
    result.raise_for_status()
    filament_2 = result.json()

    result = httpx.post(
        f"{URL}/api/v1/filament",
        json={
            "name": "Filament Z",
            "material": "PLA+",
            "density": 1.25,
            "diameter": 1.75,
        },
    )
    result.raise_for_status()
    filament_3 = result.json()

    result = httpx.post(
        f"{URL}/api/v1/filament",
        json={
            "name": "Filament W",
            "vendor": "Polymaker",
            "material": "PLA",
            "density": 1.25,
            "diameter": 1.75,
        },
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


def test_find_filaments_by_vendor(filaments: Fixture):
    """Test finding filaments by vendor string."""
    result = httpx.get(f"{URL}/api/v1/filament", params={"vendor": "eSun"})
    result.raise_for_status()
    found = result.json()
    expected = [f for f in filaments.filaments if f.get("vendor") == "eSun"]
    assert_lists_compatible(found, expected)


def test_find_filaments_by_name(filaments: Fixture):
    """Test finding filaments by name."""
    result = httpx.get(f"{URL}/api/v1/filament", params={"name": "Filament X"})
    result.raise_for_status()
    found = result.json()
    expected = [f for f in filaments.filaments if f.get("name") == "Filament X"]
    assert_lists_compatible(found, expected)


def test_find_filaments_by_material(filaments: Fixture):
    """Test finding filaments by material (exact match using quoted syntax)."""
    result = httpx.get(f"{URL}/api/v1/filament", params={"material": '"PLA"'})
    result.raise_for_status()
    found = result.json()
    expected = [f for f in filaments.filaments if f.get("material") == "PLA"]
    assert_lists_compatible(found, expected)


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

    # No overlap
    ids_p1 = {f["id"] for f in page1}
    ids_p2 = {f["id"] for f in page2}
    assert not ids_p1 & ids_p2
