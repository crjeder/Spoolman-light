## Context

Spool color is currently a manually-typed `Vec<Rgba>` (1-4 colors) plus an optional `color_name`, rendered as flat CSS-background chips in three independent places: the spool list table (`crates/spoolman-client/src/pages/spool.rs`), the spool detail view (same file), and the `/colors` swatch grid (`crates/spoolman-client/src/components/swatch_grid.rs`, `fill_style()`). There is no shared `<ColorSwatch>` component.

The existing SpoolmanDB lookup feature (`crates/spoolman-client/src/spoolmandb.rs` + `components/spoolmandb_search.rs`) is the direct precedent: WASM-only fetch, 24h localStorage cache with ETag, client-side filter, `Callback<Entry>` on select, wired into spool/filament create and edit forms. filamentcolors.xyz has `CORS_ALLOW_ALL_ORIGINS = True`, so the same direct-fetch approach works without a proxy.

Unlike SpoolmanDB, filamentcolors.xyz swatches carry a product photo, and this change stores the photo server-side (binary data does not belong in localStorage, and the server is the natural place to keep it alongside `spoolman.json`). There is currently no image storage or serving mechanism in `spoolman-server` at all — this is genuinely new server surface, not a gap in an existing pattern.

## Goals / Non-Goals

**Goals:**
- Reuse the SpoolmanDB lookup pattern for filamentcolors.xyz data and caching.
- Let the user apply a swatch's measured hex color to a spool with one click.
- Persist the swatch photo server-side and show it wherever a spool's color is currently shown as a flat chip.
- Zero impact on spools that never use the lookup (fully optional, backward compatible).

**Non-Goals:**
- Filament-level (as opposed to spool-level) filamentcolors.xyz integration.
- Editing or re-cropping the downloaded image.
- Any proxy/server-side fetch of the filamentcolors.xyz JSON API (client fetches it directly; only images go through the server).
- A generic/reusable file-upload subsystem — this ships the minimum needed for one image per spool.

## Decisions

**Fetch filamentcolors.xyz data straight from WASM, same as SpoolmanDB.** No proxy needed given `CORS_ALLOW_ALL_ORIGINS`. Alternative considered: routing through the server to shield the API key/rate limit — rejected, the API is anonymous and already rate-limited per-IP, and a server hop adds nothing.

**Cache the swatch list (JSON) in localStorage, 24h TTL, mirroring `spoolmandb.rs`'s `CacheEntry` shape.** Reuses a proven, already-reviewed pattern; no new caching design needed. Key: `filamentcolors_cache`.

**Store only images server-side; never put image bytes in localStorage.** localStorage has a small quota (5-10MB) shared with the SpoolmanDB and swatch-grid-favourites caches already living there. Images belong on disk next to `spoolman.json`.

**New Spool field `swatch_image: Option<String>` holding a stored image reference (filename/id), not the source URL.** Keeps the reference stable even if filamentcolors.xyz changes its URL scheme, and lets the server dedupe by source URL internally without exposing that mapping to the client.

**New server endpoints:**
- `POST /api/v1/swatch-image` — body: `{ url: string }` (a filamentcolors.xyz image URL). Server downloads it if not already stored (dedupe by source URL, e.g. a lookup table keyed by URL hash), saves to a new `swatch_images/` directory next to the data file, returns `{ id: string }`.
- `GET /api/v1/swatch-image/{id}` — serves the stored image bytes with content type.

  Alternative considered: store images as base64 inside `spoolman.json` — rejected, bloats the JSON store and defeats its purpose as a lightweight document; a plain file on disk is simpler and is already how the app treats its data directory.

**Extract a shared `<ColorSwatch>` (or similar) rendering helper used by all three display sites (list, detail, grid) instead of duplicating "image if present, else flat chip" logic three times.** The three sites already duplicate the flat-chip rendering; adding image-fallback logic to three call sites verbatim would be a fourth copy of near-identical logic. One small shared component/function is the lazier and more correct move here, not premature abstraction — three real call sites already exist today.

**No new crate dependency for image download/storage.** The server already depends on an HTTP client (reused for the download) and `std::fs` (reused for storage); no image-processing crate is needed since the image is stored and served as-is, unmodified.

## Risks / Trade-offs

- [Downloaded image could be large or slow to fetch, blocking spool save] → Mitigation: the image download happens as a fire-and-forget request from the client after selecting a swatch, before form submission; the form does not block on it, and a failed download simply leaves `swatch_image` unset (spool save is unaffected).
- [Disk usage grows unbounded as `swatch_images/` accumulates] → Mitigation: dedupe by source URL means one image is stored once regardless of how many spools reference it; images are typically small product photos. Orphan cleanup (when the last referencing spool is deleted) is out of scope for this change and can be a follow-up.
- [filamentcolors.xyz anonymous rate limit (100/min, 3600/hour per IP) could be hit by the full-swatch-list fetch if many users share an egress IP] → Mitigation: the 24h cache means at most one full-list fetch per browser per day; this mirrors the already-accepted SpoolmanDB rate profile.
- [`.semgrepignore` gotcha: the new `swatch-image` download route takes a URL from the client and writes to disk — this looks like the same path-traversal false-positive pattern already documented in CLAUDE.md for other `std::fs` routes] → Mitigation: never use client-supplied data as a filename; generate the on-disk filename/id server-side (e.g. hash of the source URL), never from user input directly.

## Migration Plan

- `swatch_image: Option<String>` defaults to `None` via serde default; existing `spoolman.json` files deserialize unchanged (no migration script needed, consistent with how `price` was added previously).
- New `swatch_images/` directory is created on first use (lazily), not required at startup.
- No rollback concerns beyond reverting the change; no destructive migration is performed.

## Open Questions

- Exact filamentcolors.xyz endpoint/fields to use for search (`SwatchViewSet` list vs `bulk_colormatch`) — resolved at implementation time by inspecting the live API response shape; this change uses simple client-side substring search over the cached swatch list, not `bulk_colormatch` (that endpoint is for closest-match-to-hex, a different feature not requested here).
