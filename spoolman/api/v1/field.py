"""Extra field management endpoints."""

import logging
from typing import Annotated, Union

from fastapi import APIRouter, Depends, Path
from fastapi.responses import JSONResponse

from spoolman.api.v1.models import Message
from spoolman.exceptions import ItemNotFoundError
from spoolman.extra_fields import (
    EntityType,
    ExtraField,
    ExtraFieldParameters,
    add_or_update_extra_field,
    delete_extra_field,
    get_extra_fields,
)
from spoolman.storage.dependencies import get_store
from spoolman.storage.store import JsonStore

router = APIRouter(
    prefix="/field",
    tags=["field"],
)

# ruff: noqa: D103,B008

logger = logging.getLogger(__name__)


@router.get(
    "/{entity_type}",
    name="Get extra fields",
    description="Get all extra fields for a specific entity type.",
    response_model_exclude_none=True,
)
async def get(
    store: Annotated[JsonStore, Depends(get_store)],
    entity_type: Annotated[EntityType, Path(description="Entity type this field is for")],
) -> list[ExtraField]:
    return get_extra_fields(store, entity_type)


@router.post(
    "/{entity_type}/{key}",
    name="Add or update extra field",
    response_model_exclude_none=True,
    response_model=list[ExtraField],
    responses={400: {"model": Message}},
)
async def update(
    store: Annotated[JsonStore, Depends(get_store)],
    entity_type: Annotated[EntityType, Path(description="Entity type this field is for")],
    key: Annotated[str, Path(min_length=1, max_length=64, regex="^[a-z0-9_]+$")],
    body: ExtraFieldParameters,
) -> Union[list[ExtraField], JSONResponse]:
    dict_body = body.model_dump()
    dict_body["key"] = key
    dict_body["entity_type"] = entity_type
    body_with_key = ExtraField.model_validate(dict_body)

    try:
        add_or_update_extra_field(store, entity_type, body_with_key)
    except ValueError as e:
        return JSONResponse(status_code=400, content=Message(message=str(e)).model_dump())

    return get_extra_fields(store, entity_type)


@router.delete(
    "/{entity_type}/{key}",
    name="Delete extra field",
    response_model_exclude_none=True,
    response_model=list[ExtraField],
    responses={404: {"model": Message}},
)
async def delete(
    store: Annotated[JsonStore, Depends(get_store)],
    entity_type: Annotated[EntityType, Path(description="Entity type this field is for")],
    key: Annotated[str, Path(min_length=1, max_length=64, regex="^[a-z0-9_]+$")],
) -> Union[list[ExtraField], JSONResponse]:
    try:
        delete_extra_field(store, entity_type, key)
    except ItemNotFoundError:
        return JSONResponse(
            status_code=404,
            content=Message(
                message=f"Extra field with key {key} does not exist for entity type {entity_type.name}",
            ).model_dump(),
        )

    return get_extra_fields(store, entity_type)
