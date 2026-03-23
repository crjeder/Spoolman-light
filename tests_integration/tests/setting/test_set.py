"""Integration tests for the Setting API endpoint."""

import httpx

from ..conftest import URL


def test_set_setting():
    """Test setting a value via PUT."""
    result = httpx.put(
        f"{URL}/api/v1/setting/currency_symbol",
        json={"value": "€"},
    )
    assert result.status_code == 204

    # Verify the value was stored
    settings = httpx.get(f"{URL}/api/v1/setting").json()
    assert settings.get("currency_symbol") == "€"

    # Cleanup
    httpx.put(f"{URL}/api/v1/setting/currency_symbol", json={"value": ""}).raise_for_status()


def test_overwrite_setting():
    """Test that overwriting a setting with a new value works."""
    httpx.put(f"{URL}/api/v1/setting/currency_symbol", json={"value": "€"}).raise_for_status()
    httpx.put(f"{URL}/api/v1/setting/currency_symbol", json={"value": "$"}).raise_for_status()

    settings = httpx.get(f"{URL}/api/v1/setting").json()
    assert settings.get("currency_symbol") == "$"

    # Cleanup
    httpx.put(f"{URL}/api/v1/setting/currency_symbol", json={"value": ""}).raise_for_status()


def test_set_multiple_settings():
    """Test setting multiple distinct keys."""
    httpx.put(f"{URL}/api/v1/setting/currency_symbol", json={"value": "£"}).raise_for_status()
    httpx.put(f"{URL}/api/v1/setting/locale", json={"value": "en-GB"}).raise_for_status()

    settings = httpx.get(f"{URL}/api/v1/setting").json()
    assert settings.get("currency_symbol") == "£"
    assert settings.get("locale") == "en-GB"

    # Cleanup
    httpx.put(f"{URL}/api/v1/setting/currency_symbol", json={"value": ""}).raise_for_status()
    httpx.put(f"{URL}/api/v1/setting/locale", json={"value": ""}).raise_for_status()
