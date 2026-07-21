## Context

The `async-trait` crate provides a macro that rewrites `async fn` in traits into boxed futures, which was necessary before Rust's native async-in-traits stabilisation. The `openligadb.rs` library does not define any async traits and does not use the `#[async_trait]` attribute anywhere.

## Goals / Non-Goals

**Goals:**
- Remove the dead dependency to keep the dependency graph minimal

**Non-Goals:**
- Adding async trait abstractions (out of scope for this change)

## Decisions

**Remove outright**: Since there is zero usage, removal is safe with no migration required.

## Risks / Trade-offs

- None. The dependency is provably unused.

## Open Questions

- None.
