use chrono::{DateTime, NaiveDate, Utc};
use leptos::prelude::*;
use leptos::task::spawn_local;
use leptos_router::hooks::{use_navigate, use_params_map};
use wasm_bindgen::JsCast;
use spoolman_types::{
    models::{Rgba, SurfaceFinish},
    requests::{CreateSpool, UpdateSpool},
};

use crate::{
    api,
    components::{pagination::Pagination, spoolmandb_search::SpoolmanDbSearch, swatch_grid::SwatchGrid, table::ColHeader},
    format,
    spoolmandb::parse_material,
    state::{color_distance_algorithm, color_thresholds, currency_symbol, date_format_setting, time_format_setting, use_pinned_spools, use_table_state, ViewMode},
    utils::color::{apply_finish_modifier, color_distance, hex_to_rgba, hue_sort_key},
};
use spoolman_types::requests::CreateFilament;

// ── Color rows ─────────────────────────────────────────────────────────────────

/// One editable spool color: a stable id (for `<For>` keying), a `#rrggbb`
/// hex signal, and an opacity signal.
type ColorRow = (u32, RwSignal<String>, RwSignal<u8>);

fn finish_label(f: SurfaceFinish) -> &'static str {
    match f {
        SurfaceFinish::Matte => "Matte",
        SurfaceFinish::Standard => "Standard",
        SurfaceFinish::Gloss => "Gloss",
        SurfaceFinish::Silk => "Silk",
    }
}

fn finish_from_value(v: &str) -> SurfaceFinish {
    match v {
        "matte" => SurfaceFinish::Matte,
        "gloss" => SurfaceFinish::Gloss,
        "silk" => SurfaceFinish::Silk,
        _ => SurfaceFinish::Standard,
    }
}

fn finish_value(f: SurfaceFinish) -> &'static str {
    match f {
        SurfaceFinish::Matte => "matte",
        SurfaceFinish::Standard => "standard",
        SurfaceFinish::Gloss => "gloss",
        SurfaceFinish::Silk => "silk",
    }
}

/// Finish `<select>` bound to `finish`. Shared by the add and edit spool forms.
fn finish_select(finish: RwSignal<SurfaceFinish>) -> impl IntoView {
    view! {
        <label>
            "Finish"
            <select
                prop:value=move || finish_value(finish.get())
                on:change=move |ev| finish.set(finish_from_value(&event_target_value(&ev)))
            >
                <option value="matte">"Matte"</option>
                <option value="standard">"Standard"</option>
                <option value="gloss">"Gloss"</option>
                <option value="silk">"Silk"</option>
            </select>
        </label>
    }
}

const MAX_COLORS: usize = 4;

/// Stable per-row id, distinct from array position (which shifts on remove).
/// `<For>` needs this: keying by index would tear down and recreate every
/// row's signals on each add/remove, disposing them mid-render.
fn next_row_id() -> u32 {
    use std::sync::atomic::{AtomicU32, Ordering};
    static NEXT: AtomicU32 = AtomicU32::new(0);
    NEXT.fetch_add(1, Ordering::Relaxed)
}

fn new_color_row() -> ColorRow {
    (next_row_id(), RwSignal::new(String::from("#000000")), RwSignal::new(255u8))
}

/// Collect all rows into the `colors` array, applying alpha and dropping rows
/// whose hex fails to parse. Order is preserved top-to-bottom.
fn rows_to_colors(rows: RwSignal<Vec<ColorRow>>) -> Vec<Rgba> {
    rows.get()
        .into_iter()
        .filter_map(|(_, hex, alpha)| {
            hex_to_rgba(&hex.get()).map(|mut c| {
                c.a = alpha.get();
                c
            })
        })
        .collect()
}

