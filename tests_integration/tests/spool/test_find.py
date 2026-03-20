"""Integration tests for the Spool API endpoint."""

from collections.abc import Iterable
from dataclasses import dataclass
from typing import Any

import httpx
import pytest

from ..conftest import URL, assert_lists_compatible


@dataclass
class Fixture:
    spools: list[dict[str, Any]]
    filament: dict[str, Any]


@pytest.fixture(scope="module")
def spools(
    random_filament_mod: dict[str, Any],
    random_empty_filament_mod: dict[str, Any],
) -> Iterable[Fixture]:
    """Add some spools to the database."""
    result = httpx.post(
        f"{URL}/api/v1/spool",
        json={
            "filament_id": random_filament_mod["id"],
            "initial_weight": 1000,
            "used_weight": 0,
            "location": "The Pantry",
        },
    )
    result.raise_for_status()
    spool_1 = result.json()

    result = httpx.post(
        f"{URL}/api/v1/spool",
        json={
            "filament_id": random_filament_mod["id"],
            "initial_weight": 1000,
            "used_weight": 0,
            "location": "Living Room",
        },
    )
    result.raise_for_status()
    spool_2 = result.json()

    result = httpx.post(
        f"{URL}/api/v1/spool",
        json={
            "filament_id": random_filament_mod["id"],
            "initial_weight": 1000,
            "archived": True,
        },
    )
    result.raise_for_status()
    spool_3 = result.json()

    result = httpx.post(
        f"{URL}/api/v1/spool",
        json={
            "filament_id": random_empty_filament_mod["id"],
            "used_weight": 1000,
        },
    )
    result.raise_for_status()
    spool_4 = result.json()

    yield Fixture(
        spools=[spool_1, spool_2, spool_3, spool_4],
        filament=random_filament_mod,
    )

    httpx.delete(f"{URL}/api/v1/spool/{spool_1['id']}").raise_for_status()
    httpx.delete(f"{URL}/api/v1/spool/{spool_2['id']}").raise_for_status()
    httpx.delete(f"{URL}/api/v1/spool/{spool_3['id']}").raise_for_status()
    httpx.delete(f"{URL}/api/v1/spool/{spool_4['id']}").raise_for_status()


def test_find_all_spools(spools: Fixture):
    # Execute
    result = httpx.get(f"{URL}/api/v1/spool")
    result.raise_for_status()

    # Verify — spool_3 is archived so excluded
    spools_result = result.json()
    assert_lists_compatible(
        spools_result,
        (spools.spools[0], spools.spools[1], spools.spools[3]),
    )


def test_find_all_spools_including_archived(spools: Fixture):
    # Execute
    result = httpx.get(f"{URL}/api/v1/spool?allow_archived=true")
    result.raise_for_status()

    # Verify
    spools_result = result.json()
    assert_lists_compatible(
        spools_result,
        (spools.spools[0], spools.spools[1], spools.spools[2], spools.spools[3]),
    )


def test_find_all_spools_sort_asc(spools: Fixture):
    # Execute
    result = httpx.get(f"{URL}/api/v1/spool?sort=id:asc&allow_archived=true")
    result.raise_for_status()

    # Verify
    spools_result = result.json()
    assert len(spools_result) == len(spools.spools)
    assert spools_result[0] == spools.spools[0]


def test_find_all_spools_sort_desc(spools: Fixture):
    # Execute
    result = httpx.get(f"{URL}/api/v1/spool?sort=id:desc&allow_archived=true")
    result.raise_for_status()

    # Verify
    spools_result = result.json()
    assert len(spools_result) == len(spools.spools)
    assert spools_result[-1] == spools.spools[0]


