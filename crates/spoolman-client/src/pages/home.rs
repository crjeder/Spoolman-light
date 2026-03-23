use leptos::*;
use crate::api;

#[component]
pub fn HomePage() -> impl IntoView {
    let spools = create_resource(|| (), |_| async { api::list_spools(false).await });
    let filaments = create_resource(|| (), |_| async { api::list_filaments(None).await });

    view! {
        <div class="page home-page">
            <h1>"Dashboard"</h1>
            <div class="stats-grid">
                <Suspense fallback=|| view! { <p>"Loading…"</p> }>
                    {move || spools.get().map(|r| match r {
                        Ok(s) => {
                            let total = s.len();
                            let recently_used: Vec<_> = {
                                let mut v = s.clone();
                                v.sort_by(|a, b| b.spool.last_used.cmp(&a.spool.last_used));
                                v.into_iter().take(5).collect()
                            };
                            view! {
                                <div class="stat-card">
                                    <h2>"Total Spools"</h2>
                                    <span class="stat-value">{total}</span>
                                </div>
                                <div class="recent-spools">
                                    <h2>"Recently Used"</h2>
                                    <ul>
                                        {recently_used.into_iter().map(|sr| {
                                            let name = sr.filament.display_name();
                                            let pct = sr.remaining_pct
                                                .map(|p| format!("{:.0}%", p))
                                                .unwrap_or_else(|| "?".into());
                                            view! {
                                                <li>
                                                    <a href=format!("/spools/{}", sr.spool.id)>
                                                        {name}" — "{pct}" remaining"
                                                    </a>
                                                </li>
                                            }
                                        }).collect_view()}
                                    </ul>
                                </div>
                            }.into_view()
                        }
                        Err(e) => view! { <p class="error">{e.to_string()}</p> }.into_view(),
                    })}
                </Suspense>
                <Suspense fallback=|| view! {}>
                    {move || filaments.get().map(|r| match r {
                        Ok(f) => view! {
                            <div class="stat-card">
                                <h2>"Total Filaments"</h2>
                                <span class="stat-value">{f.len()}</span>
                            </div>
                        }.into_view(),
                        Err(_) => view! {}.into_view(),
                    })}
                </Suspense>
            </div>
        </div>
    }
}
