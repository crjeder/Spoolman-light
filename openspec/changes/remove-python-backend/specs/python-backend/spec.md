## REMOVED Requirements

### Requirement: Python backend serves HTTP API
The system SHALL NOT use the Python/FastAPI application in `spoolman/` as an HTTP server.
**Reason**: Replaced by the Rust/Axum backend (`spoolman-server`), which provides the same HTTP API.
**Migration**: No action required for API consumers — the Rust server exposes the same endpoints. Operators running the Docker image continue to use the same `SPOOLMAN_HOST`, `SPOOLMAN_PORT`, and `SPOOLMAN_DATA_FILE` environment variables.

#### Scenario: Service starts without Python
- **WHEN** the Docker container or binary starts
- **THEN** the Rust `spoolman-server` binary handles all HTTP requests and no Python interpreter is invoked

### Requirement: PUID/PGID dynamic remapping via entrypoint script
The system SHALL NOT support dynamic UID/GID remapping via `entrypoint.sh`.
**Reason**: The `entrypoint.sh` script exists solely to remap the `app` user at runtime for uvicorn; the Rust binary runs directly as the fixed `app` user (uid 1000) defined in the `Dockerfile`.
**Migration**: Operators who relied on `PUID`/`PGID` env vars must ensure their volume mounts are accessible by uid/gid 1000, or rebuild the image with a different `useradd` uid.

#### Scenario: Container runs without entrypoint.sh
- **WHEN** the Docker image is started
- **THEN** the binary executes directly via `CMD ["/spoolman"]` without a shell wrapper

### Requirement: Python integration test suite validates API behaviour
The system SHALL NOT maintain the `tests_integration/` Python/pytest test suite.
**Reason**: The integration tests target the Python HTTP API implementation and depend on `spoolman/` Python modules. They are not applicable to the Rust server.
**Migration**: API behaviour is validated by the Rust type system and manual testing until a Rust integration test suite is created (tracked in `TODO.md`).

#### Scenario: CI passes without Python test suite
- **WHEN** CI runs on a pull request
- **THEN** no Python/pytest integration tests are executed and the pipeline still passes
