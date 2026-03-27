use leptos::*;
use leptos_router::use_params_map;
use spoolman_types::{
    models::MaterialType,
    requests::{CreateFilament, UpdateFilament},
};
use crate::{api, components::{pagination::Pagination, table::ColHeader}, state::{diameter_settings, use_table_state}};

// ── Shared material <select> helper ────────────────────────────────────────────

/// Renders a `<select>` element for choosing a `MaterialType`.
/// Writes the selected abbreviation (or empty string for none) into `value`.
#[component]
fn MaterialSelect(value: RwSignal<String>) -> impl IntoView {
    view! {
        <select
            prop:value=move || value.get()
            on:change=move |ev| value.set(event_target_value(&ev))
        >
            <option value="">"— Select material —"</option>
            {MaterialType::all_known().into_iter().map(|m| {
                let abbr = m.abbreviation().to_string();
                let label = if let Some(name) = m.full_name() {
                    format!("{abbr} – {name}")
                } else {
                    abbr.clone()
                };
                view! { <option value=abbr.clone()>{label}</option> }
            }).collect_view()}
        </select>
    }
}

// ── List ───────────────────────────────────────────────────────────────────────

#[component]
pub fn FilamentList() -> impl IntoView {
    let ts = use_table_state("filaments");
    let material_filter = create_rw_signal(String::new());
    let ds = diameter_settings();
    let show_diameter = move || !ds.uniform.get();

    let version = create_rw_signal(0u32);
    let confirm_delete: RwSignal<Option<u32>> = create_rw_signal(None);

    let filaments = create_resource(
        move || (material_filter.get(), version.get()),
        |(mat, _)| async move {
            let mat_opt = if mat.is_empty() { None } else { Some(mat) };
            api::list_filaments(mat_opt.as_deref()).await
        },
    );

    let on_delete = move |id: u32| {
        spawn_local(async move {
            if api::delete_filament(id).await.is_ok() {
                version.update(|v| *v += 1);
                confirm_delete.set(None);
            }
        });
    };

    let filtered = move || {
        let f = ts.filter.get().to_lowercase();
        filaments.get().and_then(|r| r.ok()).unwrap_or_default()
            .into_iter()
            .filter(|fil| f.is_empty() || fil.display_name().to_lowercase().contains(&f))
            .collect::<Vec<_>>()
    };

    let sort_field = ts.sort_field;
    let sort_asc = ts.sort_asc;
    let sorted = move || {
        let mut items = filtered();
        let field = sort_field.get();
        let asc = sort_asc.get();
        items.sort_by(|a, b| {
            use std::cmp::Ordering;
            match field.as_str() {
                "manufacturer" => match (a.manufacturer.as_deref(), b.manufacturer.as_deref()) {
                    (None, None) => Ordering::Equal,
                    (None, _) => Ordering::Greater,
                    (_, None) => Ordering::Less,
                    (Some(av), Some(bv)) => {
                        let ord = av.to_lowercase().cmp(&bv.to_lowercase());
                        if asc { ord } else { ord.reverse() }
                    }
                },
                "material" => {
                    let am = a.material.as_ref().map(|m| m.abbreviation().to_string());
                    let bm = b.material.as_ref().map(|m| m.abbreviation().to_string());
                    match (am, bm) {
                        (None, None) => Ordering::Equal,
                        (None, _) => Ordering::Greater,
                        (_, None) => Ordering::Less,
                        (Some(av), Some(bv)) => {
                            let ord = av.cmp(&bv);
                            if asc { ord } else { ord.reverse() }
                        }
                    }
                },
                "density" => {
                    let ord = a.density.partial_cmp(&b.density).unwrap_or(Ordering::Equal);
                    if asc { ord } else { ord.reverse() }
                },
                "registered" => {
                    let ord = a.registered.cmp(&b.registered);
                    if asc { ord } else { ord.reverse() }
                },
                _ => Ordering::Equal,
            }
        });
        items
    };

    let total = Signal::derive(move || filtered().len());
    let page_items = move || {
        let items = sorted();
        let start = ts.page.get() * ts.page_size.get();
        items.into_iter().skip(start).take(ts.page_size.get()).collect::<Vec<_>>()
    };

    view! {
        <div class="page filament-list">
            <div class="page-header">
                <h1>"Filaments"</h1>
                <div class="page-actions">
                    <select
                        on:change=move |ev| {
                            material_filter.set(event_target_value(&ev));
                            ts.page.set(0);
                        }
                    >
                        <option value="">"All materials"</option>
                        {MaterialType::all_known().iter().map(|m| {
                            let abbr = m.abbreviation().to_string();
                            view! { <option value=abbr.clone()>{abbr}</option> }
                        }).collect_view()}
                    </select>
                    <input type="text" placeholder="Filter…"
                        on:input=move |ev| ts.filter.set(event_target_value(&ev)) />
                    <a href="/filaments/new" class="btn btn-primary ">"+ New Filament"</a>
                </div>
            </div>
            <Suspense fallback=|| view! { <p>"Loading…"</p> }>
                <table class="data-table">
                    <thead>
                        <tr>
                            <ColHeader label="Manufacturer" field="manufacturer" sort_field=ts.sort_field sort_asc=ts.sort_asc />
                            <ColHeader label="Material"     field="material"     sort_field=ts.sort_field sort_asc=ts.sort_asc />
                            <th>"Modifier"</th>
                            {move || show_diameter().then(|| view! { <th class="num">"Diameter"</th> })}
                            <ColHeader label="Density" field="density" sort_field=ts.sort_field sort_asc=ts.sort_asc num=true />
                            <ColHeader label="Registered" field="registered" sort_field=ts.sort_field sort_asc=ts.sort_asc />
                            <th>"Actions"</th>
                        </tr>
                    </thead>
                    <tbody>
                        {move || page_items().into_iter().map(|f| {
                            let id = f.id;
                            view! {
                                <tr>
                                    <td>{f.manufacturer.clone().unwrap_or_default()}</td>
                                    <td>{f.material.as_ref().map(|m| m.abbreviation().to_string()).unwrap_or_default()}</td>
                                    <td>{f.material_modifier.clone().unwrap_or_default()}</td>
                                    {show_diameter().then(|| { let d = f.diameter; view! { <td class="num">{format!("{:.2}mm", d)}</td> } })}
                                    <td class="num">{format!("{:.3}", f.density)}</td>
                                    <td>{f.registered.format("%Y-%m-%d").to_string()}</td>
                                    <td class="actions">
                                        <a href=format!("/filaments/{id}")>"View"</a>
                                        " · "
                                        <a href=format!("/filaments/{id}/edit")>"Edit"</a>
                                        " · "
                                        {move || if confirm_delete.get() == Some(id) {
                                            view! {
                                                <button class="btn btn-danger "
                                                    on:click=move |_| on_delete(id)>"Sure?"</button>
                                                " "
                                                <button class="btn "
                                                    on:click=move |_| confirm_delete.set(None)>"Cancel"</button>
                                            }.into_view()
                                        } else {
                                            view! {
                                                <button class="btn btn-danger "
                                                    on:click=move |_| confirm_delete.set(Some(id))>"Delete"</button>
                                            }.into_view()
                                        }}
                                    </td>
                                </tr>
                            }
                        }).collect_view()}
                    </tbody>
                </table>
                <Pagination page=ts.page page_size=ts.page_size total=total />
            </Suspense>
        </div>
    }
}

