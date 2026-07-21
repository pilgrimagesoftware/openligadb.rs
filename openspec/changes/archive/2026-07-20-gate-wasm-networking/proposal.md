## Why

`Plugins/Bundesliga` (the FullTime plugin architecture's reference data-provider plugin)
depends on this crate for its response model types (`Team`, `Match`, `TableTeam`, ...) and
their `Deserialize` implementations, reused via `serde_json::from_slice` rather than this
crate's own `reqwest`-based fetch methods (plugins run as WASM components with no direct
network access — see `fulltime-plugin-api`'s `docs/plugin-authoring.md`). Cross-compiling
that plugin to `wasm32-wasip2` currently fails because `reqwest` is an unconditional
dependency of this crate, and `reqwest`'s TLS stack (`aws-lc-sys`, compiled C code) cannot
build for `wasm32-wasip2` — even though the plugin never calls any of this crate's
networking code. There is no way today to depend on this crate for its types alone without
also pulling in a dependency tree that can't target `wasm32`.

## What Changes

- Add an optional Cargo feature (`http-client`, default-on) gating `reqwest`,
  `async-trait`, and every `impl` method on the model types that calls them (`League::list`,
  `Match::get`, `TableTeam::get_bl_table`, etc.) and the `util` module that backs them.
- With `http-client` disabled (`default-features = false`), the crate exposes only the
  model structs and their `serde::Deserialize`/`Serialize` implementations — no networking
  code, no `reqwest`/`async-trait` in the dependency tree — making it buildable for
  `wasm32-wasip2` and any other target `reqwest`'s TLS stack can't reach.
- **BREAKING**: none for existing consumers — `http-client` defaults on, so
  `openligadb = "0.0.9"` (no explicit `default-features = false`) keeps today's full API
  surface. This is additive from every current consumer's point of view.
- Update `README.md`/`AGENTS.md`/`CHANGELOG.md` documenting the feature and its intended
  use (types-only consumption from `wasm32` targets).

## Capabilities

### New Capabilities
- `wasm-compatible-models`: consuming this crate's model types and their `serde`
  implementations without pulling in `reqwest`/`async-trait`, via
  `default-features = false`.

### Modified Capabilities
- (none — existing behavior with default features is unchanged; this only adds an opt-out)

## Impact

- **This repo**: `Cargo.toml` gains an `[features]` table; every model's `impl` block with
  network methods gets `#[cfg(feature = "http-client")]`; `src/util.rs` (already private)
  becomes entirely feature-gated; CI gains a `cargo check --no-default-features` job (and,
  if feasible in CI, `--target wasm32-wasip2`) alongside the existing full-feature checks.
- **`Plugins/Bundesliga`**: once released, switches its `openligadb` dependency to
  `openligadb = { version = "...", default-features = false }`, unblocking a real
  `wasm32-wasip2` build of that plugin — tracked as follow-up work in that repo, not part
  of this change.
- **`Apps/rust`**: currently depends on this crate with default features (needs the
  networking methods for its direct integration); unaffected, since `http-client`
  defaults on.
- No other consumers of this crate are known to be affected.
