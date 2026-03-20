"""Spool related endpoints."""

import asyncio
import logging
from datetime import datetime
from typing import Annotated, Optional

from fastapi import APIRouter, Depends, Query, WebSocket, WebSocketDisconnect
from fastapi.encoders import jsonable_encoder
from fastapi.responses import JSONResponse
from pydantic import BaseModel, Field, field_validator, model_validator

from spoolman.api.v1.models import EventType, Message, MultiColorDirection, Spool, SpoolEvent
from spoolman.exceptions import ItemCreateError, SpoolMeasureError
from spoolman.extra_fields import EntityType, get_extra_fields, validate_extra_field_dict
from spoolman.storage.dependencies import get_store
from spoolman.storage.models import SpoolModel
from spoolman.storage.store import JsonStore
from spoolman.ws import websocket_manager

logger = logging.getLogger(__name__)

router = APIRouter(
    prefix="/spool",
    tags=["spool"],
)

# ruff: noqa: D103,B008


def _spool_to_api(store: JsonStore, item: SpoolModel) -> Spool:
    filament = store.get_filament(item.filament_id)
    return Spool.from_db(item, filament)


class SpoolParameters(BaseModel):
    first_used: Optional[datetime] = Field(None, description="First logged occurence of spool usage.")
    last_used: Optional[datetime] = Field(None, description="Last logged occurence of spool usage.")
    filament_id: int = Field(description="The ID of the filament type of this spool.")
    price: Optional[float] = Field(None, ge=0, examples=[20.0])
    initial_weight: Optional[float] = Field(None, ge=0, examples=[1000])
    spool_weight: Optional[float] = Field(None, ge=0, examples=[200])
    remaining_weight: Optional[float] = Field(None, ge=0, examples=[800])
    used_weight: Optional[float] = Field(None, ge=0, examples=[200])
    color_hex: Optional[str] = Field(None, description="Hexadecimal color code, e.g. FF0000 for red.", examples=["FF0000"])
    multi_color_hexes: Optional[str] = Field(None, description="Multiple hex color codes separated by commas.", examples=["FF0000,00FF00,0000FF"])
    multi_color_direction: Optional[MultiColorDirection] = Field(None, examples=["coaxial", "longitudinal"])
    location: Optional[str] = Field(None, max_length=64, examples=["Shelf A"])
    comment: Optional[str] = Field(None, max_length=1024, examples=[""])
    archived: bool = Field(default=False, description="Whether this spool is archived and should not be used anymore.")
    extra: Optional[dict[str, str]] = Field(None, description="Extra fields for this spool.")

    @field_validator("color_hex")
    @classmethod
    def color_hex_validator(cls, v: Optional[str]) -> Optional[str]:
        if not v:
            return None
        clr = v.upper().removeprefix("#")
        for c in clr:
            if c not in "0123456789ABCDEF":
                raise ValueError("Invalid character in color code.")
        if len(clr) not in (6, 8):  # noqa: PLR2004
            raise ValueError("Color code must be 6 or 8 characters long.")
        return clr

    @field_validator("multi_color_hexes")
    @classmethod
    def multi_color_hexes_validator(cls, v: Optional[str]) -> Optional[str]:
        if not v:
            return None
        for clr_raw in v.split(","):
            clr = clr_raw.upper().removeprefix("#")
            for c in clr:
                if c not in "0123456789ABCDEF":
                    raise ValueError("Invalid character in color code.")
            if len(clr) not in (6, 8):  # noqa: PLR2004
                raise ValueError("Color code must be 6 or 8 characters long.")
        return v

    @model_validator(mode="after")
    def validate(self) -> "SpoolParameters":
        if self.color_hex and self.multi_color_hexes:
            raise ValueError("Cannot specify both color_hex and multi_color_hexes.")
        if self.multi_color_hexes and len(self.multi_color_hexes.split(",")) < 2:  # noqa: PLR2004
            raise ValueError("Must specify at least two colors in multi_color_hexes.")
        if self.multi_color_hexes and not self.multi_color_direction:
            raise ValueError("Multi-color spool must have multi_color_direction set.")
        if not self.multi_color_hexes and self.multi_color_direction:
            raise ValueError("Single-color spool must not have multi_color_direction set.")
        return self


class SpoolUpdateParameters(SpoolParameters):
    filament_id: Optional[int] = Field(None, description="The ID of the filament type of this spool.")

    @field_validator("filament_id")
    @classmethod
    def prevent_none(cls: type["SpoolUpdateParameters"], v: Optional[int]) -> Optional[int]:
        """Prevent filament_id from being None."""
        if v is None:
            raise ValueError("Value must not be None.")
        return v


class SpoolUseParameters(BaseModel):
    use_length: Optional[float] = Field(None, description="Length of filament to reduce by, in mm.", examples=[2.2])
    use_weight: Optional[float] = Field(None, description="Filament weight to reduce by, in g.", examples=[5.3])


