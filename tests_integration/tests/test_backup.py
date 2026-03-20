"""Integration tests for the backup API endpoint."""

import httpx

from .conftest import URL


def test_backup():
    """Test triggering an automatic data file backup."""
    result = httpx.post(f"{URL}/api/v1/backup")
    result.raise_for_status()
