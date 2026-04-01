use chrono::{DateTime, NaiveDate, Utc};
use leptos::*;
use leptos_router::{use_navigate, use_params_map};
use spoolman_types::{
    models::Rgba,
    requests::{CreateSpool, UpdateSpool},
};

use crate::{
    api,
    components::{pagination::Pagination, table::ColHeader},
    format,
    state::use_table_state,
    utils::color::{color_distance, hex_to_rgba},
};

// ── List ───────────────────────────────────────────────────────────────────────

#[component]
pub fn SpoolList() -> impl IntoView {
    let ts = use_table_state("spools");
    let show_archived = create_rw_signal(false);
    let color_pick = create_rw_signal("#000000".to_string());
    let color_level = create_rw_signal("off".to_string());
    let popup_open = create_rw_signal(false);

    const FINE_THRESHOLD: f32 = 10.0;
    const MEDIUM_THRESHOLD: f32 = 20.0;
    const COARSE_THRESHOLD: f32 = 35.0;
    let _visible_cols = create_rw_signal(vec![
        "filament",
        "color",
        "remaining_weight",
        "location",
        "registered",
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
        let level = color_level.get();
        spools
            .get()
            .and_then(|r| r.ok())
            .unwrap_or_default()
            .into_iter()
            .filter(|s| {
                let text_ok = f.is_empty()
                    || s.filament.display_name().to_lowercase().contains(&f)
                    || s.spool
                        .color_name
                        .as_deref()
                        .unwrap_or("")
                        .to_lowercase()
                        .contains(&f)
                    || s.filament
                        .material
                        .as_ref()
                        .map(|m| m.abbreviation())
                        .unwrap_or("")
                        .to_lowercase()
                        .contains(&f);
                let color_ok = if level == "off" {
                    true
                } else {
                    match hex_to_rgba(&pick) {
                        None => true, // invalid hex — don't filter
                        Some(target) => {
                            let thresh = match level.as_str() {
                                "fine" => FINE_THRESHOLD,
                                "medium" => MEDIUM_THRESHOLD,
                                _ => COARSE_THRESHOLD,
                            };
                            s.spool
                                .colors
                                .iter()
                                .any(|c| color_distance(c, &target) <= thresh)
                        }
                    }
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
                    if asc {
                        ord
                    } else {
                        ord.reverse()
                    }
                }
                "filament" => {
                    let ord = a
                        .filament
                        .display_name()
                        .to_lowercase()
                        .cmp(&b.filament.display_name().to_lowercase());
                    if asc {
                        ord
                    } else {
                        ord.reverse()
                    }
                }
                "remaining_weight" => match (a.remaining_filament, b.remaining_filament) {
                    (None, None) => Ordering::Equal,
                    (None, _) => Ordering::Greater,
                    (_, None) => Ordering::Less,
                    (Some(av), Some(bv)) => {
                        let ord = av.partial_cmp(&bv).unwrap_or(Ordering::Equal);
                        if asc {
                            ord
                        } else {
                            ord.reverse()
                        }
                    }
                },
                "location" => match (a.spool.location_id, b.spool.location_id) {
                    (None, None) => Ordering::Equal,
                    (None, _) => Ordering::Greater,
                    (_, None) => Ordering::Less,
                    (Some(av), Some(bv)) => {
                        let ord = av.cmp(&bv);
                        if asc {
                            ord
                        } else {
                            ord.reverse()
                        }
                    }
                },
                "registered" => {
                    let ord = a.spool.registered.cmp(&b.spool.registered);
                    if asc {
                        ord
                    } else {
                        ord.reverse()
                    }
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
        items
            .into_iter()
            .skip(start)
            .take(ts.page_size.get())
            .collect::<Vec<_>>()
    };

    view! {
        <div class="page spool-list">
            <div class="page-header">
                <h1>"Spools"</h1>
                <div class="page-actions">
                    <div class="search-input-wrapper">
                        <input type="text" placeholder="Search…"
                            prop:value=move || ts.filter.get()
                            on:input=move |ev| ts.filter.set(event_target_value(&ev)) />
                        {move || (!ts.filter.get().is_empty()).then(|| view! {
                            <button type="button" class="search-clear"
                                on:click=move |_| ts.filter.set(String::new())
                            >"×"</button>
                        })}
                    </div>
                    <label>
                        <input type="checkbox"
                            on:change=move |ev| show_archived.set(event_target_checked(&ev))
                        />
                        " Show archived"
                    </label>
                    <a href="/spools/new" class="btn btn-primary ">"+ New Spool"</a>
                </div>
            </div>
            <Suspense fallback=|| view! { <p>"Loading…"</p> }>
                <table class="data-table">
                    <thead>
                        <tr>
                            <ColHeader label="ID"       field="id"         sort_field=ts.sort_field sort_asc=ts.sort_asc num=true />
                            <ColHeader label="Filament" field="filament"   sort_field=ts.sort_field sort_asc=ts.sort_asc />
                            <th class="color-head">
                                <span class="color-head-label" role="button" tabindex="0"
                                    on:click=move |_| popup_open.update(|v| *v = !*v)
                                    on:keydown=move |ev| {
                                        let key = ev.key();
                                        if key == "Enter" || key == " " {
                                            popup_open.update(|v| *v = !*v);
                                        }
                                    }
                                >
                                    {move || if color_level.get() != "off" {
                                        let color = color_pick.get();
                                        view! {
                                            "Color "
                                            <span style=format!("color:{color}")>"\u{25A0}"</span>
                                        }.into_view()
                                    } else {
                                        view! { "Color" }.into_view()
                                    }}
                                </span>
                                <select class="color-threshold-select"
                                    prop:value=move || color_level.get()
                                    on:click=move |ev| ev.stop_propagation()
                                    on:change=move |ev| {
                                        ev.stop_propagation();
                                        color_level.set(event_target_value(&ev));
                                    }
                                >
                                    <option value="off">"Off"</option>
                                    <option value="fine">"Fine"</option>
                                    <option value="medium">"Medium"</option>
                                    <option value="coarse">"Coarse"</option>
                                </select>
                                {move || popup_open.get().then(|| view! {
                                    <div class="color-backdrop"
                                        on:click=move |_| popup_open.set(false)
                                    ></div>
                                    <div class="color-popup">
                                        <input type="color"
                                            title="Filter by color"
                                            prop:value=move || color_pick.get()
                                            on:input=move |ev| color_pick.set(event_target_value(&ev))
                                            on:change=move |ev| color_pick.set(event_target_value(&ev))
                                        />
                                        <button type="button" class="btn"
                                            on:click=move |ev| {
                                                ev.stop_propagation();
                                                color_level.set("off".to_string());
                                                popup_open.set(false);
                                            }
                                        >"×"</button>
                                    </div>
                                })}
                            </th>
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
                            let rem = sr.remaining_filament.map(format::format_weight).unwrap_or_default();
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
                                    <td class="num">{rem}</td>
                                    <td>{sr.spool.location_id.map(|l| l.to_string()).unwrap_or_default()}</td>
                                    <td>{format::format_date(sr.spool.registered)}</td>
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
    let locations = create_resource(|| (), |_| async { api::list_locations().await });
    let navigate = use_navigate();
    let confirm_delete = create_rw_signal(false);

    // store_value gives Copy semantics so these handlers can be captured
    // by the reactive `move ||` closure inside view! without making it FnOnce.
    let nav_err = store_value(navigate.clone());
    let nav1 = navigate.clone();
    let navigate_clone = navigate;
    let on_delete = store_value(move |_: web_sys::MouseEvent| {
        let id = id();
        let nav = nav1.clone();
        confirm_delete.set(false);
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
                    {move || if confirm_delete.get() {
                        view! {
                            <span class="confirm-prompt">"Sure?"</span>
                            " "
                            <button on:click=move |e| on_delete.with_value(|f| f(e)) class="btn btn-danger ">"Yes, delete"</button>
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
            <Suspense fallback=|| view! { <p>"Loading…"</p> }>
                {move || spool.get().map(|r| match r {
                    Err(e) => {
                        if e.status == 404 {
                            nav_err.with_value(|f| f("/spools", Default::default()));
                            view! { <></> }.into_view()
                        } else {
                            view! { <p class="error">{e.to_string()}</p> }.into_view()
                        }
                    }
                    Ok(sr) => view! {
                        <dl class="detail-grid">
                            <dt>"Filament"</dt><dd>{sr.filament.display_name()}</dd>
                            <dt>"Location"</dt><dd>{
                                move || match sr.spool.location_id {
                                    None => "—".to_string(),
                                    Some(loc_id) => locations.get()
                                        .and_then(|r| r.ok())
                                        .and_then(|ls| ls.into_iter().find(|l| l.location.id == loc_id))
                                        .map(|l| l.location.name)
                                        .unwrap_or_else(|| loc_id.to_string()),
                                }
                            }</dd>
                            <dt>"Colors"</dt><dd>{sr.spool.colors.iter().map(|c| {
                                let hex = format!("#{:02x}{:02x}{:02x}", c.r, c.g, c.b);
                                view! {
                                    <span class="color-swatch"
                                        style=format!("background:rgba({},{},{},{})",
                                            c.r, c.g, c.b, c.a as f32/255.0)>
                                    </span>
                                    <span class="color-hex">{hex}</span>
                                }
                            }).collect_view()}</dd>
                            <dt>"Color name"</dt><dd>{sr.spool.color_name.clone().unwrap_or_default()}</dd>
                            <dt>"Initial weight"</dt><dd>{format::format_weight(sr.spool.initial_weight)}</dd>
                            <dt>"Current weight"</dt><dd>{format::format_weight(sr.spool.current_weight)}</dd>
                            <dt>"Used"</dt><dd>{format::format_weight(sr.used_weight)}</dd>
                            <dt>"Remaining filament"</dt><dd>{sr.remaining_filament.map(format::format_weight).unwrap_or_else(|| "unknown".into())}</dd>
                            <dt>"Registered"</dt><dd>{format::format_date(sr.spool.registered)}</dd>
                            <dt>"First used"</dt><dd>{sr.spool.first_used.map(format::format_date).unwrap_or_default()}</dd>
                            <dt>"Last used"</dt><dd>{sr.spool.last_used.map(format::format_date).unwrap_or_default()}</dd>
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
    let color_alpha = create_rw_signal(255u8);
    let color_name = create_rw_signal(String::new());
    let initial_weight = create_rw_signal(String::new());
    let net_weight = create_rw_signal(String::new());
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
                colors: hex_to_rgba(&color_hex.get())
                    .map(|mut c| {
                        c.a = color_alpha.get();
                        vec![c]
                    })
                    .unwrap_or_default(),
                color_name: Some(color_name.get()).filter(|s| !s.is_empty()),
                location_id: location_id.get(),
                initial_weight: weight,
                net_weight: net_weight.get().parse().ok(),
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
                    <span class="color-alpha-row">
                        <input type="color"
                            prop:value=move || color_hex.get()
                            on:input=move |ev| color_hex.set(event_target_value(&ev)) />
                        <input type="range" min="0" max="255" title="Opacity"
                            prop:value=move || color_alpha.get().to_string()
                            on:input=move |ev| color_alpha.set(event_target_value(&ev).parse().unwrap_or(255)) />
                        <span class="alpha-pct">{move || format!("{}%", (color_alpha.get() as u16 * 100 / 255))}</span>
                    </span>
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
                    "Net weight (g)"
                    <input type="number" step="1"
                        on:input=move |ev| net_weight.set(event_target_value(&ev)) />
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
    let net_weight = create_rw_signal(String::new());
    let color_hex = create_rw_signal(String::from("#000000"));
    let color_alpha = create_rw_signal(255u8);
    let color_name = create_rw_signal(String::new());
    let location_id = create_rw_signal(Option::<u32>::None);
    let first_used = create_rw_signal(String::new());
    let last_used = create_rw_signal(String::new());
    let comment = create_rw_signal(String::new());
    let error = create_rw_signal(Option::<String>::None);

    // Pre-fill once loaded.
    create_effect(move |_| {
        if let Some(Ok(sr)) = spool.get() {
            current_weight.set(sr.spool.current_weight.to_string());
            net_weight.set(
                sr.spool
                    .net_weight
                    .map(|w| w.to_string())
                    .unwrap_or_default(),
            );
            if let Some(c) = sr.spool.colors.first() {
                color_hex.set(format!("#{:02x}{:02x}{:02x}", c.r, c.g, c.b));
                color_alpha.set(c.a);
            }
            color_name.set(sr.spool.color_name.clone().unwrap_or_default());
            location_id.set(sr.spool.location_id);
            first_used.set(
                sr.spool
                    .first_used
                    .map(|d| d.format("%Y-%m-%d").to_string())
                    .unwrap_or_default(),
            );
            last_used.set(
                sr.spool
                    .last_used
                    .map(|d| d.format("%Y-%m-%d").to_string())
                    .unwrap_or_default(),
            );
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
            let parse_dt = |s: String| -> Option<DateTime<Utc>> {
                if s.is_empty() {
                    return None;
                }
                NaiveDate::parse_from_str(&s, "%Y-%m-%d")
                    .ok()
                    .and_then(|d| d.and_hms_opt(0, 5, 0))
                    .map(|ndt| ndt.and_utc())
            };
            let body = UpdateSpool {
                current_weight: current_weight.get().parse::<f32>().ok(),
                net_weight: net_weight.get().parse::<f32>().ok(),
                colors: Some(
                    hex_to_rgba(&color_hex.get())
                        .map(|mut c| {
                            c.a = color_alpha.get();
                            vec![c]
                        })
                        .unwrap_or_default(),
                ),
                color_name: Some(color_name.get()),
                location_id: location_id.get(),
                first_used: parse_dt(first_used.get()),
                last_used: parse_dt(last_used.get()),
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
                    "Net weight (g)"
                    <input type="number" step="1"
                        prop:value=move || net_weight.get()
                        on:input=move |ev| net_weight.set(event_target_value(&ev)) />
                </label>
                <label>
                    "Color"
                    <span class="color-alpha-row">
                        <input type="color"
                            prop:value=move || color_hex.get()
                            on:input=move |ev| color_hex.set(event_target_value(&ev)) />
                        <input type="range" min="0" max="255" title="Opacity"
                            prop:value=move || color_alpha.get().to_string()
                            on:input=move |ev| color_alpha.set(event_target_value(&ev).parse().unwrap_or(255)) />
                        <span class="alpha-pct">{move || format!("{}%", (color_alpha.get() as u16 * 100 / 255))}</span>
                    </span>
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
                    "First used"
                    <input type="date"
                        prop:value=move || first_used.get()
                        on:input=move |ev| first_used.set(event_target_value(&ev)) />
                </label>
                <label>
                    "Last used"
                    <input type="date"
                        prop:value=move || last_used.get()
                        on:input=move |ev| last_used.set(event_target_value(&ev)) />
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
