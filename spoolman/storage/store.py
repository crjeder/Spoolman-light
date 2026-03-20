"""JSON file-based storage backend for Spoolman."""

import json
import logging
import os
import threading
from datetime import datetime, timezone
from pathlib import Path
from typing import Optional

from spoolman.exceptions import ItemNotFoundError
from spoolman.storage.models import DataStore, FilamentModel, SpoolModel

logger = logging.getLogger(__name__)


def _utcnow() -> datetime:
    return datetime.now(tz=timezone.utc).replace(microsecond=0, tzinfo=None)


class JsonStore:
    """In-memory store backed by a single JSON file."""

    def __init__(self, path: Path) -> None:
        self._path = path
        self._data = DataStore()
        self._lock = threading.RLock()

    def load(self) -> None:
        """Load data from disk, or create a new empty file if none exists."""
        if not self._path.exists():
            logger.info("No data file found at %s, creating new empty store.", self._path)
            self._flush()
            return
        with self._path.open(encoding="utf-8") as f:
            raw = json.load(f)
        self._data = DataStore.model_validate(raw)
        logger.info(
            "Loaded %d filaments, %d spools from %s",
            len(self._data.filaments),
            len(self._data.spools),
            self._path,
        )

    def _flush(self) -> None:
        """Atomically write data to disk."""
        with self._lock:
            tmp = self._path.with_suffix(".tmp")
            self._path.parent.mkdir(parents=True, exist_ok=True)
            with tmp.open("w", encoding="utf-8") as f:
                f.write(self._data.model_dump_json(indent=2))
            os.replace(tmp, self._path)

    # ── Filament ──────────────────────────────────────────────────────────────

    def _next_filament_id(self) -> int:
        if not self._data.filaments:
            return 1
        return max(f.id for f in self._data.filaments) + 1

    def create_filament(
        self,
        *,
        density: float,
        diameter: float,
        name: Optional[str] = None,
        vendor: Optional[str] = None,
        material: Optional[str] = None,
        settings_extruder_temp: Optional[int] = None,
        settings_bed_temp: Optional[int] = None,
        comment: Optional[str] = None,
        extra: Optional[dict[str, str]] = None,
    ) -> FilamentModel:
        filament = FilamentModel(
            id=self._next_filament_id(),
            registered=_utcnow(),
            name=name,
            vendor=vendor,
            material=material,
            density=density,
            diameter=diameter,
            settings_extruder_temp=settings_extruder_temp,
            settings_bed_temp=settings_bed_temp,
            comment=comment,
            extra=extra or {},
        )
        self._data.filaments.append(filament)
        self._flush()
        return filament

    def get_filament(self, filament_id: int) -> FilamentModel:
        for f in self._data.filaments:
            if f.id == filament_id:
                return f
        raise ItemNotFoundError(f"No filament with ID {filament_id} found.")

    def find_filaments(
        self,
        *,
        ids: Optional[list[int]] = None,
        vendor: Optional[str] = None,
        name: Optional[str] = None,
        material: Optional[str] = None,
        sort_by: Optional[dict[str, str]] = None,
        limit: Optional[int] = None,
        offset: int = 0,
    ) -> tuple[list[FilamentModel], int]:
        result = list(self._data.filaments)
        if ids is not None:
            result = [f for f in result if f.id in ids]
        if vendor is not None:
            result = _filter_str_opt(result, "vendor", vendor)
        if name is not None:
            result = _filter_str_opt(result, "name", name)
        if material is not None:
            result = _filter_str_opt(result, "material", material)
        if sort_by:
            result = _sort(result, sort_by)
        total = len(result)
        if limit is not None:
            result = result[offset : offset + limit]
        else:
            result = result[offset:]
        return result, total

    def update_filament(self, filament_id: int, data: dict) -> FilamentModel:
        filament = self.get_filament(filament_id)
        idx = self._data.filaments.index(filament)
        updated = filament.model_copy(update=data)
        self._data.filaments[idx] = updated
        self._flush()
        return updated

    def delete_filament(self, filament_id: int) -> FilamentModel:
        filament = self.get_filament(filament_id)
        # Check no spools reference this filament
        if any(s.filament_id == filament_id for s in self._data.spools):
            from spoolman.exceptions import ItemDeleteError
            raise ItemDeleteError("Cannot delete filament: it is referenced by one or more spools.")
        self._data.filaments = [f for f in self._data.filaments if f.id != filament_id]
        self._flush()
        return filament

    def find_materials(self) -> list[str]:
        return sorted({f.material for f in self._data.filaments if f.material is not None})

    def find_vendors(self) -> list[str]:
        return sorted({f.vendor for f in self._data.filaments if f.vendor is not None})

    def clear_extra_field_filaments(self, key: str) -> None:
        for i, f in enumerate(self._data.filaments):
            if key in f.extra:
                new_extra = {k: v for k, v in f.extra.items() if k != key}
                self._data.filaments[i] = f.model_copy(update={"extra": new_extra})
        self._flush()

    # ── Spool ─────────────────────────────────────────────────────────────────

    def _next_spool_id(self) -> int:
        if not self._data.spools:
            return 1
        return max(s.id for s in self._data.spools) + 1

    def create_spool(
        self,
        *,
        filament_id: int,
        remaining_weight: Optional[float] = None,
        initial_weight: Optional[float] = None,
        spool_weight: Optional[float] = None,
        used_weight: Optional[float] = None,
        first_used: Optional[datetime] = None,
        last_used: Optional[datetime] = None,
        price: Optional[float] = None,
        color_hex: Optional[str] = None,
        multi_color_hexes: Optional[str] = None,
        multi_color_direction: Optional[str] = None,
        location: Optional[str] = None,
        comment: Optional[str] = None,
        archived: bool = False,
        extra: Optional[dict[str, str]] = None,
    ) -> SpoolModel:
        from spoolman.exceptions import ItemCreateError
        self.get_filament(filament_id)  # validate filament exists

        if used_weight is None:
            if remaining_weight is not None:
                if initial_weight is None or initial_weight == 0:
                    raise ItemCreateError(
                        "remaining_weight can only be used if the initial_weight is "
                        "defined or the filament has a weight set.",
                    )
                used_weight = max(initial_weight - remaining_weight, 0)
            else:
                used_weight = 0

        first_used = _utc_naive(first_used)
        last_used = _utc_naive(last_used)

        spool = SpoolModel(
            id=self._next_spool_id(),
            registered=_utcnow(),
            filament_id=filament_id,
            initial_weight=initial_weight,
            spool_weight=spool_weight,
            used_weight=used_weight,
            price=price,
            color_hex=color_hex,
            multi_color_hexes=multi_color_hexes,
            multi_color_direction=multi_color_direction,
            first_used=first_used,
            last_used=last_used,
            location=location,
            comment=comment,
            archived=archived,
            extra=extra or {},
        )
        self._data.spools.append(spool)
        self._flush()
        return spool

    def get_spool(self, spool_id: int) -> SpoolModel:
        for s in self._data.spools:
            if s.id == spool_id:
                return s
        raise ItemNotFoundError(f"No spool with ID {spool_id} found.")

    def find_spools(
        self,
        *,
        filament_name: Optional[str] = None,
        filament_id: Optional[list[int]] = None,
        filament_material: Optional[str] = None,
        filament_vendor: Optional[str] = None,
        color_hex: Optional[str] = None,
        color_similarity_threshold: float = 20.0,
        location: Optional[str] = None,
        allow_archived: bool = False,
        sort_by: Optional[dict[str, str]] = None,
        limit: Optional[int] = None,
        offset: int = 0,
    ) -> tuple[list[SpoolModel], int]:
        result = list(self._data.spools)
        if not allow_archived:
            result = [s for s in result if not s.archived]
        if filament_id is not None:
            result = [s for s in result if s.filament_id in filament_id]
        if filament_name is not None:
            fids = {f.id for f in self._data.filaments if _str_matches(f.name, filament_name)}
            result = [s for s in result if s.filament_id in fids]
        if filament_material is not None:
            fids = {f.id for f in self._data.filaments if _str_matches(f.material, filament_material)}
            result = [s for s in result if s.filament_id in fids]
        if filament_vendor is not None:
            fids = {f.id for f in self._data.filaments if _str_matches(f.vendor, filament_vendor)}
            result = [s for s in result if s.filament_id in fids]
        if color_hex is not None:
            matched_ids = {s.id for s in self.find_spools_by_color(color_hex, color_similarity_threshold)}
            result = [s for s in result if s.id in matched_ids]
        if location is not None:
            result = _filter_str_opt(result, "location", location)
        if sort_by:
            result = _sort_spools(result, sort_by, self._data.filaments)
        total = len(result)
        if limit is not None:
            result = result[offset : offset + limit]
        else:
            result = result[offset:]
        return result, total

    def update_spool(self, spool_id: int, data: dict) -> SpoolModel:
        from spoolman.exceptions import ItemCreateError
        spool = self.get_spool(spool_id)
        idx = self._data.spools.index(spool)
        update_dict = {}
        for k, v in data.items():
            if k == "filament_id":
                self.get_filament(v)  # validate exists
                update_dict["filament_id"] = v
            elif k == "remaining_weight":
                if spool.initial_weight is None:
                    raise ItemCreateError("remaining_weight can only be used if initial_weight is set.")
                update_dict["used_weight"] = max(spool.initial_weight - v, 0)
            elif isinstance(v, datetime):
                update_dict[k] = _utc_naive(v)
            elif k == "extra":
                merged = dict(spool.extra)
                merged.update(v)
                update_dict["extra"] = merged
            elif k == "multi_color_direction":
                update_dict[k] = v.value if v is not None else None
            else:
                update_dict[k] = v
        updated = spool.model_copy(update=update_dict)
        self._data.spools[idx] = updated
        self._flush()
        return updated

    def delete_spool(self, spool_id: int) -> SpoolModel:
        spool = self.get_spool(spool_id)
        self._data.spools = [s for s in self._data.spools if s.id != spool_id]
        self._flush()
        return spool

    def use_weight(self, spool_id: int, weight: float) -> SpoolModel:
        spool = self.get_spool(spool_id)
        idx = self._data.spools.index(spool)
        new_used = max(spool.used_weight + weight, 0)
        first_used = spool.first_used if spool.first_used is not None else _utcnow()
        updated = spool.model_copy(update={
            "used_weight": new_used,
            "first_used": first_used,
            "last_used": _utcnow(),
        })
        self._data.spools[idx] = updated
        self._flush()
        return updated

    def use_length(self, spool_id: int, length: float) -> SpoolModel:
        from spoolman.math import weight_from_length
        spool = self.get_spool(spool_id)
        filament = self.get_filament(spool.filament_id)
        weight = weight_from_length(length=length, diameter=filament.diameter, density=filament.density)
        return self.use_weight(spool_id, weight)

    def measure_spool(self, spool_id: int, weight: float) -> SpoolModel:
        from spoolman.exceptions import SpoolMeasureError
        spool = self.get_spool(spool_id)

        initial_weight = spool.initial_weight or 0
        spool_weight = spool.spool_weight or 0

        if initial_weight == 0:
            raise SpoolMeasureError("Initial weight is not set.")

        initial_gross = initial_weight + spool_weight

        if weight > initial_gross:
            return self.reset_initial_weight(spool_id, weight - spool_weight)

        current_use = initial_gross - spool.used_weight
        weight_to_use = current_use - weight

        if (initial_gross - weight_to_use) < spool_weight:
            weight_to_use = current_use - spool_weight

        return self.use_weight(spool_id, weight_to_use)

    def reset_initial_weight(self, spool_id: int, weight: float) -> SpoolModel:
        spool = self.get_spool(spool_id)
        idx = self._data.spools.index(spool)
        updated = spool.model_copy(update={"initial_weight": weight, "used_weight": 0})
        self._data.spools[idx] = updated
        self._flush()
        return updated

    def find_locations(self) -> list[str]:
        return sorted({s.location for s in self._data.spools if s.location is not None})

    def rename_location(self, current_name: str, new_name: str) -> None:
        for i, s in enumerate(self._data.spools):
            if s.location == current_name:
                self._data.spools[i] = s.model_copy(update={"location": new_name})
        self._flush()

    def find_spools_by_color(self, color_query_hex: str, similarity_threshold: float = 25) -> list[SpoolModel]:
        from spoolman.math import delta_e, hex_to_rgb, rgb_to_lab
        color_query_lab = rgb_to_lab(hex_to_rgb(color_query_hex))
        found: list[SpoolModel] = []
        for spool in self._data.spools:
            if spool.color_hex is not None:
                colors = [spool.color_hex]
            elif spool.multi_color_hexes is not None:
                colors = spool.multi_color_hexes.split(",")
            else:
                continue
            for color in colors:
                color_lab = rgb_to_lab(hex_to_rgb(color))
                if delta_e(color_query_lab, color_lab) <= similarity_threshold:
                    found.append(spool)
                    break
        return found

    def clear_extra_field_spools(self, key: str) -> None:
        for i, s in enumerate(self._data.spools):
            if key in s.extra:
                new_extra = {k: v for k, v in s.extra.items() if k != key}
                self._data.spools[i] = s.model_copy(update={"extra": new_extra})
        self._flush()

    # ── Settings ──────────────────────────────────────────────────────────────

    def get_setting(self, key: str) -> Optional[str]:
        return self._data.settings.get(key)

    def get_all_settings(self) -> dict[str, str]:
        return dict(self._data.settings)

    def set_setting(self, key: str, value: str) -> None:
        self._data.settings[key] = value
        self._flush()

    def delete_setting(self, key: str) -> None:
        self._data.settings.pop(key, None)
        self._flush()