def test_find_all_spools_sort_multiple(spools: Fixture):
    # Execute
    result = httpx.get(f"{URL}/api/v1/spool?sort=used_weight:desc,id:asc&allow_archived=true")
    result.raise_for_status()

    # Verify — spool_4 has used_weight=1000, rest have 0; then sort by id asc
    spools_result = result.json()
    assert len(spools_result) == len(spools.spools)
    assert spools_result == [spools.spools[3], spools.spools[0], spools.spools[1], spools.spools[2]]


def test_find_all_spools_limit_asc(spools: Fixture):
    # Execute
    result = httpx.get(f"{URL}/api/v1/spool?sort=id:asc&limit=2")
    result.raise_for_status()

    # Verify — 3 non-archived spools total
    assert result.headers["X-Total-Count"] == "3"
    spools_result = result.json()
    assert len(spools_result) == 2
    assert spools_result == [spools.spools[0], spools.spools[1]]


def test_find_all_spools_limit_desc(spools: Fixture):
    # Execute
    result = httpx.get(f"{URL}/api/v1/spool?sort=id:desc&limit=2")
    result.raise_for_status()

    # Verify — 3 non-archived; in desc order: spool_4, spool_2, spool_1; first 2: spool_4, spool_2
    assert result.headers["X-Total-Count"] == "3"
    spools_result = result.json()
    assert len(spools_result) == 2
    assert spools_result == [spools.spools[3], spools.spools[1]]


def test_find_all_spools_limit_asc_offset(spools: Fixture):
    # Execute
    result = httpx.get(f"{URL}/api/v1/spool?sort=id:asc&limit=2&offset=1&allow_archived=true")
    result.raise_for_status()

    # Verify
    assert result.headers["X-Total-Count"] == "4"
    spools_result = result.json()
    assert len(spools_result) == 2
    assert spools_result == [spools.spools[1], spools.spools[2]]


def test_find_all_spools_limit_desc_offset(spools: Fixture):
    # Execute
    result = httpx.get(f"{URL}/api/v1/spool?sort=id:desc&limit=2&offset=1&allow_archived=true")
    result.raise_for_status()

    # Verify — desc order: spool_4, spool_3, spool_2, spool_1; offset 1 → spool_3, spool_2
    assert result.headers["X-Total-Count"] == "4"
    spools_result = result.json()
    assert len(spools_result) == 2
    assert spools_result == [spools.spools[2], spools.spools[1]]


def test_find_all_spools_limit_asc_offset_outside_range(spools: Fixture):  # noqa: ARG001
    # Execute
    result = httpx.get(f"{URL}/api/v1/spool?sort=id:asc&limit=2&offset=100")
    result.raise_for_status()

    # Verify — 3 non-archived total
    assert result.headers["X-Total-Count"] == "3"
    spools_result = result.json()
    assert len(spools_result) == 0


@pytest.mark.parametrize(
    "field_name",
    [
        "id",
        "registered",
        "first_used",
        "last_used",
        "filament_id",
        "used_weight",
        "remaining_weight",
        "used_length",
        "remaining_length",
        "location",
        "color_hex",
        "multi_color_hexes",
        "multi_color_direction",
        "price",
        "comment",
        "archived",
        "filament.id",
        "filament.registered",
        "filament.name",
        "filament.vendor",
        "filament.material",
        "filament.density",
        "filament.diameter",
        "filament.comment",
        "filament.settings_extruder_temp",
        "filament.settings_bed_temp",
    ],
)
def test_find_all_spools_sort_fields(spools: Fixture, field_name: str):
    """Test sorting by all fields."""
    # Execute
    result = httpx.get(f"{URL}/api/v1/spool?sort={field_name}:asc&allow_archived=true")
    result.raise_for_status()

    # Verify
    spools_result = result.json()
    assert len(spools_result) == len(spools.spools)


def test_find_spools_by_filament_name(spools: Fixture):
    # Execute
    result = httpx.get(
        f"{URL}/api/v1/spool",
        params={"filament.name": spools.filament["name"]},
    )
    result.raise_for_status()

    # Verify — spool_1 and spool_2 use random_filament; spool_3 is archived
    spools_result = result.json()
    assert_lists_compatible(spools_result, (spools.spools[0], spools.spools[1]))


