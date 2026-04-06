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
            </section>
            <section>
                <h2>"Tracking weight"</h2>
                <p>"Weigh your spool (filament + plastic spool together) on a kitchen scale. "
                   "Enter the total reading as the current weight. "
                   "Spoolman calculates how much filament has been used and how much remains."</p>
            </section>
            <section>
                <h2>"NFC tags (OpenTag3D / OpenPrintTag)"</h2>
                <p>"Write your spool URL to an NFC tag so you can scan it directly from the printer. "
                   "The spool detail URL is "<code>"/api/v1/spool/<id>"</code>
                   " — use this as the Online Data URL field in your NFC writer."</p>
            </section>
            <section>
                <h2>"Data file"</h2>
                <p>"All data is stored in "<code>"spoolman.json"</code>" in the platform data directory. "
                   "The path is shown at "
                   <a href="/api/v1/info">"/api/v1/info"</a>"."</p>
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
