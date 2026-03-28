## Context

Currently `Filament.net_weight: Option<f32>` stores the intended filament mass per spool. `SpoolResponse::new` reads that value from the embedded filament to compute `remaining_filament` and `remaining_pct`. Because net weight lives on the filament, all spools sharing the same filament type must have the same net weight, which breaks down when a manufacturer sells the same material in 250g, 750g, and 1kg formats.

The change moves `net_weight` to `Spool` so each spool can carry its own net weight independently of the filament specification.

Affected surfaces:
- `spoolman-types`: `Filament`, `Spool`, `CreateFilament`, `UpdateFilament`, `CreateSpool`, `UpdateSpool`, `SpoolResponse`
- `spoolman-server`: `SpoolResponse::new`, JSON store migration (schema bump)
- `spoolman-client`: filament form, spool create/edit form, weight display components

## Goals / Non-Goals

**Goals:**
- `Spool` owns `net_weight: Option<f32>`
- `Filament` no longer has `net_weight`
- Weight derivation (`remaining_filament`, `remaining_pct`) reads from `spool.net_weight`
- Existing data is migrated non-destructively on server start
- Frontend forms updated accordingly

**Non-Goals:**
- Inferring net weight from spool purchase history or any external catalog
- Removing `spool_weight` from `Filament` (it stays — different concept)
- Changing how `used_weight` is derived

## Decisions

### 1. Migration strategy: copy-then-clear on schema version bump

On server start, if `meta.schema_version == 1`, the server migrates the store to version 2:
- For each spool, look up its filament's `net_weight` and copy it into `spool.net_weight`
- After all spools are updated, clear `net_weight` on all filaments
- Bump `meta.schema_version` to 2 and save atomically

**Alternatives considered:**
- *Lazy migration per request* — deferred correctness, harder to reason about and test.
- *No migration, accept data loss* — net weights on existing filaments would be silently dropped. Rejected: user data loss is unacceptable.

### 2. SpoolResponse::new reads spool.net_weight directly

The constructor signature stays the same but the computation changes:
```rust
let remaining_filament = spool.net_weight.map(|nw| nw - used_weight);
let remaining_pct = spool.net_weight.filter(|&nw| nw > 0.0)
    .map(|nw| (nw - used_weight) / nw * 100.0);
```
`filament.net_weight` is no longer accessed.

### 3. SpoolmanDbEntry keeps its own net_weight

`SpoolmanDbEntry` (proxy result from SpoolmanDB) retains `net_weight` since it describes a product listing. When the user imports from SpoolmanDB, the frontend populates `CreateSpool.net_weight` from the entry.

## Risks / Trade-offs

- **Migration is one-way** → No rollback to schema v1 without restoring a backup. Mitigation: atomic write + backup stub already in place; server logs migration clearly.
- **Spools created before this change with no filament net_weight** → Their `spool.net_weight` will be `None` post-migration; users must enter manually. Expected: most affected users had `None` already.
- **Client form change is a UX shift** — users who previously set net weight on a filament and cloned spools will need to set it per spool. Acceptable given the semantic improvement.

## Migration Plan

1. Server starts and loads `DataStore` from disk.
2. If `meta.schema_version < 2`:
   a. Build a `filament_id → net_weight` lookup map.
   b. For each spool: set `spool.net_weight = lookup[spool.filament_id]`.
   c. For each filament: set `filament.net_weight = None`.
   d. Set `meta.schema_version = 2`.
   e. Atomically write store to disk.
3. Continue normal startup.

Rollback: restore `spoolman.json` from pre-migration backup (user responsibility; document in release notes).

## Open Questions

- Should the clone-spool operation copy `net_weight` from the source spool? (Likely yes — clone pre-fills all spool-specific fields.)