def test_find_spools_by_empty_filament_name(spools: Fixture):
    # Execute
    result = httpx.get(
        f"{URL}/api/v1/spool",
        params={"filament.name": ""},
    )
    result.raise_for_status()

    # Verify — spool_4 uses empty filament (no name)
    spools_result = result.json()
    assert spools_result == [spools.spools[3]]


def test_find_spools_by_filament_id(spools: Fixture):
    # Execute
    result = httpx.get(
        f"{URL}/api/v1/spool",
        params={"filament.id": spools.filament["id"]},
    )
    result.raise_for_status()

    # Verify
    spools_result = result.json()
    assert_lists_compatible(spools_result, (spools.spools[0], spools.spools[1]))


def test_find_spools_by_multiple_filament_ids(spools: Fixture):
    # Execute
    filament_1 = spools.spools[0]["filament"]["id"]
    filament_2 = spools.spools[3]["filament"]["id"]

    result = httpx.get(
        f"{URL}/api/v1/spool",
        params={"filament.id": f"{filament_1},{filament_2}"},
    )
    result.raise_for_status()

    # Verify
    spools_result = result.json()
    assert_lists_compatible(spools_result, (spools.spools[0], spools.spools[1], spools.spools[3]))


def test_find_spools_by_filament_material(spools: Fixture):
    # Execute
    result = httpx.get(
        f"{URL}/api/v1/spool",
        params={"filament.material": spools.filament["material"]},
    )
    result.raise_for_status()

    # Verify
    spools_result = result.json()
    assert_lists_compatible(spools_result, (spools.spools[0], spools.spools[1]))


def test_find_spools_by_empty_filament_material(spools: Fixture):
    # Execute
    result = httpx.get(
        f"{URL}/api/v1/spool",
        params={"filament.material": ""},
    )
    result.raise_for_status()

    # Verify
    spools_result = result.json()
    assert spools_result == [spools.spools[3]]


def test_find_spools_by_filament_vendor(spools: Fixture):
    # Execute
    result = httpx.get(
        f"{URL}/api/v1/spool",
        params={"filament.vendor": spools.filament["vendor"]},
    )
    result.raise_for_status()

    # Verify — spool_1, spool_2 have vendor "TestVendor"; spool_3 is archived
    spools_result = result.json()
    assert_lists_compatible(spools_result, (spools.spools[0], spools.spools[1]))


def test_find_spools_by_empty_filament_vendor(spools: Fixture):
    # Execute
    result = httpx.get(
        f"{URL}/api/v1/spool",
        params={"filament.vendor": ""},
    )
    result.raise_for_status()

    # Verify — spool_4 uses empty filament (no vendor)
    spools_result = result.json()
    assert spools_result == [spools.spools[3]]


def test_find_spools_by_location(spools: Fixture):
    # Execute
    result = httpx.get(
        f"{URL}/api/v1/spool",
        params={"location": "The Pantry"},
    )
    result.raise_for_status()

    # Verify
    spools_result = result.json()
    assert spools_result == [spools.spools[0]]


def test_find_spools_by_empty_location(spools: Fixture):
    # Execute
    result = httpx.get(
        f"{URL}/api/v1/spool",
        params={"location": ""},
    )
    result.raise_for_status()

    # Verify — spool_4 has no location; spool_3 is archived
    spools_result = result.json()
    assert spools_result == [spools.spools[3]]


def test_find_spools_by_empty_and_filled_location(spools: Fixture):
    # Execute
    result = httpx.get(
        f"{URL}/api/v1/spool",
        params={"location": "The Pantry,"},
    )
    result.raise_for_status()

    # Verify — spool_1 has "The Pantry", spool_4 has no location
    spools_result = result.json()
    assert_lists_compatible(spools_result, (spools.spools[0], spools.spools[3]))
