## Context

The Spoolman-light frontend is built with Leptos 0.6 (WASM/CSR). All components apply class names (`.app-shell`, `.sidebar`, `.main-content`, `.data-table`, `.color-swatch`, `.dark`, `.btn`, etc.) but no CSS file exists. `cargo-leptos` compiles `spoolman-server.css` from any `style-file` configured in `Leptos.toml`, or from `stylers`-generated scoped CSS — currently both are empty/absent.

The `stylers` crate (0.3.x) provides a `style!` proc-macro that accepts CSS literals inside a Rust macro, scopes them to the component via a generated class hash, and emits them for `cargo-leptos` to bundle into `spoolman-server.css` at build time. Version 0.3.2 targets Leptos 0.6.

## Goals / Non-Goals

**Goals:**
- Make the app visually usable: layout, navigation, table, forms, buttons
- Implement dark mode (`.dark` on `<body>`) with visible effect
- Color swatch (`.color-swatch`) renders as a colored inline square
- All existing class names continue to work — no template changes required
- Styles live in the same file as the Leptos component (co-location)

**Non-Goals:**
- Full design-system polish or brand theming (light theme matching logo is a separate TODO)
- CSS animations or transitions
- Responsive/mobile breakpoints (deferred)
- Replacing existing class-based approach with a CSS framework (Tailwind, etc.)

## Decisions

### Decision 1: Use `stylers` scoped CSS over a plain `assets/style.css`

**Choice:** `stylers 0.3.2` proc-macro with `style!` / `use_style!` in each component.

**Rationale:**
- Styles are co-located with components — easier to maintain.
- Compile-time scoping prevents class collisions as the codebase grows.
- `cargo-leptos` natively supports `stylers` output bundling into `spoolman-server.css`.
- The TODO item explicitly lists `stylers 0.3.2` as the preferred approach.

**Alternatives considered:**
- Plain `assets/style.css` — simpler, but global namespace; requires manually keeping class names in sync.
- Tailwind — requires Node.js toolchain; out of scope for a Rust-only build.

### Decision 2: One `style!` block per file, not per component

**Choice:** Define all styles for a file in a single `style!` macro at the top of the file, shared via `use_style!` across all components in that file.

**Rationale:** Reduces boilerplate. Components in the same file share the same logical scope (e.g., `spool.rs` owns all spool-related classes). A single `StyleId` per file is sufficient.

### Decision 3: CSS variable-based dark mode

**Choice:** Define light-mode values as CSS custom properties on `:root`, and override them under `body.dark { ... }`.

**Rationale:** The existing dark-mode toggle already adds/removes the `.dark` class on `<body>`. CSS variables make the override compact and consistent with the existing mechanism.

### Decision 4: Do not change existing class names

**Choice:** Keep all existing class attributes in Leptos templates as-is.

**Rationale:** `stylers` scopes by adding a generated attribute to the host element; the class names themselves remain unchanged. Avoids touching template code and reduces diff noise.

## Risks / Trade-offs

- **`stylers` proc-macro may conflict with `wasm32` target build** → `stylers` runs at compile time on the host (proc-macros are always host-native), so WASM target is not an issue.
- **`cargo-leptos` version compatibility** → `stylers 0.3.2` is documented for Leptos 0.6 + cargo-leptos 0.2/0.3. Verify output CSS is non-empty after build.
- **Scoped class names vs global `.dark` on `<body>`** → `body.dark` selectors must be written as `:global(body.dark)` inside a `style!` block, or placed in a global stylesheet. Mitigation: use `style_sheet!` macro for the body-level dark override, or define a thin `assets/dark.css` for the single body override only.
- **Windows build still blocked** → validation requires WSL/Docker build; cannot verify CSS output on Windows.

## Migration Plan

1. Add `stylers = "0.3.2"` to `crates/spoolman-client/Cargo.toml`.
2. Add `[features] stylers = []` or ensure `cargo-leptos` picks up stylers output (check `Leptos.toml` for `style-file` vs auto-detection).
3. Add `style!` blocks to: `layout.rs`, `table.rs`, `components/pagination.rs`, `pages/spool.rs`, `pages/filament.rs`, `pages/location.rs`, `pages/settings.rs`.
4. Build in WSL/Docker and confirm `target/site/pkg/spoolman-server.css` is non-empty and styles render.

**Rollback:** Remove `stylers` dependency and `style!` / `use_style!` calls. No data or API changes.
