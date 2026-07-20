# AGENTS.md

This file provides guidance to Claude Code (claude.ai/code), Codex (openai.com/codex/), GitHub
Copilot (copilot.github.com), and other models when working with code in this repository.

## About This Project

This is the Rust SDK library for the Football App (Fussballergebnisse) project. It provides an
async Rust client for the [OpenLigaDB API](https://api.openligadb.de/index.html), which surfaces
data about football leagues, matches, teams, goals, standings, and more.

The crate is published to [crates.io](https://crates.io/crates/openligadb) and documented on
[docs.rs](https://docs.rs/openligadb).

## Repository

- **GitHub**: https://github.com/pilgrimagesoftware/openligadb.rs
- **Crate name**: `openligadb`
- **Minimum Rust version**: 1.82.0

## Branches and Workflow

- `develop` is the source of truth for active development (Git Flow).
- Feature branches are cut from `develop` and merged back via pull request.
- `master` / `main` tracks released, published versions.

## Coding Conventions

- All public items **must** have doc comments (`///`). The crate enables `#![warn(missing_docs)]`.
- Follow `#![warn(rust_2018_idioms)]`; avoid deprecated Rust 2015 idioms.
- Clippy is configured via `clippy.toml`. Run `cargo clippy` before committing and resolve all
  warnings that are not explicitly suppressed in `lib.rs`.
- YAML files use the `.yaml` extension (not `.yml`).
- Commit messages follow the [Conventional Commits](https://www.conventionalcommits.org/en/v1.0.0/)
  format, e.g. `feat(league): add get-by-season endpoint`.

## Adding a New Model

1. Create `src/models/<name>.rs` following the pattern of an existing model (e.g. `league.rs`).
2. Derive `serde::Deserialize` (and `Serialize` if serialization is needed) on the struct — this
   part must stay reachable with `default-features = false` (see "Feature gating" below).
3. Implement any public query functions using `crate::util::list` or `crate::util::get`, passing a
   URL built from `crate::constants::API_BASE_URL`.
4. Register the new module in `src/models/mod.rs`.
5. Add doc comments to every public struct, field, and function.

## Feature gating

The crate has an `http-client` feature (default-on) that gates `reqwest`, `async-trait`, `url`,
and every network method, so a consumer that only needs the model structs and their `serde`
impls (e.g. a WASM component deserializing bytes fetched through its own host `fetch`
capability) can depend on this crate with `default-features = false` and pull in no networking
stack — see `openspec/changes/gate-wasm-networking/design.md` for the full rationale.

**Any new network method on a model `impl` block must be gated the same way as the existing
ones**, or it silently reintroduces `reqwest` as a mandatory dependency:

- Put `#[cfg(feature = "http-client")]` on the `impl` block (or the individual method, if the
  block also holds non-network methods).
- Gate any `use` statement that exists only to support a gated method (`crate::util`,
  `std::error::Error`, `url::Url`, `crate::constants::API_BASE_URL`) the same way — an ungated
  `use` of something only referenced by gated code fails `--no-default-features` builds with an
  unused-import/dead-code error.
- Gate the corresponding test function(s) with `#[cfg(feature = "http-client")]` alongside
  `#[cfg(test)]`, unless the whole test module only contains network tests, in which case gate
  the module itself (`#[cfg(all(test, feature = "http-client"))]`).
- `cargo check --no-default-features` and `cargo clippy --no-default-features --all-targets -- -D
  warnings` must both pass — CI runs both alongside the default-feature checks.

Struct definitions and their `serde` derives are never gated, even for fields/structs that exist
only to be returned by a network method (e.g. `GlobalResultInfo`, `Goal`) — a consumer that
deserializes a fixture or a response fetched by its own transport still needs the type.

## Running Checks Locally

```bash
# Build
cargo build

# Run tests
cargo test

# Lint
cargo clippy -- -D warnings

# Format
cargo fmt --check

# Verify the http-client feature is genuinely optional
cargo check --no-default-features
cargo test --no-default-features
cargo clippy --no-default-features --all-targets -- -D warnings
```
