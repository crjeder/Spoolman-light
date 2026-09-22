use leptos::ev;
use leptos::prelude::*;
use leptos_router::components::A;
use leptos_router::hooks::use_location;

use crate::state::{default_view, ViewMode};

#[component]
pub fn Layout(children: Children) -> impl IntoView {
    let dark = use_context::<RwSignal<bool>>().expect("dark mode signal");
    let menu_open = RwSignal::new(false);

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

    window_event_listener(ev::keydown, move |ev| {
        if menu_open.get_untracked() && ev.key() == "Escape" {
            menu_open.set(false);
        }
    });

    view! {
        <div class="app-shell">
            <button
                class="burger"
                aria-label="Menu"
                aria-expanded=move || menu_open.get().to_string()
                on:click=move |_| menu_open.update(|o| *o = !*o)
            >"☰"</button>
            {move || menu_open.get().then(|| view! {
                <div class="sidebar-backdrop" on:click=move |_| menu_open.set(false)></div>
            })}
            <Sidebar menu_open=menu_open />
            <main class="main-content">
                {children()}
            </main>
        </div>
    }
}

#[component]
fn Sidebar(menu_open: RwSignal<bool>) -> impl IntoView {
    let dark = use_context::<RwSignal<bool>>().expect("dark mode signal");
    let location = use_location();
    let dv = default_view();
    let spools_active = move || {
        let path = location.pathname.get();
        path.starts_with("/spools") || (path == "/" && dv.0.get() != Some(ViewMode::Color))
    };
    let color_active = move || {
        let path = location.pathname.get();
        path == "/colors" || (path == "/" && dv.0.get() == Some(ViewMode::Color))
    };
    let nav_class = move || if menu_open.get() { "sidebar open" } else { "sidebar" };

    view! {
        <nav class=nav_class>
            <div class="sidebar-header">
                <span class="logo">"Spoolman"</span>
            </div>
            <ul class="nav-links" on:click=move |_| menu_open.set(false)>
                <li class=move || if spools_active() { "active" } else { "" }><A href="/spools">"Spools"</A></li>
                <li class=move || if color_active() { "active" } else { "" }><A href="/colors">"Color"</A></li>
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