// ── Show ───────────────────────────────────────────────────────────────────────

#[component]
pub fn FilamentShow() -> impl IntoView {
    let params = use_params_map();
    let id = move || params.with(|p| p.get("id").and_then(|v| v.parse::<u32>().ok()).unwrap_or(0));
    let filament = create_resource(id, |id| async move { api::get_filament(id).await });
    let navigate = leptos_router::use_navigate();
    let confirm_delete = create_rw_signal(false);

    let nav_delete = navigate.clone();
    let nav_err = navigate.clone();

    let on_delete = store_value(move |_: web_sys::MouseEvent| {
        let id = id();
        let nav = nav_delete.clone();
        spawn_local(async move {
            if api::delete_filament(id).await.is_ok() {
                nav("/filaments", Default::default());
            }
        });
    });

    view! {
        <div class="page filament-show">
            <Suspense fallback=|| view! { <p>"Loading…"</p> }>
                {move || filament.get().map(|r| match r {
                    Err(e) => {
                        if e.status == 404 {
                            nav_err.clone()("/filaments", Default::default());
                            view! { <></> }.into_view()
                        } else {
                            view! { <p class="error">{e.to_string()}</p> }.into_view()
                        }
                    }
                    Ok(f) => view! {
                        <div class="page-header">
                            <h1>{f.display_name()}</h1>
                            <div class="page-actions">
                                <a href=format!("/filaments/{}/edit", f.id) class="btn ">"Edit"</a>
                                {move || if confirm_delete.get() {
                                    view! {
                                        <button on:click=move |e| on_delete.with_value(|f| f(e)) class="btn btn-danger ">"Sure?"</button>
                                        " "
                                        <button on:click=move |_| confirm_delete.set(false) class="btn ">"Cancel"</button>
                                    }.into_view()
                                } else {
                                    view! {
                                        <button on:click=move |_| confirm_delete.set(true) class="btn btn-danger ">"Delete"</button>
                                    }.into_view()
                                }}
                            </div>
                        </div>
                        <dl class="detail-grid">
                            <dt>"Manufacturer"</dt><dd>{f.manufacturer.clone().unwrap_or_default()}</dd>
                            <dt>"Material"</dt><dd>{
                                f.material.as_ref().map(|m| {
                                    let abbr = m.abbreviation().to_string();
                                    let name = m.full_name().unwrap_or("");
                                    if name.is_empty() { abbr } else { format!("{abbr} – {name}") }
                                }).unwrap_or_default()
                            }</dd>
                            <dt>"Modifier"</dt><dd>{f.material_modifier.clone().unwrap_or_default()}</dd>
                            <dt>"Diameter"</dt><dd>{format!("{:.2}mm", f.diameter)}</dd>
                            <dt>"Density"</dt><dd>{format!("{:.3} g/cm³", f.density)}</dd>
                            <dt>"Print temp"</dt><dd>{f.print_temp.map(|t| format!("{}°C", t)).unwrap_or_default()}</dd>
                            <dt>"Bed temp"</dt><dd>{f.bed_temp.map(|t| format!("{}°C", t)).unwrap_or_default()}</dd>
                            <dt>"Spool weight"</dt><dd>{f.spool_weight.map(|w| format!("{:.0}g", w)).unwrap_or_default()}</dd>
                            <dt>"Comment"</dt><dd>{f.comment.clone().unwrap_or_default()}</dd>
                        </dl>
                    }.into_view(),
                })}
            </Suspense>
        </div>
    }
}

