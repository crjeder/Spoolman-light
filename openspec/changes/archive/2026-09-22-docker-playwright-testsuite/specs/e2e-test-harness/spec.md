## ADDED Requirements

### Requirement: Docker compose test configuration exists
A `docker-compose.test.yml` SHALL exist at the repository root that defines a service using the project `Dockerfile`, mounts `assets/spoolman.json` as the data file, and exposes port 8000.

#### Scenario: Test compose file builds and starts the app
- **WHEN** `docker compose -f docker-compose.test.yml up --build -d` is run on Linux/macOS
- **THEN** the spoolman container starts and `GET http://localhost:8000/api/v1/info` returns HTTP 200 within 60 seconds

#### Scenario: Fixture data is loaded at startup
- **WHEN** the test container starts with `assets/spoolman.json` bind-mounted
- **THEN** the API returns at least one spool, filament, and location from the fixture data

#### Scenario: Container teardown is clean
- **WHEN** `docker compose -f docker-compose.test.yml down` is run after tests
- **THEN** no lingering containers or named volumes remain from the test run

### Requirement: Run script orchestrates the full test lifecycle
A `scripts/run-e2e.sh` shell script SHALL build the Docker image, start the container, wait for the server to be ready, run the Playwright tests, and tear down the container regardless of test outcome.

#### Scenario: Successful test run exits zero
- **WHEN** `scripts/run-e2e.sh` is executed and all Playwright tests pass
- **THEN** the script exits with code 0 and the container has been stopped

#### Scenario: Test failure exits non-zero
- **WHEN** `scripts/run-e2e.sh` is executed and at least one Playwright test fails
- **THEN** the script exits with a non-zero code and the container has been stopped

#### Scenario: Server readiness polling with timeout
- **WHEN** the server does not respond within 60 seconds after container start
- **THEN** the script prints a diagnostic message and exits with a non-zero code without running tests

### Requirement: Fixture data is isolated from development data
The test harness SHALL NOT read from or write to the development `spoolman_data` Docker volume.

#### Scenario: Test run does not pollute dev volume
- **WHEN** `scripts/run-e2e.sh` is run while the development volume `spoolman_data` exists
- **THEN** the development volume contents are unchanged after the test run
