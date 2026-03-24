use deltae::{DEMethod::DE2000, DeltaE, LabValue};
use spoolman_types::models::Rgba;

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

// ── Public API ──────────────────────────────────────────────────────────────

/// Perceptual color difference between two sRGB colours using CIEDE2000 (ΔE*00).
/// Alpha is ignored. Returns a value in [0, ~100]:
///   < 1   → imperceptible difference
///   < 10  → similar colours
///   > 25  → clearly distinct colours
pub fn color_distance(a: &Rgba, b: &Rgba) -> f32 {
    let lab_a = rgba_to_lab(a);
    let lab_b = rgba_to_lab(b);
    *DeltaE::new(&lab_a, &lab_b, DE2000).value()
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