// ── Create ─────────────────────────────────────────────────────────────────────

#[component]
pub fn FilamentCreate() -> impl IntoView {
    let navigate = leptos_router::use_navigate();
    let ds = diameter_settings();
    let manufacturer = create_rw_signal(String::new());
    let material = create_rw_signal(String::new());
    let modifier = create_rw_signal(String::new());
    // Initialise diameter from the configured default.
    let diameter = create_rw_signal(ds.default_mm.get_untracked().to_string());
    let density = create_rw_signal("1.24".to_string());
    let print_temp = create_rw_signal(String::new());
    let bed_temp = create_rw_signal(String::new());
    let comment = create_rw_signal(String::new());
    let error = create_rw_signal(Option::<String>::None);

    let on_submit = move |ev: web_sys::SubmitEvent| {
        ev.prevent_default();
        let navigate = navigate.clone();
        spawn_local(async move {
            let mat = material.get();
            // When uniform mode is on, use the configured default diameter.
            let resolved_diameter = if ds.uniform.get() {
                ds.default_mm.get() as f32
            } else {
                diameter.get().parse().unwrap_or(ds.default_mm.get() as f32)
            };
            let body = CreateFilament {
                manufacturer: Some(manufacturer.get()).filter(|s| !s.is_empty()),
                material: if mat.is_empty() { None } else { Some(MaterialType::from_abbreviation(&mat)) },
                material_modifier: Some(modifier.get()).filter(|s| !s.is_empty()),
                diameter: resolved_diameter,
                density: density.get().parse().unwrap_or(1.24),
                print_temp: print_temp.get().parse().ok(),
                bed_temp: bed_temp.get().parse().ok(),
                spool_weight: None,
                min_print_temp: None, max_print_temp: None,
                min_bed_temp: None, max_bed_temp: None,
                comment: Some(comment.get()).filter(|s| !s.is_empty()),
            };
            match api::create_filament(&body).await {
                Ok(f) => navigate(&format!("/filaments/{}", f.id), Default::default()),
                Err(e) => error.set(Some(e.to_string())),
            }
        });
    };

    view! {
        <div class="page filament-create">
            <h1>"New Filament"</h1>
            {move || error.get().map(|e| view! { <p class="error">{e}</p> })}
            <form on:submit=on_submit>
                <label>"Manufacturer"<input type="text" on:input=move |ev| manufacturer.set(event_target_value(&ev)) /></label>
                <label>"Material"
                    <MaterialSelect value=material />
                </label>
                <label>"Modifier"<input type="text" on:input=move |ev| modifier.set(event_target_value(&ev)) /></label>
                {move || (!ds.uniform.get()).then(|| view! {
                    <label>"Diameter (mm)"<input type="number" step="0.01" prop:value=move || diameter.get() on:input=move |ev| diameter.set(event_target_value(&ev)) /></label>
                })}
                <label>"Density (g/cm³)"<input type="number" step="0.001" prop:value=move || density.get() on:input=move |ev| density.set(event_target_value(&ev)) /></label>
                <label>"Print temp (°C)"<input type="number" on:input=move |ev| print_temp.set(event_target_value(&ev)) /></label>
                <label>"Bed temp (°C)"<input type="number" on:input=move |ev| bed_temp.set(event_target_value(&ev)) /></label>
                <label>"Comment"<textarea on:input=move |ev| comment.set(event_target_value(&ev))></textarea></label>
                <button type="submit" class="btn btn-primary ">"Create"</button>
                <a href="/filaments" class="btn ">"Cancel"</a>
            </form>
        </div>
    }
}

