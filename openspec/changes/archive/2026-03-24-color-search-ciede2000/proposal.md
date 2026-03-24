## Why

Euclidean distance in RGB space does not correlate with human color perception — two colors that look nearly identical can have a large RGB distance (e.g., a saturated red vs. a slightly less saturated red), while colors that look very different may score as "close." CIEDE2000 (ΔE\*00) is the ISO/CIE-standardized perceptual color difference metric and directly measures how different two colors *appear* to human observers, giving users accurate "find spools near this color" results.

## What Changes

- Replace `rgb_distance()` in `crates/spoolman-client/src/utils/color.rs` with a CIEDE2000 implementation that converts sRGB → CIE L\*a\*b\* and returns ΔE\*00.
- Add RGB-to-Lab conversion helpers (sRGB linearisation → XYZ D65 → Lab).
- Update the default threshold in `crates/spoolman-client/src/pages/spool.rs` from `60` (Euclidean, 0–441 scale) to `10` (ΔE\*00, 0–100 scale; ΔE < 10 is "similar", < 1 is "indistinguishable").
- Rename the public function to `color_distance` and update all call sites.

## Capabilities

### New Capabilities

- `ciede2000-color-distance`: WASM-compatible CIEDE2000 color distance computation — sRGB input, perceptually uniform ΔE\*00 output, no external crate required.

### Modified Capabilities

<!-- No existing spec-level behavior changes; color search is an implementation detail not covered by existing specs. -->

## Impact

- **Modified files:** `crates/spoolman-client/src/utils/color.rs`, `crates/spoolman-client/src/pages/spool.rs`
- **No new dependencies** — CIEDE2000 is self-contained math; no crate needed.
- **No API changes** — filtering remains client-side; server routes unaffected.
- **Threshold scale change** — existing users relying on the default threshold will see a different (more accurate) match radius; this is intentional and an improvement.
