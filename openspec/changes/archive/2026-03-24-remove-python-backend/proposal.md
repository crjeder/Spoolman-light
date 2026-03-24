## Why

The Rust rewrite (Axum + Leptos WASM) is the verified successor to the original Python/FastAPI backend. The Python stack in `spoolman/` is dead code — it adds maintenance burden, confuses contributors, and keeps unnecessary dependencies in the project.

## What Changes

- **BREAKING**: Remove `spoolman/` Python package entirely
- Remove `tests_integration/` Docker-based integration test suite (Python-dependent)
- Remove `pyproject.toml`, `pdm.lock` / `uv.lock`, and all Python tooling config (`ruff`, `black`, `.python-version`)
- Remove Python-specific Docker artifacts (`Dockerfile` Python stage, `entrypoint.sh` if Python-only)
- Remove `requirements*.txt` or any pip/pdm/uv lock files
- Remove Python-related CI workflow steps (lint, format, test)
- Update `CLAUDE.md` and `README.md` to drop Python stack references

## Capabilities

### New Capabilities
<!-- None — this is a removal change -->

### Modified Capabilities
<!-- None — no spec-level behavior changes; the Rust stack implements the same API -->

## Impact

- `spoolman/` — deleted entirely
- `tests_integration/` — deleted entirely
- `pyproject.toml`, lock files, `.python-version` — deleted
- `Dockerfile` — remove Python build stage (keep Rust/cargo-leptos stage)
- `CLAUDE.md`, `README.md`, `TODO.md` — update to reflect Rust-only stack
- CI pipelines — remove Python lint/test jobs
- No API contract changes; Rust server provides the same HTTP API
