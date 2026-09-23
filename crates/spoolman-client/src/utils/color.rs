use deltae::{DEMethod::DE2000, DeltaE, LabValue};
use oklab::{LinearRgb, Oklab};
use spoolman_types::models::{Rgba, SurfaceFinish};

// ── Algorithm selector ──────────────────────────────────────────────────────

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum ColorAlgorithm {
    Ciede2000,
    OkLab,
    Din99d,
}

// ── sRGB → CIE L*a*b* conversion ───────────────────────────────────────────

/// Linearise a single sRGB channel value (0–255) via the exact IEC 61966-2-1
/// piecewise EOTF inverse. This is more accurate than the γ ≈ 2.2 approximation,
/// especially in near-black tones.
fn srgb_channel_to_linear(v: u8) -> f32 {
    let c = v as f32 / 255.0;
    if c <= 0.04045 {
        c / 12.92
    } else {
        ((c + 0.055) / 1.055_f32).powf(2.4)
    }
}

/// Convert linear-light sRGB to CIE XYZ (D65 illuminant) using the
/// ITU-R BT.709 / IEC 61966-2-1 matrix. Returns (X, Y, Z) normalised so
/// that D65 white point is (0.95047, 1.00000, 1.08883).
fn linear_rgb_to_xyz(r: f32, g: f32, b: f32) -> (f32, f32, f32) {
    let x = r * 0.412_456_4 + g * 0.357_576_1 + b * 0.180_437_5;
    let y = r * 0.212_672_9 + g * 0.715_152_2 + b * 0.072_174_9;
    let z = r * 0.019_333_9 + g * 0.119_192_0 + b * 0.950_304_1;
    (x, y, z)
}

/// CIE L*a*b* cube-root transfer function (f(t)).
fn lab_f(t: f32) -> f32 {
    const DELTA: f32 = 6.0 / 29.0;
    const DELTA2: f32 = DELTA * DELTA;
    const DELTA3: f32 = DELTA2 * DELTA;
    if t > DELTA3 {
        t.cbrt()
    } else {
        t / (3.0 * DELTA2) + 4.0 / 29.0
    }
}

/// Convert an `Rgba` to CIE L*a*b* (D65 illuminant). Alpha is ignored.
fn rgba_to_lab(c: &Rgba) -> LabValue {
    let r = srgb_channel_to_linear(c.r);
    let g = srgb_channel_to_linear(c.g);
    let b = srgb_channel_to_linear(c.b);
    let (x, y, z) = linear_rgb_to_xyz(r, g, b);

    // D65 white point normalisation
    let fx = lab_f(x / 0.950_47);
    let fy = lab_f(y / 1.000_00);
    let fz = lab_f(z / 1.088_83);

    let l = 116.0 * fy - 16.0;
    let a = 500.0 * (fx - fy);
    let b_val = 200.0 * (fy - fz);

    // deltae::LabValue::new clamps/validates; use unchecked struct init to
    // avoid the Result when we know our inputs are always valid sRGB.
    LabValue { l, a, b: b_val }
}

// ── CIEDE2000 ───────────────────────────────────────────────────────────────

fn ciede2000_distance(a: &Rgba, b: &Rgba) -> f32 {
    let lab_a = rgba_to_lab(a);
    let lab_b = rgba_to_lab(b);
    *DeltaE::new(&lab_a, &lab_b, DE2000).value()
}

// ── OKLab ───────────────────────────────────────────────────────────────────

fn oklab_distance(a: &Rgba, b: &Rgba) -> f32 {
    let ok_a = Oklab::from_linear_rgb(LinearRgb {
        r: srgb_channel_to_linear(a.r),
        g: srgb_channel_to_linear(a.g),
        b: srgb_channel_to_linear(a.b),
    });
    let ok_b = Oklab::from_linear_rgb(LinearRgb {
        r: srgb_channel_to_linear(b.r),
        g: srgb_channel_to_linear(b.g),
        b: srgb_channel_to_linear(b.b),
    });
    let dl = ok_a.l - ok_b.l;
    let da = ok_a.a - ok_b.a;
    let db = ok_a.b - ok_b.b;
    (dl * dl + da * da + db * db).sqrt()
}

// ── DIN99d ──────────────────────────────────────────────────────────────────