class SpoolMeasureParameters(BaseModel):
    weight: float = Field(description="Current gross weight of the spool, in g.", examples=[200])


async def _spool_changed(store: JsonStore, spool_id: int, typ: EventType) -> None:
    try:
        item = store.get_spool(spool_id)
        await websocket_manager.send(
            ("spool", str(spool_id)),
            SpoolEvent(
                type=typ,
                resource="spool",
                date=datetime.utcnow(),
                payload=_spool_to_api(store, item),
            ),
        )
    except Exception:
        logger.exception("Failed to send websocket message")


@router.get(
    "",
    name="Find spool",
    response_model_exclude_none=True,
    responses={
        200: {"model": list[Spool]},
        299: {"model": SpoolEvent, "description": "Websocket message"},
    },
)
async def find(
    *,
    store: Annotated[JsonStore, Depends(get_store)],
    filament_name: Annotated[Optional[str], Query(alias="filament.name")] = None,
    filament_id: Annotated[Optional[str], Query(alias="filament.id", pattern=r"^-?\d+(,-?\d+)*$")] = None,
    filament_material: Annotated[Optional[str], Query(alias="filament.material")] = None,
    filament_vendor: Annotated[Optional[str], Query(alias="filament.vendor")] = None,
    location: Annotated[Optional[str], Query(title="Location")] = None,
    allow_archived: Annotated[bool, Query(title="Allow Archived")] = False,
    sort: Annotated[Optional[str], Query(title="Sort")] = None,
    limit: Annotated[Optional[int], Query(title="Limit")] = None,
    offset: Annotated[int, Query(title="Offset")] = 0,
) -> JSONResponse:
    sort_by: dict[str, str] = {}
    if sort is not None:
        for sort_item in sort.split(","):
            field, direction = sort_item.split(":")
            sort_by[field] = direction

    filament_ids = [int(v) for v in filament_id.split(",")] if filament_id is not None else None

    items, total_count = await asyncio.to_thread(
        store.find_spools,
        filament_name=filament_name,
        filament_id=filament_ids,
        filament_material=filament_material,
        filament_vendor=filament_vendor,
        location=location,
        allow_archived=allow_archived,
        sort_by=sort_by,
        limit=limit,
        offset=offset,
    )

    return JSONResponse(
        content=jsonable_encoder(
            (_spool_to_api(store, item) for item in items),
            exclude_none=True,
        ),
        headers={"x-total-count": str(total_count)},
    )


@router.get(
    "/find-by-color",
    name="Find spools by color",
    response_model_exclude_none=True,
    responses={200: {"model": list[Spool]}},
)
async def find_by_color(
    *,
    store: Annotated[JsonStore, Depends(get_store)],
    color: Annotated[str, Query(description="Hexadecimal color to search for, e.g. FF0000.")],
    threshold: Annotated[float, Query(description="Similarity threshold (0-100).", ge=0, le=100)] = 20.0,
) -> JSONResponse:
    items = await asyncio.to_thread(store.find_spools_by_color, color, threshold)
    return JSONResponse(
        content=jsonable_encoder(
            (_spool_to_api(store, item) for item in items),
            exclude_none=True,
        ),
    )


@router.websocket("", name="Listen to spool changes")
async def notify_any(websocket: WebSocket) -> None:
    await websocket.accept()
    websocket_manager.connect(("spool",), websocket)
    try:
        while True:
            await asyncio.sleep(0.5)
            if await websocket.receive_text():
                await websocket.send_json({"status": "healthy"})
    except WebSocketDisconnect:
        websocket_manager.disconnect(("spool",), websocket)


@router.get(
    "/{spool_id}",
    name="Get spool",
    response_model_exclude_none=True,
    responses={404: {"model": Message}, 299: {"model": SpoolEvent, "description": "Websocket message"}},
)
async def get(
    store: Annotated[JsonStore, Depends(get_store)],
    spool_id: int,
) -> Spool:
    item = await asyncio.to_thread(store.get_spool, spool_id)
    return _spool_to_api(store, item)


@router.websocket("/{spool_id}", name="Listen to spool changes")
async def notify(websocket: WebSocket, spool_id: int) -> None:
    await websocket.accept()
    websocket_manager.connect(("spool", str(spool_id)), websocket)
    try:
        while True:
            await asyncio.sleep(0.5)
            if await websocket.receive_text():
                await websocket.send_json({"status": "healthy"})
    except WebSocketDisconnect:
        websocket_manager.disconnect(("spool", str(spool_id)), websocket)


