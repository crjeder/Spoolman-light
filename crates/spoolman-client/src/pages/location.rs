use leptos::*;
use spoolman_types::requests::{CreateLocation, UpdateLocation};

use crate::api;

#[component]
pub fn LocationList() -> impl IntoView {
    let locations = create_resource(|| (), |_| async { api::list_locations().await });
    let locations_refetch = locations;

    // Inline create form state.
    let new_name = create_rw_signal(String::new());
    let create_error = create_rw_signal(Option::<String>::None);

    // Edit state: (id, name).
    let editing = create_rw_signal(Option::<(u32, String)>::None);
    let edit_error = create_rw_signal(Option::<String>::None);

    let on_create = move |ev: web_sys::SubmitEvent| {
        ev.prevent_default();
        spawn_local(async move {
            let body = CreateLocation { name: new_name.get() };
            match api::create_location(&body).await {
                Ok(_) => {
                    new_name.set(String::new());
                    locations_refetch.refetch();
                }
                Err(e) => create_error.set(Some(e.to_string())),
            }
        });
    };

    let on_save_edit = move |_| {
        if let Some((id, name)) = editing.get() {
            spawn_local(async move {
                let body = UpdateLocation { name };
                match api::update_location(id, &body).await {
                    Ok(_) => {
                        editing.set(None);
                        locations_refetch.refetch();
                    }
                    Err(e) => edit_error.set(Some(e.to_string())),
                }
            });
        }
    };

    let on_delete = move |id: u32| {
        spawn_local(async move {
            if api::delete_location(id).await.is_ok() {
                locations_refetch.refetch();
            }
        });
    };

    view! {
        <div class="page location-list">
            <h1>"Locations"</h1>

            // Create form
            <form class="inline-create" on:submit=on_create>
                {move || create_error.get().map(|e| view! { <p class="error">{e}</p> })}
                <input type="text" placeholder="New location name"
                    prop:value=move || new_name.get()
                    on:input=move |ev| new_name.set(event_target_value(&ev)) />
                <button type="submit" class="btn btn-primary">"Add"</button>
            </form>

            <Suspense fallback=|| view! { <p>"Loading…"</p> }>
                <table class="data-table">
                    <thead>
                        <tr>
                            <th>"Name"</th>
                            <th>"Spools"</th>
                            <th>"Actions"</th>
                        </tr>
                    </thead>
                    <tbody>
                        {move || locations.get().map(|r| match r {
                            Err(e) => view! { <tr><td colspan="3" class="error">{e.to_string()}</td></tr> }.into_view(),
                            Ok(ls) => ls.into_iter().map(|loc| {
                                let id = loc.location.id;
                                let name = loc.location.name.clone();
                                let name2 = name.clone(); // for second reactive closure
                                let count = loc.spool_count;
                                let is_editing = move || editing.get().map(|(eid, _)| eid) == Some(id);
                                view! {
                                    <tr>
                                        <td>
                                            {move || if is_editing() {
                                                view! {
                                                    <input type="text"
                                                        prop:value=move || editing.get().map(|(_, n)| n).unwrap_or_default()
                                                        on:input=move |ev| {
                                                            editing.update(|e| {
                                                                if let Some((i, _)) = e {
                                                                    *e = Some((*i, event_target_value(&ev)));
                                                                }
                                                            });
                                                        }
                                                    />
                                                }.into_view()
                                            } else {
                                                view! { <span>{name.clone()}</span> }.into_view()
                                            }}
                                        </td>
                                        <td>{count}</td>
                                        <td class="actions">
                                            {move || if is_editing() {
                                                view! {
                                                    <button class="btn" on:click=on_save_edit>"Save"</button>
                                                    " "
                                                    <button class="btn" on:click=move |_| editing.set(None)>"Cancel"</button>
                                                }.into_view()
                                            } else {
                                                let n = name2.clone();
                                                view! {
                                                    <button class="btn" on:click=move |_| editing.set(Some((id, n.clone())))>"Edit"</button>
                                                    " "
                                                    <button class="btn btn-danger"
                                                        disabled=move || count > 0
                                                        on:click=move |_| on_delete(id)>"Delete"</button>
                                                }.into_view()
                                            }}
                                        </td>
                                    </tr>
                                }
                            }).collect_view().into_view(),
                        })}
                    </tbody>
                </table>
            </Suspense>
        </div>
    }
}
