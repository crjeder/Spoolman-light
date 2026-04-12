## Context

The color matching system supports three algorithms: CIEDE2000, OKLab, and DIN99d. CIEDE2000 is currently the default. DIN99d is a perceptually uniform, computationally simple metric (DIN 6176:2001) that is well-suited for spool matching because its ΔE scale maps naturally to practical filament tolerance ranges. The existing DIN99d threshold defaults (10/20/35) were copied from CIEDE2000 and are not well-calibrated — 13/19/25 provide better spread and match the practical color matching behaviour for filament spools.

## Goals / Non-Goals

**Goals:**
- Change the fallback default algorithm from CIEDE2000 to DIN99d in all code paths where no persisted setting exists
- Update the DIN99d fallback threshold defaults to 13.0 / 19.0 / 25.0 for same / close / ballpark
- Update the Settings page algorithm selector labels to reflect the new default

**Non-Goals:**
- Migrating existing persisted user settings (users who have already saved a preference keep it)
- Changing CIEDE2000 or OKLab threshold defaults
- Changing the DIN99d algorithm implementation

## Decisions

**D1 — No settings migration**
Users with existing persisted `color_distance_algorithm` and threshold keys are unaffected. Only the in-code fallback changes. This avoids any server-side migration logic and respects saved preferences.

*Alternative considered*: Clear or overwrite persisted CIEDE2000 settings for users who never explicitly chose it. Rejected because it is impossible to distinguish "user chose CIEDE2000" from "system defaulted to CIEDE2000".

**D2 — Change default in three places**
Three touch-points must be updated consistently:
1. `default_threshold_for()` in `color.rs` — DIN99d arm returns 13/19/25
2. `provide_color_distance_algorithm_context()` in `state.rs` — initial signal value becomes `Din99d`
3. `app.rs` settings-load fallback — the `_ =>` arm of the algorithm parse becomes `Din99d`

*Selector label update* in `settings.rs` is a cosmetic fourth touch-point but carries no logic change.

## Risks / Trade-offs

- **Behaviour change for new installs** → Intentional. New users start with DIN99d + 13/19/25 thresholds. Existing users are unaffected.
- **Test assertions referencing old defaults** → Any test hardcoding the CIEDE2000 default or DIN99d threshold 35.0 must be updated. Low risk; unit tests for `default_threshold_for()` will catch any gap.
- **Label mismatch if label not updated** → Settings page still shows "CIEDE2000 (default)" label until `settings.rs` is changed; tasks must include the label change together with the logic change.