/// Convert CIE L*a*b* to DIN99d coordinates (DIN 6176:2001, D65 variant).
fn lab_to_din99d(l: f32, a: f32, b: f32) -> (f32, f32, f32) {
    let angle: f32 = 50.0_f32.to_radians();
    let cos_a = angle.cos();
    let sin_a = angle.sin();

    let eo = a * cos_a + b * sin_a;
    let fo = 1.14 * (-a * sin_a + b * cos_a);

    let g = (eo * eo + fo * fo).sqrt();
    let h_ef = fo.atan2(eo);

    let l99 = 325.22 * (1.0 + 0.0036 * l).ln();
    let c99 = (1.0 + 0.045 * g).ln() / 0.045;

    let a99 = c99 * h_ef.cos();
    let b99 = c99 * h_ef.sin();

    (l99, a99, b99)
}

fn din99d_distance(a: &Rgba, b: &Rgba) -> f32 {
    let lab_a = rgba_to_lab(a);
    let lab_b = rgba_to_lab(b);

    let (l1, a1, b1) = lab_to_din99d(lab_a.l, lab_a.a, lab_a.b);
    let (l2, a2, b2) = lab_to_din99d(lab_b.l, lab_b.a, lab_b.b);

    let dl = l1 - l2;
    let da = a1 - a2;
    let db = b1 - b2;
    (dl * dl + da * da + db * db).sqrt()
}

// ── Surface finish modifier ────────────────────────────────────────────

/// Convert an `Rgba` to (hue in [0,360), saturation in [0,1], value in [0,1]).
/// Alpha is ignored.
fn rgba_to_hsv(c: &Rgba) -> (f32, f32, f32) {
    let r = c.r as f32 / 255.0;
    let g = c.g as f32 / 255.0;
    let b = c.b as f32 / 255.0;
    let max = r.max(g).max(b);
    let min = r.min(g).min(b);
    let delta = max - min;

    let h = if delta == 0.0 {
        0.0
    } else if max == r {
        60.0 * (((g - b) / delta) % 6.0)
    } else if max == g {
        60.0 * ((b - r) / delta + 2.0)
    } else {
        60.0 * ((r - g) / delta + 4.0)
    };
    let h = if h < 0.0 { h + 360.0 } else { h };
    let s = if max == 0.0 { 0.0 } else { delta / max };
    (h, s, max)
}

/// Inverse of `rgba_to_hsv`. `alpha` is carried through unchanged.
fn hsv_to_rgba(h: f32, s: f32, v: f32, alpha: u8) -> Rgba {
    let c = v * s;
    let x = c * (1.0 - (((h / 60.0) % 2.0) - 1.0).abs());
    let m = v - c;
    let (r, g, b) = match h {
        h if h < 60.0 => (c, x, 0.0),
        h if h < 120.0 => (x, c, 0.0),
        h if h < 180.0 => (0.0, c, x),
        h if h < 240.0 => (0.0, x, c),
        h if h < 300.0 => (x, 0.0, c),
        _ => (c, 0.0, x),
    };
    let to_u8 = |v: f32| (((v + m) * 255.0).round().clamp(0.0, 255.0)) as u8;
    Rgba { r: to_u8(r), g: to_u8(g), b: to_u8(b), a: alpha }
}

/// Predict how a stored color actually appears given the spool's surface finish.
///
/// Matte scatters light (desaturate, lift brightness); gloss concentrates it
/// (saturate, slightly darken). Hue is unchanged; S and V are clamped to [0, 1].
pub fn apply_finish_modifier(color: &Rgba, finish: SurfaceFinish) -> Rgba {
    let (s_mul, v_mul) = match finish {
        SurfaceFinish::Matte => (0.85, 1.10),
        SurfaceFinish::Standard => return color.clone(),
        SurfaceFinish::Gloss => (1.15, 0.95),
        // ponytail: silk reuses gloss's curve, tune separately if silk swatches look off
        SurfaceFinish::Silk => (1.15, 1.05),
    };
    let (h, s, v) = rgba_to_hsv(color);
    hsv_to_rgba(h, (s * s_mul).clamp(0.0, 1.0), (v * v_mul).clamp(0.0, 1.0), color.a)
}

// ── Public API ──────────────────────────────────────────────────────────────

/// Perceptual color difference between two sRGB colours using the specified
/// algorithm. Alpha is ignored.
///
/// Return value scales:
///   - `Ciede2000`: ΔE\*00 in [0, ~100]
///   - `OkLab`: ΔE_ok in [0, ~1]
///   - `Din99d`: ΔE_99d in [0, ~100]
///
/// Use `default_threshold_for` to obtain hardcoded defaults, or read the reactive
/// `ColorThresholds` context for user-configured values.
pub fn color_distance(a: &Rgba, b: &Rgba, algo: ColorAlgorithm) -> f32 {
    match algo {
        ColorAlgorithm::Ciede2000 => ciede2000_distance(a, b),
        ColorAlgorithm::OkLab => oklab_distance(a, b),
        ColorAlgorithm::Din99d => din99d_distance(a, b),
    }
}

