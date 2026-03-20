"""Main entrypoint to the server."""

import logging
from logging.handlers import TimedRotatingFileHandler

import uvicorn
from fastapi import FastAPI
from fastapi.middleware.cors import CORSMiddleware
from fastapi.middleware.gzip import GZipMiddleware
from fastapi.responses import RedirectResponse, Response
from scheduler.asyncio.scheduler import Scheduler

from spoolman import env, externaldb
from spoolman.api.v1.router import app as v1_app
from spoolman.client import SinglePageApplication
from spoolman.storage.dependencies import set_store
from spoolman.storage.store import JsonStore

# Define a console logger
console_handler = logging.StreamHandler()
console_handler.setFormatter(logging.Formatter("%(name)-26s %(levelname)-8s %(message)s"))

# Setup the spoolman logger, which all spoolman modules will use
log_level = env.get_logging_level()
root_logger = logging.getLogger()
root_logger.setLevel(log_level)
root_logger.addHandler(console_handler)

# Fix uvicorn logging
logging.getLogger("uvicorn").setLevel(log_level)
if logging.getLogger("uvicorn").handlers:
    logging.getLogger("uvicorn").removeHandler(logging.getLogger("uvicorn").handlers[0])
logging.getLogger("uvicorn").addHandler(console_handler)

logging.getLogger("uvicorn.error").setLevel(log_level)
logging.getLogger("uvicorn.error").addHandler(console_handler)

access_handlers = logging.getLogger("uvicorn.access").handlers
if access_handlers:
    logging.getLogger("uvicorn.access").setLevel(log_level)
    logging.getLogger("uvicorn.access").removeHandler(access_handlers[0])
    logging.getLogger("uvicorn.access").addHandler(console_handler)

# Get logger instance for this module
logger = logging.getLogger(__name__)


# Setup FastAPI
app = FastAPI(
    debug=env.is_debug_mode(),
    title="Spoolman",
    version=env.get_version(),
)
app.add_middleware(GZipMiddleware)
app.mount(env.get_base_path() + "/api/v1", v1_app)


base_path = env.get_base_path()
if base_path != "":
    logger.info("Base path is: %s", base_path)

    @app.get(base_path)
    def root_redirect() -> Response:
        """Redirect to base path."""
        return RedirectResponse(base_path + "/")


@app.get(env.get_base_path() + "/config.js")
def get_configjs() -> Response:
    """Return a dynamic js config file."""
    if '"' in base_path:
        raise ValueError("Base path contains quotes, which are not allowed.")

    return Response(
        content=f"""
window.SPOOLMAN_BASE_PATH = "{base_path}";
""",
        media_type="text/javascript",
    )


# Mount the client side app
app.mount(base_path, app=SinglePageApplication(directory="client/dist", base_path=env.get_base_path()))


def add_cors_middleware() -> None:
    """Add CORS middleware to the FastAPI app based on environment settings."""
    origins = []
    if env.is_debug_mode():
        logger.warning("Running in debug mode, allowing all origins.")
        origins = ["*"]
    elif env.is_cors_defined():
        cors_origins = env.get_cors_origin()
        if cors_origins:
            logger.info("CORS origins defined: %s", cors_origins)
            origins = cors_origins
        else:
            logger.warning("CORS origins are not defined, no CORS will be applied.")

    if not origins:
        return

    app.add_middleware(
        CORSMiddleware,
        allow_origins=origins,
        allow_credentials=True,
        allow_methods=["*"],
        allow_headers=["*"],
        expose_headers=["X-Total-Count"],
    )


add_cors_middleware()


def add_file_logging() -> None:
    """Add file logging to the root logger."""
    log_file = env.get_logs_dir().joinpath("spoolman.log")
    file_handler = TimedRotatingFileHandler(log_file, when="midnight", backupCount=5)
    file_handler.setFormatter(logging.Formatter("%(asctime)s:%(levelname)s:%(message)s", "%Y-%m-%d %H:%M:%S"))
    root_logger.addHandler(file_handler)


_store: JsonStore | None = None


def _backup_task() -> None:
    if _store is None:
        return
    import shutil
    backups_dir = env.get_backups_dir()
    num_backups = 5
    # Rotate existing backups
    for i in range(num_backups - 1, -1, -1):
        src = backups_dir / (f"spoolman.json.{i}" if i > 0 else "spoolman.json")
        dst = backups_dir / f"spoolman.json.{i + 1}"
        if src.exists():
            if dst.exists() and i + 1 >= num_backups:
                dst.unlink()
            elif not dst.exists():
                shutil.copy2(src, dst)
    # Copy current data file as backup
    data_file = env.get_data_file()
    if data_file.exists():
        shutil.copy2(data_file, backups_dir / "spoolman.json")
    logger.info("Backup complete.")


@app.on_event("startup")
async def startup() -> None:
    """Run the service's startup sequence."""
    global _store  # noqa: PLW0603

    env.check_write_permissions()
    add_file_logging()

    logger.info(
        "Starting Spoolman v%s (commit: %s) (built: %s)",
        app.version,
        env.get_commit_hash(),
        env.get_build_date(),
    )

    logger.info("Using data directory: %s", env.get_data_dir().resolve())
    logger.info("Using logs directory: %s", env.get_logs_dir().resolve())
    logger.info("Using backups directory: %s", env.get_backups_dir().resolve())

    data_file = env.get_data_file()
    logger.info("Using data file: %s", data_file.resolve())

    logger.info("Initializing JSON store...")
    _store = JsonStore(data_file)
    _store.load()
    set_store(_store)

    # Setup scheduler
    schedule = Scheduler()

    if env.is_automatic_backup_enabled():
        import datetime as dt
        logger.info("Scheduling automatic data file backup for midnight.")
        schedule.daily(dt.time(hour=0, minute=0, second=0), _backup_task)  # type: ignore[arg-type]

    externaldb.schedule_tasks(schedule)

    logger.info("Startup complete.")

    if env.is_docker() and not env.is_data_dir_mounted():
        logger.warning("!!!! WARNING !!!!")
        logger.warning("The data directory is not mounted.")
        logger.warning(
            'Spoolman stores its data in the container directory "%s". '
            "If this directory isn't mounted to the host OS, data will be lost when the container is stopped.",
            env.get_data_dir(),
        )
        logger.warning("Please carefully read the docker part of the README.md file.")
        logger.warning("!!!! WARNING !!!!")


if __name__ == "__main__":
    uvicorn.run(app, host="0.0.0.0", port=8000)
