## Why

The README still describes the original Spoolman's full feature set — multi-database backends, vendor management, Prometheus integration, WebSockets, and enterprise-scale multi-printer support — none of which apply to this fork. New users reading the README get a misleading picture of what the software does and what it requires.

## What Changes

- Rewrite the intro paragraph to position the fork as a lightweight, home-use filament tracker optimized for one or two printers and tens of spools.
- Replace the Features list to reflect what is actually present: JSON file storage, simplified filament/spool data model (no vendor entity, color/price on spool), REST API, and web client.
- Remove references to: multi-database support (SQLite/Postgres/MySQL/CockroachDB), Alembic migrations, vendor management, Prometheus monitoring, WebSocket real-time updates, SpoolmanDB community database, Weblate translations, QR-code label printing, and multi-printer simultaneous management.
- Remove or update the integration list (OctoPrint, Moonraker, etc.) — these may still work via the REST API but should not be prominently advertised as a core feature.
- Update environment variable table to remove `SPOOLMAN_DB_*` vars and add `SPOOLMAN_DATA_FILE`.
- Remove the Installation wiki link (this fork has its own setup story via `pdm`/`uv`).

## Capabilities

### New Capabilities

- `readme-home-use`: Updated README that accurately describes the simplified, home-use fork — its scope, feature set, data model, and configuration.

### Modified Capabilities

*(none — no existing specs change requirements here)*

## Impact

- `README.md` only — no code changes.