@router.post(
    "",
    name="Add spool",
    response_model_exclude_none=True,
    response_model=Spool,
    responses={400: {"model": Message}},
)
async def create(  # noqa: ANN201
    store: Annotated[JsonStore, Depends(get_store)],
    body: SpoolParameters,
):
    if body.remaining_weight is not None and body.used_weight is not None:
        return JSONResponse(status_code=400, content={"message": "Only specify either remaining_weight or used_weight."})

    if body.extra:
        all_fields = get_extra_fields(store, EntityType.spool)
        try:
            validate_extra_field_dict(all_fields, body.extra)
        except ValueError as e:
            return JSONResponse(status_code=400, content=Message(message=str(e)).model_dump())

    try:
        item = await asyncio.to_thread(
            store.create_spool,
            filament_id=body.filament_id,
            price=body.price,
            initial_weight=body.initial_weight,
            spool_weight=body.spool_weight,
            remaining_weight=body.remaining_weight,
            used_weight=body.used_weight,
            first_used=body.first_used,
            last_used=body.last_used,
            color_hex=body.color_hex,
            multi_color_hexes=body.multi_color_hexes,
            multi_color_direction=body.multi_color_direction.value if body.multi_color_direction is not None else None,
            location=body.location,
            comment=body.comment,
            archived=body.archived,
            extra=body.extra,
        )
        await _spool_changed(store, item.id, EventType.ADDED)
        return _spool_to_api(store, item)
    except ItemCreateError:
        logger.exception("Failed to create spool.")
        return JSONResponse(status_code=400, content={"message": "Failed to create spool, see server logs for more information."})


@router.patch(
    "/{spool_id}",
    name="Update spool",
    response_model_exclude_none=True,
    response_model=Spool,
    responses={400: {"model": Message}, 404: {"model": Message}},
)
async def update(  # noqa: ANN201
    store: Annotated[JsonStore, Depends(get_store)],
    spool_id: int,
    body: SpoolUpdateParameters,
):
    patch_data = body.model_dump(exclude_unset=True)

    if body.remaining_weight is not None and body.used_weight is not None:
        return JSONResponse(status_code=400, content={"message": "Only specify either remaining_weight or used_weight."})

    if body.extra:
        all_fields = get_extra_fields(store, EntityType.spool)
        try:
            validate_extra_field_dict(all_fields, body.extra)
        except ValueError as e:
            return JSONResponse(status_code=400, content=Message(message=str(e)).model_dump())

    try:
        item = await asyncio.to_thread(store.update_spool, spool_id, patch_data)
    except ItemCreateError:
        logger.exception("Failed to update spool.")
        return JSONResponse(status_code=400, content={"message": "Failed to update spool, see server logs for more information."})

    await _spool_changed(store, item.id, EventType.UPDATED)
    return _spool_to_api(store, item)


@router.delete(
    "/{spool_id}",
    name="Delete spool",
    responses={404: {"model": Message}},
)
async def delete(
    store: Annotated[JsonStore, Depends(get_store)],
    spool_id: int,
) -> Message:
    item = await asyncio.to_thread(store.delete_spool, spool_id)
    try:
        await websocket_manager.send(
            ("spool", str(spool_id)),
            SpoolEvent(
                type=EventType.DELETED,
                resource="spool",
                date=datetime.utcnow(),
                payload=_spool_to_api(store, item),
            ),
        )
    except Exception:
        logger.exception("Failed to send websocket message")
    return Message(message="Success!")


@router.put(
    "/{spool_id}/use",
    name="Use spool filament",
    response_model_exclude_none=True,
    response_model=Spool,
    responses={400: {"model": Message}, 404: {"model": Message}},
)
async def use(  # noqa: ANN201
    store: Annotated[JsonStore, Depends(get_store)],
    spool_id: int,
    body: SpoolUseParameters,
):
    if body.use_weight is not None and body.use_length is not None:
        return JSONResponse(status_code=400, content={"message": "Only specify either use_weight or use_length."})

    if body.use_weight is not None:
        item = await asyncio.to_thread(store.use_weight, spool_id, body.use_weight)
        await _spool_changed(store, item.id, EventType.UPDATED)
        return _spool_to_api(store, item)

    if body.use_length is not None:
        item = await asyncio.to_thread(store.use_length, spool_id, body.use_length)
        await _spool_changed(store, item.id, EventType.UPDATED)
        return _spool_to_api(store, item)

    return JSONResponse(status_code=400, content={"message": "Either use_weight or use_length must be specified."})


@router.put(
    "/{spool_id}/measure",
    name="Use spool filament based on the current weight measurement",
    response_model_exclude_none=True,
    response_model=Spool,
    responses={400: {"model": Message}, 404: {"model": Message}},
)
async def measure(  # noqa: ANN201
    store: Annotated[JsonStore, Depends(get_store)],
    spool_id: int,
    body: SpoolMeasureParameters,
):
    try:
        item = await asyncio.to_thread(store.measure_spool, spool_id, body.weight)
        await _spool_changed(store, item.id, EventType.UPDATED)
        return _spool_to_api(store, item)
    except SpoolMeasureError as e:
        logger.exception("Failed to update spool measurement.")
        return JSONResponse(status_code=400, content={"message": e.args[0]})
