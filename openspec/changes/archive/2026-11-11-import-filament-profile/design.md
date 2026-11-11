## Context

`FilamentCreate` in `crates/spoolman-client/src/pages/filament.rs` already pre-fills its signals from a SpoolmanDB entry (`on_db_select`). The settings page already reads a user-selected file in the browser with `gloo_file` (`pages/settings.rs`). Orca profiles are flat JSON objects whose values are one-element string arrays (e.g. `"filament_density": ["1.28"]`), with `"nil"` for unset values.

## Goals / Non-Goals

**Goals:**
- One button on New Filament that fills the form from an Orca filament profile.
- Pure, natively unit-testable parser.

**Non-Goals:**
- Other slicer formats (PrusaSlicer INI, Cura), bundle files, multi-profile files.
- Import on the Edit form, import of min/max temps, cost, retraction or fan settings.
- Server-side parsing or storing the original profile.

## Decisions

- **Client-side only.** The browser reads the file and fills signals; no API change. Alternative (upload to server) adds an endpoint for no benefit.
- **New module `slicer_profile.rs`** with `parse_orca(&str) -> Result<ProfileData, String>`, where `ProfileData` has `Option` fields (manufacturer, material, modifier, diameter, density, print_temp, bed_temp). Keeps `filament.rs` free of parsing logic and allows `cargo test` on the host target. Parsing goes through `serde_json::Value` (no struct) since Orca profiles have hundreds of optional keys and vary by version.
- **Value access:** take first array element if the value is an array, or the string itself if it is a plain string; treat `""` and `"nil"` as absent.
- **Material:** pass `filament_type` through `MaterialType::from_abbreviation` (as `parse_material` does) so unknown types become the existing custom/other variant.
- **Modifier heuristic:** from `name`: cut at first ` @`, strip a leading vendor name, strip a trailing material abbreviation and trailing `-`/space; empty becomes none. `Polymaker HT-PLA @User` -> `HT`. Heuristic, user can edit.
- **Bed temp:** map `bed_type` ("Cool Plate" -> `cool_plate_temp`, "Engineering Plate" -> `eng_plate_temp`, "High Temp Plate" / "Smooth PEI Plate" -> `hot_plate_temp`, "Textured PEI Plate" -> `textured_plate_temp`); fallback `hot_plate_temp`.
- **Validation:** reject if JSON is not an object or lacks all of `filament_type`, `filament_vendor`, `filament_density`.
- **UI:** a `<button type="button">` next to `SpoolmanDbSearch` triggers a hidden `<input type="file" accept=".json">`; on change, `gloo_file::futures::read_as_text` then apply to signals, mirroring `on_db_select` (diameter skipped when `ds.uniform`). Errors go to the existing `error` signal.

## Risks / Trade-offs

- Modifier heuristic may produce odd values for unusual names -> field is editable and shown before submit.
- Profile exported from "System" presets may lack `filament_vendor`/`filament_density` (they use `inherits`) -> those fields are left unchanged; no inheritance resolution.
- Bed-type names are taken from Orca/Bambu; unknown names fall back to `hot_plate_temp`.
