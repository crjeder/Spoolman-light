## 1. Data Model

- [x] 1.1 Add `price: Option<f32>` field to the `Spool` struct in `crates/spoolman-types/src/models.rs`, annotated with `#[serde(default)]`
- [x] 1.2 Add `price: Option<f32>` field to `CreateSpool` in `crates/spoolman-types/src/requests.rs`
- [x] 1.3 Add `price: Option<f32>` field to `UpdateSpool` in `crates/spoolman-types/src/requests.rs`

## 2. Server Response

- [x] 2.1 Add `price_per_gram: Option<f32>` field to `SpoolResponse` in `crates/spoolman-types/src/responses.rs`
- [x] 2.2 Derive `price_per_gram` in `SpoolResponse::new`: `price / net_weight` when `net_weight` is set, else `price / initial_weight`; `None` when `price` is `None`

## 3. Server Handler

- [x] 3.1 Apply `price` from `CreateSpool` request when constructing a new `Spool` in the create handler (`crates/spoolman-server/src/routes/spool.rs` or equivalent)
- [x] 3.2 Apply `price` from `UpdateSpool` request when patching a `Spool` in the update handler

## 4. Client — Form

- [x] 4.1 Add a `price` signal (`create_rw_signal(String::new())`) to both the create and edit spool dialog components in `crates/spoolman-client/src/pages/spool.rs`
- [x] 4.2 In the edit dialog initialisation block, populate the `price` signal from `sr.spool.price` (`.map(|v| v.to_string()).unwrap_or_default()`)
- [x] 4.3 Include `price: price.get().parse::<f32>().ok()` in both the `CreateSpool` and `UpdateSpool` request structs sent from the form
- [x] 4.4 Add a `<label>"Price"</label><input type="number" …>` field to the create dialog view, bound to the `price` signal
- [x] 4.5 Add the same `Price` input field to the edit dialog view

## 5. Client — Table Column

- [x] 5.1 Add `"price_per_gram"` to the default-sortable columns list in `crates/spoolman-client/src/pages/spool.rs`
- [x] 5.2 Add a `"price_per_gram"` match arm to the `sort_by` closure: sort `Option<f32>` with `None` last (same pattern as `remaining_weight`)
- [x] 5.3 Add a `<ColHeader label="Price/g" field="price_per_gram" sort_field=… sort_asc=… num=true />` header to the spool table
- [x] 5.4 Add the corresponding table cell that calls `format_currency(sr.price_per_gram as f64, &currency_symbol)` when `price_per_gram` is `Some`, or renders `"—"` when `None`

## 6. Verification

- [x] 6.1 Run `cargo check -p spoolman-types -p spoolman-server` and confirm zero errors
- [x] 6.2 Manually verify: create a spool with a price, confirm `price_per_gram` appears in the API response
- [ ] 6.3 Manually verify: the `Price/g` column renders correctly in the spool list and is sortable
- [ ] 6.4 Manually verify: existing spools without a price show `—` in the `Price/g` column
