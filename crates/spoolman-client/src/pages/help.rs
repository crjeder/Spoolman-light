use leptos::prelude::*;

#[component]
pub fn HelpPage() -> impl IntoView {
    view! {
        <div class="page help-page">
            <h1>"Help"</h1>
            <section>
                <h2>"About Spoolman"</h2>
                <p>"Spoolman is a self-hosted filament spool tracker. "
                   "It stores spool data in a local JSON file — no external database required."</p>
                <p>"Version "<code>{env!("CARGO_PKG_VERSION")}</code></p>
            </section>
            <section>
                <h2>"Tracking weight"</h2>
                <p>"Weigh your spool (filament + plastic spool together) on a kitchen scale. "
                   "Enter the total reading as the current weight. "
                   "Spoolman calculates how much filament has been used and how much remains."</p>
            </section>
            <section>
                <h2>"NFC tags (OpenTag3D / OpenPrintTag)"</h2>
                <p>"Spoolman doesn't write NFC tags itself — use a separate NFC writer app. "
                   "Write your spool's URL, "<code>"<host>/api/v1/spool/<id>"</code>" (no "<code>"https://"</code>" prefix), "
                   "as the Online Data URL field so the tag scans directly to that spool's data."</p>
            </section>
            <section>
                <h2>"Data file"</h2>
                <p>"All data is stored in "<code>"spoolman.json"</code>" in the platform data directory. "
                   "The path is shown at "<code>"/api/v1/info"</code>" (open this URL directly in a new tab — "
                   "it's a backend endpoint, not a page in this app)."</p>
                <p>"There is no in-app data upload/restore. To import or restore data, stop the server, "
                   "replace "<code>"spoolman.json"</code>" at that path, and restart."</p>
            </section>
            <section>
                <h2>"Environment variables"</h2>
                <table class="data-table">
                    <thead><tr><th>"Variable"</th><th>"Default"</th><th>"Purpose"</th></tr></thead>
                    <tbody>
                        <tr><td><code>"SPOOLMAN_DATA_FILE"</code></td><td><em>"platform default"</em></td><td>"Path to JSON data file"</td></tr>
                        <tr><td><code>"SPOOLMAN_HOST"</code></td><td><code>"0.0.0.0"</code></td><td>"Bind address"</td></tr>
                        <tr><td><code>"SPOOLMAN_PORT"</code></td><td><code>"8000"</code></td><td>"Bind port"</td></tr>
                        <tr><td><code>"SPOOLMAN_CORS_ORIGIN"</code></td><td><code>"FALSE"</code></td><td>"Allowed CORS origin (or FALSE to disable)"</td></tr>
                        <tr><td><code>"SPOOLMAN_LOGGING_LEVEL"</code></td><td><code>"info"</code></td><td>"Log level (trace/debug/info/warn/error)"</td></tr>
                        <tr><td><code>"SPOOLMAN_AUTOMATIC_BACKUP"</code></td><td><code>"TRUE"</code></td><td>"Enable daily backup"</td></tr>
                    </tbody>
                </table>
            </section>
        </div>
    }
}
