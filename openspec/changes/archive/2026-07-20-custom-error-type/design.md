## Context

`Box<dyn std::error::Error>` is the simplest possible error type in Rust, but it is a bad default for library crates because it strips type information and is not `Send`. Library consumers cannot write `match err { OpenLigaError::Http(_) => ..., OpenLigaError::Json(_) => ... }` — they can only print the error message.

## Goals / Non-Goals

**Goals:**
- Give consumers a typed, matchable error type
- Ensure errors are `Send + Sync` so futures can be used in multithreaded executors
- Preserve the ergonomic `?` operator throughout the call chain

**Non-Goals:**
- Rich error context (e.g., URL that failed) — can be added incrementally
- Changing the async runtime or executor

## Decisions

**Use `thiserror`**: The `thiserror` crate generates `Display`, `Error`, and `From` impls from a derive macro. It is the idiomatic choice for library error types and adds zero runtime overhead.

**Two variants**: `Reqwest` (wraps `reqwest::Error`, covering both the request `.await?` and the `.json::<M>().await?` deserialization step) and `Url` (wraps `url::ParseError` from `Url::parse`). Both are `Send + Sync`.

`Http` and `Json` were merged into a single `Reqwest` variant during implementation: `thiserror`'s `#[from]` generates one `From<T>` impl per source type, and both original variants wrapped `reqwest::Error`, so only one of them could derive `#[from]` — the other would need a manual `.map_err(OpenLigaError::Json)` construction. Merging keeps every conversion a plain `?` and avoids an inconsistent mix of `#[from]` and manual mapping for what is, in both cases, a `reqwest` failure.

**This is a breaking change**: Downstream code calling `.map_err(|e| e.to_string())` continues to compile. Code that matches on `Box<dyn Error>` will need updating, but that pattern is rare.

## Risks / Trade-offs

- Semver: this change must be released as a minor or major version bump depending on the current semver policy (currently `0.0.x`, so any `0.0.y` bump technically allows breaking changes).
