## Context

The `spoolman-client` crate uses `create_effect(move |_| { … })` from `leptos::prelude` in 11 places across 6 source files. Leptos has marked this function as deprecated in favour of the `Effect::new()` associated function on the `Effect` type. The change is purely mechanical — the closure signature and reactive semantics are identical.

## Goals / Non-Goals

**Goals:**
- Replace every `create_effect(move |_| { … })` call with `Effect::new(move |_| { … })`.
- Remove any `use leptos::prelude::create_effect;` imports and add `use leptos::prelude::Effect;` where needed.
- Eliminate all deprecation compiler warnings related to `create_effect`.

**Non-Goals:**
- Changing reactive logic, closure bodies, or effect ordering.
- Updating any other deprecated Leptos APIs beyond `create_effect`.
- Modifying the server or types crates.

## Decisions

**Mechanical text substitution per file, not a blanket `sed` across the workspace.**

Each file already imports from `leptos::prelude` via a glob `use leptos::prelude::*` or explicit import. For glob imports no import change is needed — `Effect` is already in scope. For explicit imports, swap `create_effect` for `Effect` in the use list.

Rationale: targeted per-file edits are easier to review and less likely to touch unrelated code than a project-wide sed pass.

**Use `Effect::new` (not `Effect::new_sync` or other variants).**

`create_effect` maps directly to `Effect::new` — same closure type, same dependency tracking behaviour. The `_sync` variant has different semantics (runs synchronously) and is not a drop-in replacement.

## Risks / Trade-offs

- **Subtle API difference** → `Effect::new` returns an `Effect` handle; `create_effect` returned `()`. Discarding the handle is fine (the effect is kept alive by the reactive runtime for the lifetime of the owner), but if any call-site were to bind the return value that code would need adjustment. A quick scan shows no call-sites capture the return value, so this is low risk.
- **Upstream Leptos version** → If the project upgrades Leptos before this lands, the deprecated symbol may already be removed, causing compile errors instead of warnings. Implementing promptly removes this risk.

## Migration Plan

1. Edit each of the 6 files, replacing `create_effect(` with `Effect::new(` and updating explicit imports.
2. Run `cargo check -p spoolman-client --target wasm32-unknown-unknown` (or equivalent) to confirm zero deprecation warnings and no new errors.
3. Commit the change on a feature branch and open a PR.
