use crate::{api, state::diameter_settings};
use leptos::*;

#[component]
pub fn SettingsPage() -> impl IntoView {
    let settings = create_resource(|| (), |_| async { api::fetch_settings().await });
    let currency = create_rw_signal(String::new());
    let saved = create_rw_signal(false);
    let error = create_rw_signal(Option::<String>::None);

    // Diameter settings — read from shared context; local copies for the form.
    let ds = diameter_settings();
    let uniform = create_rw_signal(true);
    let default_mm = create_rw_signal("1.75".to_string());

    create_effect(move |_| {
        if let Some(Ok(s)) = settings.get() {
            currency.set(
                s.get("currency_symbol")
                    .cloned()
                    .unwrap_or_else(|| "€".into()),
            );
            uniform.set(
                s.get("uniform_diameter")
                    .map(|v| v == "true")
                    .unwrap_or(true),
            );
            default_mm.set(
                s.get("default_diameter")
                    .cloned()
                    .unwrap_or_else(|| "1.75".into()),
            );
        }
    });

    let on_submit = move |ev: web_sys::SubmitEvent| {
        ev.prevent_default();
        let uniform_val = uniform.get();
        let default_mm_val = default_mm.get();
        spawn_local(async move {
            let r1 = api::put_setting("currency_symbol", currency.get()).await;
            let r2 = api::put_setting(
                "uniform_diameter",
                if uniform_val {
                    "true".into()
                } else {
                    "false".into()
                },
            )
            .await;
            let r3 = api::put_setting("default_diameter", default_mm_val.clone()).await;
            match (r1, r2, r3) {
                (Ok(_), Ok(_), Ok(_)) => {
                    // Update the app-wide context signals so other components
                    // see the change without a reload.
                    ds.uniform.set(uniform_val);
                    if let Ok(v) = default_mm_val.parse::<f64>() {
                        ds.default_mm.set(v);
                    }
                    saved.set(true);
                }
                (Err(e), _, _) | (_, Err(e), _) | (_, _, Err(e)) => {
                    error.set(Some(e.to_string()));
                }
            }
        });
    };

    view! {
        <div class="page settings-page">
            <h1>"Settings"</h1>
            {move || error.get().map(|e| view! { <p class="error">{e}</p> })}
            {move || saved.get().then(|| view! { <p class="success ">"Saved."</p> })}
            <form on:submit=on_submit>
                <label>
                    "Currency symbol"
                    <input type="text" maxlength="4"
                        prop:value=move || currency.get()
                        on:input=move |ev| {
                            saved.set(false);
                            currency.set(event_target_value(&ev));
                        }
                    />
                </label>
                <label class="checkbox-label">
                    <input type="checkbox"
                        prop:checked=move || uniform.get()
                        on:change=move |ev| {
                            saved.set(false);
                            uniform.set(event_target_checked(&ev));
                        }
                    />
                    "Same diameter for all filaments"
                </label>
                <label>
                    "Default diameter (mm)"
                    <input type="number" step="0.01" min="0.1"
                        prop:value=move || default_mm.get()
                        on:input=move |ev| {
                            saved.set(false);
                            default_mm.set(event_target_value(&ev));
                        }
                    />
                </label>
                <button type="submit" class="btn btn-primary ">"Save"</button>
            </form>
        </div>
    }
}
