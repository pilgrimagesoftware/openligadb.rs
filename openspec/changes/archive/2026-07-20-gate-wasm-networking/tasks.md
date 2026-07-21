## 1. Cargo Manifest

- [x] 1.1 Add `[features]` table: `default = ["http-client"]`, `http-client = ["dep:reqwest", "dep:async-trait", "dep:url"]`
- [x] 1.2 Mark `reqwest`, `async-trait`, `url` `optional = true` in `[dependencies]`

## 2. Gate Network Methods and Their Imports

- [x] 2.1 `src/lib.rs`: gate `mod util;` behind `#[cfg(feature = "http-client")]`
- [x] 2.2 `src/util.rs`: gate the entire file's contents behind `#[cfg(feature = "http-client")]`
- [x] 2.3 `src/models/league.rs`: gate `impl League { list }` and its now-exclusive `use`
  statements
- [x] 2.4 `src/models/match.rs`: gate every network method on `impl Match` and its
  now-exclusive `use` statements
- [x] 2.5 `src/models/table.rs`: gate `impl TableTeam { get_bl_table, get_group_table }`
  and its now-exclusive `use` statements
- [x] 2.6 `src/models/team.rs`: gate `impl Team { available }` and its now-exclusive `use`
  statements
- [x] 2.7 `src/models/group.rs`: gate `impl Group { current, available }` and its
  now-exclusive `use` statements
- [x] 2.8 `src/models/sport.rs`: gate `impl Sport { list }` and its now-exclusive `use`
  statements
- [x] 2.9 `src/models/result.rs`: gate `impl ResultInfo { list }` and its now-exclusive
  `use` statements (leave `GlobalResultInfo`/`MatchResult`/`ResultInfo` struct definitions
  ungated)
- [x] 2.10 `src/models/goal.rs`: gate `impl GoalGetter { list }` and its now-exclusive
  `use` statements (leave `Goal`/`GoalGetter` struct definitions ungated)
- [x] 2.11 `src/models/location.rs` needed no changes (pure data). `src/constants.rs`
  deviated from design.md: `API_BASE_URL` also gated behind `#[cfg(feature =
  "http-client")]`, not left ungated as originally planned — every one of its `use`
  sites already ended up gated (2.3-2.10), so leaving the constant itself ungated
  produced a `dead_code` error under `cargo clippy --no-default-features -- -D
  warnings` with no compensating benefit.

## 3. Gate Tests

- [x] 3.1 Add `#[cfg(feature = "http-client")]` alongside `#[cfg(test)]` on every test
  module/function that calls a gated network method, across all files touched in section 2
- [x] 3.2 Confirm `test_deserialize_match` (`src/models/match.rs`, fixture-based, no
  network call) stays ungated

## 4. Verification

- [x] 4.1 `cargo build`/`test`/`clippy --all-targets --all-features -- -D warnings`/`fmt
  --check` all pass (default features, matches current CI)
- [x] 4.2 `cargo check --no-default-features` passes
- [x] 4.3 `cargo test --no-default-features` passes (only fixture-based tests run; no
  network methods compiled)
- [x] 4.4 `cargo check --no-default-features --target wasm32-wasip2` passes
- [x] 4.5 Add a `--no-default-features` (and, if the runner supports it, `--target
  wasm32-wasip2 --no-default-features`) job to CI alongside the existing checks

## 5. Documentation

- [x] 5.1 Document the `http-client` feature in `README.md` (what it gates, how to opt out,
  the `wasm32` use case)
- [x] 5.2 Update `AGENTS.md` with the feature-gating convention for any future network
  method
- [x] 5.3 Add a `CHANGELOG.md` `[Unreleased]` entry
