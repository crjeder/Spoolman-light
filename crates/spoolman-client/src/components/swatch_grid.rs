use leptos::prelude::*;
use spoolman_types::responses::{LocationResponse, SpoolResponse};

use crate::{api::ApiError, format};

/// CSS `background` value for a spool's colour(s): a flat fill for one colour,
/// an equal-width hard-stop gradient for two to four. Empty `colors` falls
/// back to the same neutral grey the spool table paints.
fn fill_style(colors: &[spoolman_types::models::Rgba]) -> String {
    let rgba = |c: &spoolman_types::models::Rgba| format!("rgba({},{},{},{})", c.r, c.g, c.b, c.a as f32 / 255.0);
    match colors {
        [] => "background: rgba(200,200,200,1)".to_string(),
        [c] => format!("background: {}", rgba(c)),
        cs => {
            let n = cs.len();
            let stops: Vec<String> = cs
                .iter()
                .enumerate()
                .flat_map(|(i, c)| {
                    let start = i * 100 / n;
                    let end = (i + 1) * 100 / n;
                    let color = rgba(c);
                    [format!("{color} {start}%"), format!("{color} {end}%")]
                })
                .collect();
            format!("background: linear-gradient(90deg, {})", stops.join(", "))
        }
    }
}

/// Colour swatch grid: one card per spool, filled with its colour(s) and
/// labelled with colour name, filament, material, remaining weight and
/// location. A material checkbox row above the grid writes into the shared
/// `material_filter` set.
#[component]
pub fn SwatchGrid(
    items: Signal<Vec<SpoolResponse>>,
    locations: LocalResource<Result<Vec<LocationResponse>, ApiError>>,
    material_filter: RwSignal<Vec<String>>,
    available_materials: Signal<Vec<String>>,
) -> impl IntoView {
    let toggle_material = move |m: String| {
        material_filter.update(|set| {
            if let Some(pos) = set.iter().position(|x| *x == m) {
                set.remove(pos);
            } else {
                set.push(m);
            }
        });
    };

    view! {
        <div class="material-checks">
            {move || available_materials.get().into_iter().map(|m| {
                let checked_m = m.clone();
                let click_m = m.clone();
                view! {
                    <label class="checkbox-label">
                        <input type="checkbox"
                            prop:checked=move || material_filter.get().contains(&checked_m)
                            on:change=move |_| toggle_material(click_m.clone())
                        />
                        {m}
                    </label>
                }
            }).collect_view()}
        </div>
        <div class="swatch-grid">
            {move || items.get().into_iter().map(|sr| {
                let id = sr.spool.id;
                let name = sr.filament.display_name();
                let color_label = sr.spool.color_name.clone().unwrap_or_else(|| {
                    sr.spool.colors.first()
                        .map(|c| format!("#{:02x}{:02x}{:02x}", c.r, c.g, c.b))
                        .unwrap_or_else(|| "#c8c8c8".to_string())
                });
                let material = sr.filament.material.as_ref().map(|m| m.abbreviation().to_string()).unwrap_or_default();
                let rem = sr.remaining_filament.map(format::format_weight).unwrap_or_default();
                let location = sr.spool.location_id.and_then(|lid| {
                    locations.get()
                        .and_then(|r| r.ok())
                        .and_then(|ls| ls.into_iter().find(|l| l.location.id == lid))
                        .map(|l| l.location.name)
                }).unwrap_or_else(|| "\u{2014}".to_string());
                let style = fill_style(&sr.spool.colors);
                let card_class = if sr.spool.archived { "swatch-card archived" } else { "swatch-card" };
                view! {
                    <a href=format!("/spools/{id}") class=card_class>
                        <div class="swatch-card-fill" style=style></div>
                        <div class="swatch-card-meta">
                            <strong>{color_label}</strong>
                            <span>{name}</span>
                            <span>{material}" · "{rem}</span>
                            <span>{location}</span>
                        </div>
                    </a>
                }
            }).collect_view()}
        </div>
    }
}
