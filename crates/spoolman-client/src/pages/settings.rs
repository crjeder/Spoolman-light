use crate::{
    api,
    state::{color_distance_algorithm, color_thresholds, diameter_settings, ColorAlgorithm},
};
use leptos::prelude::*;
use leptos::task::spawn_local;

#[component]
pub fn SettingsPage() -> impl IntoView {
    let settings = LocalResource::new(|| async { api::fetch_settings().await });
    let currency = create_rw_signal(String::new());
    let saved = create_rw_signal(false);
    let error = create_rw_signal(Option::<String>::None);

    // Diameter settings — read from shared context; local copies for the form.
    let ds = diameter_settings();
    let uniform = create_rw_signal(true);
    let default_mm = create_rw_signal("1.75".to_string());

    // Color distance algorithm — read from shared context; local copy for the form.
    let cda = color_distance_algorithm();
    let algo = create_rw_signal("ciede2000".to_string());

    // Color search thresholds — read from shared context; local copies for the form.
    let ct = color_thresholds();
    let thresh_same     = create_rw_signal(format!("{:.1}", ct.ciede2000_same.get_untracked()));
    let thresh_close    = create_rw_signal(format!("{:.1}", ct.ciede2000_close.get_untracked()));
    let thresh_ballpark = create_rw_signal(format!("{:.1}", ct.ciede2000_ballpark.get_untracked()));

    Effect::new(move |_| {
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
            let algo_str = s
                .get("color_distance_algorithm")
                .cloned()
                .unwrap_or_else(|| "ciede2000".into());
            algo.set(algo_str.clone());
            // Seed threshold fields from context (already hydrated by App).
            update_thresh_fields(algo_str.as_str(), ct, &thresh_same, &thresh_close, &thresh_ballpark);
        }
    });

    // When the algorithm selector changes, swap the threshold fields to show
    // that algorithm's current values.
    Effect::new(move |_| {
        update_thresh_fields(algo.get().as_str(), ct, &thresh_same, &thresh_close, &thresh_ballpark);
    });

    let on_submit = move |ev: web_sys::SubmitEvent| {
        ev.prevent_default();
        let uniform_val = uniform.get();
        let default_mm_val = default_mm.get();
        let algo_val = algo.get();
        let same_val     = thresh_same.get();
        let close_val    = thresh_close.get();
        let ballpark_val = thresh_ballpark.get();
        spawn_local(async move {
            let r1 = api::put_setting("currency_symbol", currency.get()).await;
            let r2 = api::put_setting(
                "uniform_diameter",
                if uniform_val { "true".into() } else { "false".into() },
            ).await;
            let r3 = api::put_setting("default_diameter", default_mm_val.clone()).await;
            let r4 = api::put_setting("color_distance_algorithm", algo_val.clone()).await;

            // Persist threshold keys for the active algorithm.
            let r5 = api::put_setting(&format!("color_threshold_{algo_val}_same"),     same_val.clone()).await;
            let r6 = api::put_setting(&format!("color_threshold_{algo_val}_close"),    close_val.clone()).await;
            let r7 = api::put_setting(&format!("color_threshold_{algo_val}_ballpark"), ballpark_val.clone()).await;

            if matches!((&r1, &r2, &r3, &r4, &r5, &r6, &r7),
                (Ok(_), Ok(_), Ok(_), Ok(_), Ok(_), Ok(_), Ok(_)))
            {
                // Update app-wide context signals so other components see the
                // change without a reload.
                ds.uniform.set(uniform_val);
                if let Ok(v) = default_mm_val.parse::<f64>() {
                    ds.default_mm.set(v);
                }
                let new_algo = match algo_val.as_str() {
                    "oklab"  => ColorAlgorithm::OkLab,
                    "din99d" => ColorAlgorithm::Din99d,
                    _        => ColorAlgorithm::Ciede2000,
                };
                cda.0.set(new_algo);

                // Update the matching threshold signals for the active algorithm.
                set_thresh_signals(new_algo, ct, &same_val, &close_val, &ballpark_val);

                saved.set(true);
            } else {
                let first_err = [r1, r2, r3, r4, r5, r6, r7]
                    .into_iter()
                    .find_map(|r| r.err());
                error.set(first_err.map(|e| e.to_string()));
            }
        });
    };

    // Range hint shown below threshold fields, varies by algorithm.
    let range_hint = move || match algo.get().as_str() {
        "oklab"  => "typical range 0.050 – 0.500",
        "din99d" => "sensitive — small changes matter; typical range 5.0 – 50.0",
        _        => "typical range 5.0 – 50.0",
    };
    let step_val = move || if algo.get() == "oklab" { "0.001" } else { "0.1" };

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
                <label>
                    "Color search algorithm"
                    <select
                        prop:value=move || algo.get()
                        on:change=move |ev| {
                            saved.set(false);
                            algo.set(event_target_value(&ev));
                        }
                    >
                        <option value="ciede2000">"CIEDE2000 (default)"</option>
                        <option value="oklab">"OKLab"</option>
                        <option value="din99d">"DIN99d"</option>
                    </select>
                </label>
                <fieldset class="threshold-fieldset">
                    <legend>"Color search thresholds — "{move || algo.get().to_uppercase()}</legend>
                    <p class="threshold-hint">{range_hint}</p>
                    <label>
                        "Same"
                        <input type="number" prop:step=step_val min="0"
                            prop:value=move || thresh_same.get()
                            on:input=move |ev| {
                                saved.set(false);
                                thresh_same.set(event_target_value(&ev));
                            }
                        />
                    </label>
                    <label>
                        "Close"
                        <input type="number" prop:step=step_val min="0"
                            prop:value=move || thresh_close.get()
                            on:input=move |ev| {
                                saved.set(false);
                                thresh_close.set(event_target_value(&ev));
                            }
                        />
                    </label>
                    <label>
                        "Ballpark"
                        <input type="number" prop:step=step_val min="0"
                            prop:value=move || thresh_ballpark.get()
                            on:input=move |ev| {
                                saved.set(false);
                                thresh_ballpark.set(event_target_value(&ev));
                            }
                        />
                    </label>
                </fieldset>
                <button type="submit" class="btn btn-primary ">"Save"</button>
            </form>
        </div>
    }
}

