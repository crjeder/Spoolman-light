## 1. Delete Prometheus Module

- [x] 1.1 Delete `spoolman/prometheus/metrics.py`
- [x] 1.2 Delete `spoolman/prometheus/__init__.py`
- [x] 1.3 Remove the `spoolman/prometheus/` directory

## 2. Update main.py

- [x] 2.1 Remove `from prometheus_client import generate_latest` import
- [x] 2.2 Remove `from spoolman.prometheus.metrics import registry` import
- [x] 2.3 Remove the `/metrics` GET route and its handler function `get_metrics()`
- [x] 2.4 Remove the `_metrics_task()` function
- [x] 2.5 Remove the `if env.is_metrics_enabled():` block (scheduler setup)

## 3. Update env.py

- [x] 3.1 Remove the `is_metrics_enabled()` function from `spoolman/env.py`

## 4. Remove Dependency

- [x] 4.1 Remove `prometheus-client` from `pyproject.toml` dependencies
- [x] 4.2 Run `uv lock` to regenerate `uv.lock`

## 5. Verify & Clean Up

- [x] 5.1 Run `ruff check .` and fix any lint errors
- [ ] 5.2 Run integration tests (`python tests_integration/run.py sqlite`) to confirm nothing is broken
- [x] 5.3 Remove `SPOOLMAN_METRICS_ENABLED` row from the env var table in `CLAUDE.md`
- [x] 5.4 Add CHANGELOG entry under `## [Unreleased] > Removed` for the metrics endpoint and env var
