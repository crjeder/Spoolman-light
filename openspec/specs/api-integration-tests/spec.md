# API Integration Tests Specification

## Requirements

### Requirement: Test infrastructure setup
The integration test suite SHALL live in `crates/spoolman-server/tests/` and run via `cargo test`. Each test SHALL construct the Axum router using `routes::build_router` with an isolated `JsonStore` backed by a per-test temp file, and dispatch requests in-process via `tower::ServiceExt::oneshot`.

#### Scenario: Tests run without network or Docker
- **WHEN** `cargo test -p spoolman-server` is executed
- **THEN** all integration tests pass without any external process, bound TCP port, or Docker container

#### Scenario: Test isolation — no state leaks between tests
- **WHEN** two tests run in the same `cargo test` invocation
- **THEN** each test operates on its own empty store; writes in one test MUST NOT affect another

### Requirement: Health endpoint coverage
The test suite SHALL verify the `/health` endpoint returns HTTP 200.

#### Scenario: Health check returns OK
- **WHEN** `GET /health` is sent
- **THEN** the response status SHALL be 200

### Requirement: Filament CRUD coverage
The test suite SHALL cover create, read, list, update, and delete operations on the filament API at `/api/v1/filament`.

#### Scenario: Create filament returns 201
- **WHEN** `POST /api/v1/filament` is sent with a valid JSON body containing at minimum a `diameter` and `density` field
- **THEN** the response status SHALL be 201 and the body SHALL contain the created filament with a non-zero `id`

#### Scenario: Get filament returns 200
- **WHEN** `GET /api/v1/filament/{id}` is sent with an existing ID
- **THEN** the response status SHALL be 200 and the body SHALL contain the filament matching that ID

#### Scenario: Get unknown filament returns 404
- **WHEN** `GET /api/v1/filament/{id}` is sent with an ID that does not exist
- **THEN** the response status SHALL be 404

#### Scenario: List filaments returns all created filaments
- **WHEN** two filaments are created and `GET /api/v1/filament` is sent
- **THEN** the response status SHALL be 200 and `items` SHALL contain both filaments

#### Scenario: Update filament returns 200
- **WHEN** `PATCH /api/v1/filament/{id}` is sent with a valid partial update body
- **THEN** the response status SHALL be 200 and the returned filament SHALL reflect the updated fields

#### Scenario: Delete filament returns 200
- **WHEN** `DELETE /api/v1/filament/{id}` is sent for an existing filament with no referencing spools
- **THEN** the response status SHALL be 200 and a subsequent GET SHALL return 404

#### Scenario: Delete filament blocked by referencing spool returns 409
- **WHEN** a spool references a filament and `DELETE /api/v1/filament/{filament_id}` is sent
- **THEN** the response status SHALL be 409

### Requirement: Spool CRUD coverage
The test suite SHALL cover create, read, list, update, clone, and delete operations on the spool API at `/api/v1/spool`.

#### Scenario: Create spool returns 201
- **WHEN** `POST /api/v1/spool` is sent with a valid body referencing an existing filament ID
- **THEN** the response status SHALL be 201 and the body SHALL contain the new spool with nested filament data

#### Scenario: Create spool with unknown filament returns 404
- **WHEN** `POST /api/v1/spool` is sent with a `filament_id` that does not exist
- **THEN** the response status SHALL be 404

#### Scenario: Get spool returns 200
- **WHEN** `GET /api/v1/spool/{id}` is sent with an existing ID
- **THEN** the response status SHALL be 200 and the body SHALL contain the spool with nested filament

#### Scenario: Get unknown spool returns 404
- **WHEN** `GET /api/v1/spool/{id}` is sent with an ID that does not exist
- **THEN** the response status SHALL be 404

#### Scenario: List spools returns all active spools
- **WHEN** two spools are created and `GET /api/v1/spool` is sent
- **THEN** the response status SHALL be 200 and `items` SHALL contain both spools

#### Scenario: Update spool weight and last_used
- **WHEN** `PATCH /api/v1/spool/{id}` is sent with a `current_weight` value
- **THEN** the response status SHALL be 200, the returned `current_weight` SHALL match the new value, and `last_used` SHALL be set automatically

#### Scenario: Clone spool returns 201
- **WHEN** `POST /api/v1/spool/{id}/clone` is sent for an existing spool
- **THEN** the response status SHALL be 201 and the cloned spool SHALL have a different ID but the same filament reference

#### Scenario: Delete spool returns 200
- **WHEN** `DELETE /api/v1/spool/{id}` is sent for an existing spool
- **THEN** the response status SHALL be 200 and a subsequent GET SHALL return 404

### Requirement: Location CRUD coverage
The test suite SHALL cover create, read, list, update, and delete operations on the location API at `/api/v1/location`.

#### Scenario: Create location returns 201
- **WHEN** `POST /api/v1/location` is sent with `{ "name": "Shelf A" }`
- **THEN** the response status SHALL be 201 and the body SHALL contain the location with a non-zero `id`

#### Scenario: Get location returns 200
- **WHEN** `GET /api/v1/location/{id}` is sent with an existing ID
- **THEN** the response status SHALL be 200 and the body SHALL contain the location with `spool_count`

#### Scenario: Get unknown location returns 404
- **WHEN** `GET /api/v1/location/{id}` is sent with an ID that does not exist
- **THEN** the response status SHALL be 404

#### Scenario: List locations
- **WHEN** a location is created and `GET /api/v1/location` is sent
- **THEN** the response status SHALL be 200 and the created location SHALL appear in the list

#### Scenario: Update location name
- **WHEN** `PATCH /api/v1/location/{id}` is sent with a new `name`
- **THEN** the response status SHALL be 200 and the returned location SHALL have the updated name

#### Scenario: Delete location returns 200
- **WHEN** `DELETE /api/v1/location/{id}` is sent for a location with no referencing spools
- **THEN** the response status SHALL be 200

#### Scenario: Delete location blocked by referencing spool returns 409
- **WHEN** a spool is assigned to a location and `DELETE /api/v1/location/{id}` is sent
- **THEN** the response status SHALL be 409

### Requirement: Settings endpoint coverage
The test suite SHALL verify that settings can be written and read via `/api/v1/setting`.

#### Scenario: Put and get a setting
- **WHEN** `PUT /api/v1/setting/{key}` is sent with a string value, then `GET /api/v1/setting` is sent
- **THEN** the response from GET SHALL contain the key-value pair that was written
