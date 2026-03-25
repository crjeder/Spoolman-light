## Context

The Leptos frontend defines its route table in `crates/spoolman-client/src/app.rs`. Currently `"/"` maps to `HomePage`, a simple welcome/dashboard stub. `"/spools"` maps to `SpoolList`, the primary working view. Users must click "Spools" in the nav after every page load to get to their data.

The change is a single-file routing update with a small cleanup.

## Goals / Non-Goals

**Goals:**
- `"/"` renders `SpoolList` directly (no redirect, no extra round-trip)
- Remove the now-unused `HomePage` component and module
- Nav bar active-link highlight treats `"/"` as equivalent to `"/spools"`

**Non-Goals:**
- No new features on the spool list page
- No backend or API changes
- No persistent user preference for a configurable landing page

## Decisions

**Render `SpoolList` at `"/"` rather than issuing a `<Redirect>`**

A redirect adds a navigation round-trip and changes the URL in the browser history. Rendering the component directly at `"/"` keeps the URL clean and avoids a flicker. Leptos `<Route>` supports this natively.

**Delete `HomePage` rather than keeping it as a dead route**

The home page was a placeholder. No links point to it and it carries no unique content. Keeping dead routes increases maintenance surface; removing it is the right call.

**Active-link highlight for `"/spools"`**

The nav link for Spools currently uses `href="/spools"`. After this change, users arriving at `"/"` will see the Spools page but the nav link won't be highlighted. The fix is to also match `"/"` when deciding whether the Spools link is active. Leptos's `<A>` component with `exact=false` or a manual `is_active` signal can handle this.

## Risks / Trade-offs

- **Bookmarked `/` URLs** — users who bookmarked `"/"` expecting the home page will now land on the spool list. This is the intended outcome.
- **Nav highlight logic** — if the active-link logic is not updated, the Spools nav item will appear inactive when the user is at `"/"`. Low risk but worth a deliberate fix.

## Migration Plan

1. Update `app.rs` route table
2. Update nav active-link logic (if needed)
3. Delete `pages/home.rs` and remove `pub mod home` from `pages/mod.rs`
4. `cargo check` to verify no remaining references

Rollback: revert the three file changes; no data or schema migration involved.
