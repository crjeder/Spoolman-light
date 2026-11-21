## Context

`SpoolEdit` renders one long form and always submits all fields. `last_used` is currently set to today whenever the field is untouched (`last_used == last_used_loaded`), regardless of weight. Server `update_spool` stamps `last_used`/`first_used` whenever `req.current_weight.is_some()`, and the form always sends it, so the stamp fires on every save via the API too.

## Goals / Non-Goals

**Goals:**
- Quick mode by default: current weight, location, last used, comment.
- Rest of the fields reachable under "Advanced" without leaving the page.
- `last_used` only auto-updates when current weight changed.

**Non-Goals:**
- Roles or auth ("admin/super user"): the app has no user model. "Advanced" is a UI disclosure, not a permission.
- Persisting the advanced-open state across visits.
- Changing PATCH field semantics or the create form.

## Decisions

- **Single form, `<details class="advanced">` for the rest.** All signals and the submit handler stay as they are; hidden fields keep their loaded values and are submitted unchanged. Alternative rejected: separate quick-edit route/page, which duplicates load/save logic.
- **Client: track `current_weight_loaded`.** On submit, `last_used` sent is: the user's value if they changed the date field; else today if `current_weight != current_weight_loaded`; else the loaded value, or omitted (`None`) if the spool had no `last_used`. This replaces the "untouched means now" rule from `spool-last-used-on-edit`.
- **Initial-weight edit** (spool with `initial_weight == 0`) also sets current weight in the form, so it counts as a weight change; acceptable.
- **Server: stamp only on real change.** `weight_changed = req.current_weight.is_some_and(|w| w != spool.current_weight)`, compared before assignment. Keeps API clients that PATCH an unchanged weight from bumping `last_used`/`first_used`. Explicit `req.last_used` still wins.
- Required `location_id` validation is unchanged and applies in quick mode (location is visible there).

## Risks / Trade-offs

- Users editing only a comment no longer bump `last_used` -> intended.
- Browser validation errors on a field inside a closed "Advanced" section would be invisible -> advanced inputs are plain number/date/text pre-filled with valid loaded values, so risk is low.