/// Render the multi-color editor: one `.color-alpha-row` per color, a "−" button
/// on every row past the first, and a "+" button on the last row until 4 rows exist.
fn color_rows_editor(rows: RwSignal<Vec<ColorRow>>) -> impl IntoView {
    // New rows' signals must outlive the "+" button that creates them.
    let owner = Owner::current().expect("editor has an owner");
    view! {
        <div class="color-rows">
            <For each=move || rows.get() key=|row| row.0 let:row>
                {
                    let (id, hex, alpha) = row;
                    let owner = owner.clone();
                    let is_first = move || rows.with(|v| v.first().map(|r| r.0) == Some(id));
                    let show_add = move || {
                        rows.with(|v| v.len() < MAX_COLORS && v.last().map(|r| r.0) == Some(id))
                    };
                    view! {
                        <span class="color-alpha-row">
                            <input type="color"
                                prop:value=move || hex.get()
                                on:input=move |ev| hex.set(event_target_value(&ev)) />
                            <input type="text" class="color-hex-input" maxlength="7" placeholder="#rrggbb"
                                prop:value=move || hex.get()
                                on:input=move |ev| {
                                    let v = event_target_value(&ev);
                                    if hex_to_rgba(&v).is_some() { hex.set(v); }
                                } />
                            <input type="range" min="0" max="255" title="Opacity"
                                prop:value=move || alpha.get().to_string()
                                on:input=move |ev| alpha.set(event_target_value(&ev).parse().unwrap_or(255)) />
                            <span class="alpha-pct">{move || format!("{}%", (alpha.get() as u16 * 100 / 255))}</span>
                            {move || (!is_first()).then(|| view! {
                                <button type="button" class="btn-color-row" title="Remove color"
                                    on:click=move |_| rows.update(|v| v.retain(|r| r.0 != id))>"−"</button>
                            })}
                            {move || show_add().then(|| view! {
                                <button type="button" class="btn-color-row" title="Add color"
                                    on:click={
                                        let owner = owner.clone();
                                        move |_| { let row = owner.with(new_color_row); rows.update(|v| v.push(row)) }
                                    }>"+"</button>
                            })}
                        </span>
                    }
                }
            </For>
        </div>
    }
}

// ── List ───────────────────────────────────────────────────────────────────────

