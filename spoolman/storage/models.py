"""Pydantic storage models for the JSON data store."""

from datetime import datetime
from typing import Optional

from pydantic import BaseModel, Field


class FilamentModel(BaseModel):
    id: int
    registered: datetime
    name: Optional[str] = None
    vendor: Optional[str] = None
    material: Optional[str] = None
    density: float
    diameter: float
    settings_extruder_temp: Optional[int] = None
    settings_bed_temp: Optional[int] = None
    comment: Optional[str] = None
    extra: dict[str, str] = Field(default_factory=dict)


class SpoolModel(BaseModel):
    id: int
    registered: datetime
    first_used: Optional[datetime] = None
    last_used: Optional[datetime] = None
    price: Optional[float] = None
    filament_id: int
    initial_weight: Optional[float] = None
    spool_weight: Optional[float] = None
    used_weight: float
    color_hex: Optional[str] = None
    multi_color_hexes: Optional[str] = None
    multi_color_direction: Optional[str] = None
    location: Optional[str] = None
    comment: Optional[str] = None
    archived: bool = False
    extra: dict[str, str] = Field(default_factory=dict)


class DataStore(BaseModel):
    meta: dict = Field(default_factory=lambda: {"schema_version": 2})
    filaments: list[FilamentModel] = Field(default_factory=list)
    spools: list[SpoolModel] = Field(default_factory=list)
    settings: dict[str, str] = Field(default_factory=dict)
