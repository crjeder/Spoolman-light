"""Pydantic data models for typing the FastAPI request/responses."""

from datetime import datetime, timezone
from enum import Enum
from typing import TYPE_CHECKING, Annotated, Literal, Optional

from pydantic import BaseModel, Field, PlainSerializer

from spoolman.math import length_from_weight
from spoolman.settings import SettingDefinition, SettingType

if TYPE_CHECKING:
    from spoolman.storage.models import FilamentModel, SpoolModel


def datetime_to_str(dt: datetime) -> str:
    """Convert a datetime object to a string."""
    if dt.tzinfo is None:
        dt = dt.replace(tzinfo=timezone.utc)
    return dt.isoformat().replace("+00:00", "Z")


SpoolmanDateTime = Annotated[datetime, PlainSerializer(datetime_to_str)]


class Message(BaseModel):
    message: str = Field()


class SettingResponse(BaseModel):
    value: str = Field(description="Setting value.")
    is_set: bool = Field(description="Whether the setting has been set. If false, 'value' contains the default value.")
    type: SettingType = Field(description="Setting type. This corresponds with JSON types.")


class SettingKV(BaseModel):
    key: str = Field(description="Setting key.")
    setting: SettingResponse = Field(description="Setting value.")

    @staticmethod
    def from_db(definition: SettingDefinition, set_value: Optional[str]) -> "SettingKV":
        """Create a new Pydantic setting object from a storage setting object."""
        return SettingKV(
            key=definition.key,
            setting=SettingResponse(
                value=set_value if set_value is not None else definition.default,
                is_set=set_value is not None,
                type=definition.type,
            ),
        )


class MultiColorDirection(Enum):
    """Enum for multi-color direction."""

    COAXIAL = "coaxial"
    LONGITUDINAL = "longitudinal"


class Filament(BaseModel):
    id: int = Field(description="Unique internal ID of this filament type.")
    registered: SpoolmanDateTime = Field(description="When the filament was registered in the database. UTC Timezone.")
    name: Optional[str] = Field(None, max_length=64, description="Filament name.", examples=["PolyTerra™ PLA"])
    vendor: Optional[str] = Field(None, max_length=64, description="Vendor/brand name.", examples=["Polymaker"])
    material: Optional[str] = Field(None, max_length=64, description="The material of this filament, e.g. PLA.", examples=["PLA"])
    density: float = Field(gt=0, description="The density of this filament in g/cm3.", examples=[1.24])
    diameter: float = Field(gt=0, description="The diameter of this filament in mm.", examples=[1.75])
    settings_extruder_temp: Optional[int] = Field(None, ge=0, description="Overridden extruder temperature, in °C.", examples=[210])
    settings_bed_temp: Optional[int] = Field(None, ge=0, description="Overridden bed temperature, in °C.", examples=[60])
    comment: Optional[str] = Field(None, max_length=1024, description="Free text comment about this filament type.", examples=[""])
    extra: dict[str, str] = Field(
        description=(
            "Extra fields for this filament. All values are JSON-encoded data. "
            "Query the /fields endpoint for more details about the fields."
        ),
    )

    @staticmethod
    def from_db(item: "FilamentModel") -> "Filament":
        """Create a new Pydantic filament object from a storage filament object."""
        return Filament(
            id=item.id,
            registered=item.registered,
            name=item.name,
            vendor=item.vendor,
            material=item.material,
            density=item.density,
            diameter=item.diameter,
            settings_extruder_temp=item.settings_extruder_temp,
            settings_bed_temp=item.settings_bed_temp,
            comment=item.comment,
            extra=item.extra,
        )