// ── Edit ───────────────────────────────────────────────────────────────────────

#[component]
pub fn FilamentEdit() -> impl IntoView {
    let params = use_params_map();
    let id = move || params.with(|p| p.get("id").and_then(|v| v.parse::<u32>().ok()).unwrap_or(0));
    let filament = create_resource(id, |id| async move { api::get_filament(id).await });
    let navigate = leptos_router::use_navigate();
    let ds = diameter_settings();

    let manufacturer = create_rw_signal(String::new());
    let material = create_rw_signal(String::new());
    let modifier = create_rw_signal(String::new());
    let diameter = create_rw_signal("1.75".to_string());
    let density = create_rw_signal("1.24".to_string());
    let print_temp = create_rw_signal(String::new());
    let bed_temp = create_rw_signal(String::new());
    let comment = create_rw_signal(String::new());
    let error = create_rw_signal(Option::<String>::None);

    create_effect(move |_| {
        if let Some(Ok(f)) = filament.get() {
            manufacturer.set(f.manufacturer.clone().unwrap_or_default());
            material.set(f.material.as_ref().map(|m| m.abbreviation().to_string()).unwrap_or_default());
            modifier.set(f.material_modifier.clone().unwrap_or_default());
            diameter.set(f.diameter.to_string());
            density.set(f.density.to_string());
            print_temp.set(f.print_temp.map(|t| t.to_string()).unwrap_or_default());
            bed_temp.set(f.bed_temp.map(|t| t.to_string()).unwrap_or_default());
            comment.set(f.comment.clone().unwrap_or_default());
        }
    });

    let on_submit = move |ev: web_sys::SubmitEvent| {
        ev.prevent_default();
        let navigate = navigate.clone();
        let id = id();
        spawn_local(async move {
            let mat = material.get();
            let body = UpdateFilament {
                manufacturer: Some(manufacturer.get()).filter(|s| !s.is_empty()),
                material: if mat.is_empty() { None } else { Some(MaterialType::from_abbreviation(&mat)) },
                material_modifier: Some(modifier.get()).filter(|s| !s.is_empty()),
                diameter: diameter.get().parse().ok(),
                density: density.get().parse().ok(),
                print_temp: print_temp.get().parse().ok(),
                bed_temp: bed_temp.get().parse().ok(),
                comment: Some(comment.get()).filter(|s| !s.is_empty()),
                ..Default::default()
            };
            match api::update_filament(id, &body).await {
                Ok(_) => navigate(&format!("/filaments/{id}"), Default::default()),
                Err(e) => error.set(Some(e.to_string())),
            }
        });
    };

    view! {
        <div class="page filament-edit">
            <h1>"Edit Filament"</h1>
            {move || error.get().map(|e| view! { <p class="error">{e}</p> })}
            <form on:submit=on_submit>
                <label>"Manufacturer"<input type="text" prop:value=move || manufacturer.get() on:input=move |ev| manufacturer.set(event_target_value(&ev)) /></label>
                <label>"Material"
                    <MaterialSelect value=material />
                </label>
                <label>"Modifier"<input type="text" prop:value=move || modifier.get() on:input=move |ev| modifier.set(event_target_value(&ev)) /></label>
                {move || (!ds.uniform.get()).then(|| view! {
                    <label>"Diameter (mm)"<input type="number" step="0.01" prop:value=move || diameter.get() on:input=move |ev| diameter.set(event_target_value(&ev)) /></label>
                })}
                <label>"Density (g/cm³)"<input type="number" step="0.001" prop:value=move || density.get() on:input=move |ev| density.set(event_target_value(&ev)) /></label>
                <label>"Print temp (°C)"<input type="number" prop:value=move || print_temp.get() on:input=move |ev| print_temp.set(event_target_value(&ev)) /></label>
                <label>"Bed temp (°C)"<input type="number" prop:value=move || bed_temp.get() on:input=move |ev| bed_temp.set(event_target_value(&ev)) /></label>
                <label>"Comment"<textarea prop:value=move || comment.get() on:input=move |ev| comment.set(event_target_value(&ev))></textarea></label>
                <button type="submit" class="btn btn-primary ">"Save"</button>
                <a href=move || format!("/filaments/{}", id()) class="btn ">"Cancel"</a>
            </form>
        </div>
    }
}
