use crate::filamentcolors::{load_filamentcolors, FilamentColorsSwatch};
use leptos::prelude::*;
use leptos::task::spawn_local;

/// Inline search panel that queries the locally-cached filamentcolors.xyz swatch list.
///
/// Displays a text input; as the user types, results are filtered client-side
/// (manufacturer + color name) and shown as a clickable list (up to 10, with
/// thumbnails). Selecting an entry calls `on_select`.
#[component]
pub fn FilamentColorsSearch(
    on_select: Callback<FilamentColorsSwatch>,
    /// Manufacturer of the spool's filament; its swatches are listed first.
    #[prop(into, optional)]
    manufacturer: Signal<Option<String>>,
) -> impl IntoView {
    // Async load state: None = loading, Some(Err) = unavailable, Some(Ok) = ready.
    let db: RwSignal<Option<Result<Vec<FilamentColorsSwatch>, String>>> = RwSignal::new(None);
    let query = RwSignal::new(String::new());

    Effect::new(move |_| {
        spawn_local(async move {
            let result = load_filamentcolors().await;
            db.set(Some(result));
        });
    });

    let results = move || -> Vec<FilamentColorsSwatch> {
        let q = query.get();
        if q.is_empty() {
            return vec![];
        }
        let tokens: Vec<String> = q.to_lowercase().split_whitespace().map(String::from).collect();
        match db.get() {
            Some(Ok(entries)) => rank_swatches(entries, &tokens, manufacturer.get().as_deref()),
            _ => vec![],
        }
    };

    view! {
        <div class="filamentcolors-search">
            <label class="filamentcolors-search-label">
                "Search filamentcolors.xyz"
                <input
                    type="text"
                    placeholder="e.g. Polymaker Aurora"
                    prop:value=move || query.get()
                    on:input=move |ev| query.set(event_target_value(&ev))
                />
            </label>
            {move || match db.get() {
                None => view! { <p class="filamentcolors-status">"Loading swatches…"</p> }.into_any(),
                Some(Err(e)) => view! {
                    <p class="filamentcolors-status filamentcolors-error">
                        "filamentcolors.xyz unavailable: " {e}
                    </p>
                }.into_any(),
                Some(Ok(_)) => {
                    let q = query.get();
                    if q.is_empty() {
                        view! { <></> }.into_any()
                    } else {
                        let items = results();
                        if items.is_empty() {
                            view! {
                                <p class="filamentcolors-status">"No results"</p>
                            }.into_any()
                        } else {
                            view! {
                                <ul class="filamentcolors-results">
                                    {items.into_iter().map(|entry| {
                                        let entry_clone = entry.clone();
                                        let label = format!("{} \u{00b7} {}", entry.manufacturer.name, entry.color_name);
                                        let thumb = entry.card_img.clone();
                                        view! {
                                            <li>
                                                <button
                                                    type="button"
                                                    class="filamentcolors-result-btn"
                                                    on:click=move |_| on_select.run(entry_clone.clone())
                                                >
                                                    {thumb.map(|src| view! {
                                                        <img class="filamentcolors-thumb" src=src alt="" />
                                                    })}
                                                    {label}
                                                </button>
                                            </li>
                                        }
                                    }).collect_view()}
                                </ul>
                            }.into_any()
                        }
                    }
                }
            }}
        </div>
    }
}

/// Token-filter swatches, list those from `manufacturer` first (stable), cap at 10.
fn rank_swatches(
    entries: Vec<FilamentColorsSwatch>,
    tokens: &[String],
    manufacturer: Option<&str>,
) -> Vec<FilamentColorsSwatch> {
    let pref = manufacturer.map(|m| m.trim().to_lowercase()).filter(|m| !m.is_empty());
    let is_pref = |e: &FilamentColorsSwatch| {
        pref.as_ref().is_some_and(|p| {
            let m = e.manufacturer.name.trim().to_lowercase();
            !m.is_empty() && (m.contains(p.as_str()) || p.contains(m.as_str()))
        })
    };
    let mut hits: Vec<_> = entries
        .into_iter()
        .filter(|e| {
            let mfr = e.manufacturer.name.to_lowercase();
            let color = e.color_name.to_lowercase();
            tokens.iter().all(|t| mfr.contains(t) || color.contains(t))
        })
        .collect();
    hits.sort_by_key(|e| !is_pref(e));
    hits.truncate(10);
    hits
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::filamentcolors::FilamentColorsManufacturer;

    fn sw(id: u32, m: &str) -> FilamentColorsSwatch {
        FilamentColorsSwatch {
            id,
            manufacturer: FilamentColorsManufacturer { name: m.into() },
            color_name: "Black".into(),
            hex_color: "000000".into(),
            card_img: None,
        }
    }

    fn ids(v: &[FilamentColorsSwatch]) -> Vec<u32> {
        v.iter().map(|e| e.id).collect()
    }

    #[test]
    fn manufacturer_first_then_fallback() {
        let mut all = vec![sw(1, "Other"), sw(2, "Polymaker PolyLite"), sw(3, "Generic")];
        all.extend((10..30).map(|i| sw(i, "Other")));
        let r = rank_swatches(all.clone(), &["black".into()], Some("Polymaker"));
        assert_eq!(ids(&r)[..3], [2, 1, 3]);
        assert_eq!(r.len(), 10);
    }

    #[test]
    fn no_manufacturer_keeps_order() {
        let all = vec![sw(1, "A"), sw(2, "B")];
        assert_eq!(ids(&rank_swatches(all.clone(), &["black".into()], None)), [1, 2]);
        assert_eq!(ids(&rank_swatches(all, &["black".into()], Some(" "))), [1, 2]);
    }
}
