## Context

The current Dockerfile is a two-stage build:

1. **python-builder** — installs `g++`, `python3-dev`, `libpq-dev`, `libffi-dev`, and `python3-pdm` via apt, then runs `pdm sync --prod --no-editable` to populate `.venv`.
2. **python-runner** — copies the built `.venv` and app source from the builder, installs only `gosu`, and runs the app.

The project has already migrated its lock file to `uv.lock` (present in the repo). The Dockerfile still references `pdm.lock` and installs pdm via apt. pdm is a Python application, so installing it via apt pulls in extra Python packages into the builder layer. `uv` is a single compiled binary with no Python dependencies, making it strictly lighter.

The `.venv` produced by both tools is layout-compatible — both use the standard virtualenv layout that uvicorn/FastAPI rely on.

## Goals / Non-Goals

**Goals:**
- Replace `python3-pdm` (apt) with `uv` (binary install) in the builder stage
- Use `uv sync --frozen --no-dev` to install only production dependencies
- `uv` binary must NOT be present in the final runner image (only the `.venv`)
- Final image behavior unchanged: same `.venv` layout, same `PATH`, same entrypoint

**Non-Goals:**
- Changing the base image (`python:3.12-slim-bookworm`) — out of scope
- Moving to distroless or Alpine — separate concern
- Removing `gosu` from the runner stage — out of scope
- Changing the app's dependency set — no package adds/removes

## Decisions

### Use uv's official installer in the builder stage

**Decision:** Install uv via the official `curl | sh` installer pinned to a specific version, not via apt or pip.

**Rationale:** apt does not package uv; pip could work but adds overhead. The official installer fetches a pre-compiled binary and is the supported installation path for CI/Docker environments. Pinning the version ensures reproducible builds.

**Alternative considered:** `pip install uv` — works but slower (pip is already available), and pip itself adds overhead. The binary installer is faster and has no Python-level side effects.

### Copy only the .venv into the runner stage

**Decision:** The runner stage copies only `/home/app/spoolman/.venv` from the builder (same as today), and uv itself is not copied.

**Rationale:** The runtime only needs the installed packages, not the package manager. This keeps the runner image lean and avoids uv's binary from being present in the attack surface.

### Keep builder base as `python:3.12-slim-bookworm`

**Decision:** No base image change.

**Rationale:** The builder needs a Python interpreter to run `uv sync` (uv calls into Python to resolve environment markers). The runner already uses the same slim base. Changing bases is a separate optimization.

### Retain build-time apt packages (g++, python3-dev, libpq-dev, libffi-dev)

**Decision:** Keep the compile-time dependencies in the builder stage.

**Rationale:** Some Python packages (e.g., `psycopg2`) require native compilation. These packages only exist in the builder layer and do not appear in the runner image — this is already correct behavior. Removing them would break builds for non-SQLite database drivers.

## Risks / Trade-offs

- **uv version pinning drift** → Mitigate by using an explicit version pin (e.g., `UV_VERSION=0.6.x`) as a Docker ARG so it can be bumped via a single line change.
- **uv sync behavior difference** → uv's `--frozen` flag is equivalent to pdm's locked install mode; `--no-dev` excludes `[tool.pdm.dev-dependencies]` groups. Verify that `setuptools` (listed as a prod dep) is still included — it is, since it's in `[project.dependencies]`.
- **pyproject.toml still has `[tool.pdm.*]` sections** → uv ignores these; no conflict. Scripts (`pdm run app`) continue to work for local dev via pdm or uv run.
- **CI pipelines that pass `pdm.lock` as a build context file** → These would need updating if the Dockerfile COPY instruction changes from `pdm.lock` to `uv.lock`.

## Migration Plan

1. Update `Dockerfile`: remove pdm apt install, add uv binary install, update COPY and sync commands.
2. Verify local Docker build (`docker build .`) produces a working image.
3. Run integration tests against the new image to confirm runtime behavior is unchanged.
4. No rollback complexity — the change is isolated to the Dockerfile; reverting is a single git revert.
