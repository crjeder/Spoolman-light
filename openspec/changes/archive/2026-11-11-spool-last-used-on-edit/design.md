## Context

The edit form (`crates/spoolman-client/src/pages/spool.rs`) prefills `last_used` from the spool (or today if none) and always submits it. The server (`SpoolStore::update_spool`) only stamps `last_used = now` when `current_weight` is sent and `last_used` is absent. Because the form sends the old date, the stamp never happens for spools that already have one.

## Goals / Non-Goals

**Goals:**
- Saving the edit form refreshes `last_used` to today unless the user changed that field.

**Non-Goals:**
- Changing the PATCH API semantics or `first_used` handling.
- Making the server detect "real" weight changes.

## Decisions

- **Client-side fix.** The server cannot distinguish "form resubmitted the old date" from "user chose that date", so the form decides. On load, remember the loaded `last_used` string (`last_used_loaded`). On submit, if `last_used.get() == last_used_loaded`, send today's date instead. A user who edits the date input to any other value keeps their choice.
- **Any save counts as an edit**, not only weight changes. Simpler than diffing every field, and matches the request ("when editing a spool, e.g. current weight").
- Replaces the existing `last_used_was_none`/`prune_default` pair for `last_used` only: with this rule the untouched default is always "today", so no pruning is needed. `first_used` keeps its pruning.
- Alternative rejected: server stamps `last_used` whenever `current_weight` differs from stored. Does not cover non-weight edits and still needs the client to stop sending a stale `last_used`.

## Risks / Trade-offs

- Saving an unmodified form bumps `last_used`. Acceptable: saving is treated as use.
- User who deliberately wants to keep an old date unchanged cannot (it is overwritten when untouched). They can retype a different date, or set it after; accepted trade-off.
