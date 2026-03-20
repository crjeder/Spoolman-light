"""Functionality for exporting data in various formats."""

import csv
import json
from typing import TYPE_CHECKING, Any

from pydantic import BaseModel

if TYPE_CHECKING:
    from _typeshed import SupportsWrite


def flatten_pydantic_object(obj: BaseModel, parent_key: str = "", sep: str = ".") -> dict[str, Any]:
    """Recursively flatten a Pydantic model into a dictionary with dot-separated keys."""
    fields: dict[str, Any] = {}
    for key, value in obj.model_dump().items():
        full_key = f"{parent_key}{key}" if not parent_key else f"{parent_key}{sep}{key}"
        if isinstance(value, dict):
            for k, v in value.items():
                fields[f"{full_key}.{k}"] = v
        elif isinstance(value, BaseModel):
            fields.update(flatten_pydantic_object(value, full_key, sep=sep))
        else:
            fields[full_key] = value
    return fields


def dump_as_csv(objects: list[BaseModel], writer: "SupportsWrite[str]") -> None:
    """Export a list of Pydantic models as CSV."""
    all_flattened = [flatten_pydantic_object(obj) for obj in objects]

    headers: set[str] = set()
    for flattened in all_flattened:
        headers.update(flattened.keys())
    sorted_headers = sorted(headers)

    csv_writer = csv.DictWriter(writer, fieldnames=sorted_headers)
    csv_writer.writeheader()
    for flattened in all_flattened:
        csv_writer.writerow(flattened)


def dump_as_json(objects: list[BaseModel], writer: "SupportsWrite[str]") -> None:
    """Export a list of Pydantic models as JSON."""
    all_flattened = [flatten_pydantic_object(obj) for obj in objects]
    json.dump(all_flattened, writer, default=str)
