use leptos::*;
use leptos_router::*;

use crate::{
    pages::{
        filament::{FilamentCreate, FilamentEdit, FilamentList, FilamentShow},
        help::HelpPage,
        location::LocationList,
        settings::SettingsPage,
        spool::{SpoolCreate, SpoolEdit, SpoolList, SpoolShow},
    },
    state::{CurrencySymbol, DiameterSettings},
};

#[component]
pub fn App() -> impl IntoView {
    // Provide dark-mode signal globally.
    let dark = create_rw_signal(load_dark_mode());
    provide_context(dark);

    // Provide diameter settings globally (defaults: uniform=true, 1.75 mm).
    let diam_uniform = create_rw_signal(true);
    let diam_default = create_rw_signal(1.75f64);
    provide_context(DiameterSettings {
        uniform: diam_uniform,
        default_mm: diam_default,
    });

    // Provide currency symbol globally (default: "€").
    let currency_sym = create_rw_signal("€".to_string());
    provide_context(CurrencySymbol(currency_sym));

    // Fetch persisted settings and update the diameter + currency signals.
    let settings_res = create_resource(|| (), |_| async { crate::api::fetch_settings().await });
    create_effect(move |_| {
        if let Some(Ok(s)) = settings_res.get() {
            diam_uniform.set(
                s.get("uniform_diameter")
                    .map(|v| v == "true")
                    .unwrap_or(true),
            );
            diam_default.set(
                s.get("default_diameter")
                    .and_then(|v| v.parse().ok())
                    .unwrap_or(1.75),
            );
            if let Some(sym) = s.get("currency_symbol") {
                currency_sym.set(sym.clone());
            }
        }
    });

    view! {
        <Router>
            <crate::components::layout::Layout>
                <Routes>
                    <Route path="/"              view=SpoolList />
                    <Route path="/spools"        view=SpoolList />
                    <Route path="/spools/new"    view=SpoolCreate />
                    <Route path="/spools/:id"    view=SpoolShow />
                    <Route path="/spools/:id/edit" view=SpoolEdit />
                    <Route path="/filaments"     view=FilamentList />
                    <Route path="/filaments/new" view=FilamentCreate />
                    <Route path="/filaments/:id" view=FilamentShow />
                    <Route path="/filaments/:id/edit" view=FilamentEdit />
                    <Route path="/locations"     view=LocationList />
                    <Route path="/settings"      view=SettingsPage />
                    <Route path="/help"          view=HelpPage />
                </Routes>
            </crate::components::layout::Layout>
        </Router>
    }
}

fn load_dark_mode() -> bool {
    use web_sys::window;
    window()
        .and_then(|w| w.local_storage().ok().flatten())
        .and_then(|s| s.get_item("dark_mode").ok().flatten())
        .map(|v| v == "true")
        .unwrap_or(false)
}
