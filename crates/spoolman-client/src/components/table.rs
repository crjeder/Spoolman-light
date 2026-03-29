use leptos::*;

/// Column header with sort indicator and optional text filter.
#[component]
pub fn ColHeader(
    label: &'static str,
    field: &'static str,
    sort_field: RwSignal<String>,
    sort_asc: RwSignal<bool>,
    #[prop(optional)] filter: Option<RwSignal<String>>,
    #[prop(optional)] num: bool,
) -> impl IntoView {
    let is_active = move || sort_field.get() == field;
    let indicator = move || {
        if is_active() {
            if sort_asc.get() {
                " ↑"
            } else {
                " ↓"
            }
        } else {
            ""
        }
    };
    let toggle_sort = move |_| {
        if sort_field.get() == field {
            sort_asc.update(|a| *a = !*a);
        } else {
            sort_field.set(field.to_string());
            sort_asc.set(true);
        }
    };

    view! {
        <th class=move || {
            let mut cls = if is_active() { "col-header active" } else { "col-header" }.to_string();
            if num { cls.push_str(" num"); }
            cls
        }>
            <button class="sort-btn" on:click=toggle_sort>
                {label}{indicator}
            </button>
            {filter.map(|f| view! {
                <input
                    class="col-filter"
                    type="text"
                    placeholder="filter…"
                    prop:value=move || f.get()
                    on:input=move |ev| f.set(event_target_value(&ev))
                />
            })}
        </th>
    }
}