class Spool(BaseModel):
    id: int = Field(description="Unique internal ID of this spool of filament.")
    registered: SpoolmanDateTime = Field(description="When the spool was registered in the database. UTC Timezone.")
    first_used: Optional[SpoolmanDateTime] = Field(None, description="First logged occurence of spool usage. UTC Timezone.")
    last_used: Optional[SpoolmanDateTime] = Field(None, description="Last logged occurence of spool usage. UTC Timezone.")
    filament: Filament = Field(description="The filament type of this spool.")
    price: Optional[float] = Field(None, ge=0, description="The price of this spool in the system configured currency.", examples=[20.0])
    color_hex: Optional[str] = Field(
        None,
        min_length=6,
        max_length=8,
        description=(
            "Hexadecimal color code of the filament on this spool, e.g. FF0000 for red. "
            "Supports alpha channel at the end. If multi-color, use multi_color_hexes instead."
        ),
        examples=["FF0000"],
    )
    multi_color_hexes: Optional[str] = Field(
        None,
        min_length=6,
        description="Multiple hexadecimal color codes separated by commas.",
        examples=["FF0000,00FF00,0000FF"],
    )
    multi_color_direction: Optional[MultiColorDirection] = Field(
        None,
        description="Type of multi-color filament. Only set if multi_color_hexes is set.",
        examples=["coaxial", "longitudinal"],
    )
    remaining_weight: Optional[float] = Field(
        default=None,
        ge=0,
        description="Estimated remaining weight of filament on the spool in grams. Only set if initial_weight is set.",
        examples=[500.6],
    )
    initial_weight: Optional[float] = Field(
        default=None,
        ge=0,
        description="The initial weight, in grams, of the filament on the spool (net weight).",
        examples=[1000],
    )
    spool_weight: Optional[float] = Field(
        default=None,
        ge=0,
        description="Weight of an empty spool (tare weight).",
        examples=[200],
    )
    used_weight: float = Field(ge=0, description="Consumed weight of filament from the spool in grams.", examples=[500.3])
    remaining_length: Optional[float] = Field(
        default=None,
        ge=0,
        description="Estimated remaining length of filament on the spool in millimeters. Only set if initial_weight is set.",
        examples=[5612.4],
    )
    used_length: float = Field(ge=0, description="Consumed length of filament from the spool in millimeters.", examples=[50.7])
    location: Optional[str] = Field(None, max_length=64, description="Where this spool can be found.", examples=["Shelf A"])
    comment: Optional[str] = Field(None, max_length=1024, description="Free text comment about this specific spool.", examples=[""])
    archived: bool = Field(description="Whether this spool is archived and should not be used anymore.")
    extra: dict[str, str] = Field(
        description=(
            "Extra fields for this spool. All values are JSON-encoded data. "
            "Query the /fields endpoint for more details about the fields."
        ),
    )

    @staticmethod
    def from_db(item: "SpoolModel", filament_model: "FilamentModel") -> "Spool":
        """Create a new Pydantic spool object from a storage spool object."""
        filament = Filament.from_db(filament_model)

        remaining_weight: Optional[float] = None
        remaining_length: Optional[float] = None

        if item.initial_weight is not None:
            remaining_weight = max(item.initial_weight - item.used_weight, 0)
            remaining_length = length_from_weight(
                weight=remaining_weight,
                density=filament.density,
                diameter=filament.diameter,
            )

        used_length = length_from_weight(
            weight=item.used_weight,
            density=filament.density,
            diameter=filament.diameter,
        )

        return Spool(
            id=item.id,
            registered=item.registered,
            first_used=item.first_used,
            last_used=item.last_used,
            filament=filament,
            price=item.price,
            color_hex=item.color_hex,
            multi_color_hexes=item.multi_color_hexes,
            multi_color_direction=(
                MultiColorDirection(item.multi_color_direction) if item.multi_color_direction is not None else None
            ),
            initial_weight=item.initial_weight,
            spool_weight=item.spool_weight,
            used_weight=item.used_weight,
            used_length=used_length,
            remaining_weight=remaining_weight,
            remaining_length=remaining_length,
            location=item.location,
            comment=item.comment,
            archived=item.archived,
            extra=item.extra,
        )


class Info(BaseModel):
    version: str = Field(examples=["0.7.0"])
    debug_mode: bool = Field(examples=[False])
    automatic_backups: bool = Field(examples=[True])
    data_dir: str = Field(examples=["/home/app/.local/share/spoolman"])
    logs_dir: str = Field(examples=["/home/app/.local/share/spoolman"])
    backups_dir: str = Field(examples=["/home/app/.local/share/spoolman/backups"])
    data_file: str = Field(examples=["/home/app/.local/share/spoolman/spoolman.json"])
    git_commit: Optional[str] = Field(None, examples=["a1b2c3d"])
    build_date: Optional[SpoolmanDateTime] = Field(None, examples=["2021-01-01T00:00:00Z"])


class HealthCheck(BaseModel):
    status: str = Field(examples=["healthy"])


class BackupResponse(BaseModel):
    path: str = Field(
        default=None,
        description="Path to the created backup file.",
        examples=["/home/app/.local/share/spoolman/backups/spoolman.db"],
    )


class EventType(str, Enum):
    """Event types."""

    ADDED = "added"
    UPDATED = "updated"
    DELETED = "deleted"


class Event(BaseModel):
    """Event."""

    type: EventType = Field(description="Event type.")
    resource: str = Field(description="Resource type.")
    date: SpoolmanDateTime = Field(description="When the event occured. UTC Timezone.")
    payload: BaseModel


class SpoolEvent(Event):
    """Event."""

    payload: Spool = Field(description="Updated spool.")
    resource: Literal["spool"] = Field(description="Resource type.")


class FilamentEvent(Event):
    """Event."""

    payload: Filament = Field(description="Updated filament.")
    resource: Literal["filament"] = Field(description="Resource type.")


class SettingEvent(Event):
    """Event."""

    payload: SettingKV = Field(description="Updated setting.")
    resource: Literal["setting"] = Field(description="Resource type.")
