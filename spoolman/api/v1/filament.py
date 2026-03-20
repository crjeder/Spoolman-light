"""Filament related endpoints."""

import asyncio
import logging
from datetime import datetime
from typing import Annotated, Optional

from fastapi import APIRouter, Depends, Query, WebSocket, WebSocketDisconnect
from fastapi.encoders import jsonable_encoder
from fastapi.responses import JSONResponse
from pydantic import BaseModel, Field, field_validator

from spoolman.api.v1.models import EventType, Filament, FilamentEvent, Message
from spoolman.exceptions import ItemDeleteError
from spoolman.extra_fields import EntityType, get_extra_fields, validate_extra_field_dict
from spoolman.storage.dependencies import get_store
from spoolman.storage.models import FilamentModel
from spoolman.storage.store import JsonStore
from spoolman.ws import websocket_manager

logger = logging.getLogger(__name__)

router = APIRouter(
    prefix="/filament",
    tags=["filament"],
)

# ruff: noqa: D103, B008


def _filament_to_api(item: FilamentModel) -> Filament:
    return Filament.from_db(item)


class FilamentParameters(BaseModel):
    name: Optional[str] = Field(None, max_length=64, description="Filament name.", examples=["PolyTerra™ PLA"])
    vendor: Optional[str] = Field(None, max_length=64, description="Vendor/brand name.", examples=["Polymaker"])
    material: Optional[str] = Field(None, max_length=64, description="The material of this filament, e.g. PLA.", examples=["PLA"])
    density: float = Field(gt=0, description="The density of this filament in g/cm3.", examples=[1.24])
    diameter: float = Field(gt=0, description="The diameter of this filament in mm.", examples=[1.75])
    settings_extruder_temp: Optional[int] = Field(None, ge=0, description="Overridden extruder temperature, in °C.", examples=[210])
    settings_bed_temp: Optional[int] = Field(None, ge=0, description="Overridden bed temperature, in °C.", examples=[60])
    comment: Optional[str] = Field(None, max_length=1024, description="Free text comment about this filament type.", examples=[""])
    extra: Optional[dict[str, str]] = Field(None, description="Extra fields for this filament.")


class FilamentUpdateParameters(FilamentParameters):
    density: Optional[float] = Field(None, gt=0, examples=[1.24])
    diameter: Optional[float] = Field(None, gt=0, examples=[1.75])

    @field_validator("density", "diameter")
    @classmethod
    def prevent_none(cls: type["FilamentUpdateParameters"], v: Optional[float]) -> Optional[float]:
        """Prevent density and diameter from being None."""
        if v is None:
            raise ValueError("Value must not be None.")
        return v


async def _filament_changed(store: JsonStore, filament_id: int, typ: EventType) -> None:
    try:
        item = store.get_filament(filament_id)
        await websocket_manager.send(
            ("filament", str(filament_id)),
            FilamentEvent(
                type=typ,
                resource="filament",
                date=datetime.utcnow(),
                payload=Filament.from_db(item),
            ),
        )
    except Exception:
        logger.exception("Failed to send websocket message")


@router.get(
    "",
    name="Find filaments",
    response_model_exclude_none=True,
    responses={
        200: {"model": list[Filament]},
        299: {"model": FilamentEvent, "description": "Websocket message"},
    },
)
async def find(
    *,
    store: Annotated[JsonStore, Depends(get_store)],
    vendor: Annotated[Optional[str], Query(title="Vendor Name")] = None,
    name: Annotated[Optional[str], Query(title="Filament Name")] = None,
    material: Annotated[Optional[str], Query(title="Filament Material")] = None,
    sort: Annotated[Optional[str], Query(title="Sort", example="vendor:asc,material:desc")] = None,
    limit: Annotated[Optional[int], Query(title="Limit")] = None,
    offset: Annotated[int, Query(title="Offset")] = 0,
) -> JSONResponse:
    sort_by: dict[str, str] = {}
    if sort is not None:
        for sort_item in sort.split(","):
            field, direction = sort_item.split(":")
            sort_by[field] = direction

    items, total_count = await asyncio.to_thread(
        store.find_filaments,
        vendor=vendor,
        name=name,
        material=material,
        sort_by=sort_by,
        limit=limit,
        offset=offset,
    )

    return JSONResponse(
        content=jsonable_encoder(
            (_filament_to_api(item) for item in items),
            exclude_none=True,
        ),
        headers={"x-total-count": str(total_count)},
    )


