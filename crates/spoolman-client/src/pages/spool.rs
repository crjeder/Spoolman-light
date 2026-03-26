use leptos::*;
use leptos_router::{use_navigate, use_params_map};
use spoolman_types::{models::Rgba, requests::{CreateSpool, UpdateSpool}};

use crate::{
    api,
    components::{pagination::Pagination, table::ColHeader},
    state::use_table_state,
    utils::color::{color_distance, hex_to_rgba},
};

// ── List ───────────────────────────────────────────────────────────────────────

#[component]
pub fn SpoolList() -> impl IntoView {
    let ts = use_table_state("spools");
    let show_archived = create_rw_signal(false);
    let color_pick: RwSignal<Option<String>> = create_rw_signal(None);
    let threshold: RwSignal<u8> = create_rw_signal(10u8);
    let visible_cols = create_rw_signal(vec![
        "filament", "color", "remaining_pct", "remaining_weight", "location", "registered",
    ]);

    let version = create_rw_signal(0u32);
    let confirm_delete: RwSignal<Option<u32>> = create_rw_signal(None);

    let spools = create_resource(
        move || (show_archived.get(), version.get()),
        |(archived, _)| async move { api::list_spools(archived).await },
    );

    let on_delete = move |id: u32| {
        spawn_local(async move {
            if api::delete_spool(id).await.is_ok() {
                version.update(|v| *v += 1);
                confirm_delete.set(None);
            }
        });
    };

    let filtered = move || {
        let f = ts.filter.get().to_lowercase();
        let pick = color_pick.get();
        let thresh = threshold.get() as f32;
        spools
            .get()
            .and_then(|r| r.ok())
            .unwrap_or_default()
            .into_iter()
            .filter(|s| {
                let text_ok = f.is_empty()
                    || s.filament.display_name().to_lowercase().contains(&f)
                    || s.spool.color_name.as_deref().unwrap_or("").to_lowercase().contains(&f)
                    || s.filament.material.as_ref().map(|m| m.abbreviation()).unwrap_or("").to_lowercase().contains(&f);
                let color_ok = match pick.as_deref().and_then(hex_to_rgba) {
                    None => true,
                    Some(target) => s.spool.colors.iter().any(|c| color_distance(c, &target) <= thresh),
                };
                text_ok && color_ok
            })
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
                "id" => {
                    let ord = a.spool.id.cmp(&b.spool.id);
                    if asc { ord } else { ord.reverse() }
                }
                "filament" => {
                    let ord = a.filament.display_name().to_lowercase()
                        .cmp(&b.filament.display_name().to_lowercase());
                    if asc { ord } else { ord.reverse() }
                }
                "remaining_pct" => match (a.remaining_pct, b.remaining_pct) {
                    (None, None) => Ordering::Equal,
                    (None, _) => Ordering::Greater,
                    (_, None) => Ordering::Less,
                    (Some(av), Some(bv)) => {
                        let ord = av.partial_cmp(&bv).unwrap_or(Ordering::Equal);
                        if asc { ord } else { ord.reverse() }
                    }
                },
                "remaining_weight" => match (a.remaining_filament, b.remaining_filament) {
                    (None, None) => Ordering::Equal,
                    (None, _) => Ordering::Greater,
                    (_, None) => Ordering::Less,
                    (Some(av), Some(bv)) => {
                        let ord = av.partial_cmp(&bv).unwrap_or(Ordering::Equal);
                        if asc { ord } else { ord.reverse() }
                    }
                },
                "location" => match (a.spool.location_id, b.spool.location_id) {
                    (None, None) => Ordering::Equal,
                    (None, _) => Ordering::Greater,
                    (_, None) => Ordering::Less,
                    (Some(av), Some(bv)) => {
                        let ord = av.cmp(&bv);
                        if asc { ord } else { ord.reverse() }
                    }
                },
                "registered" => {
                    let ord = a.spool.registered.cmp(&b.spool.registered);
                    if asc { ord } else { ord.reverse() }
                }
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
        <div class="page spool-list">
            <div class="page-header">
                <h1>"Spools"</h1>
                <div class="page-actions">
                    <input
                        type="text" placeholder="Filter…"
                        on:input=move |ev| ts.filter.set(event_target_value(&ev))
                    />
                    <label>
                        <input type="checkbox"
                            on:change=move |ev| show_archived.set(event_target_checked(&ev))
                        />
                        " Show archived"
                    </label>
                    <span class="color-filter">
                        <input type="color"
                            title="Filter by color"
                            on:input=move |ev| color_pick.set(Some(event_target_value(&ev)))
                        />
                        {move || color_pick.get().map(|_| view! {
                            <button type="button" class="btn"
                                on:click=move |_| color_pick.set(None)
                            >"×"</button>
                            <input type="range" min="0" max="255" step="1"
                                title="Color match threshold"
                                prop:value=threshold
                                on:input=move |ev| threshold.set(
                                    event_target_value(&ev).parse().unwrap_or(60)
                                )
                            />
                        })}
                    </span>
                    <a href="/spools/new" class="btn btn-primary ">"+ New Spool"</a>
                </div>
            </div>
            <Suspense fallback=|| view! { <p>"Loading…"</p> }>
                <table class="data-table">
                    <thead>
                        <tr>
                            <ColHeader label="ID"       field="id"         sort_field=ts.sort_field sort_asc=ts.sort_asc num=true />
                            <ColHeader label="Filament" field="filament"   sort_field=ts.sort_field sort_asc=ts.sort_asc />
                            <th>"Color"</th>
                            <ColHeader label="Remaining%" field="remaining_pct" sort_field=ts.sort_field sort_asc=ts.sort_asc num=true />
                            <ColHeader label="Remaining (g)" field="remaining_weight" sort_field=ts.sort_field sort_asc=ts.sort_asc num=true />
                            <ColHeader label="Location"      field="location"          sort_field=ts.sort_field sort_asc=ts.sort_asc />
                            <ColHeader label="Registered" field="registered" sort_field=ts.sort_field sort_asc=ts.sort_asc />
                            <th>"Actions"</th>
                        </tr>
                    </thead>
                    <tbody>
                        {move || page_items().into_iter().map(|sr| {
                            let id = sr.spool.id;
                            let name = sr.filament.display_name();
                            let color = sr.spool.colors.first().cloned()
                                .unwrap_or(Rgba { r: 200, g: 200, b: 200, a: 255 });
                            let pct = sr.remaining_pct.map(|p| format!("{:.0}%", p)).unwrap_or_default();
                            let rem = sr.remaining_filament.map(|w| format!("{:.0}g", w)).unwrap_or_default();
                            view! {
                                <tr class=if sr.spool.archived { "archived" } else { "" }>
                                    <td class="num"><a href=format!("/spools/{id}")>{id}</a></td>
                                    <td>{name}</td>
                                    <td>
                                        <span class="color-swatch"
                                            style=format!("background:rgba({},{},{},{})",
                                                color.r, color.g, color.b, color.a as f32/255.0)>
                                        </span>
                                        {sr.spool.color_name.clone().unwrap_or_default()}
                                    </td>
                                    <td class="num">{pct}</td>
                                    <td class="num">{rem}</td>
                                    <td>{sr.spool.location_id.map(|l| l.to_string()).unwrap_or_default()}</td>
                                    <td>{sr.spool.registered.format("%Y-%m-%d").to_string()}</td>
                                    <td class="actions">
                                        <a href=format!("/spools/{id}/edit")>"Edit"</a>
                                        " "
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
pub fn SpoolShow() -> impl IntoView {
    let params = use_params_map();
    let id = move || params.with(|p| p.get("id").and_then(|v| v.parse::<u32>().ok()).unwrap_or(0));
    let spool = create_resource(id, |id| async move { api::get_spool(id).await });
    let navigate = use_navigate();
    let navigate_clone = navigate.clone();

    // store_value gives Copy semantics so these handlers can be captured
    // by the reactive `move ||` closure inside view! without making it FnOnce.
    let nav1 = navigate.clone();
    let on_delete = store_value(move |_: web_sys::MouseEvent| {
        let id = id();
        let nav = nav1.clone();
        spawn_local(async move {
            if api::delete_spool(id).await.is_ok() {
                nav("/spools", Default::default());
            }
        });
    });

    let on_clone = store_value(move |_: web_sys::MouseEvent| {
        let id = id();
        let nav = navigate_clone.clone();
        spawn_local(async move {
            if let Ok(new) = api::clone_spool(id).await {
                nav(&format!("/spools/{}", new.spool.id), Default::default());
            }
        });
    });

    view! {
        <div class="page spool-show">
            // Action buttons are outside the reactive Suspense block because
            // on_clone and on_delete use the `id` signal directly, not `sr`.
            // Placing them inside {move ||...} would make that closure FnOnce.
            <div class="page-header">
                <h1>"Spool #"{move || id()}</h1>
                <div class="page-actions">
                    <a href=move || format!("/spools/{}/edit", id()) class="btn ">"Edit"</a>
                    <button on:click=move |e| on_clone.with_value(|f| f(e)) class="btn ">"Clone"</button>
                    <button on:click=move |e| on_delete.with_value(|f| f(e)) class="btn btn-danger ">"Delete"</button>
                </div>
            </div>
            <Suspense fallback=|| view! { <p>"Loading…"</p> }>
                {move || spool.get().map(|r| match r {
                    Err(e) => view! { <p class="error">{e.to_string()}</p> }.into_view(),
                    Ok(sr) => view! {
                        <dl class="detail-grid">
                            <dt>"Filament"</dt><dd>{sr.filament.display_name()}</dd>
                            <dt>"Colors"</dt><dd>{sr.spool.colors.iter().map(|c| {
                                view! {
                                    <span class="color-swatch"
                                        style=format!("background:rgba({},{},{},{})",
                                            c.r, c.g, c.b, c.a as f32/255.0)>
                                    </span>
                                }
                            }).collect_view()}</dd>
                            <dt>"Color name"</dt><dd>{sr.spool.color_name.clone().unwrap_or_default()}</dd>
                            <dt>"Initial weight"</dt><dd>{format!("{:.1}g", sr.spool.initial_weight)}</dd>
                            <dt>"Current weight"</dt><dd>{format!("{:.1}g", sr.spool.current_weight)}</dd>
                            <dt>"Used"</dt><dd>{format!("{:.1}g", sr.used_weight)}</dd>
                            <dt>"Remaining filament"</dt><dd>{sr.remaining_filament.map(|w| format!("{:.1}g", w)).unwrap_or_else(|| "unknown".into())}</dd>
                            <dt>"Remaining %"</dt><dd>{sr.remaining_pct.map(|p| format!("{:.0}%", p)).unwrap_or_else(|| "unknown".into())}</dd>
                            <dt>"Registered"</dt><dd>{sr.spool.registered.to_rfc3339()}</dd>
                            <dt>"Last used"</dt><dd>{sr.spool.last_used.map(|d| d.to_rfc3339()).unwrap_or_default()}</dd>
                            <dt>"Comment"</dt><dd>{sr.spool.comment.clone().unwrap_or_default()}</dd>
                            <dt>"Archived"</dt><dd>{if sr.spool.archived { "Yes" } else { "No" }}</dd>
                        </dl>
                    }.into_view(),
                })}
            </Suspense>
        </div>
    }
}


// ── Create ─────────────────────────────────────────────────────────────────────

#[component]
pub fn SpoolCreate() -> impl IntoView {
    let navigate = use_navigate();
    let filaments = create_resource(|| (), |_| async { api::list_filaments(None).await });
    let locations = create_resource(|| (), |_| async { api::list_locations().await });

    let filament_id = create_rw_signal(0u32);
    let color_hex = create_rw_signal(String::from("#000000"));
    let color_name = create_rw_signal(String::new());
    let initial_weight = create_rw_signal(String::new());
    let location_id = create_rw_signal(Option::<u32>::None);
    let comment = create_rw_signal(String::new());
    let error = create_rw_signal(Option::<String>::None);

    create_effect(move |_| {
        if let Some(Ok(fs)) = filaments.get() {
            if let Some(first) = fs.first() {
                filament_id.set(first.id);
            }
        }
    });

    let on_submit = move |ev: web_sys::SubmitEvent| {
        ev.prevent_default();
        if location_id.get().is_none() {
            error.set(Some("Location is required.".into()));
            return;
        }
        let navigate = navigate.clone();
        spawn_local(async move {
            let weight = initial_weight.get().parse::<f32>().unwrap_or(0.0);
            let body = CreateSpool {
                filament_id: filament_id.get(),
                colors: hex_to_rgba(&color_hex.get()).map(|c| vec![c]).unwrap_or_default(),
                color_name: Some(color_name.get()).filter(|s| !s.is_empty()),
                location_id: location_id.get(),
                initial_weight: weight,
                first_used: None,
                last_used: None,
                comment: Some(comment.get()).filter(|s| !s.is_empty()),
            };
            match api::create_spool(&body).await {
                Ok(s) => navigate(&format!("/spools/{}", s.spool.id), Default::default()),
                Err(e) => error.set(Some(e.to_string())),
            }
        });
    };

    view! {
        <div class="page spool-create">
            <h1>"New Spool"</h1>
            {move || error.get().map(|e| view! { <p class="error">{e}</p> })}
            <form on:submit=on_submit>
                <label>
                    "Filament"
                    <Suspense fallback=|| view! { <select><option>"Loading…"</option></select> }>
                        <select on:change=move |ev| {
                            filament_id.set(event_target_value(&ev).parse().unwrap_or(0));
                        }>
                            {move || filaments.get().and_then(|r| r.ok()).map(|fs| {
                                fs.into_iter().map(|f| view! {
                                    <option value=f.id.to_string()>{f.display_name()}</option>
                                }).collect_view()
                            })}
                        </select>
                    </Suspense>
                </label>
                <label>
                    "Color"
                    <input type="color"
                        prop:value=move || color_hex.get()
                        on:input=move |ev| color_hex.set(event_target_value(&ev)) />
                </label>
                <label>
                    "Color name"
                    <input type="text" on:input=move |ev| color_name.set(event_target_value(&ev)) />
                </label>
                <label>
                    "Initial weight (g)"
                    <input type="number" step="0.1"
                        on:input=move |ev| initial_weight.set(event_target_value(&ev)) />
                </label>
                <label>
                    "Location"
                    <Suspense fallback=|| view! { <select><option>"Loading…"</option></select> }>
                        <select on:change=move |ev| {
                            let v = event_target_value(&ev);
                            location_id.set(v.parse::<u32>().ok());
                        }>
                            <option value="">"— none —"</option>
                            {move || locations.get().and_then(|r| r.ok()).map(|ls| {
                                ls.into_iter().map(|l| view! {
                                    <option value=l.location.id.to_string()>{l.location.name}</option>
                                }).collect_view()
                            })}
                        </select>
                    </Suspense>
                </label>
                <label>
                    "Comment"
                    <textarea on:input=move |ev| comment.set(event_target_value(&ev))></textarea>
                </label>
                <button type="submit" class="btn btn-primary ">"Create"</button>
                <a href="/spools" class="btn ">"Cancel"</a>
            </form>
        </div>
    }
}

// ── Edit ───────────────────────────────────────────────────────────────────────

#[component]
pub fn SpoolEdit() -> impl IntoView {
    let params = use_params_map();
    let id = move || params.with(|p| p.get("id").and_then(|v| v.parse::<u32>().ok()).unwrap_or(0));
    let spool = create_resource(id, |id| async move { api::get_spool(id).await });
    let locations = create_resource(|| (), |_| async { api::list_locations().await });
    let navigate = use_navigate();

    let current_weight = create_rw_signal(String::new());
    let color_hex = create_rw_signal(String::from("#000000"));
    let color_name = create_rw_signal(String::new());
    let location_id = create_rw_signal(Option::<u32>::None);
    let comment = create_rw_signal(String::new());
    let error = create_rw_signal(Option::<String>::None);

    // Pre-fill once loaded.
    create_effect(move |_| {
        if let Some(Ok(sr)) = spool.get() {
            current_weight.set(sr.spool.current_weight.to_string());
            if let Some(c) = sr.spool.colors.first() {
                color_hex.set(format!("#{:02x}{:02x}{:02x}", c.r, c.g, c.b));
            }
            color_name.set(sr.spool.color_name.clone().unwrap_or_default());
            location_id.set(sr.spool.location_id);
            comment.set(sr.spool.comment.clone().unwrap_or_default());
        }
    });

    let on_submit = move |ev: web_sys::SubmitEvent| {
        ev.prevent_default();
        if location_id.get().is_none() {
            error.set(Some("Location is required.".into()));
            return;
        }
        let navigate = navigate.clone();
        let id = id();
        spawn_local(async move {
            let body = UpdateSpool {
                current_weight: current_weight.get().parse::<f32>().ok(),
                colors: Some(hex_to_rgba(&color_hex.get()).map(|c| vec![c]).unwrap_or_default()),
                color_name: Some(color_name.get()),
                location_id: location_id.get(),
                comment: Some(comment.get()),
                ..Default::default()
            };
            match api::update_spool(id, &body).await {
                Ok(_) => navigate("/spools", Default::default()),
                Err(e) => error.set(Some(e.to_string())),
            }
        });
    };

    view! {
        <div class="page spool-edit">
            <h1>"Edit Spool"</h1>
            {move || error.get().map(|e| view! { <p class="error">{e}</p> })}
            <form on:submit=on_submit>
                <label>
                    "Current weight (g)"
                    <input type="number" step="0.1"
                        prop:value=move || current_weight.get()
                        on:input=move |ev| current_weight.set(event_target_value(&ev)) />
                </label>
                <label>
                    "Color"
                    <input type="color"
                        prop:value=move || color_hex.get()
                        on:input=move |ev| color_hex.set(event_target_value(&ev)) />
                </label>
                <label>
                    "Color name"
                    <input type="text"
                        prop:value=move || color_name.get()
                        on:input=move |ev| color_name.set(event_target_value(&ev)) />
                </label>
                <label>
                    "Location"
                    <Suspense fallback=|| view! { <select /> }>
                        <select on:change=move |ev| {
                            location_id.set(event_target_value(&ev).parse::<u32>().ok());
                        }>
                            <option value="">"— none —"</option>
                            {move || locations.get().and_then(|r| r.ok()).map(|ls| {
                                let cur = location_id.get();
                                ls.into_iter().map(|l| view! {
                                    <option
                                        value=l.location.id.to_string()
                                        selected=cur == Some(l.location.id)
                                    >{l.location.name}</option>
                                }).collect_view()
                            })}
                        </select>
                    </Suspense>
                </label>
                <label>
                    "Comment"
                    <textarea
                        prop:value=move || comment.get()
                        on:input=move |ev| comment.set(event_target_value(&ev))>
                    </textarea>
                </label>
                <button type="submit" class="btn btn-primary ">"Save"</button>
                <a href=move || format!("/spools/{}", id()) class="btn ">"Cancel"</a>
            </form>
        </div>
    }
}
