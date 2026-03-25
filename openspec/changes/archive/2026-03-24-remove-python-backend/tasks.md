## 1. Delete Python Source and Tooling

- [x] 1.1 Delete `spoolman/` directory (entire Python package)
- [x] 1.2 Delete `tests_integration/` directory (pytest integration test suite)
- [x] 1.3 Delete `pyproject.toml`
- [x] 1.4 Delete `pdm.lock`
- [x] 1.5 Delete `uv.lock`
- [x] 1.6 Delete `entrypoint.sh` (uvicorn launcher, no longer needed)
- [x] 1.7 Delete `client/` directory if it only served the Python stack (verify first)

## 2. Update CI Workflow

- [x] 2.1 Remove the `style` job (Python/pre-commit lint) from `.github/workflows/ci.yml`
- [x] 2.2 Remove the `build-client` job's Python/pdm steps from `.github/workflows/ci.yml`
- [x] 2.3 Add a `cargo check` job for the Rust workspace
- [x] 2.4 Add a `cargo clippy` job for Rust lint
- [x] 2.5 Verify the Docker build job still passes (already Rust-only in `Dockerfile`)

## 3. Update Documentation

- [x] 3.1 Update `CLAUDE.md` — remove Python stack section, commands, and architecture references
- [x] 3.2 Update `README.md` — remove Python installation/run instructions, update stack description
- [x] 3.3 Update `TODO.md` — remove Python-related items, add note about missing Rust integration tests
- [x] 3.4 Update `CHANGELOG.md` — add entry for Python backend removal under new version

## 4. Verify Clean State

- [x] 4.1 Run `cargo check -p spoolman-types` — confirm no errors
- [x] 4.2 Run `cargo check -p spoolman-server` — confirm no errors
- [x] 4.3 Confirm no remaining references to `spoolman/`, `uvicorn`, or Python in non-doc files (`grep -r "uvicorn\|from spoolman\|import spoolman" .`)