# ── Filtering / sorting helpers ────────────────────────────────────────────────

def _parse_search_terms(query: str) -> list[tuple[str, bool]]:
    """Parse search query into (term, exact) tuples."""
    terms = []
    for raw in query.split(","):
        raw = raw.strip()
        if raw.startswith('"') and raw.endswith('"'):
            terms.append((raw[1:-1], True))
        else:
            terms.append((raw, False))
    return terms


def _str_matches(value: Optional[str], query: str) -> bool:
    """Return True if value matches any of the comma-separated search terms."""
    if query == "":
        return value is None or value == ""
    terms = _parse_search_terms(query)
    for term, exact in terms:
        if term == "":
            if value is None or value == "":
                return True
        elif value is not None:
            if exact:
                if value == term:
                    return True
            elif term.lower() in value.lower():
                return True
    return False


def _filter_str(items: list, attr: str, query: str) -> list:
    return [item for item in items if _str_matches(getattr(item, attr, None), query)]


def _filter_str_opt(items: list, attr: str, query: str) -> list:
    return [item for item in items if _str_matches(getattr(item, attr, None), query)]


def _sort(items: list, sort_by: dict[str, str]) -> list:
    for field, direction in reversed(list(sort_by.items())):
        reverse = direction.upper() == "DESC"
        items = sorted(items, key=lambda x: (getattr(x, field, None) is None, getattr(x, field, None)), reverse=reverse)
    return items


