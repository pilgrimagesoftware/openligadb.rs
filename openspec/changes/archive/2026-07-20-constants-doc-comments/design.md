## Context

Rust uses `//!` for module-level (inner) documentation and `///` for item documentation. C-style `/** */` block comments are valid Rust syntax but are not recognised as doc comments by `rustdoc` — they render as regular comments. The `@paulyhedral` annotation follows a Javadoc convention that has no meaning in a Rust context.

## Goals / Non-Goals

**Goals:**
- Make the module header idiomatic and visible in `rustdoc` output
- Pass `rustfmt` without changes

**Non-Goals:**
- Expanding the module documentation content (can be done separately)

## Decisions

**Use `//!` inner doc**: `//!` is the canonical Rust way to document a module from within the module file. This will appear correctly in `cargo doc` output.

## Risks / Trade-offs

- None. Pure formatting change.

## Open Questions

- None.