#[component]
pub fn SpoolList(mode: ViewMode) -> impl IntoView {
    let ts = match mode {
        ViewMode::Spool => use_table_state("spools", "registered"),
        ViewMode::Color => use_table_state("colors", "hue"),
    };
    // Filters persist for the tab session via sessionStorage (see state::session_get).
    let sg = |key: &str, default: &str| {
        crate::state::session_get(&format!("filter.spools.{key}"))
            .unwrap_or_else(|| default.to_string())
    };
    let show_archived = RwSignal::new(sg("show_archived", "false") == "true");
    let color_pick = RwSignal::new(sg("color_pick", "#000000"));
    let color_level = RwSignal::new(sg("color_level", "off"));
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

    // None means the user never touched this tab's material filter, so it
    // should default to "all materials selected" once the list is known.
    let material_pref = crate::state::session_get("filter.spools.material");
    let material_filter: RwSignal<Vec<String>> = RwSignal::new(
        material_pref
            .as_deref()
            .unwrap_or("")
            .split(',')
            .filter(|s| !s.is_empty())
            .map(String::from)
            .collect(),
    );
    let location_filter: RwSignal<Option<u32>> =
        RwSignal::new(sg("location", "").parse::<u32>().ok());
    let pinned = use_pinned_spools();
    let pinned_only = RwSignal::new(false);

    // Persist each filter back to sessionStorage on change (default value = cleared).
    Effect::new(move |_| crate::state::session_set("filter.spools.show_archived",
        if show_archived.get() { "true" } else { "false" }));
    Effect::new(move |_| crate::state::session_set("filter.spools.color_pick", &color_pick.get()));
    Effect::new(move |_| crate::state::session_set("filter.spools.color_level", &color_level.get()));
    Effect::new(move |_| crate::state::session_set("filter.spools.material", &material_filter.get().join(",")));
    Effect::new(move |_| crate::state::session_set("filter.spools.location",
        &location_filter.get().map(|id| id.to_string()).unwrap_or_default()));

    let version = RwSignal::new(0u32);
    let confirm_delete: RwSignal<Option<u32>> = RwSignal::new(None);

    let locations = LocalResource::new(|| async { api::list_locations().await });

    let spools = LocalResource::new(move || {
        let archived = show_archived.get();
        // Location filter is a spool-table-only control; the swatch view has
        // no UI to see or change it, so it must not silently restrict the grid.
        let loc_id = if mode == ViewMode::Color { None } else { location_filter.get() };
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

    // The user never touched this tab's material filter — once the material
    // list is known, default the filter to "all selected" so the checkboxes
    // start checked (functionally equivalent to the prior empty = "no filter").
    if material_pref.is_none() {
        Effect::new(move |_| {
            let mats = available_materials.get();
            if !mats.is_empty() && material_filter.get_untracked().is_empty() {
                material_filter.set(mats);
            }
        });
    }

    let filters_active = move || {
        !ts.filter.get().is_empty()
            || material_filter.get().len() != available_materials.get().len()
            || (mode == ViewMode::Spool && location_filter.get().is_some())
            || (mode == ViewMode::Spool && color_level.get() != "off")
            || (mode == ViewMode::Color && pinned_only.get())
            || show_archived.get()
    };
    let clear_filters = move |_| {
        ts.filter.set(String::new());
        material_filter.set(available_materials.get_untracked());
        location_filter.set(None);
        color_level.set("off".to_string());
        pinned_only.set(false);
        show_archived.set(false);
    };

    let on_delete = move |id: u32| {
        spawn_local(async move {
            if api::delete_spool(id).await.is_ok() {
                version.update(|v| *v += 1);
                confirm_delete.set(None);
            }
        });
    };

    // The color-distance filter is a spool-table-only control with no UI in
    // the swatch view, so it must not silently restrict or re-sort the grid.
    let effective_color_level = move || if mode == ViewMode::Color { "off".to_string() } else { color_level.get() };

    let filtered = move || {
        let f = ts.filter.get().to_lowercase();
        let pick = color_pick.get();
        let level = effective_color_level();
        let mat = material_filter.get();
        let loc_names: Vec<(u32, String)> = locations
            .get()
            .and_then(|r| r.ok())
            .map(|ls| {
                ls.into_iter()
                    .map(|lr| (lr.location.id, lr.location.name.to_lowercase()))
                    .collect()
            })
            .unwrap_or_default();
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
                        .contains(&f)
                    || s.spool.location_id.is_some_and(|lid| {
                        loc_names
                            .iter()
                            .any(|(id, name)| *id == lid && name.contains(&f))
                    });
                let material_ok = mat.is_empty()
                    || s.filament
                        .material
                        .as_ref()
                        .map(|m| mat.iter().any(|sel| sel == m.abbreviation()))
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
                                .any(|c| color_distance(&apply_finish_modifier(c, s.spool.finish), &target, cda.0.get()) <= thresh)
                        }
                        None => true, // invalid hex — don't filter
                    }
                };
                let pin_ok = mode != ViewMode::Color || !pinned_only.get() || pinned.get().contains(&s.spool.id);
                text_ok && material_ok && color_ok && pin_ok
            })
            .collect::<Vec<_>>()
    };

    let sort_field = ts.sort_field;
    let sort_asc = ts.sort_asc;
    let sorted = move || {
        let mut items = filtered();
        let level = effective_color_level();
        let pick = color_pick.get();

        // When a color level is active and the hex is valid, sort by ascending
        // minimum ΔE*00 distance (closest match first).
        if level != "off" {
            if let Some(target) = hex_to_rgba(&pick) {
                let min_delta = |s: &spoolman_types::responses::SpoolResponse| {
                    s.spool
                        .colors
                        .iter()
                        .map(|c| color_distance(&apply_finish_modifier(c, s.spool.finish), &target, cda.0.get()))
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
                "hue" => {
                    let ord = hue_sort_key(&a.spool.colors)
                        .partial_cmp(&hue_sort_key(&b.spool.colors))
                        .unwrap_or(Ordering::Equal);
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

    // Colors view: endless scroll instead of pagination. `colors_visible`
    // grows as the user nears the bottom of the page; it resets whenever a
    // filter narrows/widens the result set.
    let colors_visible = RwSignal::new(ts.page_size.get_untracked());
    Effect::new(move |_| {
        let _ = (
            ts.filter.get(),
            material_filter.get(),
            pinned_only.get(),
            show_archived.get(),
            ts.sort_field.get(),
            ts.sort_asc.get(),
        );
        colors_visible.set(ts.page_size.get_untracked());
    });
    let colors_items = move || sorted().into_iter().take(colors_visible.get()).collect::<Vec<_>>();

    // The app shell scrolls `.main-content`, not `window` (`.app-shell` is
    // `height: 100vh; overflow: hidden`), so the listener must live on that
    // element rather than on `window`.
    // Effect (not inline): the component body runs while building `<main>`,
    // before it is attached to the DOM, so the selector would find nothing.
    Effect::new(move |_| {
        if mode != ViewMode::Color {
            return;
        }
        if let Some(main) = web_sys::window()
            .and_then(|w| w.document())
            .and_then(|d| d.query_selector(".main-content").ok().flatten())
        {
            let closure = wasm_bindgen::closure::Closure::<dyn Fn()>::new(move || {
                if colors_visible.get_untracked() >= total.get_untracked() {
                    return;
                }
                let near_bottom = web_sys::window()
                    .and_then(|w| w.document())
                    .and_then(|d| d.query_selector(".main-content").ok().flatten())
                    .is_some_and(|el: web_sys::Element| {
                        el.scroll_top() as f64 + el.client_height() as f64 >= el.scroll_height() as f64 - 300.0
                    });
                if near_bottom {
                    colors_visible.update(|v| *v += ts.page_size.get_untracked());
                }
            });
            let _ = main.add_event_listener_with_callback("scroll", closure.as_ref().unchecked_ref());
            // ponytail: leaked on route switches (Closure isn't Send/Sync, so
            // it can't go through Leptos's on_cleanup); upgrade to a proper
            // teardown if this page starts remounting often enough to matter.
            closure.forget();
        }
    });

    view! {
        <div class="page spool-list">
            <div class="page-header">
                <h1>{if mode == ViewMode::Color { "Colors" } else { "Spools" }}</h1>
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
                            prop:checked=move || show_archived.get()
                            on:change=move |ev| show_archived.set(event_target_checked(&ev))
                        />
                        " Show archived"
                    </label>
                    {move || filters_active().then(|| view! {
                        <button type="button" class="btn" on:click=clear_filters>"Clear filters"</button>
                    })}
                    {move || (mode == ViewMode::Color && !pinned.get().is_empty()).then(|| {
                        let palette: Vec<_> = spools.get().and_then(|r| r.ok()).unwrap_or_default()
                            .into_iter()
                            .filter(|sr| pinned.get().contains(&sr.spool.id))
                            .map(|sr| serde_json::json!({
                                "name": sr.spool.color_name,
                                "colors": sr.spool.colors.iter().map(|c| format!("#{:02x}{:02x}{:02x}", c.r, c.g, c.b)).collect::<Vec<_>>(),
                            }))
                            .collect();
                        let json = serde_json::to_string_pretty(&palette).unwrap_or_default();
                        let href = format!("data:application/json;charset=utf-8,{}", js_sys::encode_uri_component(&json));
                        view! {
                            <a class="btn" href=href download="palette.json">"Export palette"</a>
                        }
                    })}
                    <a href="/spools/new" class="btn btn-primary ">"+ New Spool"</a>
                </div>
            </div>
            <Suspense fallback=|| view! { <p>"Loading…"</p> }>
                {if mode == ViewMode::Color {
                    let colors_items_signal = Signal::derive(colors_items);
                    view! {
                        <SwatchGrid
                            items=colors_items_signal
                            locations=locations
                            material_filter=material_filter
                            available_materials=available_materials
                            pinned=pinned
                            pinned_only=pinned_only
                        />
                    }.into_any()
                } else { view! {
                <table class="data-table">
                    <thead>
                        <tr>
                            <ColHeader label="Filament" field="filament"   sort_field=ts.sort_field sort_asc=ts.sort_asc />
                            <th class="material-head">
                                "Material"
                                <select class="material-filter-select"
                                    prop:value=move || {
                                        let cur = material_filter.get();
                                        if cur.len() == 1 { cur[0].clone() } else { String::new() }
                                    }
                                    on:change=move |ev| {
                                        let v = event_target_value(&ev);
                                        material_filter.set(if v.is_empty() { Vec::new() } else { vec![v] });
                                    }
                                >
                                    <option value="">"All"</option>
                                    {move || {
                                        let cur = material_filter.get();
                                        available_materials.get().into_iter().map(|m| {
                                            let sel = cur.len() == 1 && cur[0] == m;
                                            let m2 = m.clone();
                                            view! { <option value=m selected=sel>{m2}</option> }
                                        }).collect_view()
                                    }}
                                    {move || {
                                        let cur = material_filter.get();
                                        (cur.len() > 1).then(|| {
                                            let label = format!("Multiple ({})", cur.len());
                                            view! { <option value="" selected=true>{label}</option> }
                                        })
                                    }}
                                </select>
                            </th>
                            <th class=move || {
                                let active = ts.sort_field.get() == "hue";
                                if active { "col-header active color-head" } else { "col-header color-head" }
                            }>
                                <button class="sort-btn" on:click=move |_| {
                                    if ts.sort_field.get() == "hue" {
                                        ts.sort_asc.update(|a| *a = !*a);
                                    } else {
                                        ts.sort_field.set("hue".to_string());
                                        ts.sort_asc.set(true);
                                    }
                                }>
                                    "Color"
                                    {move || if ts.sort_field.get() == "hue" {
                                        if ts.sort_asc.get() { " ↑" } else { " ↓" }
                                    } else { "" }}
                                </button>
                                {move || if color_level.get() != "off" {
                                    let color = color_pick.get();
                                    view! {
                                        <span class="color-head-label" role="button" tabindex="0"
                                            style=format!("color:{color}")
                                            on:click=move |ev| { ev.stop_propagation(); popup_open.update(|v| *v = !*v); }
                                        >"\u{25A0}"</span>
                                    }.into_any()
                                } else {
                                    view! {
                                        <button type="button" class="btn-icon color-head-label" title="Filter by color"
                                            on:click=move |ev| { ev.stop_propagation(); popup_open.update(|v| *v = !*v); }
                                        >"\u{1F3A8}"</button>
                                    }.into_any()
                                }}
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
                                    <td><a href=format!("/spools/{id}")>{name}</a></td>
                                    <td>{material}</td>
                                    <td>
                                        {colors.into_iter().map(|c| view! {
                                            <span class="color-swatch"
                                                style=format!("background:rgba({},{},{},{})",
                                                    c.r, c.g, c.b, c.a as f32/255.0)>
                                            </span>
                                        }).collect_view()}
                                        {sr.spool.color_name.clone().unwrap_or_default()}
                                        {match sr.spool.finish {
                                            SurfaceFinish::Standard => None,
                                            f => Some(view! { <span class="finish-badge">{finish_label(f)}</span> }),
                                        }}
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
                }.into_any() }}
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
                            <dt>"Finish"</dt><dd>{finish_label(sr.spool.finish)}</dd>
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
    let color_rows = RwSignal::new(vec![new_color_row()]);
    let color_name = RwSignal::new(String::new());
    let finish = RwSignal::new(SurfaceFinish::default());
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
    // (query, has_results) from the SpoolmanDB search panel.
    let search_state: RwSignal<(String, bool)> = RwSignal::new((String::new(), false));
    // Disable the filament selector only when the query matched neither the
    // SpoolmanDB nor any locally known filament.
    let filament_disabled = move || {
        let (q, has) = search_state.get();
        if q.is_empty() || has {
            return false;
        }
        let ql = q.to_lowercase();
        !filaments_list
            .get()
            .iter()
            .any(|f| f.display_name().to_lowercase().contains(&ql))
    };
    // No SpoolmanDB match: pre-fill color_name from the search text and, if a
    // locally known filament matches the query, select it.
    Effect::new(move |_| {
        let (q, has) = search_state.get();
        if !q.is_empty() && !has {
            let ql = q.to_lowercase();
            if let Some(f) = filaments_list
                .get()
                .iter()
                .find(|f| f.display_name().to_lowercase().contains(&ql))
            {
                filament_id.set(f.id);
            }
            color_name.set(q);
        }
    });

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
            color_rows.with_untracked(|v| v[0].1.set(format!("#{hex}")));
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
                colors: rows_to_colors(color_rows),
                color_name: Some(color_name.get()).filter(|s| !s.is_empty()),
                finish: Some(finish.get()),
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
            <SpoolmanDbSearch
                on_select=on_db_select
                on_search_state=Callback::new(move |state| search_state.set(state))
            />
            <form on:submit=on_submit>
                <label>
                    "Filament"
                    <Suspense fallback=|| view! { <select><option>"Loading…"</option></select> }>
                        <select
                            prop:value=move || filament_id.get().to_string()
                            disabled=move || filament_disabled()
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
                    {move || filament_disabled().then(|| view! {
                        <p class="field-hint field-hint-warn">
                            "Not found in database -- filament selection disabled"
                        </p>
                    })}
                </label>
                <label>
                    "Color"
                    {color_rows_editor(color_rows)}
                </label>
                <label>
                    "Color name"
                    <input type="text" prop:value=move || color_name.get() on:input=move |ev| color_name.set(event_target_value(&ev)) />
                </label>
                {finish_select(finish)}
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
                <button type="submit" class="btn btn-primary " disabled=move || location_id.get().is_none()>"Create"</button>
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
    let color_rows = RwSignal::new(vec![new_color_row()]);
    let color_name = RwSignal::new(String::new());
    let finish = RwSignal::new(SurfaceFinish::default());
    let location_id = RwSignal::new(Option::<u32>::None);
    let first_used = RwSignal::new(String::new());
    let last_used = RwSignal::new(String::new());
    // Track whether the loaded spool had no date, so an untouched "today" default is not persisted.
    let first_used_was_none = RwSignal::new(false);
    let last_used_was_none = RwSignal::new(false);
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
            let rows: Vec<ColorRow> = sr
                .spool
                .colors
                .iter()
                .map(|c| {
                    (
                        next_row_id(),
                        RwSignal::new(format!("#{:02x}{:02x}{:02x}", c.r, c.g, c.b)),
                        RwSignal::new(c.a),
                    )
                })
                .collect();
            color_rows.set(if rows.is_empty() { vec![new_color_row()] } else { rows });
            color_name.set(sr.spool.color_name.clone().unwrap_or_default());
            finish.set(sr.spool.finish);
            location_id.set(sr.spool.location_id);
            let today = || Utc::now().format("%Y-%m-%d").to_string();
            first_used_was_none.set(sr.spool.first_used.is_none());
            last_used_was_none.set(sr.spool.last_used.is_none());
            first_used.set(
                sr.spool
                    .first_used
                    .map(|d| d.format("%Y-%m-%d").to_string())
                    .unwrap_or_else(today),
            );
            last_used.set(
                sr.spool
                    .last_used
                    .map(|d| d.format("%Y-%m-%d").to_string())
                    .unwrap_or_else(today),
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
            // Drop the auto-filled "today" default if the spool had no date and the user left it untouched.
            let prune_default = |s: String, was_none: bool| -> String {
                if was_none && s == Utc::now().format("%Y-%m-%d").to_string() {
                    String::new()
                } else {
                    s
                }
            };
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
                colors: Some(rows_to_colors(color_rows)),
                color_name: Some(color_name.get()),
                finish: Some(finish.get()),
                location_id: location_id.get(),
                first_used: parse_dt(prune_default(first_used.get(), first_used_was_none.get())),
                last_used: parse_dt(prune_default(last_used.get(), last_used_was_none.get())),
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
                    {color_rows_editor(color_rows)}
                </label>
                <label>
                    "Color name"
                    <input type="text"
                        prop:value=move || color_name.get()
                        on:input=move |ev| color_name.set(event_target_value(&ev)) />
                </label>
                {finish_select(finish)}
                <label>
                    "Location"
                    <Suspense fallback=|| view! { <select /> }>
                        <select
                            prop:value=move || location_id.get().map(|id| id.to_string()).unwrap_or_default()
                            on:change=move |ev| {
                                location_id.set(event_target_value(&ev).parse::<u32>().ok());
                            }
                        >
                            <option value="">"— none —"</option>
                            {move || locations.get().and_then(|r| r.ok()).map(|ls| {
                                let cur = location_id.get();
                                ls.into_iter().map(|l| {
                                    let lid = l.location.id;
                                    view! {
                                        <option value=lid.to_string() selected=cur == Some(lid)>{l.location.name}</option>
                                    }
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
                <button type="submit" class="btn btn-primary " disabled=move || location_id.get().is_none()>"Save"</button>
                <a href=move || format!("/spools/{}", id()) class="btn ">"Cancel"</a>
            </form>
        </div>
    }
}
