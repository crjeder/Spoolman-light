## 1. CSS Utility Class

- [x] 1.1 Add `.num { text-align: right }` rule to `style/spoolman.css`

## 2. Spools List View

- [x] 2.1 Add `class="num"` to the `<ColHeader>` for the "ID" column in `pages/spool.rs`
- [x] 2.2 Add `class="num"` to the `<td>` that renders the spool ID
- [x] 2.3 Add `class="num"` to the `<ColHeader>` for "Remaining%" column
- [x] 2.4 Add `class="num"` to the `<td>` that renders the remaining percentage
- [x] 2.5 Add `class="num"` to the `<ColHeader>` for "Remaining (g)" column
- [x] 2.6 Add `class="num"` to the `<td>` that renders the remaining weight

## 3. Filaments List View

- [x] 3.1 Add `class="num"` to the `<th>` for "Diameter" in `pages/filament.rs`
- [x] 3.2 Add `class="num"` to the `<td>` that renders the diameter value
- [x] 3.3 Add `class="num"` to the `<th>` for "Net weight"
- [x] 3.4 Add `class="num"` to the `<td>` that renders the net weight value
- [x] 3.5 Add `class="num"` to the `<ColHeader>` for "Density"
- [x] 3.6 Add `class="num"` to the `<td>` that renders the density value

## 4. Verification

- [x] 4.1 Run `cargo check -p spoolman-types && cargo check -p spoolman-server` to confirm no compilation errors
- [x] 4.2 Use Playwright to open the Spools and Filaments list pages and verify numeric columns are visually right-aligned
