<picture>
  <source media="(prefers-color-scheme: dark)" srcset="https://github.com/Donkie/Spoolman/assets/2332094/4e6e80ac-c7be-4ad2-9a33-dedc1b5ba30e">
  <source media="(prefers-color-scheme: light)" srcset="https://github.com/Donkie/Spoolman/assets/2332094/3c120b3a-1422-42f6-a16b-8d5a07c33000">
  <img alt="Icon of a filament spool" src="https://github.com/Donkie/Spoolman/assets/2332094/3c120b3a-1422-42f6-a16b-8d5a07c33000">
</picture>

<br/>

_A lightweight filament tracker for home 3D printing._

# Spoolman (light)

Spoolman light is a self-hosted web service for tracking your 3D printer filament spools. It is a simplified fork of [Donkie/Spoolman](https://github.com/Donkie/Spoolman) designed for home use — one or two printers and a shelf of spools — with no database server, no vendor management, and no external integrations required.

## Features

* **Filament & Spool Tracking**: Keep records of filament types and individual spools, including color and price directly on the spool.
* **REST API**: A clean [REST API](https://donkie.github.io/Spoolman/) for reading and updating spool data.
* **Web Client**: Built-in browser UI to view, create, edit, and delete filaments and spools.
* **Simple Storage**: All data stored in a single JSON file — no database server required. Configure the path via `SPOOLMAN_DATA_FILE`.

## Integrations

Any Spoolman-compatible REST API client can connect to this service using its standard API endpoints.

**Web client preview:**
![image](https://github.com/Donkie/Spoolman/assets/2332094/33928d5e-440f-4445-aca9-456c4370ad0d)

## Installation

Clone the repository and install dependencies with `pdm` or `uv`:

```bash
pdm install
pdm run app
```

The web client will be available at `http://localhost:8000`.

## Configuration

| Variable | Default | Purpose |
|----------|---------|---------|
| `SPOOLMAN_DATA_FILE` | `<data_dir>/spoolman.json` | Path to JSON data file |
| `SPOOLMAN_DIR_DATA` | platform default | Data directory |
| `SPOOLMAN_DIR_LOGS` | platform default | Logs directory |
| `SPOOLMAN_DIR_BACKUPS` | platform default | Backups directory |
| `SPOOLMAN_HOST` | `0.0.0.0` | Bind host |
| `SPOOLMAN_PORT` | `8000` | Bind port |
| `SPOOLMAN_CORS_ORIGIN` | `FALSE` | CORS origin (set to frontend URL if needed) |
| `SPOOLMAN_BASE_PATH` | `""` | URL base path prefix |
| `SPOOLMAN_DEBUG_MODE` | `FALSE` | Enable debug mode |
| `SPOOLMAN_LOGGING_LEVEL` | `INFO` | Log level |
| `SPOOLMAN_AUTOMATIC_BACKUP` | `TRUE` | Automatic data backup |
