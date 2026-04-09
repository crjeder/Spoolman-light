## 1. Filament list row actions

- [x] 1.1 Replace "Edit" text button with `✏` icon button (`btn btn-icon`, `title="Edit"`) in filament list rows (`pages/filament.rs`)
- [x] 1.2 Replace "Delete" trigger button with `🗑` icon button (`btn btn-icon btn-danger`, `title="Delete"`) in filament list rows
- [x] 1.3 Replace "Sure?" confirm button with `🗑` icon button (`btn btn-icon btn-danger`, `title="Confirm delete"`)
- [x] 1.4 Replace "Cancel" dismiss button with `✕` icon button (`btn btn-icon`, `title="Cancel"`) in filament delete confirmation

## 2. Location list row actions

- [x] 2.1 Replace "Edit" text button with `✏` icon button (`btn btn-icon`, `title="Edit"`) in location list rows (`pages/location.rs`)
- [x] 2.2 Replace "Save" inline-edit button with `💾` icon button (`btn btn-icon`, `title="Save changes"`)
- [x] 2.3 Replace "Cancel" inline-edit button with `✕` icon button (`btn btn-icon`, `title="Cancel"`)
- [x] 2.4 Replace "Delete" trigger button with `🗑` icon button (`btn btn-icon btn-danger`, `title="Delete"`)
- [x] 2.5 Replace "Sure?" confirm button with `🗑` icon button (`btn btn-icon btn-danger`, `title="Confirm delete"`)
- [x] 2.6 Replace "Cancel" dismiss button in delete confirmation with `✕` icon button (`btn btn-icon`, `title="Cancel"`)

## 3. Spool detail view actions

- [x] 3.1 Replace "Edit" text button/link with `✏` icon button (`btn btn-icon`, `title="Edit"`) in spool detail view (`pages/spool.rs`)
- [x] 3.2 Replace "Clone" text button with `⧉` icon button (`btn btn-icon`, `title="Clone"`)
- [x] 3.3 Replace "Delete" trigger button with `🗑` icon button (`btn btn-icon btn-danger`, `title="Delete"`)
- [x] 3.4 Replace "Yes, delete" confirm button with `🗑` icon button (`btn btn-icon btn-danger`, `title="Confirm delete"`)
- [x] 3.5 Replace "Cancel" dismiss button in delete confirmation with `✕` icon button (`btn btn-icon`, `title="Cancel"`)

## 4. Pagination

- [x] 4.1 Replace "← Prev" text with `‹` icon (`btn btn-icon`, `title="Previous page"`) in the pagination component (`components/pagination.rs`)
- [x] 4.2 Replace "Next →" text with `›` icon (`btn btn-icon`, `title="Next page"`) in the pagination component

## 5. Verification

- [x] 5.1 Run `cargo check -p spoolman-client --target wasm32-unknown-unknown` (or equivalent) to confirm no compile errors
- [ ] 5.2 Visually verify filament list: all row action buttons show icons only, tooltips appear on hover, delete confirmation flow works
- [ ] 5.3 Visually verify location list: edit/save/cancel/delete icon buttons work, inline edit flow is intact
- [ ] 5.4 Visually verify spool detail view: edit/clone/delete icon buttons work, delete confirmation flow works
- [ ] 5.5 Visually verify pagination: ‹ / › icons shown, prev/next navigation works
