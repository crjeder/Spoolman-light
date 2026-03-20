"""Test fixtures for integration tests."""

import math
import os
import time
from collections.abc import Iterable
from contextlib import contextmanager
from typing import Any

import httpx
import pytest

TIMEOUT = 30

URL = "http://spoolman:" + os.environ.get("SPOOLMAN_PORT", "8000")


def pytest_sessionstart(session):  # noqa: ARG001, ANN001
    """Wait for the server to start up."""
    start_time = time.time()
    while True:
        try:
            print("pytest: Waiting for spoolman to be available...")  # noqa: T201
            response = httpx.get(URL, timeout=1)
            response.raise_for_status()
            print("pytest: Spoolman now seems to be up!")  # noqa: T201
        except httpx.HTTPError:  # noqa: PERF203
            if time.time() - start_time > TIMEOUT:
                raise
            time.sleep(0.5)
        else:
            break


@contextmanager
def random_filament_impl():
    """Return a random filament."""
    result = httpx.post(
        f"{URL}/api/v1/filament",
        json={
            "name": "Filament X",
            "vendor": "TestVendor",
            "material": "PLA",
            "density": 1.25,
            "diameter": 1.75,
            "settings_extruder_temp": 210,
            "settings_bed_temp": 60,
            "comment": "abcdefghåäö",
        },
    )
    result.raise_for_status()

    filament: dict[str, Any] = result.json()
    yield filament

    httpx.delete(f"{URL}/api/v1/filament/{filament['id']}").raise_for_status()


@contextmanager
def random_empty_filament_impl():
    """Return a random filament with only required fields specified."""
    result = httpx.post(
        f"{URL}/api/v1/filament",
        json={
            "density": 1.25,
            "diameter": 1.75,
        },
    )
    result.raise_for_status()

    filament: dict[str, Any] = result.json()
    yield filament

    httpx.delete(f"{URL}/api/v1/filament/{filament['id']}").raise_for_status()


@contextmanager
def random_spool_impl(filament_id: int, *, initial_weight: float = 1000, spool_weight: float = 250):
    """Return a random spool."""
    result = httpx.post(
        f"{URL}/api/v1/spool",
        json={
            "filament_id": filament_id,
            "initial_weight": initial_weight,
            "spool_weight": spool_weight,
            "color_hex": "FF0000",
            "price": 25.0,
            "location": "Shelf A",
            "comment": "Test spool",
        },
    )
    result.raise_for_status()

    spool: dict[str, Any] = result.json()
    yield spool

    httpx.delete(f"{URL}/api/v1/spool/{spool['id']}").raise_for_status()


@pytest.fixture
def random_filament():
    """Return a random filament."""
    with random_filament_impl() as filament:
        yield filament


@pytest.fixture
def random_empty_filament():
    """Return a random filament with only required fields specified."""
    with random_empty_filament_impl() as filament:
        yield filament


@pytest.fixture(scope="module")
def random_filament_mod():
    """Return a random filament."""
    with random_filament_impl() as filament:
        yield filament


@pytest.fixture(scope="module")
def random_empty_filament_mod():
    """Return a random filament with only required fields specified."""
    with random_empty_filament_impl() as filament:
        yield filament


def length_from_weight(*, weight: float, diameter: float, density: float) -> float:
    """Calculate the length of a piece of filament.

    Args:
        weight (float): Filament weight in g
        diameter (float): Filament diameter in mm
        density (float): Density of filament material in g/cm3

    Returns:
        float: Length in mm

    """
    volume_cm3 = weight / density
    volume_mm3 = volume_cm3 * 1000
    return volume_mm3 / (math.pi * (diameter / 2) ** 2)


def assert_dicts_compatible(actual: Any, expected: Any, path: str = "") -> None:  # noqa: ANN401
    """Assert that two dictionaries are compatible for unit testing a REST API.

    Args:
        actual (dict): The actual dictionary.
        expected (dict): The expected dictionary.
        path (str): The path to the current level in the dictionary (used for error messages).

    Raises:
        AssertionError: If dictionaries are not compatible.

    """
    if not (isinstance(actual, dict) and isinstance(expected, dict)):
        raise TypeError(f"At {path}: Actual and expected values must be dictionaries.")

    missing_keys = [key for key in expected if key not in actual]
    if missing_keys:
        raise AssertionError(f"At {path}: Missing keys in actual dictionary: {missing_keys}")

    for key, expected_value in expected.items():
        actual_value = actual[key]
        subpath = f"{path}.{key}" if path else key

        if isinstance(expected_value, dict):
            assert_dicts_compatible(actual_value, expected_value, path=subpath)
        elif actual_value != expected_value:
            raise AssertionError(
                f"At {subpath}: Values do not match. Expected: {expected_value}, Actual: {actual_value}",
            )


def assert_lists_compatible(a: Iterable[dict[str, Any]], b: Iterable[dict[str, Any]], sort_key: str = "id") -> None:
    """Compare two lists of items where the order of the items is not guaranteed."""
    a_sorted = sorted(a, key=lambda x: x[sort_key])
    b_sorted = sorted(b, key=lambda x: x[sort_key])
    if len(a_sorted) != len(b_sorted):
        pytest.fail(f"Lists have different lengths: {len(a_sorted)} != {len(b_sorted)}")

    for a_item, b_item in zip(a_sorted, b_sorted):
        assert_dicts_compatible(a_item, b_item)


def assert_httpx_success(response: httpx.Response) -> None:
    """Assert that a response is successful."""
    if not response.is_success:
        pytest.fail(f"Request failed: {response.status_code} {response.text}")


def assert_httpx_code(response: httpx.Response, code: int) -> None:
    """Assert that a response has the expected status code."""
    if response.status_code != code:
        pytest.fail(f"Request failed: {response.status_code} {response.text}")