/// Return the hardcoded default threshold for the given search level and
/// algorithm. Used to seed the `ColorThresholds` context on startup.
/// Returns `None` for `"off"` or unknown levels.
/// Levels: `"same"`, `"close"`, `"ballpark"`.
pub fn default_threshold_for(level: &str, algo: ColorAlgorithm) -> f32 {
    threshold_for_opt(level, algo).unwrap_or(0.0)
}

fn threshold_for_opt(level: &str, algo: ColorAlgorithm) -> Option<f32> {
    match level {
        "same" => Some(match algo {
            ColorAlgorithm::Ciede2000 => 10.0,
            ColorAlgorithm::OkLab => 0.10,
            ColorAlgorithm::Din99d => 13.0,
        }),
        "close" => Some(match algo {
            ColorAlgorithm::Ciede2000 => 20.0,
            ColorAlgorithm::OkLab => 0.20,
            ColorAlgorithm::Din99d => 19.0,
        }),
        "ballpark" => Some(match algo {
            ColorAlgorithm::Ciede2000 => 35.0,
            ColorAlgorithm::OkLab => 0.35,
            ColorAlgorithm::Din99d => 25.0,
        }),
        _ => None,
    }
}

// ── Hue / lightness ordering ─────────────────────────────────────────────────

/// Convert an `Rgba` to OkLab-derived `(lightness, chroma, hue)`. Hue is
/// normalised to `[0, 2π)` starting at red (`atan2(b, a)`). Alpha is ignored.
pub fn rgba_to_oklch(c: &Rgba) -> (f32, f32, f32) {
    let ok = Oklab::from_linear_rgb(LinearRgb {
        r: srgb_channel_to_linear(c.r),
        g: srgb_channel_to_linear(c.g),
        b: srgb_channel_to_linear(c.b),
    });
    let chroma = (ok.a * ok.a + ok.b * ok.b).sqrt();
    let hue = ok.b.atan2(ok.a).rem_euclid(std::f32::consts::TAU);
    (ok.l, chroma, hue)
}

/// Below this OkLab chroma a colour is treated as achromatic (grey/black/white)
/// for sort purposes — hue is unstable at low chroma.
const ACHROMATIC_CHROMA: f32 = 0.02;

/// Fallback colour used for an empty `colors` slice — matches the swatch
/// fallback already painted in the spool table.
const NO_COLOR_FALLBACK: Rgba = Rgba { r: 200, g: 200, b: 200, a: 255 };

/// Sort key for hue-then-lightness ordering, taken from a spool's first
/// colour: `(is_achromatic, hue_or_0, lightness)`. Tuple ordering places all
/// chromatic colours (sorted by hue, then lightness) before achromatic ones
/// (sorted by lightness alone).
pub fn hue_sort_key(colors: &[Rgba]) -> (bool, f32, f32) {
    let c = colors.first().unwrap_or(&NO_COLOR_FALLBACK);
    let (lightness, chroma, hue) = rgba_to_oklch(c);
    if chroma < ACHROMATIC_CHROMA {
        (true, 0.0, lightness)
    } else {
        (false, hue, lightness)
    }
}

/// Parse a `#rrggbb` hex string (as produced by `<input type="color">`)
/// into an `Rgba` with alpha = 255. Returns `None` for any other format.
pub fn hex_to_rgba(hex: &str) -> Option<Rgba> {
    let hex = hex.strip_prefix('#')?;
    if hex.len() != 6 {
        return None;
    }
    let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
    let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
    let b = u8::from_str_radix(&hex[4..6], 16).ok()?;
    Some(Rgba { r, g, b, a: 255 })
}

