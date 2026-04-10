//! Locale-aware display formatting via the browser's `Intl` API.
//!
//! All functions call into JavaScript `Intl.NumberFormat` / `Intl.DateTimeFormat`
//! using the browser's default locale (`undefined`).  This produces correct
//! decimal separators, thousands grouping, and date ordering without bundling
//! any locale data inside the WASM binary.
//!
//! **Form inputs are excluded** — `<input type="number">` and `<input type="date">`
//! must keep machine-format values so the browser can parse them back.

use chrono::DateTime;
use chrono::Utc;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(inline_js = "
export function sm_format_decimal(value, min_fd, max_fd) {
    return new Intl.NumberFormat(undefined, {
        style: 'decimal',
        minimumFractionDigits: min_fd,
        maximumFractionDigits: max_fd,
    }).format(value);
}
export function sm_format_date_medium(timestamp_ms) {
    return new Intl.DateTimeFormat(undefined, {
        dateStyle: 'medium',
        timeZone: 'UTC',
    }).format(new Date(timestamp_ms));
}
export function sm_format_currency(value, symbol) {
    const parts = new Intl.NumberFormat(undefined, {
        style: 'currency',
        currency: 'USD',
        minimumFractionDigits: 2,
        maximumFractionDigits: 2,
    }).formatToParts(value);
    return parts.map(p => p.type === 'currency' ? symbol : p.value).join('');
}
")]
extern "C" {
    fn sm_format_decimal(value: f64, min_fd: u32, max_fd: u32) -> String;
    fn sm_format_date_medium(timestamp_ms: f64) -> String;
    fn sm_format_currency(value: f64, symbol: &str) -> String;
}

/// Format a weight value with the browser locale, appending " g".
/// Uses up to 1 fractional digit (trailing zeros suppressed).
pub fn format_weight(grams: f32) -> String {
    format!("{} g", sm_format_decimal(f64::from(grams), 0, 1))
}

/// Format a density value with the browser locale, appending " g/cm³".
/// Uses up to 3 fractional digits (trailing zeros suppressed).
pub fn format_density(g_per_cm3: f32) -> String {
    format!("{} g/cm³", sm_format_decimal(f64::from(g_per_cm3), 0, 3))
}

/// Format a diameter in millimetres with the browser locale, appending " mm".
pub fn format_mm(mm: f32) -> String {
    format!("{} mm", sm_format_decimal(f64::from(mm), 0, 2))
}

/// Format a `DateTime<Utc>` as a locale-aware date (date portion only).
/// Uses `Intl.DateTimeFormat` with `dateStyle: "medium"`, e.g. "Mar 29, 2026"
/// or "29 mars 2026" depending on the browser locale.
pub fn format_date(dt: DateTime<Utc>) -> String {
    sm_format_date_medium(dt.timestamp_millis() as f64)
}

/// Format a currency amount with locale-aware symbol positioning.
///
/// If `symbol_override` is non-empty, it is placed before or after the number
/// according to the browser locale (e.g. `"$10.00"` for `en-US`, `"10,00 €"`
/// for `de-DE`). Position is derived via `Intl.NumberFormat.formatToParts()`
/// using USD as a locale probe.
/// If empty, the amount is formatted as a plain locale decimal with 2 fraction
/// digits (no symbol).
pub fn format_currency(amount: f64, symbol_override: &str) -> String {
    if symbol_override.is_empty() {
        sm_format_decimal(amount, 2, 2)
    } else {
        sm_format_currency(amount, symbol_override)
    }
}
