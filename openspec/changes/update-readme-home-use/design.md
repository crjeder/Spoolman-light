## Context

The current `README.md` was carried over from the upstream Donkie/Spoolman project. It describes a feature set that no longer exists in this fork: multiple database backends, vendor entities, Prometheus metrics, WebSocket live updates, and enterprise-grade multi-printer orchestration. The fork has replaced all of that with a single JSON file, a simplified data model, and a scope appropriate for a hobbyist with one or two printers and a shelf of spools.

## Goals / Non-Goals

**Goals:**
- README accurately reflects what the fork actually does today.
- Intro clearly sets expectations: self-hosted, home-scale, simple.
- Feature list matches the current codebase (JSON storage, REST API, web client, filament + spool tracking).
- Environment variable table only lists variables that still exist (`SPOOLMAN_DATA_FILE` and the operational ones like `SPOOLMAN_HOST`, `SPOOLMAN_PORT`, etc.).
- Removed features are not mentioned (no orphaned promises).

**Non-Goals:**
- Documenting planned/pending simplifications from TODO.md (WebSocket removal, export endpoint removal, etc.) — these aren't done yet.
- Changing any code or configuration files.
- Writing a migration guide (that belongs in CHANGELOG.md, which already has one).
- Reproducing the full API reference (that lives in the OpenAPI spec).

## Decisions

**Keep the existing image/logo block.** The branding is unchanged; the visuals are still accurate.

**Drop all external integration callouts (Moonraker, OctoPrint, etc.).** These integrations were designed against the original API surface. The REST API is still technically compatible for basic spool use operations, but advertising them as supported integrations would imply ongoing compatibility guarantees this fork doesn't make.

**Retain a minimal "Integrations" note.** A short sentence acknowledging that any Spoolman-compatible client can connect via the REST API is enough — without listing specific third-party projects.

**Keep the web client screenshot.** The web UI is largely unchanged in appearance; the screenshot is still representative.

**Simplify the env var table.** Only variables present in the current `env.py` should appear. Remove all `SPOOLMAN_DB_*` entries; add `SPOOLMAN_DATA_FILE`.

## Risks / Trade-offs

[Risk] Downstream users copy-paste the README for their own forks and expect it to match the original.
→ Not a concern — this is a private fork; the README is for whoever uses this deployment.

[Risk] Some features mentioned in the old README are still partially present (e.g., Prometheus metrics endpoint exists in code but isn't advertised as a goal).
→ Omit rather than advertise. Prometheus section can be dropped entirely; it's not a home-use feature and its removal is in TODO.md.

## Open Questions

*(none)*
