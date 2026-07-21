## Why

The `openligadb` library crate lists `actix-web` as a runtime dependency even though it is only used as a test executor (`#[actix_web::test]`). This causes every consumer of the library to transitively pull in the full actix-web framework at compile time. Additionally, `openssl-sys` is declared with `features = ["vendored"]`, which forces all downstream crates to compile OpenSSL from source — a slow and unexpected side effect for a lightweight API client library. A stray `[build] profiler = true` key is also present; this is not a valid Cargo configuration key and silently does nothing.

## What Changes

- Move `actix-web` from `[dependencies]` to `[dev-dependencies]` only
- Remove `openssl-sys = { version = "0.9", features = ["vendored"] }` from `[dependencies]`; let `reqwest` manage TLS negotiation via its own feature flags
- Remove the `[build]` table entirely (the `profiler = true` key is not a valid Cargo key)

## Capabilities

**Modified Capabilities**
- `openligadb` (library crate): compile-time dependency surface reduced; no runtime behaviour change

## Impact

- `openligadb.rs/Cargo.toml`: remove `actix-web` from `[dependencies]`, add it to `[dev-dependencies]`; remove `openssl-sys` entry; remove `[build]` table
