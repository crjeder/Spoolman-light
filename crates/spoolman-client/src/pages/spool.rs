use chrono::{DateTime, NaiveDate, Utc};
use leptos::prelude::*;
use leptos::task::spawn_local;
use leptos_router::hooks::{use_navigate, use_params_map};
use spoolman_types::{
    models::Rgba,
    requests::{CreateSpool, UpdateSpool},
};

use crate::{
    api,
    components::{pagination::Pagination, spoolmandb_search::SpoolmanDbSearch, table::ColHeader},
    format,
    spoolmandb::parse_material,
    state::{color_distance_algorithm, color_thresholds, currency_symbol, date_format_setting, time_format_setting, use_table_state},
    utils::color::{color_distance, hex_to_rgba},
};
use spoolman_types::requests::CreateFilament;

// ── List ───────────────────────────────────────────────────────────────────────

#[component]
pub fn SpoolList() -> impl IntoView {
    let ts = use_table_state("spools");
    let show_archived = RwSignal::new(false);
    let color_pick = RwSignal::new("#000000".to_string());
    let color_level = RwSignal::new("off".to_string());
    let popup_open = RwSignal::new(false);
    let cda = color_distance_algorithm();
    let ct = color_thresholds();
    let cur_sym = currency_symbol();
    let df = date_format_setting();
    let tf = time_format_setting();

    let _visible_cols = RwSignal::new(vec![
        "filament",
        "color",
        "remaining_weight",
        "price_per_kg",
        "location",
        "registered",
    ]);

    let material_filter = RwSignal::new(String::new());
    let location_filter: RwSignal<Option<u32>> = RwSignal::new(None);

    let version = RwSignal::new(0u32);
    let confirm_delete: RwSignal<Option<u32>> = RwSignal::new(None);

    let locations = LocalResource::new(|| async { api::list_locations().await });

    let spools = LocalResource::new(move || {
        let archived = show_archived.get();
        let loc_id = location_filter.get();
        let _ = version.get();
        async move { api::list_spools(archived, loc_id).await }
    });

    let available_materials = Signal::derive(move || {
        let mut mats: Vec<String> = spools
            .get()
            .and_then(|r| r.ok())
            .unwrap_or_default()
            .into_iter()
            .filter_map(|s| s.filament.material.map(|m| m.abbreviation().to_string()))
            .collect();
        mats.sort_unstable();
        mats.dedup();
        mats
    });

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
        let mat = material_filter.get();
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
                let material_ok = mat.is_empty()
                    || s.filament
                        .material
                        .as_ref()
                        .map(|m| m.abbreviation() == mat.as_str())
                        .unwrap_or(false);
                let color_ok = if level == "off" {
                    true
                } else {
                    match hex_to_rgba(&pick) {
                        Some(target) => {
                            let thresh = ct.get(&level, cda.0.get());
                            s.spool
                                .colors
                                .iter()
                                .any(|c| color_distance(c, &target, cda.0.get()) <= thresh)
                        }
                        None => true, // invalid hex — don't filter
                    }
                };
                text_ok && material_ok && color_ok
            })
            .collect::<Vec<_>>()
    };

    let sort_field = ts.sort_field;
    let sort_asc = ts.sort_asc;
    let sorted = move || {
        let mut items = filtered();
        let level = color_level.get();
        let pick = color_pick.get();

        // When a color level is active and the hex is valid, sort by ascending
        // minimum ΔE*00 distance (closest match first).
        if level != "off" {
            if let Some(target) = hex_to_rgba(&pick) {
                let min_delta = |s: &spoolman_types::responses::SpoolResponse| {
                    s.spool
                        .colors
                        .iter()
                        .map(|c| color_distance(c, &target, cda.0.get()))
                        .fold(f32::MAX, f32::min)
                };
                items.sort_by(|a, b| {
                    min_delta(a)
                        .partial_cmp(&min_delta(b))
                        .unwrap_or(std::cmp::Ordering::Equal)
                });
                return items;
            }
        }

        // Default: column-based sort.
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
                "price_per_kg" => match (a.price_per_kg, b.price_per_kg) {
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
                            <ColHeader label="Filament" field="filament"   sort_field=ts.sort_field sort_asc=ts.sort_asc />
                            <th class="material-head">
                                {move || if !material_filter.get().is_empty() {
                                    view! { "Material \u{25A0}" }.into_any()
                                } else {
                                    view! { "Material" }.into_any()
                                }}
                                <select class="material-filter-select"
                                    prop:value=move || material_filter.get()
                                    on:change=move |ev| material_filter.set(event_target_value(&ev))
                                >
                                    <option value="">"All"</option>
                                    {move || available_materials.get().into_iter().map(|m| {
                                        let m2 = m.clone();
                                        view! { <option value=m>{m2}</option> }
                                    }).collect_view()}
                                </select>
                            </th>
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
                                        }.into_any()
                                    } else {
                                        view! { "Color" }.into_any()
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
                                    <option value="same">"Same"</option>
                                    <option value="close">"Close"</option>
                                    <option value="ballpark">"Ballpark"</option>
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
                            <ColHeader label="Price/kg"      field="price_per_kg"      sort_field=ts.sort_field sort_asc=ts.sort_asc num=true />
                            <th class=move || {
                                let active = ts.sort_field.get() == "location";
                                if active { "col-header active location-head" } else { "col-header location-head" }
                            }>
                                <button class="sort-btn" on:click=move |_| {
                                    if ts.sort_field.get() == "location" {
                                        ts.sort_asc.update(|a| *a = !*a);
                                    } else {
                                        ts.sort_field.set("location".to_string());
                                        ts.sort_asc.set(true);
                                    }
                                }>
                                    "Location"
                                    {move || if ts.sort_field.get() == "location" {
                                        if ts.sort_asc.get() { " ↑" } else { " ↓" }
                                    } else { "" }}
                                    {move || if location_filter.get().is_some() { " \u{25A0}" } else { "" }}
                                </button>
                                <Suspense>
                                    <select class="location-filter-select"
                                        prop:value=move || location_filter.get().map(|id| id.to_string()).unwrap_or_default()
                                        on:change=move |ev| {
                                            let v = event_target_value(&ev);
                                            location_filter.set(v.parse::<u32>().ok());
                                        }
                                    >
                                        <option value="">"All"</option>
                                        {move || locations.get().and_then(|r| r.ok()).map(|ls| {
                                            let cur = location_filter.get();
                                            ls.into_iter().map(|lr| {
                                                let id = lr.location.id;
                                                let name = lr.location.name.clone();
                                                view! {
                                                    <option value=id.to_string() selected=cur == Some(id)>{name}</option>
                                                }
                                            }).collect_view()
                                        })}
                                    </select>
                                </Suspense>
                            </th>
                            <ColHeader label="Registered" field="registered" sort_field=ts.sort_field sort_asc=ts.sort_asc />
                            <th>"Actions"</th>
                        </tr>
                    </thead>
                    <tbody>
                        {move || page_items().into_iter().map(|sr| {
                            let id = sr.spool.id;
                            let filament_id = sr.filament.id;
                            let name = sr.filament.display_name();
                            let colors = if sr.spool.colors.is_empty() {
                                vec![Rgba { r: 200, g: 200, b: 200, a: 255 }]
                            } else {
                                sr.spool.colors.clone()
                            };
                            let rem = sr.remaining_filament.map(format::format_weight).unwrap_or_default();
                            let ppg = sr.price_per_kg
                                .map(|p| format::format_currency(p as f64, &cur_sym.0.get()))
                                .unwrap_or_else(|| "—".into());
                            let material = sr.filament.material.as_ref().map(|m| m.abbreviation().to_string()).unwrap_or_default();
                            view! {
                                <tr class=if sr.spool.archived { "archived" } else { "" }>
                                    <td><a href=format!("/filaments/{filament_id}")>{name}</a></td>
                                    <td>{material}</td>
                                    <td>
                                        {colors.into_iter().map(|c| view! {
                                            <span class="color-swatch"
                                                style=format!("background:rgba({},{},{},{})",
                                                    c.r, c.g, c.b, c.a as f32/255.0)>
                                            </span>
                                        }).collect_view()}
                                        {sr.spool.color_name.clone().unwrap_or_default()}
                                    </td>
                                    <td class="num">{rem}</td>
                                    <td class="num">{ppg}</td>
                                    <td>{
                                        move || match sr.spool.location_id {
                                            None => "—".to_string(),
                                            Some(lid) => locations.get()
                                                .and_then(|r| r.ok())
                                                .and_then(|ls| ls.into_iter().find(|lr| lr.location.id == lid))
                                                .map(|lr| lr.location.name.clone())
                                                .unwrap_or_else(|| lid.to_string()),
                                        }
                                    }</td>
                                    <td>{format::format_date(sr.spool.registered, &df.0.get(), &tf.0.get())}</td>
                                    <td class="actions">
                                        <a href=format!("/spools/{id}") class="btn btn-icon" title="View">"\u{1F441}"</a>
                                        " "
                                        <a href=format!("/spools/{id}/edit") class="btn btn-icon" title="Edit">"\u{270F}"</a>
                                        " "
                                        {move || if confirm_delete.get() == Some(id) {
                                            view! {
                                                <button class="btn btn-icon btn-danger"
                                                    on:click=move |_| on_delete(id)
                                                    title="Confirm delete"
                                                >"\u{1F5D1}"</button>
                                                " "
                                                <button class="btn btn-icon"
                                                    on:click=move |_| confirm_delete.set(None)
                                                    title="Cancel"
                                                >"\u{2715}"</button>
                                            }.into_any()
                                        } else {
                                            view! {
                                                <button class="btn btn-icon btn-danger"
                                                    on:click=move |_| confirm_delete.set(Some(id))
                                                    title="Delete"
                                                >"\u{1F5D1}"</button>
                                            }.into_any()
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
    let spool = LocalResource::new(move || { let id = id(); async move { api::get_spool(id).await } });
    let locations = LocalResource::new(|| async { api::list_locations().await });
    let cur_sym = currency_symbol();
    let navigate = use_navigate();
    let confirm_delete = RwSignal::new(false);
    let df = date_format_setting();
    let tf = time_format_setting();

    // store_value gives Copy semantics so these handlers can be captured
    // by the reactive `move ||` closure inside view! without making it FnOnce.
    let nav_err = StoredValue::new(navigate.clone());
    let nav1 = navigate.clone();
    let navigate_clone = navigate;
    let on_delete = StoredValue::new(move |_: web_sys::MouseEvent| {
        let id = id();
        let nav = nav1.clone();
        confirm_delete.set(false);
        spawn_local(async move {
            if api::delete_spool(id).await.is_ok() {
                nav("/spools", Default::default());
            }
        });
    });

    let on_clone = StoredValue::new(move |_: web_sys::MouseEvent| {
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
                    <a href=move || format!("/spools/{}/edit", id()) class="btn btn-icon" title="Edit">"\u{270F}"</a>
                    <button on:click=move |e| on_clone.with_value(|f| f(e)) class="btn btn-icon" title="Clone">"\u{29C9}"</button>
                    {move || if confirm_delete.get() {
                        view! {
                            <button on:click=move |e| on_delete.with_value(|f| f(e)) class="btn btn-icon btn-danger" title="Confirm delete">"\u{1F5D1}"</button>
                            <button on:click=move |_| confirm_delete.set(false) class="btn btn-icon" title="Cancel">"\u{2715}"</button>
                        }.into_any()
                    } else {
                        view! {
                            <button on:click=move |_| confirm_delete.set(true) class="btn btn-icon btn-danger" title="Delete">"\u{1F5D1}"</button>
                        }.into_any()
                    }}
                </div>
            </div>
            <Suspense fallback=|| view! { <p>"Loading…"</p> }>
                {move || spool.get().map(|r| match r {
                    Err(e) => {
                        if e.status == 404 {
                            nav_err.with_value(|f| f("/spools", Default::default()));
                            ().into_any()
                        } else {
                            view! { <p class="error">{e.to_string()}</p> }.into_any()
                        }
                    }
                    Ok(sr) => view! {
                        <dl class="detail-grid">
                            <dt>"Filament"</dt><dd><a href=format!("/filaments/{}", sr.filament.id)>{sr.filament.display_name()}</a></dd>
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
                            <dt>"Net weight"</dt><dd>{sr.spool.net_weight.map(format::format_weight).unwrap_or_else(|| "—".into())}</dd>
                            <dt>"Current weight"</dt><dd>{format::format_weight(sr.spool.current_weight)}</dd>
                            <dt>"Used"</dt><dd>{format::format_weight(sr.used_weight)}</dd>
                            <dt>"Remaining filament"</dt><dd>{sr.remaining_filament.map(format::format_weight).unwrap_or_else(|| "unknown".into())}</dd>
                            <dt>"Price"</dt><dd>{sr.spool.price.map(|p| format::format_currency(p as f64, &cur_sym.0.get())).unwrap_or_else(|| "—".into())}</dd>
                            <dt>"Price/kg"</dt><dd>{sr.price_per_kg.map(|p| format::format_currency(p as f64, &cur_sym.0.get())).unwrap_or_else(|| "—".into())}</dd>
                            <dt>"Registered"</dt><dd>{format::format_date(sr.spool.registered, &df.0.get(), &tf.0.get())}</dd>
                            <dt>"First used"</dt><dd>{sr.spool.first_used.map(|dt| format::format_date(dt, &df.0.get(), &tf.0.get())).unwrap_or_default()}</dd>
                            <dt>"Last used"</dt><dd>{sr.spool.last_used.map(|dt| format::format_date(dt, &df.0.get(), &tf.0.get())).unwrap_or_default()}</dd>
                            <dt>"Comment"</dt><dd>{sr.spool.comment.clone().unwrap_or_default()}</dd>
                            <dt>"Archived"</dt><dd>{if sr.spool.archived { "Yes" } else { "No" }}</dd>
                        </dl>
                    }.into_any(),
                })}
            </Suspense>
        </div>
    }
}

// ── Create ─────────────────────────────────────────────────────────────────────

#[component]
pub fn SpoolCreate() -> impl IntoView {
    let navigate = use_navigate();
    let filaments = LocalResource::new(|| async { api::list_filaments(None).await });
    let locations = LocalResource::new(|| async { api::list_locations().await });

    let filament_id = RwSignal::new(0u32);
    let color_hex = RwSignal::new(String::from("#000000"));
    let color_alpha = RwSignal::new(255u8);
    let color_name = RwSignal::new(String::new());
    let initial_weight = RwSignal::new(String::new());
    let net_weight = RwSignal::new(String::new());
    let price = RwSignal::new(String::new());
    let location_id = RwSignal::new(Option::<u32>::None);
    let comment = RwSignal::new(String::new());
    let error = RwSignal::new(Option::<String>::None);
    // Cache of known filaments — updated when LocalResource resolves and when auto-create adds one.
    let filaments_list: RwSignal<Vec<spoolman_types::models::Filament>> = RwSignal::new(vec![]);
    // Notification shown when a filament is auto-created by the DB lookup.
    let auto_create_msg: RwSignal<Option<String>> = RwSignal::new(None);

    Effect::new(move |_| {
        if let Some(Ok(fs)) = filaments.get() {
            if let Some(first) = fs.first() {
                filament_id.set(first.id);
            }
            filaments_list.set(fs);
        }
    });

    // SpoolmanDB selection: auto-fill color/weight, find or create matching filament.
    let on_db_select = Callback::new(move |entry: crate::spoolmandb::SpoolmanEntry| {
        // Fill color fields.
        if let Some(ref hex) = entry.color_hex {
            color_hex.set(format!("#{hex}"));
        }
        color_name.set(entry.name.clone());
        if let Some(w) = entry.weight {
            net_weight.set(w.to_string());
        }

        // Find a matching filament in our known list.
        let (mat_type, modifier_opt) = parse_material(&entry.material);
        let entry_manufacturer_lc = entry.manufacturer.to_lowercase();
        let entry_diameter = entry.diameter;
        let known = filaments_list.get_untracked();
        let matched = known.into_iter().find(|f| {
            f.manufacturer
                .as_deref()
                .map(|m| m.to_lowercase())
                .as_deref()
                == Some(entry_manufacturer_lc.as_str())
                && f.material.as_ref() == Some(&mat_type)
                && (f.diameter - entry_diameter).abs() < 0.01
        });

        if let Some(f) = matched {
            filament_id.set(f.id);
        } else {
            // Auto-create the filament and notify the user.
            let mfr = entry.manufacturer.clone();
            let mat_abbr = mat_type.abbreviation().to_string();
            spawn_local(async move {
                let body = CreateFilament {
                    manufacturer: Some(mfr.clone()).filter(|s| !s.is_empty()),
                    material: Some(mat_type),
                    material_modifier: modifier_opt.filter(|s| !s.is_empty()),
                    diameter: entry_diameter,
                    density: if entry.density > 0.0 { entry.density } else { 1.24 },
                    print_temp: entry.extruder_temp.map(|t| t as i32),
                    bed_temp: entry.bed_temp.map(|t| t as i32),
                    spool_weight: entry.spool_weight,
                    min_print_temp: None,
                    max_print_temp: None,
                    min_bed_temp: None,
                    max_bed_temp: None,
                    comment: None,
                };
                match api::create_filament(&body).await {
                    Ok(new_f) => {
                        let new_id = new_f.id;
                        filaments_list.update(|list| list.push(new_f));
                        filament_id.set(new_id);
                        auto_create_msg.set(Some(format!(
                            "Filament '{} {}' was created automatically.",
                            mfr, mat_abbr
                        )));
                    }
                    Err(e) => error.set(Some(e.to_string())),
                }
            });
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
                price: price.get().parse::<f32>().ok(),
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
            {move || auto_create_msg.get().map(|msg| view! {
                <div class="info-banner">
                    {msg}
                    <button type="button" class="btn-dismiss"
                        on:click=move |_| auto_create_msg.set(None)
                    >"×"</button>
                </div>
            })}
            <SpoolmanDbSearch on_select=on_db_select />
            <form on:submit=on_submit>
                <label>
                    "Filament"
                    <Suspense fallback=|| view! { <select><option>"Loading…"</option></select> }>
                        <select
                            prop:value=move || filament_id.get().to_string()
                            on:change=move |ev| {
                                filament_id.set(event_target_value(&ev).parse().unwrap_or(0));
                            }
                        >
                            {move || {
                                let fs = filaments_list.get();
                                let selected = filament_id.get();
                                fs.into_iter().map(|f| {
                                    let id_str = f.id.to_string();
                                    let is_selected = f.id == selected;
                                    view! {
                                        <option value=id_str prop:selected=is_selected>{f.display_name()}</option>
                                    }
                                }).collect_view()
                            }}
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
                    <input type="text" prop:value=move || color_name.get() on:input=move |ev| color_name.set(event_target_value(&ev)) />
                </label>
                <label>
                    "Initial weight (g)"
                    <input type="number" step="0.1"
                        on:input=move |ev| initial_weight.set(event_target_value(&ev)) />
                </label>
                <label>
                    "Net weight (g)"
                    <input type="number" step="1"
                        prop:value=move || net_weight.get()
                        on:input=move |ev| net_weight.set(event_target_value(&ev)) />
                </label>
                <label>
                    "Price"
                    <input type="number" step="0.01" min="0"
                        on:input=move |ev| price.set(event_target_value(&ev)) />
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
    let spool = LocalResource::new(move || { let id = id(); async move { api::get_spool(id).await } });
    let locations = LocalResource::new(|| async { api::list_locations().await });
    let navigate = use_navigate();

    let current_weight = RwSignal::new(String::new());
    let net_weight = RwSignal::new(String::new());
    let price = RwSignal::new(String::new());
    let color_hex = RwSignal::new(String::from("#000000"));
    let color_alpha = RwSignal::new(255u8);
    let color_name = RwSignal::new(String::new());
    let location_id = RwSignal::new(Option::<u32>::None);
    let first_used = RwSignal::new(String::new());
    let last_used = RwSignal::new(String::new());
    let comment = RwSignal::new(String::new());
    let error = RwSignal::new(Option::<String>::None);

    // Pre-fill once loaded.
    Effect::new(move |_| {
        if let Some(Ok(sr)) = spool.get() {
            current_weight.set(sr.spool.current_weight.to_string());
            net_weight.set(
                sr.spool
                    .net_weight
                    .map(|w| w.to_string())
                    .unwrap_or_default(),
            );
            price.set(sr.spool.price.map(|v| v.to_string()).unwrap_or_default());
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
                price: price.get().parse::<f32>().ok(),
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
                    "Price"
                    <input type="number" step="0.01" min="0"
                        prop:value=move || price.get()
                        on:input=move |ev| price.set(event_target_value(&ev)) />
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
