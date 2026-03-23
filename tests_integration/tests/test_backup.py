"""Integration tests for data export (replaces the old manual backup endpoint test).

The Rust backend performs backups automatically in the background; there is no
API endpoint to trigger a backup manually.  This module tests the export
endpoint instead, which is the user-facing equivalent.
"""

import httpx

from .conftest import URL


def test_export_returns_full_store():
    """Test that GET /api/v1/export returns a valid DataStore JSON."""
    result = httpx.get(f"{URL}/api/v1/export")
    result.raise_for_status()

    data = result.json()
    assert "meta" in data
    assert "filaments" in data
    assert "spools" in data
    assert isinstance(data["filaments"], list)
    assert isinstance(data["spools"], list)
