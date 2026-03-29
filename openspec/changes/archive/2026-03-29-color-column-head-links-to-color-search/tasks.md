## 1. Wire up NodeRef on color picker input

- [x] 1.1 Add a `NodeRef<Input>` signal (`color_input_ref`) in `SpoolList`
- [x] 1.2 Attach `node_ref=color_input_ref` to the `<input type="color">` element in the page header

## 2. Make "Color" column header interactive

- [x] 2.1 Replace `<th>"Color"</th>` with `<th class="color-head" role="button" tabindex="0" on:click=...>` that calls `color_input_ref.get().map(|el| el.focus())`
- [x] 2.2 Add `on:keydown` handler to the `<th>` so Enter/Space also triggers focus (keyboard accessibility)

## 3. CSS

- [x] 3.1 Add `th.color-head { cursor: pointer; }` and a hover style (e.g., `text-decoration: underline` or background tint) to [style.css](crates/spoolman-client/style/style.css) or equivalent stylesheet
