use leptos::prelude::*;
use leptos_router::components::A;
use leptos_router::hooks::use_location;

#[component]
pub fn Layout(children: Children) -> impl IntoView {
    let dark = use_context::<RwSignal<bool>>().expect("dark mode signal");

    // Apply or remove the `dark` class on <body> whenever the signal changes.
    Effect::new(move |_| {
        let body = web_sys::window()
            .and_then(|w| w.document())
            .and_then(|d| d.body());
        if let Some(body) = body {
            if dark.get() {
                let _ = body.class_list().add_1("dark");
            } else {
                let _ = body.class_list().remove_1("dark");
            }
            // Persist preference.
            if let Some(storage) = web_sys::window().and_then(|w| w.local_storage().ok().flatten())
            {
                let _ = storage.set_item("dark_mode", if dark.get() { "true" } else { "false" });
            }
        }
    });

    view! {
        <div class="app-shell">
            <Sidebar />
            <main class="main-content">
                {children()}
            </main>
        </div>
    }
}

#[component]
fn Sidebar() -> impl IntoView {
    let dark = use_context::<RwSignal<bool>>().expect("dark mode signal");
    let location = use_location();
    let spools_active = move || {
        let path = location.pathname.get();
        path == "/" || path.starts_with("/spools")
    };

    view! {
        <nav class="sidebar">
            <div class="sidebar-header">
                <span class="logo">"Spoolman"</span>
            </div>
            <ul class="nav-links">
                <li class=move || if spools_active() { "active" } else { "" }><A href="/spools">"Spools"</A></li>
                <li><A href="/filaments">"Filaments"</A></li>
                <li><A href="/locations">"Locations"</A></li>
                <li><A href="/settings">"Settings"</A></li>
                <li><A href="/help">"Help"</A></li>
            </ul>
            <div class="sidebar-footer">
                <button
                    class="dark-toggle"
                    on:click=move |_| dark.update(|d| *d = !*d)
                >
                    {move || if dark.get() { "☀ Light" } else { "☾ Dark" }}
                </button>
                <span class="version">{format!("v{}", env!("CARGO_PKG_VERSION"))}</span>
            </div>
        </nav>
    }
}
