## Context

`FilamentColorsSearch` filters the cached swatch list client-side with a whitespace-token match against manufacturer + color name and takes the first 10 hits in API order. It has no knowledge of the spool being edited. On Spool Create the filament is chosen via `filament_id` (a `u32` into `filaments_list`), possibly set automatically by the SpoolmanDB search; on Spool Edit the filament comes from the loaded `SpoolResponse`.

## Goals / Non-Goals

**Goals:**
- Swatches from the selected filament's manufacturer appear before all other matches.
- Ranking reacts when the user changes the selected filament.

**Non-Goals:**
- Changing which swatches match (token filter stays as is).
- Fuzzy/alias manufacturer mapping (e.g. "Prusa" vs "Prusament" beyond substring).
- Showing manufacturer swatches for an empty query.
- Detecting a dedicated "Generic" manufacturer as a separate middle tier.

## Decisions

- **Prop shape:** `manufacturer: Signal<Option<String>>` (`#[prop(into, optional)]`), so callers pass a closure-derived signal and the default is "no preference". Alternative - a plain `Option<String>` - would not react to filament changes on Spool Create.
- **Ranking:** collect all token matches, then stable-sort by a boolean key "is manufacturer match" (matches first), then `take(10)`. A stable sort keeps API order within each tier and is one line; the swatch list is ~2.3k entries, so sorting the filtered subset is negligible.
- **Manufacturer match:** both names lowercased and trimmed; a swatch matches when either name contains the other. This handles common variants like `Polymaker` vs `Polymaker PolyLite` without an alias table. Empty manufacturer means no preference.
- **Create page source:** `move || filaments_list.get().iter().find(|f| f.id == filament_id.get()).and_then(|f| f.manufacturer.clone())`. Edit page: from `spool.get()`'s `sr.filament.manufacturer`.

## Risks / Trade-offs

- [Brand names differ between Spoolman filament and filamentcolors.xyz (e.g. abbreviations)] -> substring match covers most cases; mismatch degrades to today's ordering, never hides results.
- [Very short manufacturer names (e.g. "3D") could substring-match unrelated brands] -> only affects ordering, not filtering; acceptable.

## Open Questions

- If "generics" was meant as a specific filamentcolors.xyz manufacturer (e.g. a "Generic" brand) that should rank as a second tier ahead of other brands, add that tier later; this change treats all non-matching manufacturers as one fallback tier.
