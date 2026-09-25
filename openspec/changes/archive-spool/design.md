## Context

`Spool.archived` and `UpdateSpool.archived` already exist; the server handles `PATCH`. Only the Leptos UI is missing. List page (`SpoolList`) already uses a `confirm_delete: RwSignal<Option<u32>>` two-step pattern; `SpoolShow` uses `confirm_delete: RwSignal<bool>`.

## Goals / Non-Goals

**Goals:** archive/unarchive from list and detail; warn when the spool is not empty.
**Non-Goals:** backend changes, bulk archive, warning based on `remaining_filament` or tare-adjusted weight.

## Decisions

- **Mirror the delete-confirm pattern**: add `confirm_archive` signal per page; button click archives directly if the spool is empty or already archived, else sets `confirm_archive`. Confirm state shows the warning (title/tooltip on list row, inline text on detail page) plus Confirm and Cancel icon buttons. Chosen over `window.confirm` for consistency with existing UI and testability in Playwright.
- **Not-empty rule**: `SpoolResponse.remaining_filament` (net_weight - used_weight) is the tare-free filament amount, so a spool is not empty when `remaining_filament > Some(0.0)`. When `net_weight` is unknown (`remaining_filament == None`), fall back to `spool.current_weight > 0.0`. This avoids false warnings for physically empty spools whose scale reading still includes the tare. Implemented as one helper `is_empty(&SpoolResponse) -> bool` used by both pages.
- **Icon**: box/archive char `\u{1F4E6}` for Archive, `\u{21A9}` for Unarchive, per icon-only convention.
- **Refresh**: list page refetches its resource after update; detail page refetches `spool` resource.
- **Layout**: extend the fixed-width reservation in the actions cell (see recent fix c67cf38) for the extra button.

## Risks / Trade-offs

- Extra button widens actions column slightly.
- Fallback branch (no net_weight) can still warn for a tare-only spool; acceptable and dismissible.
