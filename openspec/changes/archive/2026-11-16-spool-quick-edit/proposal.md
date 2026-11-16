## Why

The spool edit form shows every field, but day-to-day use only touches current weight, location, last used and comment. A reduced form is faster to use. Separately, saving the form currently bumps `last_used` even when the weight did not change, which records use that did not happen.

## What Changes

- The spool edit page opens in a quick mode showing only current weight, location, last used and comment.
- All other fields (initial/net weight, price, colors, color name, finish, first used, swatch lookup) move into a collapsed "Advanced" section on the same form.
- **BEHAVIOR CHANGE**: `last_used` is refreshed to today only when the current weight was actually changed. Saving without touching the weight leaves `last_used` as it was. A manually chosen `last_used` is still honoured.
- Server: the automatic `last_used`/`first_used` stamp on PATCH applies only when `current_weight` differs from the stored value, not whenever the field is present.

## Capabilities

### New Capabilities

### Modified Capabilities
- `spool-management`: "Edit spool" requirement changes (quick/advanced edit form, last-used refresh only on weight change).

## Impact

- `crates/spoolman-client/src/pages/spool.rs` (`SpoolEdit`)
- `crates/spoolman-server/src/store.rs` (`update_spool` stamping condition)
- Server tests for PATCH, e2e tests touching the edit form
- Supersedes the "any save counts as an edit" decision of the archived `spool-last-used-on-edit` change.
