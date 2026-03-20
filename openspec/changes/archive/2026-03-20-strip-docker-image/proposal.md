## Why

The Docker builder stage installs `python3-pdm` via apt, pulling in a heavy Python-based package manager even though the project has already migrated to `uv` (a compiled Rust binary). Replacing pdm with uv in the build stage reduces layer bloat, speeds up builds, and ensures only production runtime dependencies land in the final image.

## What Changes

- Remove `python3-pdm` and related apt packages from the builder stage
- Install `uv` in the builder stage (single binary, no apt dependencies)
- Replace `pdm sync --prod --no-editable` with `uv sync --frozen --no-dev`
- Copy `uv.lock` instead of `pdm.lock` into the build context
- Ensure the final runner image contains only the `.venv` with production packages (no dev deps, no uv binary)

## Capabilities

### New Capabilities

- `docker-uv-build`: Builder stage uses uv to install only production dependencies into an isolated `.venv`, with uv itself not present in the final runner image

### Modified Capabilities

<!-- No existing spec-level behavior changes — this is a build/packaging concern only -->

## Impact

- **Dockerfile**: Builder stage rewritten to use uv; runner stage unchanged
- **uv.lock**: Already present; used as the locked dependency manifest
- **pdm.lock**: No longer referenced by Docker build (still used by local pdm workflows if any)
- **CI/build pipelines**: Any scripts that build the Docker image continue to work unchanged (no ARG/ENV interface changes)
- **Final image**: Same Python venv layout under `.venv/`, same `PATH` configuration — no runtime behavior changes