// ── helpers ────────────────────────────────────────────────────────────────────

fn update_thresh_fields(
    algo: &str,
    ct: crate::state::ColorThresholds,
    same: &RwSignal<String>,
    close: &RwSignal<String>,
    ballpark: &RwSignal<String>,
) {
    let (s, c, b) = match algo {
        "oklab" => (
            ct.oklab_same.get_untracked(),
            ct.oklab_close.get_untracked(),
            ct.oklab_ballpark.get_untracked(),
        ),
        "din99d" => (
            ct.din99d_same.get_untracked(),
            ct.din99d_close.get_untracked(),
            ct.din99d_ballpark.get_untracked(),
        ),
        _ => (
            ct.ciede2000_same.get_untracked(),
            ct.ciede2000_close.get_untracked(),
            ct.ciede2000_ballpark.get_untracked(),
        ),
    };
    let fmt = |v: f32| -> String {
        if algo == "oklab" { format!("{:.3}", v) } else { format!("{:.1}", v) }
    };
    same.set(fmt(s));
    close.set(fmt(c));
    ballpark.set(fmt(b));
}

fn set_thresh_signals(
    algo: ColorAlgorithm,
    ct: crate::state::ColorThresholds,
    same: &str,
    close: &str,
    ballpark: &str,
) {
    let parse = |v: &str| v.parse::<f32>().unwrap_or(0.0);
    match algo {
        ColorAlgorithm::OkLab => {
            ct.oklab_same.set(parse(same));
            ct.oklab_close.set(parse(close));
            ct.oklab_ballpark.set(parse(ballpark));
        }
        ColorAlgorithm::Din99d => {
            ct.din99d_same.set(parse(same));
            ct.din99d_close.set(parse(close));
            ct.din99d_ballpark.set(parse(ballpark));
        }
        ColorAlgorithm::Ciede2000 => {
            ct.ciede2000_same.set(parse(same));
            ct.ciede2000_close.set(parse(close));
            ct.ciede2000_ballpark.set(parse(ballpark));
        }
    }
}
