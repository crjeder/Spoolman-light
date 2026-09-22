## Context

The spool create and edit forms already enforce that a location must be selected before submission. This change formalizes that behavior as a spec. The backend stores `location_id` as an optional field — the constraint is purely frontend validation.

## Goals / Non-Goals

**Goals:**
- Document the existing frontend validation behavior as a formal spec requirement.
- Ensure the spec correctly describes the current UI behavior so future changes don't regress it.

**Non-Goals:**
- Making `location_id` required on the server/API level.
- Changing any existing behavior — the implementation is already complete.

## Decisions

**Frontend-only enforcement.**
Location is optional in the data model (`Option<u32>`) and the REST API accepts spools without a location. The requirement is a UX constraint: a spool without a location is hard to find physically. Enforcing this in the UI is sufficient.

**Disable submit rather than show inline error.**
The form submit button is disabled when no location is selected. This is simpler than a separate validation error message and prevents accidental submission.

## Risks / Trade-offs

- None — implementation already exists and is stable. This is a documentation-only change.

## Open Questions

- None.