def _sort_spools(
    spools: list[SpoolModel],
    sort_by: dict[str, str],
    filaments: list[FilamentModel],
) -> list[SpoolModel]:
    filament_map = {f.id: f for f in filaments}
    for field, direction in reversed(list(sort_by.items())):
        reverse = direction.upper() == "DESC"
        if field == "remaining_weight":
            def key_rw(s: SpoolModel, fm: dict = filament_map) -> tuple:  # noqa: ANN001
                iw = s.initial_weight
                val = (iw - s.used_weight) if iw is not None else None
                return (val is None, val)
            spools = sorted(spools, key=key_rw, reverse=reverse)
        elif field == "remaining_length":
            def key_rl(s: SpoolModel, fm: dict = filament_map) -> tuple:  # noqa: ANN001
                f = fm.get(s.filament_id)
                if f is None or s.initial_weight is None:
                    return (True, None)
                val = (s.initial_weight - s.used_weight) / f.density / (f.diameter * f.diameter)
                return (False, val)
            spools = sorted(spools, key=key_rl, reverse=reverse)
        elif field == "used_length":
            def key_ul(s: SpoolModel, fm: dict = filament_map) -> tuple:  # noqa: ANN001
                f = fm.get(s.filament_id)
                if f is None:
                    return (True, None)
                val = s.used_weight / f.density / (f.diameter * f.diameter)
                return (False, val)
            spools = sorted(spools, key=key_ul, reverse=reverse)
        elif field == "filament.combined_name":
            def key_cn(s: SpoolModel, fm: dict = filament_map) -> tuple:  # noqa: ANN001
                f = fm.get(s.filament_id)
                return (f.vendor if f and f.vendor else "", f.name if f and f.name else "")
            spools = sorted(spools, key=key_cn, reverse=reverse)
        elif field == "price":
            def key_price(s: SpoolModel) -> tuple:  # noqa: ANN001
                return (s.price is None, s.price)
            spools = sorted(spools, key=key_price, reverse=reverse)
        elif field.startswith("filament."):
            sub = field[len("filament."):]
            def key_f(s: SpoolModel, sub: str = sub, fm: dict = filament_map) -> tuple:  # noqa: ANN001
                f = fm.get(s.filament_id)
                val = getattr(f, sub, None) if f else None
                return (val is None, val)
            spools = sorted(spools, key=key_f, reverse=reverse)
        else:
            spools = sorted(spools, key=lambda s, fld=field: (getattr(s, fld, None) is None, getattr(s, fld, None)), reverse=reverse)
    return spools


def _utc_naive(dt: Optional[datetime]) -> Optional[datetime]:
    if dt is None:
        return None
    if dt.tzinfo is not None:
        return dt.astimezone(tz=timezone.utc).replace(tzinfo=None)
    return dt