@router.websocket("", name="Listen to filament changes")
async def notify_any(websocket: WebSocket) -> None:
    await websocket.accept()
    websocket_manager.connect(("filament",), websocket)
    try:
        while True:
            await asyncio.sleep(0.5)
            if await websocket.receive_text():
                await websocket.send_json({"status": "healthy"})
    except WebSocketDisconnect:
        websocket_manager.disconnect(("filament",), websocket)


@router.get(
    "/{filament_id}",
    name="Get filament",
    response_model_exclude_none=True,
    responses={404: {"model": Message}, 299: {"model": FilamentEvent, "description": "Websocket message"}},
)
async def get(
    store: Annotated[JsonStore, Depends(get_store)],
    filament_id: int,
) -> Filament:
    item = await asyncio.to_thread(store.get_filament, filament_id)
    return _filament_to_api(item)


@router.websocket("/{filament_id}", name="Listen to filament changes")
async def notify(websocket: WebSocket, filament_id: int) -> None:
    await websocket.accept()
    websocket_manager.connect(("filament", str(filament_id)), websocket)
    try:
        while True:
            await asyncio.sleep(0.5)
            if await websocket.receive_text():
                await websocket.send_json({"status": "healthy"})
    except WebSocketDisconnect:
        websocket_manager.disconnect(("filament", str(filament_id)), websocket)


@router.post(
    "",
    name="Add filament",
    response_model_exclude_none=True,
    response_model=Filament,
    responses={400: {"model": Message}},
)
async def create(  # noqa: ANN201
    store: Annotated[JsonStore, Depends(get_store)],
    body: FilamentParameters,
):
    if body.extra:
        all_fields = get_extra_fields(store, EntityType.filament)
        try:
            validate_extra_field_dict(all_fields, body.extra)
        except ValueError as e:
            return JSONResponse(status_code=400, content=Message(message=str(e)).model_dump())

    item = await asyncio.to_thread(
        store.create_filament,
        density=body.density,
        diameter=body.diameter,
        name=body.name,
        vendor=body.vendor,
        material=body.material,
        settings_extruder_temp=body.settings_extruder_temp,
        settings_bed_temp=body.settings_bed_temp,
        comment=body.comment,
        extra=body.extra,
    )
    await _filament_changed(store, item.id, EventType.ADDED)
    return _filament_to_api(item)


@router.patch(
    "/{filament_id}",
    name="Update filament",
    response_model_exclude_none=True,
    response_model=Filament,
    responses={400: {"model": Message}, 404: {"model": Message}},
)
async def update(  # noqa: ANN201
    store: Annotated[JsonStore, Depends(get_store)],
    filament_id: int,
    body: FilamentUpdateParameters,
):
    patch_data = body.model_dump(exclude_unset=True)

    if body.extra:
        all_fields = get_extra_fields(store, EntityType.filament)
        try:
            validate_extra_field_dict(all_fields, body.extra)
        except ValueError as e:
            return JSONResponse(status_code=400, content=Message(message=str(e)).model_dump())

    item = await asyncio.to_thread(store.update_filament, filament_id, patch_data)
    await _filament_changed(store, item.id, EventType.UPDATED)
    return _filament_to_api(item)


@router.delete(
    "/{filament_id}",
    name="Delete filament",
    response_model=Message,
    responses={403: {"model": Message}, 404: {"model": Message}},
)
async def delete(  # noqa: ANN201
    store: Annotated[JsonStore, Depends(get_store)],
    filament_id: int,
):
    try:
        item = await asyncio.to_thread(store.delete_filament, filament_id)
    except ItemDeleteError:
        logger.exception("Failed to delete filament.")
        return JSONResponse(
            status_code=403,
            content={"message": "Failed to delete filament, see server logs for more information."},
        )
    try:
        await websocket_manager.send(
            ("filament", str(filament_id)),
            FilamentEvent(
                type=EventType.DELETED,
                resource="filament",
                date=datetime.utcnow(),
                payload=Filament.from_db(item),
            ),
        )
    except Exception:
        logger.exception("Failed to send websocket message")
    return Message(message="Success!")
