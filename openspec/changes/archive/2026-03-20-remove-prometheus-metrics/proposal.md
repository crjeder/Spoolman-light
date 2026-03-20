## Why

Prometheus metrics add a dependency on `prometheus_client`, a background scheduler, and a dedicated `/metrics` endpoint that most users never enable (`SPOOLMAN_METRICS_ENABLED` defaults to `FALSE`). Removing this optional, rarely-used feature reduces maintenance surface, eliminates dead code, and simplifies the codebase.

## What Changes

- Remove `spoolman/prometheus/` package (`metrics.py`, `__init__.py`)
- Remove the `/metrics` HTTP endpoint from `spoolman/main.py`
- Remove the `_metrics_task` background job and related scheduling logic from `spoolman/main.py`
- Remove `is_metrics_enabled()` from `spoolman/env.py` and the `SPOOLMAN_METRICS_ENABLED` environment variable
- Remove `prometheus_client` from project dependencies (`pyproject.toml`)
- **BREAKING**: The `/metrics` Prometheus endpoint is no longer available; the `SPOOLMAN_METRICS_ENABLED` env var is ignored/removed

## Capabilities

### New Capabilities

_None._

### Modified Capabilities

_None_ — this is a pure removal; no existing spec-level behavior changes.

## Impact

- **Backend code**: `spoolman/prometheus/`, `spoolman/main.py`, `spoolman/env.py`
- **Dependencies**: `prometheus_client` package removed from `pyproject.toml` / `uv.lock`
- **API**: `/metrics` endpoint removed (breaking for any Prometheus scrapers pointed at Spoolman)
- **Docs/Config**: `SPOOLMAN_METRICS_ENABLED` env var no longer recognized; CLAUDE.md env var table should be updated
- **Tests**: Integration tests that may reference metrics behavior should be checked
