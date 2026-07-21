## Context

The `openligadb.rs` crate is a thin async HTTP client that wraps the OpenLigaDB REST API. Its `Cargo.toml` has accumulated three dependency hygiene issues: `actix-web` is both a runtime and dev dependency (when it only needs to be the latter), `openssl-sys` with `vendored` forces a full OpenSSL compile on every downstream crate, and a `[build] profiler = true` key exists that Cargo ignores entirely.

## Goals / Non-Goals

**Goals:**
- Eliminate `actix-web` from the runtime dependency tree
- Remove the vendored OpenSSL constraint from downstream consumers
- Clean up the invalid `[build]` table

**Non-Goals:**
- Changing the TLS backend used by `reqwest` (leave as default)
- Changing any test behaviour

## Decisions

**1. actix-web dev-only**
`actix-web` provides the `#[actix_web::test]` macro used in every test module. It belongs in `[dev-dependencies]` exclusively. Moving it there removes it from the compiled library artifact and from the dependency graph seen by library consumers.

**2. Remove openssl-sys vendored**
The `vendored` feature compiles OpenSSL from source. For a library crate this is an aggressive default that surprises downstream users and significantly increases build times. `reqwest` with the `json` feature works fine with the platform's native TLS on macOS and Linux, and with `rustls` if preferred. Removing the explicit `openssl-sys` entry lets the consumer decide.

**3. Remove invalid [build] table**
`profiler = true` under `[build]` is not a Cargo-recognised key. It has no effect and creates confusion. Remove the entire section.

## Risks / Trade-offs

- CI environments that relied on vendored OpenSSL may need `OPENSSL_DIR` set or a switch to `rustls`. This should be handled at the consumer or CI level, not forced on the library.

## Open Questions

- Should the library offer a `vendored-openssl` Cargo feature to opt back in for consumers who need it? (Probably yes, but can be deferred.)
