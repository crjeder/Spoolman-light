//! Shared reactive state — table state with localStorage persistence.

use leptos::*;
use web_sys::window;

// ── Diameter settings ──────────────────────────────────────────────────────────

/// Reactive diameter settings provided via Leptos context from `App`.
/// `uniform` — when true, all filaments share one diameter.
/// `default_mm` — the diameter used when uniform mode is on.
#[derive(Clone, Copy)]
pub struct DiameterSettings {
    pub uniform: RwSignal<bool>,
    pub default_mm: RwSignal<f64>,
}

/// Read the `DiameterSettings` from context.  Panics if not provided (i.e. if
/// called outside of the `App` component tree).
pub fn diameter_settings() -> DiameterSettings {
    expect_context::<DiameterSettings>()
}

#[derive(Clone, Debug)]
pub struct TableState {
    pub sort_field: RwSignal<String>,
    pub sort_asc: RwSignal<bool>,
    pub page: RwSignal<usize>,
    pub page_size: RwSignal<usize>,
    pub filter: RwSignal<String>,
}

/// Create table state for a named table.  State is persisted in localStorage
/// under keys like `table.<namespace>.sort_field` etc.
pub fn use_table_state(namespace: &'static str) -> TableState {
    let load = |key: &str, default: &str| -> String {
        storage_get(&format!("table.{namespace}.{key}"))
            .unwrap_or_else(|| default.to_string())
    };

    let sort_field = create_rw_signal(load("sort_field", "registered"));
    let sort_asc = create_rw_signal(load("sort_asc", "false") == "true");
    let page = create_rw_signal(
        load("page", "0").parse::<usize>().unwrap_or(0),
    );
    let page_size = create_rw_signal(
        load("page_size", "25").parse::<usize>().unwrap_or(25),
    );
    let filter = create_rw_signal(String::new()); // filters are session-only

    // Persist changes back to localStorage.
    {
        let ns = namespace;
        create_effect(move |_| {
            storage_set(&format!("table.{ns}.sort_field"), &sort_field.get());
        });
        create_effect(move |_| {
            storage_set(
                &format!("table.{ns}.sort_asc"),
                if sort_asc.get() { "true" } else { "false" },
            );
        });
        create_effect(move |_| {
            storage_set(&format!("table.{ns}.page"), &page.get().to_string());
        });
        create_effect(move |_| {
            storage_set(&format!("table.{ns}.page_size"), &page_size.get().to_string());
        });
    }

    TableState { sort_field, sort_asc, page, page_size, filter }
}

fn storage_get(key: &str) -> Option<String> {
    window()
        .and_then(|w| w.local_storage().ok().flatten())
        .and_then(|s| s.get_item(key).ok().flatten())
}

fn storage_set(key: &str, value: &str) {
    if let Some(storage) = window().and_then(|w| w.local_storage().ok().flatten()) {
        let _ = storage.set_item(key, value);
    }
}
