## Context

Color search in the spool list computes perceptual distance using CIEDE2000 (ΔE\*00) via the `deltae` crate. CIEDE2000 is the ICC/ISO standard, but it is computationally complex and its corrections for blues and bright chromas can still feel non-uniform to users picking filament colors visually. OKLab (Björn Ottosson, 2020) is a modern perceptually-uniform space that produces more intuitively consistent results; DIN99d is a German standard variant of CIELAB with improved uniformity. The TODO explicitly requests configurable algorithm selection using the `oklab` crate.

The existing settings system stores key-value `String` pairs and is already wired for context propagation (see `DiameterSettings`, `CurrencySymbol`). No server-side changes are needed.

## Goals / Non-Goals

**Goals:**

- Support three color distance algorithms: CIEDE2000 (default), OKLab, DIN99d.
- Persist user choice under the `color_distance_algorithm` settings key.
- Surface the choice as a `<select>` on the Settings page.
- Propagate the active algorithm reactively via Leptos context so the spool list responds to settings changes without reload.
- Keep threshold labels (Off / Same / Close / Ballpark) working correctly for each algorithm by using per-algorithm numeric values.

**Non-Goals:**

- Server-side distance computation.
- Exposing algorithm choice in the REST API.
- Changing the color picker, color display, or NFC/QR URL behavior.
- Sorting spools by delta (a separate TODO item).

## Decisions

### D1 — Use `oklab` crate for OKLab; implement DIN99d directly

The `oklab` crate is a tiny, pure-Rust, `no_std`-compatible crate that converts sRGB to OKLab and computes Euclidean distance (ΔE_ok). It compiles to WASM without issues.

DIN99d is a simple closed-form transform on CIE L\*a\*b\* — roughly 8 lines of math. There is no widely-used dedicated crate for it; the `palette` crate supports it but adds ~200 KB to WASM binary. Since we already compute CIE L\*a\*b\* for CIEDE2000, implementing DIN99d inline is low-cost and avoids a heavy dependency.

**Alternative rejected**: `palette` crate — feature-complete but large binary footprint and complex API for this narrow use case.

### D2 — Algorithm enum lives in `utils::color`, dispatched per call

```rust
pub enum ColorAlgorithm { Ciede2000, OkLab, Din99d }

pub fn color_distance(a: &Rgba, b: &Rgba, algo: ColorAlgorithm) -> f32 { … }
```

All three algorithms operate on the same input type (`Rgba`) and return an `f32`. No trait object or dynamic dispatch needed. The existing `color_distance(a, b)` signature becomes `color_distance(a, b, algo)` — call sites (only `spool.rs`) pass the algorithm from context.

**Alternative rejected**: Keeping old signature with a global default — would require a hidden global or thread-local, conflicting with Leptos reactivity model.

### D3 — Per-algorithm threshold constants for Same / Close / Ballpark

The three algorithms use different numeric scales:

| Level    | CIEDE2000 | OKLab   | DIN99d |
|----------|-----------|---------|--------|
| Same     | 10.0      | 0.10    | 10.0   |
| Close    | 20.0      | 0.20    | 20.0   |
| Ballpark | 35.0      | 0.35    | 35.0   |

DIN99d uses the same 0–100 scale as CIEDE2000 (by construction), so thresholds are identical. OKLab ΔE_ok is in [0, 1]; 0.10 is approximately the CIEDE2000 "similar" threshold when normalized to the [0, 1] range.

A function `threshold_for(level: &str, algo: ColorAlgorithm) -> Option<f32>` encapsulates this, keeping `spool.rs` free of algorithm-specific constants.

**Label rationale**: `Same` signals the strictest non-off match without overpromising exactness (ΔE ≤ 10 is visually similar, not pixel-identical). `Close` is self-evident. `Ballpark` is intentionally idiomatic — its informal register reinforces the looseness of the match (ΔE ≤ 35, same hue family).

**Alternative rejected**: Normalizing all algorithms to a common 0–1 scale internally — obscures the natural scale of each metric, making it harder to reason about raw values in future features (e.g., sort-by-delta).

### D4 — `ColorDistanceAlgorithm` context follows `DiameterSettings` pattern

```rust
// state.rs
#[derive(Clone, Copy, PartialEq)]
pub enum ColorAlgorithm { Ciede2000, OkLab, Din99d }

#[derive(Clone, Copy)]
pub struct ColorDistanceAlgorithm(pub RwSignal<ColorAlgorithm>);
```

`app.rs` reads `color_distance_algorithm` from settings on load and calls `provide_context(ColorDistanceAlgorithm(signal))`. The Settings page writes back via `api::put_setting` and updates the signal directly (same pattern as `DiameterSettings`).

## Risks / Trade-offs

- **OKLab thresholds feel different** — Users who switch from CIEDE2000 to OKLab will see the same label ("Same") but different results because the scale change is not perfectly proportional at all hue angles. Mitigation: document in Settings page tooltip or label (e.g., "OKLab — more uniform for saturated colors").
- **DIN99d is hand-implemented** — The transform formula is well-documented (DIN 6176:2001) but not validated by a third-party crate. Mitigation: add a unit test against known reference pairs.
- **`deltae` crate retained** — We continue to use `deltae` for CIEDE2000. Adding `oklab` means two color-math dependencies. Both are small; combined weight increase is negligible.
- **Algorithm context not available in SSR** — Leptos SSR would need to hydrate the context server-side. This project is currently CSR-only, so no issue today.

## Migration Plan

1. Add `oklab` to `spoolman-client/Cargo.toml`.
2. Extend `utils/color.rs` with the `ColorAlgorithm` enum and updated `color_distance`.
3. Add `ColorDistanceAlgorithm` to `state.rs`.
4. Wire context provision and settings fetch in `app.rs`.
5. Update `settings.rs` to add the algorithm selector.
6. Update `spool.rs` to consume algorithm from context.
7. Run `cargo clippy` to catch any exhaustiveness issues on the new enum.

No data migration needed — the setting key defaults to `ciede2000` when absent, preserving existing behavior for all users.

## Open Questions

- None blocking implementation. The threshold values for OKLab (D3) are reasonable defaults but could be tuned based on user feedback post-release.
