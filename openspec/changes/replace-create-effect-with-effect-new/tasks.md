## 1. Create worktree

- [x] 1.1 Create a git worktree on a new branch (e.g. `feat/replace-create-effect`) in `.worktrees/`

## 2. Replace call-sites

- [x] 2.1 Update `crates/spoolman-client/src/app.rs`: replace `create_effect(` with `Effect::new(` and fix imports
- [x] 2.2 Update `crates/spoolman-client/src/state.rs`: replace 4 occurrences of `create_effect(` with `Effect::new(` and fix imports
- [x] 2.3 Update `crates/spoolman-client/src/components/layout.rs`: replace `create_effect(` with `Effect::new(` and fix imports
- [x] 2.4 Update `crates/spoolman-client/src/pages/filament.rs`: replace `create_effect(` with `Effect::new(` and fix imports
- [x] 2.5 Update `crates/spoolman-client/src/pages/settings.rs`: replace 2 occurrences of `create_effect(` with `Effect::new(` and fix imports
- [x] 2.6 Update `crates/spoolman-client/src/pages/spool.rs`: replace 2 occurrences of `create_effect(` with `Effect::new(` and fix imports

## 3. Verify

- [x] 3.1 Run `cargo check -p spoolman-client` (or with `--target wasm32-unknown-unknown` if available) and confirm zero `create_effect` deprecation warnings and no new errors
- [x] 3.2 Grep the `crates/` directory to confirm no remaining `create_effect` references
