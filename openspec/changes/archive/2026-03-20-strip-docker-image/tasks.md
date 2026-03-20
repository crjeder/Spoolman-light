## 1. Update Dockerfile builder stage

- [x] 1.1 Remove `python3-pdm` from the apt-get install list in the builder stage
- [x] 1.2 Add a `UV_VERSION` ARG with a pinned version (e.g., `0.6.6`)
- [x] 1.3 Add a `RUN curl -LsSf https://astral.sh/uv/${UV_VERSION}/install.sh | sh` step to install uv binary in the builder stage
- [x] 1.4 Replace `COPY pdm.lock` with `COPY uv.lock` in the builder stage
- [x] 1.5 Replace `RUN pdm sync --prod --no-editable` with `RUN uv sync --frozen --no-dev`

## 2. Verify build and runtime correctness

- [x] 2.1 Run `docker build .` locally and confirm it completes without errors
- [x] 2.2 Confirm the runner image does not contain the `uv` binary (`docker run --rm <image> which uv` should fail)
- [x] 2.3 Confirm the app starts correctly (`docker run --rm -p 8000:8000 <image>` and hit `/api/v1/health`)
- [x] 2.4 Run integration tests against the new image (`python tests_integration/run.py sqlite`)

## 3. Changelog

- [x] 3.1 Add entry under `## [Unreleased]` → `Changed`: "Docker builder stage now uses uv instead of pdm to install production dependencies, reducing build time and layer size"
