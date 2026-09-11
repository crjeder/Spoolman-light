## 1. Extend SpoolmanDbSearch component

- [x] 1.1 Add `on_search_state: Option<Callback<(String, bool)>>` prop to `SpoolmanDbSearch` in `spoolmandb_search.rs`
- [x] 1.2 Add an `Effect` (or reactive closure) that fires whenever `query` or `results` changes and calls `on_search_state` with `(query, !results.is_empty())`
- [x] 1.3 Verify existing callers (filament.rs) compile without change -- the new prop is optional and defaults to `None`

## 2. Wire up state in Spool Create

- [x] 2.1 Add a `search_state: RwSignal<(String, bool)>` signal in the `SpoolCreate` component in `spool.rs`
- [x] 2.2 Pass `on_search_state=Callback::new(move |(q, r)| search_state.set((q, r)))` to `<SpoolmanDbSearch>`
- [x] 2.3 Derive a `filament_disabled` memo: `move || { let (q, has) = search_state.get(); !q.is_empty() && !has }`

## 3. Disable filament dropdown on no results

- [x] 3.1 Bind `attr:disabled=filament_disabled` to the filament `<select>` element
- [x] 3.2 Add a conditional label/paragraph below the select that reads "Not found in database -- filament selection disabled", shown only when `filament_disabled` is true

## 4. Pre-fill spool name from search text

- [x] 4.1 Add a `name: RwSignal<String>` signal (if not already present) to hold the spool name field value
- [x] 4.2 Add an `Effect` that watches `search_state`: when `filament_disabled` transitions to true, call `name.set(query.clone())` -- do NOT clear name when it transitions back to false
- [x] 4.3 Bind the spool name `<input>` to use `prop:value=move || name.get()` and `on:input=move |ev| name.set(event_target_value(&ev))`
- [x] 4.4 Ensure the `CreateSpoolBody` on submit uses `name.get()` for the name field

## 5. Verification

- [x] 5.1 `cargo check -p spoolman-client` (using wasm32 target) compiles without errors
- [x] 5.2 Manual test: type a query with no DB results -- filament dropdown disables, name pre-fills
- [x] 5.3 Manual test: clear the query -- filament dropdown re-enables, name field retains value
- [x] 5.4 Manual test: type a query with results -- normal flow unchanged, filament dropdown active
- [x] 5.5 Manual test: select a DB result -- auto-fill and auto-create behavior unchanged
