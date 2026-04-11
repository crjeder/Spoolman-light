# Spec: Leptos API Compatibility

## Purpose

Ensure the `spoolman-client` crate uses current, non-deprecated Leptos APIs so that builds remain warning-free and the codebase stays aligned with upstream Leptos development.

## Requirements

### Requirement: No deprecated Leptos effect API usage
The `spoolman-client` crate SHALL use `Effect::new()` from `leptos::prelude` instead of the deprecated `create_effect()` function. No call-site in the codebase SHALL reference `create_effect`.

#### Scenario: Build produces no create_effect deprecation warnings
- **WHEN** `cargo check` (targeting `wasm32-unknown-unknown`) is run on the `spoolman-client` crate
- **THEN** no warnings referencing `create_effect` are emitted

#### Scenario: Effect behaviour is preserved
- **WHEN** a reactive dependency changes at runtime
- **THEN** the associated effect closure runs exactly as it did before the migration (same reactive semantics, same execution order)
