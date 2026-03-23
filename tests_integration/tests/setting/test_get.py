"""Integration tests for the Setting API endpoint."""

import httpx

from ..conftest import URL


def test_get_all_settings_returns_dict():
    """Test that GET /api/v1/setting returns a dict."""
    result = httpx.get(f"{URL}/api/v1/setting")
    result.raise_for_status()

    settings = result.json()
    assert isinstance(settings, dict)


def test_get_setting_reflects_put():
    """Test that a PUT value is reflected in GET."""
    httpx.put(f"{URL}/api/v1/setting/currency_symbol", json={"value": "€"}).raise_for_status()

    result = httpx.get(f"{URL}/api/v1/setting")
    result.raise_for_status()
    settings = result.json()
    assert settings.get("currency_symbol") == "€"

    # Cleanup: remove the value by setting it to empty (no delete endpoint; overwrite)
    httpx.put(f"{URL}/api/v1/setting/currency_symbol", json={"value": ""}).raise_for_status()
