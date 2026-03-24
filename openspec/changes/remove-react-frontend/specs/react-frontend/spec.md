## REMOVED Requirements

### Requirement: React frontend build and dev tooling
The repository SHALL NOT contain the React/TypeScript/Vite frontend codebase (`client/`) or its Node.js build tooling. The Leptos WASM frontend (`crates/spoolman-client/`) is the sole frontend implementation.

**Reason**: The React frontend is superseded by the Rust/Leptos rewrite. Retaining it creates maintenance burden, repository bloat, and ambiguity about which frontend is canonical.

**Migration**: Use the Leptos frontend in `crates/spoolman-client/`. Build with `cargo leptos build` (requires WSL/Linux/Docker on Windows due to OpenSSL dependency).

#### Scenario: No client directory in repository
- **WHEN** the repository is checked out
- **THEN** there SHALL be no `client/` directory at the repository root

#### Scenario: No npm tooling for frontend
- **WHEN** a developer examines build tooling
- **THEN** there SHALL be no `package.json`, `vite.config.ts`, or `node_modules/` under `client/`

#### Scenario: Documentation does not reference React frontend
- **WHEN** a developer reads CLAUDE.md or README.md
- **THEN** no commands or stack details SHALL reference `npm`, `cd client`, or the React/Vite/Refine stack
