"""Router setup for the v1 version of the API."""

# ruff: noqa: D103

import asyncio
import logging
import shutil
from typing import Annotated

from fastapi import Depends, FastAPI, WebSocket, WebSocketDisconnect
from fastapi.responses import JSONResponse
from starlette.requests import Request
from starlette.responses import Response

from spoolman import env
from spoolman.exceptions import ItemNotFoundError
from spoolman.storage.dependencies import get_store
from spoolman.storage.store import JsonStore
from spoolman.ws import websocket_manager

from . import export, externaldb, field, filament, models, other, setting, spool

logger = logging.getLogger(__name__)

app = FastAPI(
    title="Spoolman REST API v1",
    version="1.0.0",
    description="""
    REST API for Spoolman.

    The API is served on the path `/api/v1/`.

    Some endpoints also serve a websocket on the same path. The websocket is used to listen for changes to the data
    that the endpoint serves. The websocket messages are JSON objects. Additionally, there is a root-level websocket
    endpoint that listens for changes to any data in the database.
    """,
)


@app.exception_handler(ItemNotFoundError)
async def itemnotfounderror_exception_handler(_request: Request, exc: ItemNotFoundError) -> Response:
    logger.debug(exc, exc_info=True)
    return JSONResponse(
        status_code=404,
        content={"message": exc.args[0]},
    )


@app.get("/info")
async def info() -> models.Info:
    """Return general info about the API."""
    return models.Info(
        version=env.get_version(),
        debug_mode=env.is_debug_mode(),
        automatic_backups=env.is_automatic_backup_enabled(),
        data_dir=str(env.get_data_dir().resolve()),
        logs_dir=str(env.get_logs_dir().resolve()),
        backups_dir=str(env.get_backups_dir().resolve()),
        data_file=str(env.get_data_file().resolve()),
        git_commit=env.get_commit_hash(),
        build_date=env.get_build_date(),
    )


@app.get("/health")
async def health() -> models.HealthCheck:
    """Return a health check."""
    return models.HealthCheck(status="healthy")


@app.post(
    "/backup",
    description="Trigger a backup of the JSON data file.",
    response_model=models.BackupResponse,
    responses={500: {"model": models.Message}},
)
async def backup(store: Annotated[JsonStore, Depends(get_store)]) -> JSONResponse:  # noqa: ANN201
    """Trigger a backup of the data file."""
    data_file = env.get_data_file()
    if not data_file.exists():
        return JSONResponse(
            status_code=500,
            content={"message": "Data file does not exist, nothing to back up."},
        )
    backups_dir = env.get_backups_dir()
    num_backups = 5

    def do_backup() -> str:
        # Rotate old backups
        for i in range(num_backups - 1, 0, -1):
            src = backups_dir / f"spoolman.json.{i}"
            dst = backups_dir / f"spoolman.json.{i + 1}"
            if i + 1 > num_backups and dst.exists():
                dst.unlink()
            if src.exists():
                shutil.copy2(src, dst)
        # Copy backup 0 to 1
        b0 = backups_dir / "spoolman.json"
        if b0.exists():
            shutil.copy2(b0, backups_dir / "spoolman.json.1")
        # Create new backup
        shutil.copy2(data_file, b0)
        return str(b0)

    try:
        path = await asyncio.to_thread(do_backup)
        return models.BackupResponse(path=path)
    except Exception:
        logger.exception("Backup failed.")
        return JSONResponse(
            status_code=500,
            content={"message": "Backup failed. See server logs for more information."},
        )


@app.websocket(
    "/",
    name="Listen to any changes",
)
async def notify(
    websocket: WebSocket,
) -> None:
    await websocket.accept()
    websocket_manager.connect((), websocket)
    try:
        while True:
            await asyncio.sleep(0.5)
            if await websocket.receive_text():
                await websocket.send_json({"status": "healthy"})
    except WebSocketDisconnect:
        websocket_manager.disconnect((), websocket)


# Add routers
app.include_router(filament.router)
app.include_router(spool.router)
app.include_router(setting.router)
app.include_router(field.router)
app.include_router(other.router)
app.include_router(externaldb.router)
app.include_router(export.router)
