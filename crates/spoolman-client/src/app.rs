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
    state::{ColorAlgorithm, ColorDistanceAlgorithm, ColorThresholds, CurrencySymbol, DiameterSettings},
    utils::color::default_threshold_for,
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

    // Provide color distance algorithm globally (default: CIEDE2000).
    let color_algo = create_rw_signal(ColorAlgorithm::Ciede2000);
    provide_context(ColorDistanceAlgorithm(color_algo));

    // Provide color search thresholds globally (defaults from hardcoded table).
    let thresholds = ColorThresholds {
        ciede2000_same:     create_rw_signal(default_threshold_for("same",     ColorAlgorithm::Ciede2000)),
        ciede2000_close:    create_rw_signal(default_threshold_for("close",    ColorAlgorithm::Ciede2000)),
        ciede2000_ballpark: create_rw_signal(default_threshold_for("ballpark", ColorAlgorithm::Ciede2000)),
        oklab_same:         create_rw_signal(default_threshold_for("same",     ColorAlgorithm::OkLab)),
        oklab_close:        create_rw_signal(default_threshold_for("close",    ColorAlgorithm::OkLab)),
        oklab_ballpark:     create_rw_signal(default_threshold_for("ballpark", ColorAlgorithm::OkLab)),
        din99d_same:        create_rw_signal(default_threshold_for("same",     ColorAlgorithm::Din99d)),
        din99d_close:       create_rw_signal(default_threshold_for("close",    ColorAlgorithm::Din99d)),
        din99d_ballpark:    create_rw_signal(default_threshold_for("ballpark", ColorAlgorithm::Din99d)),
    };
    provide_context(thresholds);

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
            color_algo.set(match s.get("color_distance_algorithm").map(String::as_str) {
                Some("oklab") => ColorAlgorithm::OkLab,
                Some("din99d") => ColorAlgorithm::Din99d,
                _ => ColorAlgorithm::Ciede2000,
            });
            // Load persisted threshold overrides (fall back to hardcoded default
            // when a key is absent).
            let load_thresh = |key: &str, level: &str, algo: ColorAlgorithm| -> f32 {
                s.get(key)
                    .and_then(|v| v.parse().ok())
                    .unwrap_or_else(|| default_threshold_for(level, algo))
            };
            thresholds.ciede2000_same.set(load_thresh("color_threshold_ciede2000_same",     "same",     ColorAlgorithm::Ciede2000));
            thresholds.ciede2000_close.set(load_thresh("color_threshold_ciede2000_close",   "close",    ColorAlgorithm::Ciede2000));
            thresholds.ciede2000_ballpark.set(load_thresh("color_threshold_ciede2000_ballpark", "ballpark", ColorAlgorithm::Ciede2000));
            thresholds.oklab_same.set(load_thresh("color_threshold_oklab_same",             "same",     ColorAlgorithm::OkLab));
            thresholds.oklab_close.set(load_thresh("color_threshold_oklab_close",           "close",    ColorAlgorithm::OkLab));
            thresholds.oklab_ballpark.set(load_thresh("color_threshold_oklab_ballpark",     "ballpark", ColorAlgorithm::OkLab));
            thresholds.din99d_same.set(load_thresh("color_threshold_din99d_same",           "same",     ColorAlgorithm::Din99d));
            thresholds.din99d_close.set(load_thresh("color_threshold_din99d_close",         "close",    ColorAlgorithm::Din99d));
            thresholds.din99d_ballpark.set(load_thresh("color_threshold_din99d_ballpark",   "ballpark", ColorAlgorithm::Din99d));
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
