## Why

`openligadb.rs/src/constants.rs` uses a C-style `/** ... @paulyhedral */` block comment as a module header, and the `API_BASE_URL` constant is declared with a non-idiomatic extra space before the colon (`pub const API_BASE_URL : &str`). Both deviate from Rust conventions and will trigger `rustfmt` diffs.

## What Changes

- Replace the `/** */` block comment with a `//!` inner doc comment for the module
- Remove the stray space in `pub const API_BASE_URL : &str` → `pub const API_BASE_URL: &str`

## Capabilities

No capability change — purely cosmetic.

## Impact

- `openligadb.rs/src/constants.rs`: comment style and formatting fix
