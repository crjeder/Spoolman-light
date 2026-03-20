## REMOVED Requirements

### Requirement: Prometheus metrics endpoint
The system SHALL NOT expose a `/metrics` HTTP endpoint. The `prometheus_client` package SHALL NOT be a dependency.

**Reason**: The feature was opt-in (defaulting to disabled) and rarely used. Removing it reduces dependency surface and maintenance burden.
**Migration**: Remove `SPOOLMAN_METRICS_ENABLED=TRUE` from your environment. If Prometheus scraping is required, use an external exporter or community plugin.

#### Scenario: Metrics endpoint no longer exists
- **WHEN** a client sends GET `/metrics`
- **THEN** the server returns 404 Not Found

#### Scenario: Metrics env var has no effect
- **WHEN** `SPOOLMAN_METRICS_ENABLED=TRUE` is set in the environment
- **THEN** Spoolman starts normally and does NOT expose a `/metrics` endpoint

### Requirement: Prometheus background collection task
The system SHALL NOT run a periodic background task that collects spool and filament metrics into Prometheus gauges.

**Reason**: Removed along with the metrics endpoint.
**Migration**: No migration required for users who did not enable metrics.

#### Scenario: No background metrics job on startup
- **WHEN** Spoolman starts
- **THEN** no minutely metrics collection task is scheduled
