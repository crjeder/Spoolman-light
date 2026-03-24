## Why

The React/TypeScript frontend (`client/`) is being replaced by the Rust/Leptos WASM frontend as part of the full Rust rewrite. Keeping the deprecated React frontend in the repository increases maintenance burden, causes confusion about which frontend is authoritative, and bloats the repository with dead code.

## What Changes

- **BREAKING**: Remove the `client/` directory entirely (React 19 + TypeScript + Vite frontend)
- Remove all npm/Node.js tooling configuration (`client/package.json`, `client/vite.config.ts`, etc.)
- Remove frontend-related CI steps that reference `client/`
- Remove references to the React frontend from documentation, CLAUDE.md, and README
- Update `CHANGELOG.md` to document the removal

## Capabilities

### New Capabilities

_None — this is a removal, not an addition._

### Modified Capabilities

_None — no spec-level behavior changes. The React frontend has no corresponding spec (it was the legacy stack being replaced). The Leptos frontend already covers UI capabilities via existing specs._

## Impact

- **Removed**: `client/` directory (~entire React frontend codebase)
- **Removed**: `client/node_modules/` (if present), `client/dist/` (build output)
- **Updated**: `CLAUDE.md` — remove references to React frontend commands and stack details
- **Updated**: `README.md` (if it references the React frontend)
- **Updated**: `CHANGELOG.md` — log the removal
- **No API impact** — the Python backend and Rust backend are unaffected
- **No Rust impact** — `crates/` directory is unaffected
