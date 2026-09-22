## Context

The data file lives at an operator-configured path (`SPOOLMAN_DATA_FILE`), served by `JsonStore` (`crates/spoolman-server/src/store.rs`). `JsonStore::reload()` already re-reads and migrates that path in place. `GET /api/v1/export` already returns the full `DataStore` as JSON (in-memory, not the raw file), and the Settings page already has a "Reload database" section with its own status signals (`crates/spoolman-client/src/pages/settings.rs`). This change adds a symmetric download/upload pair that operates on the raw file rather than the in-memory export, so a downloaded file can be re-uploaded byte-for-byte and round-trips through the same validation the server already applies on startup.

## Goals / Non-Goals

**Goals:**
- Let an operator download the current data file as an attachment from the Settings page.
- Let an operator upload a replacement data file, validated before it overwrites anything, then reloaded into memory immediately.
- Reuse `JsonStore`'s existing read/migrate/flush logic rather than adding a parallel file-handling path.

**Non-Goals:**
- No versioned backup history or automatic retention (that's the separate, still-stubbed `backup.rs` feature).
- No partial/merge import — upload always replaces the whole file.
- No new auth/permission model — same trust level as the rest of the settings API (single-operator deployment, no per-endpoint auth today).

## Decisions

- **Download serves the raw file bytes, not `get_full_store()` re-serialized.** Re-serializing risks reformatting or silently normalizing fields; streaming the file as-is guarantees round-trip fidelity for backup/restore. Read via `std::fs::read` on the store's already-canonicalized path (same trust boundary as `read_and_migrate`).
- **Upload validates by deserializing into `DataStore` before writing anything to disk.** Reuses `serde_json::from_str` the same way `read_and_migrate` does, so a malformed upload never touches the live file. On success, write to a temp file in the same directory and rename over the target (atomic replace, avoids truncating the live file on a crash mid-write), then call the existing `reload()` path to run migrations and refresh memory.
- **No new request size limit beyond Axum's default body limit.** The data file is a small JSON document (spool/filament/location records); default limits are generous enough and adding a custom one is unwarranted complexity for this data shape.
- **Client uses a plain `<input type="file">` + `fetch` with the raw file body** (`Content-Type: application/json`), not multipart — the endpoint only ever receives one whole-file body, so multipart parsing would be unused ceremony.
- **Confirmation before upload.** Upload is destructive (replaces the live file); the button requires a native `confirm()`-style guard before firing the request, matching the risk level of "Reload database" but calling it out explicitly since data loss (not just staleness) is possible if the wrong file is chosen.

## Risks / Trade-offs

- [Uploading an old/wrong file silently discards recent changes] → Confirmation dialog states this plainly; no auto-backup is created by this change (out of scope — see `backup.rs` stub for the planned safety net).
- [Concurrent write during upload (another request mutating the store while the temp-file rename happens)] → The rename only replaces the on-disk file; the in-memory store is swapped atomically under the existing `RwLock` inside `reload()`, same as manual-file-replace-then-reload already behaves today.
- [Large file upload blocking the async runtime] → Data files are small (KB-MB range for realistic spool counts); no streaming/chunked upload needed.

## Migration Plan

No data migration. New endpoints and UI section only; existing endpoints and routes are unchanged. Deployable and revertible independently (no schema or config changes).

## Open Questions

None — scope is bounded to the two endpoints and the Settings UI section described in the proposal.
