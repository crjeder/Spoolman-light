## 1. UI — Spool detail view

- [ ] 1.1 In `crates/spoolman-client/src/pages/spool.rs`, update the Colors `<dd>` (around line 408) to append a `<span class="color-hex">` with the hex string after each swatch `<span>`
- [ ] 1.2 Verify the hex string is formatted as `#rrggbb` (lowercase, 6 hex digits) using `format!("#{:02x}{:02x}{:02x}", c.r, c.g, c.b)`

## 2. Styling

- [ ] 2.1 Add a `.color-hex` rule in the stylesheet (small monospace font, vertically aligned with the swatch, slight left margin)

## 3. Verification

- [ ] 3.1 `cargo check -p spoolman-client` compiles without errors
- [ ] 3.2 Build and visually confirm the hex label appears next to each swatch in the spool detail view
