## ADDED Requirements

### Requirement: Builder stage uses uv to install production dependencies
The Docker builder stage SHALL install uv (a compiled binary package manager) and use it to install only production Python dependencies into a `.venv`, without any dev dependencies present.

#### Scenario: uv is available in the builder stage
- **WHEN** the Docker image is built
- **THEN** the builder stage contains the uv binary and can execute `uv sync`

#### Scenario: Only production dependencies are installed
- **WHEN** `uv sync --frozen --no-dev` is run in the builder stage
- **THEN** the `.venv` contains exactly the packages listed under `[project.dependencies]` in `pyproject.toml`, with no dev packages

#### Scenario: Dependencies match the lock file exactly
- **WHEN** `uv sync --frozen` is run
- **THEN** package versions match `uv.lock` exactly (no resolution is performed at build time)

### Requirement: uv binary is absent from the final runner image
The Docker runner stage SHALL NOT contain the uv binary; only the populated `.venv` from the builder stage SHALL be copied.

#### Scenario: uv binary not present in runner image
- **WHEN** the final Docker image is inspected
- **THEN** no `uv` executable exists in the image filesystem

#### Scenario: .venv is copied from builder to runner
- **WHEN** the final Docker image is built
- **THEN** the `.venv` directory at `/home/app/spoolman/.venv` is present and contains all production packages

### Requirement: Runtime behavior is unchanged
The runner image SHALL start and serve requests identically to the previous image, using the same virtualenv path and entrypoint.

#### Scenario: Application starts successfully
- **WHEN** the runner container is started
- **THEN** the FastAPI application starts on port 8000 with no import errors

#### Scenario: PATH resolves to venv Python
- **WHEN** the container runs any Python command
- **THEN** the interpreter and packages from `/home/app/spoolman/.venv` are used
