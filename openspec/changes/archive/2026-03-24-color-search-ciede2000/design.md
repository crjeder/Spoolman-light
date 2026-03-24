## Context

Color search in the spool list page filters spools by comparing their stored `Rgba` colors against a user-picked color using `rgb_distance()` — a straight Euclidean distance in sRGB space. The result is that the perceived "closeness" of a color match does not correspond to what the user sees, because sRGB is highly non-uniform perceptually.

CIEDE2000 (ΔE\*00) is the current CIE standard for perceptual color difference. It operates in CIE L\*a\*b\* space with additional correction terms for blue hues, chroma, and hue angle. ΔE < 1 is imperceptible; ΔE < 10 is "similar"; ΔE > 25 is clearly different. The algorithm has well-known edge cases (achromatic hue, the R_T blue-range correction term) where hand-rolled implementations commonly contain subtle bugs.

The change is entirely within the Leptos WASM frontend crate (`spoolman-client`). No server-side code, data model, or API is affected.

## Goals / Non-Goals

**Goals:**
- Replace Euclidean RGB distance with CIEDE2000 ΔE\*00 in `color.rs`.
- Add pure-Rust sRGB → XYZ D65 → CIE L\*a\*b\* conversion helpers.
- Update the default threshold in `spool.rs` to a value appropriate for the 0–100 ΔE scale.
- Use `deltae = "0.3.2"` for the CIEDE2000 computation; implement sRGB→Lab conversion inline.
- Preserve the existing function signature shape so call sites need only minimal changes.

**Non-Goals:**
- Server-side color filtering (remains client-side).
- Support for ICC profiles or wide-gamut color spaces.
- Exposing the threshold as a user-configurable setting (that is a future change).
- Changing the `Rgba` data model or wire format.

## Decisions

### 1. Use `deltae` crate for the CIEDE2000 formula; implement sRGB→Lab inline

**Decision:** Add `deltae = "0.3.2"` as the only new dependency. Write the sRGB→Lab conversion (linearisation + XYZ matrix + cube-root transform) ourselves in `color.rs`.

**Rationale:** CIEDE2000 has known edge cases — achromatic color handling (ΔH\' = 0 when C\' = 0), the R_T blue-range correction, and precise atan2 branching — where hand-rolled implementations commonly introduce subtle bugs that only surface with specific inputs. `deltae` is a battle-tested, actively maintained implementation of the formula. Critically, it has **zero transitive dependencies** (verified via `cargo tree`) and is pure Rust, so WASM compatibility is not a concern and binary size impact is negligible. The sRGB→Lab conversion is simpler, standard math with no edge cases, so implementing it inline is appropriate.

**Alternatives considered:**
- Full inline CIEDE2000: reduces dependencies to zero but reintroduces the risk of subtle formula bugs; not worth it given `deltae`'s zero-dep profile.
- `palette` crate: comprehensive but has many transitive dependencies; overkill for this use case.

### 2. sRGB linearisation via the IEC 61966-2-1 standard piecewise function

**Decision:** Use the exact sRGB electro-optical transfer function (EOTF) inverse for linearisation, not a gamma ≈ 2.2 approximation.

**Rationale:** The approximate gamma formula introduces measurable error in near-black tones. The exact piecewise function is a handful of extra lines and is correct per the standard. Error tolerance is not an issue here since this is pure arithmetic on small integer inputs.

### 3. Default threshold of `10` ΔE\*00

**Decision:** Change the hardcoded default from `60` (Euclidean, 0–441 scale) to `10` (ΔE\*00, 0–100 scale).

**Rationale:** ΔE < 10 is commonly used in industry as "acceptably similar" for product color matching. ΔE < 1 is imperceptible to most observers. A threshold of 10 therefore gives a useful, intuitive match range.

### 4. Rename `rgb_distance` → `color_distance`

**Decision:** Rename the public function so the name does not imply a specific algorithm.

**Rationale:** The old name is misleading once the implementation is no longer RGB-based. `color_distance` is algorithm-agnostic.

## Risks / Trade-offs

- **Threshold scale change** → Users with hard-coded or bookmarked URLs that include a threshold query param (if any) will need to re-calibrate. Mitigation: the threshold is currently a UI-only reactive signal, not a URL param, so no stored state is affected.
- **Floating-point correctness in Lab conversion** → The sRGB→Lab path involves `atan2`, `cbrt`, and matrix multiply; edge cases around achromatic inputs require care. Mitigation: `deltae` handles the CIEDE2000 formula; only the Lab conversion is hand-rolled, where the math is straightforward.
- **Performance** → CIEDE2000 is more expensive than Euclidean distance (~10× more operations). For a list of hundreds of spools with a few colors each, this is still sub-millisecond on WASM. Mitigation: no change needed; profile only if performance complaints arise.

## Migration Plan

1. Add `color_distance` (CIEDE2000) to `color.rs`; keep `rgb_distance` as a deprecated alias initially.
2. Update `spool.rs` to call `color_distance` with the new default threshold.
3. Delete `rgb_distance` once all call sites are updated (same PR, single commit).

No rollback strategy needed — this is a self-contained client-side change with no persistent state or wire-format impact.

## Open Questions

- Should the threshold UI display the ΔE value to the user (e.g., a labelled slider 0–50) rather than the raw number? Deferred — UI improvements are a separate change.
