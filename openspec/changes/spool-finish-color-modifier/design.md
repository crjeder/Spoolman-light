## Context

Color search in the spool list (`spool.rs`) filters spools by perceptual distance between the stored `Spool.colors[]` and a user-picked hex target. The current pipeline treats all spool colors as identical in surface finish. In practice, matte filaments scatter light and appear desaturated and brighter, while gloss filaments appear more saturated and slightly darker. A user searching for "that matte blue they printed with" against a stored RGB value measured from a gloss reference will see inflated distances and miss matches.

Stored colors are treated as ground truth (cf. filamentcolors.xyz). Finish modifiers predict effective appearance: the system transforms stored colors to predicted appearance, then compares against what the user sees.

`Filament.material_modifier` carries free-text product descriptions ("Silk CF", "95A") and is not parsed for finish — it remains unchanged and coexists.

Transparency (translucency) is already represented by `Rgba.a` and is deferred.

## Goals / Non-Goals

**Goals:**
- Add `SurfaceFinish` enum (`Matte | Standard | Gloss`) to the shared types crate
- Store finish on `Spool` with `Standard` as serde default (no migration required)
- Apply HSV modifiers to spool color before distance computation in color search
- Expose finish in add/edit form, spool table, and detail view

**Non-Goals:**
- Parsing finish from `material_modifier` free text
- Translucency modifier (deferred until alpha entry is added to the UI)
- Server-side filtering by finish
- Changing `material_modifier` semantics

## Decisions

### Finish on Spool, not Filament

Surface finish is a property of how a specific spool prints and looks, not of the filament formulation. Two spools of the same filament can have different finishes (manufacturer variation, different batches). Color lives on `Spool`; finish lives alongside it.

### HSV modifier, not Lab-space shift

The modifiers are applied in HSV space before converting to RGB → Lab for distance calculation. Alternatives:
- **Direct Lab shift**: would require per-algorithm tuning and doesn't map naturally to the S/V semantics of "more/less saturated, lighter/darker".
- **Multiply in linear RGB**: perceptually uneven; HSV more closely matches how finish affects perceived hue, saturation, and brightness independently.

HSV approach is simple, intuitive, and algorithm-agnostic — the same modifier feeds all three distance algorithms (CIEDE2000, OKLab, DIN99d).

**Modifier values:**
```
Matte:    S×0.85, V×1.10   (scatter → desaturate, lift brightness)
Standard: S×1.00, V×1.00   (identity)
Gloss:    S×1.15, V×0.95   (concentrate → saturate, slightly darken)
```

V is clamped to [0, 1] after multiplication. High-V colors (near white) naturally hit the ceiling — this is physically correct (a white translucent spool can't get brighter).

### `serde(default)` for backward compatibility

`Spool.finish` uses `#[serde(default)]` with `Standard` as the `Default` impl. Existing JSON data files are read without error and yield `Standard` finish — no migration, no schema version bump needed.

### Finish-aware distance is client-only

The comparison happens in the Leptos client during filter evaluation. The server stores and returns `finish` as a plain field. No server query parameter or filtering logic is added.

## Risks / Trade-offs

- **Modifier values are empirical, not derived** — S×0.85/V×1.10 etc. are reasonable approximations but not calibrated against real measurements. → Acceptable for V1; values can be tuned later without API or schema changes (client-only constants).
- **V clamp loses information for bright colors** — A very bright gloss color gets V×0.95, which works, but a very bright matte color's V×1.10 clamps to 1.0, effectively reducing the matte effect. → Physically correct behaviour; document as known limitation.
- **User must set finish manually** — No auto-detection from `material_modifier`. Users with large existing inventories will see no change (Standard is the safe default). → Acceptable; finish is opt-in signal for better search accuracy.

## Migration Plan

None required. `#[serde(default)]` on `Spool.finish` means:
- Existing `spoolman.json` files without a `finish` key deserialize as `Standard`
- New writes include `"finish": "standard"` in the JSON
- Rollback: old code ignores unknown fields (Serde default) — a downgrade drops `finish` silently

## Open Questions

- Should the spool table show finish as a column or only as a badge in the color cell? (Suggest: inline badge next to color swatch — low visual cost, high contextual value)
