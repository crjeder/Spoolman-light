"""FastAPI dependency for accessing the singleton JsonStore."""

from typing import Generator

from spoolman.storage.store import JsonStore

_store: JsonStore | None = None


def set_store(store: JsonStore) -> None:
    """Set the global store instance (called at app startup)."""
    global _store  # noqa: PLW0603
    _store = store


def get_store() -> Generator[JsonStore, None, None]:
    """FastAPI dependency that yields the JsonStore instance."""
    if _store is None:
        raise RuntimeError("JsonStore has not been initialized.")
    yield _store
