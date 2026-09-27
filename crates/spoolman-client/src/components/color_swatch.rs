use leptos::prelude::*;
use spoolman_types::models::Rgba;

use crate::api::swatch_image_url;

/// Renders a spool's colour(s): the stored filamentcolors.xyz swatch image
/// when `swatch_image` is present, otherwise one flat CSS-background chip
/// per colour (the pre-existing behaviour). Shared by the spool list table,
/// spool detail view, and `/colors` swatch grid so the fallback logic lives
/// in exactly one place.
pub fn color_swatch_view(colors: &[Rgba], swatch_image: Option<&str>) -> AnyView {
    if let Some(id) = swatch_image {
        let src = swatch_image_url(id);
        view! { <img class="color-swatch color-swatch-image" src=src alt="" /> }.into_any()
    } else {
        let colors = colors.to_vec();
        colors
            .into_iter()
            .map(|c| {
                view! {
                    <span class="color-swatch"
                        style=format!("background:rgba({},{},{},{})", c.r, c.g, c.b, c.a as f32 / 255.0)>
                    </span>
                }
            })
            .collect_view()
            .into_any()
    }
}
