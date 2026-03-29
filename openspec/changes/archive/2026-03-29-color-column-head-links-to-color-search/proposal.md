## Why

The color filter (color picker) lives in the page header, making it hard to discover. Clicking the "Color" column header should activate the color search — surfacing the filter where users naturally look for column-specific controls.

## What Changes

- The "Color" `<th>` in the spool table becomes a clickable element that activates/focuses the color picker filter in the page header.
- Clicking the header scrolls or focuses the color picker input so the user can immediately pick a color.
- No new route, API, or data model changes.

## Capabilities

### New Capabilities

- `color-column-head-activates-filter`: Clicking the "Color" column header activates the color picker filter (scrolls to it and/or focuses the input).

### Modified Capabilities

<!-- No existing spec-level requirements change. The color filter behavior itself is unchanged; only the trigger point is added. -->

## Impact

- `crates/spoolman-client/src/pages/spool.rs`: add click handler on the `<th>"Color"</th>` that focuses/activates the color picker input
- CSS may need a minor tweak to show the column header as interactive (cursor, hover style)
- No backend, API, or data model changes
