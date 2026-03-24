use leptos::*;
use leptos_router::*;

use crate::pages::{
    filament::{FilamentCreate, FilamentEdit, FilamentList, FilamentShow},
    help::HelpPage,
    location::LocationList,
    settings::SettingsPage,
    spool::{SpoolCreate, SpoolEdit, SpoolList, SpoolShow},
};

#[component]
pub fn App() -> impl IntoView {
    // Provide dark-mode signal globally.
    let dark = create_rw_signal(load_dark_mode());
    provide_context(dark);

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
