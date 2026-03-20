# TODO

Items to address. Move completed items to [CHANGELOG.md](CHANGELOG.md) under the appropriate release.

## Pending

### In Progress
- [ ] Remove DB (SQLAlchemy/Alembic) — replace with JSON storage (`openspec/changes/remove-db-use-json-storage/`)
- [ ] Run full integration test suite to verify `simplify-data-model` changes (`openspec/changes/simplify-data-model/`) — requires Docker Desktop running

### Enhancements
- [ ] Support `.env` files via `python-dotenv` (`load_dotenv()` at startup in `spoolman/env.py`)
- [ ] NFC or QR code stickers to identify spools and autmate update [OpenSpoolMan](https://github.com/drndos/openspoolman) or [Spoolman QR](https://github.com/Donkie/Spoolman/wiki/Printing-Labels)

### Simplification
- [ ] Drop export endpoints (`spoolman/api/v1/export.py`) — replace with a simple JSON data-file download
- [ ] Drop extra fields system — remove `/field/*` endpoints, `spoolman/extra_fields.py`, field management UI; keep raw `extra: dict[str, str]` on entities as-is
- [ ] Drop WebSocket support (`spoolman/ws.py`, WebSocket routes in spool/filament/vendor)
- [ ] Drop QR code / label printing page (`client/src/pages/printing/`)
- [ ] Remove deprecated query param aliases in `GET /spool` (`filament_name`, `filament_id` old-style underscore params — dotted form is now canonical)

