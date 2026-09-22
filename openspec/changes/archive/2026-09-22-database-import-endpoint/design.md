## Context

`JsonStore` (crates/spoolman-server/src/store.rs) wraps `Arc<RwLock<DataStore>>` and flushes to disk atomically (write `.tmp`, rename). `GET /api/v1/export` (routes/other.rs) already serializes the full `DataStore`. `reload()` re-reads the file from disk and swaps the in-memory copy, leaving the old data untouched on read/parse failure. Import is the inverse: take a client-supplied `DataStore`, validate it, then apply the same swap-and-flush.

## Goals / Non-Goals

**Goals:**
- Restore a full `DataStore` backup (as produced by `/export`) via `POST /api/v1/import`.
- Reject invalid/unparseable payloads and unsupported schema versions before touching disk or memory.
- Run existing migrations on an imported older-schema store, same as on startup load.
- Settings UI control to pick a JSON file and restore it, mirroring the existing reload button.

**Non-Goals:**
- Partial/merge import (only full replace).
- Import format conversion from other tools (SpoolmanDB, OctoPrint, etc.) — out of scope, separate TODO item.
- Automatic backup-before-import (the export endpoint already covers "make a backup first"; documented in the UI confirmation instead of built server-side).

## Decisions

- **Full replace, not merge.** Matches `/export`'s full-dump shape and avoids ID-collision/merge-conflict logic. Alternative (merge by ID) rejected as unnecessary complexity for a restore feature.
- **Validate via `serde_json::from_value::<DataStore>` + explicit schema_version check**, reusing the existing `migrate()` path so an older exported file still loads. Mirrors `read_and_migrate()`; a raw `Json<DataStore>` extractor gives free deserialization validation (missing/wrong-typed fields → 400 automatically via existing error handling).
- **New `JsonStore::import(&self, data: DataStore) -> Result<()>`**: validates schema_version isn't newer than current (`StoreError::Validation` if so), runs `migrate()`, `flush()`s to disk, then swaps `self.inner`. Flush-before-swap keeps disk and memory consistent even if flush fails (memory untouched on error, same as `reload`).
- **No separate confirmation endpoint.** The client-side confirmation dialog (native `confirm()` or existing modal pattern) is the safety gate, consistent with how destructive actions are handled elsewhere in this codebase (no server-side "are you sure" step exists today).

## Risks / Trade-offs

- [Importing a backup silently discards current data] → Client-side confirmation dialog before calling the endpoint; recommend users hit `/export` first (documented in Settings UI copy).
- [Schema version newer than server supports] → Reject with 400 rather than attempting to load; matches `migrate()` only handling forward migrations, never downgrades.
- [Large JSON body] → No explicit size cap beyond axum's default body limit; acceptable since this mirrors the trust level of the existing `/setting` PUT and local-network deployment model.
