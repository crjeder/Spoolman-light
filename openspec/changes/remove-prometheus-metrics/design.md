## Context

Spoolman exposes an optional Prometheus `/metrics` endpoint backed by `prometheus_client`. It is disabled by default (`SPOOLMAN_METRICS_ENABLED=FALSE`). The feature consists of:

- `spoolman/prometheus/metrics.py` — gauge definitions and collection functions
- A scheduled background task in `main.py` that runs every minute when enabled
- A GET `/metrics` route mounted directly on the FastAPI app as a workaround for SPA root routing
- `env.is_metrics_enabled()` reading `SPOOLMAN_METRICS_ENABLED`

The removal is straightforward: delete the module, remove the route and scheduler integration, drop the env helper, and remove the PyPI dependency.

## Goals / Non-Goals

**Goals:**
- Delete all Prometheus-related code and the `prometheus_client` dependency
- Remove the `SPOOLMAN_METRICS_ENABLED` env var from `env.py`
- Keep all other functionality intact (no behavior changes beyond metrics removal)
- Update documentation (CLAUDE.md env var table)

**Non-Goals:**
- Adding an alternative metrics system (e.g., OpenTelemetry)
- Providing a migration path for existing Prometheus scrape configs (breaking change, by design)
- Changing any other background scheduler behavior

## Decisions

### Delete the module rather than feature-flag it
The feature is already opt-in via an env var that defaults to `FALSE`. Given it's being removed entirely, there is no value in keeping a disabled code path. A clean deletion reduces diff noise in future PRs and eliminates the risk of accidentally re-enabling dead code.

### Remove `prometheus_client` from dependencies immediately
Keeping an unused transitive dependency is a maintenance and security burden. Remove it from `pyproject.toml` and regenerate `uv.lock` in the same PR so the dependency graph stays accurate.

### No deprecation period
The feature has a `FALSE` default, meaning the vast majority of deployments have never enabled it. A deprecation cycle would add work for no meaningful user benefit.

## Risks / Trade-offs

- **Breaking change for metrics scrapers** → Document clearly in CHANGELOG under `Removed`; affected users are a small subset who explicitly set `SPOOLMAN_METRICS_ENABLED=TRUE`
- **`uv.lock` churn** → Expected; regenerate with `uv lock` after editing `pyproject.toml`
- **Integration tests** → Grep tests for any metrics-related assertions and remove them

## Migration Plan

1. Delete `spoolman/prometheus/` directory
2. Edit `spoolman/main.py`: remove metrics import, `/metrics` route, `_metrics_task`, and the `if env.is_metrics_enabled()` block
3. Edit `spoolman/env.py`: remove `is_metrics_enabled()` function
4. Edit `pyproject.toml`: remove `prometheus-client` dependency
5. Run `uv lock` to regenerate `uv.lock`
6. Run integration tests to confirm nothing is broken
7. Update CLAUDE.md env var table (remove `SPOOLMAN_METRICS_ENABLED` row)
8. Add CHANGELOG entry under `## [Unreleased] > Removed`

Rollback: revert the commit. No DB migrations involved.