// ── Tests ───────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    fn rgba(r: u8, g: u8, b: u8) -> Rgba {
        Rgba { r, g, b, a: 255 }
    }

    // ── Identical colours → 0.0 ─────────────────────────────────────────────

    #[test]
    fn ciede2000_identical_is_zero() {
        let red = rgba(255, 0, 0);
        assert_eq!(color_distance(&red, &red, ColorAlgorithm::Ciede2000), 0.0);
    }

    #[test]
    fn oklab_identical_is_zero() {
        let red = rgba(255, 0, 0);
        assert_eq!(color_distance(&red, &red, ColorAlgorithm::OkLab), 0.0);
    }

    #[test]
    fn din99d_identical_is_zero() {
        let red = rgba(255, 0, 0);
        assert!(color_distance(&red, &red, ColorAlgorithm::Din99d) < 1e-4);
    }

    // ── Red vs blue → clearly different ────────────────────────────────────

    #[test]
    fn ciede2000_red_vs_blue_high() {
        assert!(color_distance(&rgba(255, 0, 0), &rgba(0, 0, 255), ColorAlgorithm::Ciede2000) > 25.0);
    }

    #[test]
    fn oklab_red_vs_blue_high() {
        assert!(color_distance(&rgba(255, 0, 0), &rgba(0, 0, 255), ColorAlgorithm::OkLab) > 0.1);
    }

    #[test]
    fn din99d_red_vs_blue_high() {
        assert!(color_distance(&rgba(255, 0, 0), &rgba(0, 0, 255), ColorAlgorithm::Din99d) > 25.0);
    }

    // ── DIN99d reference pair: white vs black ≈ 100 ─────────────────────────

    #[test]
    fn din99d_white_vs_black_approx_100() {
        let d = color_distance(&rgba(255, 255, 255), &rgba(0, 0, 0), ColorAlgorithm::Din99d);
        assert!(d > 95.0 && d < 105.0, "expected ~100, got {d}");
    }

    // ── default_threshold_for sanity checks ────────────────────────────────

    #[test]
    fn threshold_off_returns_zero() {
        assert_eq!(default_threshold_for("off", ColorAlgorithm::Ciede2000), 0.0);
    }

    #[test]
    fn threshold_same_oklab_is_0_10() {
        assert_eq!(default_threshold_for("same", ColorAlgorithm::OkLab), 0.10);
    }

    // ── Surface finish modifier ─────────────────────────────────────

    #[test]
    fn finish_standard_is_identity() {
        let c = rgba(200, 40, 90);
        assert_eq!(apply_finish_modifier(&c, SurfaceFinish::Standard), c);
    }

    #[test]
    fn finish_matte_desaturates_and_brightens() {
        let red = rgba(200, 0, 0);
        let (_, s0, v0) = rgba_to_hsv(&red);
        let (_, s1, v1) = rgba_to_hsv(&apply_finish_modifier(&red, SurfaceFinish::Matte));
        assert!(s1 < s0, "expected lower saturation: {s0} -> {s1}");
        assert!(v1 > v0, "expected higher value: {v0} -> {v1}");
    }

    #[test]
    fn finish_gloss_saturates_and_darkens() {
        let red = rgba(200, 60, 60);
        let (_, s0, v0) = rgba_to_hsv(&red);
        let (_, s1, v1) = rgba_to_hsv(&apply_finish_modifier(&red, SurfaceFinish::Gloss));
        assert!(s1 > s0, "expected higher saturation: {s0} -> {s1}");
        assert!(v1 < v0, "expected lower value: {v0} -> {v1}");
    }

    #[test]
    fn finish_matte_clamps_value_at_one() {
        let bright = rgba(255, 250, 250);
        let out = apply_finish_modifier(&bright, SurfaceFinish::Matte);
        let (_, _, v) = rgba_to_hsv(&out);
        assert!(v <= 1.0, "value must stay clamped, got {v}");
        assert_eq!(out.r, 255);
    }

    // ── Hue ordering ────────────────────────────────────────────────────────

    #[test]
    fn hue_sort_key_orders_chromatic_before_achromatic() {
        let black = rgba(0, 0, 0);
        let white = rgba(255, 255, 255);
        let grey = rgba(128, 128, 128);
        let red = rgba(255, 0, 0);
        let blue = rgba(0, 0, 255);

        let mut colors = vec![white.clone(), grey.clone(), blue.clone(), black.clone(), red.clone()];
        colors.sort_by(|a, b| {
            hue_sort_key(std::slice::from_ref(a))
                .partial_cmp(&hue_sort_key(std::slice::from_ref(b)))
                .unwrap()
        });
        assert_eq!(colors, vec![red.clone(), blue.clone(), black.clone(), grey.clone(), white.clone()]);

        colors.sort_by(|a, b| {
            hue_sort_key(std::slice::from_ref(b))
                .partial_cmp(&hue_sort_key(std::slice::from_ref(a)))
                .unwrap()
        });
        assert_eq!(colors, vec![white, grey, black, blue, red]);
    }

    #[test]
    fn finish_roundtrips_hue() {
        let c = rgba(30, 140, 200);
        let (h0, _, _) = rgba_to_hsv(&c);
        let (h1, _, _) = rgba_to_hsv(&apply_finish_modifier(&c, SurfaceFinish::Gloss));
        assert!((h0 - h1).abs() < 2.0, "hue drifted: {h0} -> {h1}");
    }
}
