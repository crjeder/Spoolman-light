## Context

The Spoolman project has two frontend implementations:
1. **`client/`** — The original React 19 + TypeScript + Vite frontend (legacy, deprecated)
2. **`crates/spoolman-client/`** — The new Leptos WASM frontend (active, in progress)

The React frontend is no longer being developed and will be removed once the Rust stack is verified. That verification milestone is now considered reached for the purposes of this cleanup. The `client/` directory contains ~the entire legacy frontend codebase including npm dependencies, Vite config, Refine framework integration, Ant Design components, and i18n files.

## Goals / Non-Goals

**Goals:**
- Delete the `client/` directory and all its contents
- Remove Node.js/npm tooling references from docs and configs that only served the React frontend
- Update CLAUDE.md to remove the "Frontend" commands section and the React stack from "Stack Details"
- Update CHANGELOG.md to record the removal

**Non-Goals:**
- Modifying the Python backend (`spoolman/`) — unaffected
- Modifying the Rust workspace (`crates/`, `Cargo.toml`, `Leptos.toml`) — unaffected
- Removing the Python backend itself — that is a separate future change
- Changing CI/CD pipelines (there are none in this repo)
- Migrating any React components to Leptos — that work belongs to the Rust rewrite change

## Decisions

### Delete `client/` outright — no archiving
The React frontend has no unique content worth preserving. All functional requirements it served are being reimplemented in Leptos. Git history preserves the code for anyone who needs it. Archiving (e.g., moving to `_archive/`) would leave dead code in the tree.

**Alternative considered:** Keep `client/` until Leptos frontend reaches feature parity.
**Rejected:** Feature parity is the goal of the ongoing Rust rewrite work; blocking this cleanup on it adds ongoing confusion and maintenance friction. The two concerns are independent.

### Update docs in the same commit
CLAUDE.md explicitly documents `cd client && npm run dev` commands and lists the React stack. Leaving these after deletion would cause immediate confusion. Update them in the same change.

## Risks / Trade-offs

- **Risk: Leptos frontend not yet fully usable** → The Python backend still serves the API; the Leptos client is in active development. Removing the React frontend means no browser UI until Leptos is complete.
  **Mitigation:** This is an accepted trade-off per the project's design decision to replace the stack. The API remains fully functional for CLI/direct access.

- **Risk: `vite.config.ts` has uncommitted modifications** (per git status) → Must verify what changed and whether it matters before deleting.
  **Mitigation:** Read the diff; if it's trivial, discard and delete. If it contains meaningful work, surface to user before proceeding.

## Migration Plan

1. Check `git diff client/vite.config.ts` — verify the uncommitted change is disposable
2. Delete the `client/` directory
3. Update `CLAUDE.md` — remove Frontend commands block, remove React from Stack Details
4. Update `README.md` if it references `client/` or React
5. Update `CHANGELOG.md`
6. Commit all changes on the current feature branch

No rollback strategy needed — git history preserves everything.

## Open Questions

_None._
