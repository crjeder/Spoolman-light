"""Functions for exporting data."""

import asyncio
import io
from enum import Enum
from typing import Annotated

from fastapi import APIRouter, Depends, Response

from spoolman.api.v1.models import Filament, Spool
from spoolman.export import dump_as_csv, dump_as_json
from spoolman.storage.dependencies import get_store
from spoolman.storage.store import JsonStore

# ruff: noqa: D103,B008
router = APIRouter(
    prefix="/export",
    tags=["export"],
)


class ExportFormat(Enum):
    CSV = "csv"
    JSON = "json"


@router.get(
    "/spools",
    name="Export spools",
    description="Export the list of spools in various formats. Filament data is included.",
)
async def export_spools(
    *,
    store: Annotated[JsonStore, Depends(get_store)],
    fmt: ExportFormat,
) -> Response:
    all_spools, _ = await asyncio.to_thread(store.find_spools)
    filament_map = {f.id: f for f in store._data.filaments}  # noqa: SLF001
    api_spools = []
    for s in all_spools:
        f = filament_map.get(s.filament_id)
        if f is None:
            continue
        api_spools.append(Spool.from_db(s, f))
    return _export(api_spools, fmt)


@router.get(
    "/filaments",
    name="Export filaments",
    description="Export the list of filaments in various formats.",
)
async def export_filaments(
    *,
    store: Annotated[JsonStore, Depends(get_store)],
    fmt: ExportFormat,
) -> Response:
    all_filaments, _ = await asyncio.to_thread(store.find_filaments)
    api_filaments = [Filament.from_db(f) for f in all_filaments]
    return _export(api_filaments, fmt)


def _export(objects: list, fmt: ExportFormat) -> Response:
    """Export objects in various formats."""
    buffer = io.StringIO()
    media_type = ""

    if fmt == ExportFormat.CSV:
        media_type = "text/csv"
        dump_as_csv(objects, buffer)
    elif fmt == ExportFormat.JSON:
        media_type = "application/json"
        dump_as_json(objects, buffer)
    else:
        raise ValueError(f"Unknown export format: {fmt}")

    return Response(content=buffer.getvalue(), media_type=media_type)
