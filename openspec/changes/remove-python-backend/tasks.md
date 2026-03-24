## 1. Delete Python Source and Tooling

- [ ] 1.1 Delete `spoolman/` directory (entire Python package)
- [ ] 1.2 Delete `tests_integration/` directory (pytest integration test suite)
- [ ] 1.3 Delete `pyproject.toml`
- [ ] 1.4 Delete `pdm.lock`
- [ ] 1.5 Delete `uv.lock`
- [ ] 1.6 Delete `entrypoint.sh` (uvicorn launcher, no longer needed)
- [ ] 1.7 Delete `client/` directory if it only served the Python stack (verify first)

## 2. Update CI Workflow

- [ ] 2.1 Remove the `style` job (Python/pre-commit lint) from `.github/workflows/ci.yml`
- [ ] 2.2 Remove the `build-client` job's Python/pdm steps from `.github/workflows/ci.yml`
- [ ] 2.3 Add a `cargo check` job for the Rust workspace
- [ ] 2.4 Add a `cargo clippy` job for Rust lint
- [ ] 2.5 Verify the Docker build job still passes (already Rust-only in `Dockerfile`)

## 3. Update Documentation

- [ ] 3.1 Update `CLAUDE.md` — remove Python stack section, commands, and architecture references
- [ ] 3.2 Update `README.md` — remove Python installation/run instructions, update stack description
- [ ] 3.3 Update `TODO.md` — remove Python-related items, add note about missing Rust integration tests
- [ ] 3.4 Update `CHANGELOG.md` — add entry for Python backend removal under new version

## 4. Verify Clean State

- [ ] 4.1 Run `cargo check -p spoolman-types` — confirm no errors
- [ ] 4.2 Run `cargo check -p spoolman-server` — confirm no errors
- [ ] 4.3 Confirm no remaining references to `spoolman/`, `uvicorn`, or Python in non-doc files (`grep -r "uvicorn\|from spoolman\|import spoolman" .`)
