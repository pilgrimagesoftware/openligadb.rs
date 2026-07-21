## Context

The `#[serde(rename)]` attribute is necessary when the Rust field name differs from the JSON key (e.g., `pub id` from `"goalID"`). When they are identical it is pure noise. The presence of unnecessary renames can confuse future contributors into thinking the names must differ and should not be changed.

## Goals / Non-Goals

**Goals:**
- Remove attributes that have no effect on deserialization
- Reduce cognitive load when reading the struct definitions

**Non-Goals:**
- Auditing all serde attributes across the codebase for correctness (different scope)

## Decisions

**Remove outright**: Since the rename is a no-op, removal is a zero-risk change. If in doubt, a test with a JSON fixture will confirm round-trip behaviour is unchanged.

## Risks / Trade-offs

- None. The deserialized output is bit-for-bit identical before and after.

## Open Questions

- None.
