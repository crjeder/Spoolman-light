## Why

The color search level thresholds (Same / Close / Ballpark) are hardcoded per algorithm in `threshold_for()`. Different filament collections and use cases need different tolerances — a user searching for an exact match replacement may want a tighter "Same" than someone browsing for inspiration — and the fixed values are an opinionated guess that doesn't fit everyone.

## What Changes

- Add six user-configurable threshold values to the settings store: one per (level × algorithm) group, using the current hardcoded values as defaults.
- Expose threshold editing on the Settings page, grouped by active algorithm.
- The `threshold_for()` function reads from persisted settings instead of returning constants.

## Capabilities

### New Capabilities

- `color-threshold-settings`: User can view and edit the numeric threshold for each color search level (Same, Close, Ballpark) on the Settings page; values are persisted per algorithm and applied immediately to the color filter.

### Modified Capabilities

- `color-distance-algorithm`: The per-algorithm threshold table is now user-configurable rather than hardcoded; the spec requirement changes from "thresholds SHALL be [table]" to "thresholds SHALL default to [table] and SHALL be overridable via settings."

## Impact

- **Settings store** — six new keys, e.g. `color_threshold_ciede2000_same`, `color_threshold_oklab_close`, etc. (or a single structured key).
- **`spoolman-client`** — `threshold_for()` replaced by a reactive lookup from settings signals; Settings page gains a threshold editor section.
- **`spoolman-types`** — possible new request/response types for threshold settings if structured as a single object key.
- **No API or data-model breaking changes** — thresholds are client-side display preferences stored as existing settings key/value pairs.
