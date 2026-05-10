## Why

`async-trait = "0.1"` is declared in `openligadb.rs/Cargo.toml` but is never used anywhere in the source tree — no `#[async_trait]` attribute appears in any file. The dependency adds unnecessary compile overhead and creates a false impression that the crate uses trait-based async dispatch.

## What Changes

- Remove `async-trait` from `[dependencies]` in `openligadb.rs/Cargo.toml`

## Capabilities

**Modified Capabilities**
- `openligadb` (library crate): unused dependency removed; no behaviour change

## Impact

- `openligadb.rs/Cargo.toml`: remove `async-trait = "0.1"` line
