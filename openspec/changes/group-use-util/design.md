## Context

The `util` module was introduced precisely to avoid repeating the `reqwest::get → .json()` chain across every model file. `group.rs` was either written before `util.rs` existed or was overlooked during a refactor. The divergence means that if the shared HTTP logic ever needs to change (e.g., adding a User-Agent header, retry logic, or switching from `Box<dyn Error>` to a typed error), `group.rs` will be left behind.

## Goals / Non-Goals

**Goals:**
- Make `group.rs` consistent with every other model module
- Ensure future changes to the HTTP layer only need to be made in one place

**Non-Goals:**
- Changing `util.rs` itself (covered by `lib-util-improvements`)
- Adding error type changes (covered by `lib-custom-error-type`)

## Decisions

**Direct drop-in replacement**: The signatures of `util::get` and `util::list` match exactly what `group.rs` currently does inline. No intermediate refactoring is needed.

## Risks / Trade-offs

- None. The only observable behaviour change would be if `util.rs` itself is changed in the same commit; this change should be applied before or alongside `lib-util-improvements`.

## Open Questions

- None.
