use crate::spoolmandb::{load_spoolmandb, SpoolmanEntry};
use leptos::prelude::*;
use leptos::task::spawn_local;

/// Inline search panel that queries the locally-cached SpoolmanDB.
///
/// Displays a text input; as the user types, results are filtered client-side
/// and shown as a clickable list (up to 10). Selecting an entry calls `on_select`.
#[component]
pub fn SpoolmanDbSearch(on_select: Callback<SpoolmanEntry>) -> impl IntoView {
    // Async load state: None = loading, Some(Err) = unavailable, Some(Ok) = ready.
    let db: RwSignal<Option<Result<Vec<SpoolmanEntry>, String>>> = RwSignal::new(None);
    let query = RwSignal::new(String::new());

    // Fetch (or serve from cache) on mount.
    Effect::new(move |_| {
        spawn_local(async move {
            let result = load_spoolmandb().await;
            db.set(Some(result));
        });
    });

    // Filtered results derived from query + db.
    let results = move || -> Vec<SpoolmanEntry> {
        let q = query.get();
        if q.is_empty() {
            return vec![];
        }
        let q_lower = q.to_lowercase();
        match db.get() {
            Some(Ok(entries)) => entries
                .into_iter()
                .filter(|e| {
                    e.manufacturer.to_lowercase().contains(&q_lower)
                        || e.material.to_lowercase().contains(&q_lower)
                        || e.name.to_lowercase().contains(&q_lower)
                })
                .take(10)
                .collect(),
            _ => vec![],
        }
    };

    view! {
        <div class="spoolmandb-search">
            <label class="spoolmandb-search-label">
                "Search filament database"
                <input
                    type="text"
                    placeholder="e.g. Prusament PLA"
                    prop:value=move || query.get()
                    on:input=move |ev| query.set(event_target_value(&ev))
                />
            </label>
            {move || match db.get() {
                None => view! { <p class="spoolmandb-status">"Loading database…"</p> }.into_any(),
                Some(Err(e)) => view! {
                    <p class="spoolmandb-status spoolmandb-error">
                        "Database unavailable: " {e}
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
                                <p class="spoolmandb-status">"No results"</p>
                            }.into_any()
                        } else {
                            view! {
                                <ul class="spoolmandb-results">
                                    {items.into_iter().map(|entry| {
                                        let entry_clone = entry.clone();
                                        let label = format!(
                                            "{} \u{00b7} {} \u{00b7} {}",
                                            entry.manufacturer, entry.material, entry.name
                                        );
                                        view! {
                                            <li>
                                                <button
                                                    type="button"
                                                    class="spoolmandb-result-btn"
                                                    on:click=move |_| on_select.run(entry_clone.clone())
                                                >
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
