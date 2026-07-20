## Context

Every model file mixes two concerns: a `#[derive(Serialize, Deserialize)]` struct (pure
data, no dependencies beyond `serde`) and an `impl` block with one or more `async fn`s that
build a request URL and call either `crate::util::{get,list}` (which wraps
`reqwest::get(...).json()`) or `reqwest::get` directly. `src/util.rs` is `mod util;`
(private) and depends on `reqwest`. Nothing in the current crate separates "the shape of an
API response" from "how to fetch one" at the dependency-graph level — both come bundled
whether a consumer needs the network methods or not.

`Plugins/Bundesliga` needs only the struct definitions and their `Deserialize` impls: it
deserializes bytes obtained through its own transport (a WASM host `fetch` capability, not
`reqwest`) directly into these types via `serde_json::from_slice`. It never calls
`League::list`, `Match::by_league`, or any other network method on this crate's types. But
because `reqwest` (and transitively its TLS backend, `aws-lc-sys`, which compiles C source)
is an unconditional dependency, the whole crate — including the parts `Plugins/Bundesliga`
doesn't use — fails to cross-compile for `wasm32-wasip2`.

## Goals / Non-Goals

**Goals:**
- Let a consumer depend on this crate for its model types and `serde` impls alone, with
  `default-features = false`, pulling in no `reqwest`/`async-trait`/`url`.
- Keep every existing consumer's build (default features) byte-for-byte unaffected —
  this is purely additive.
- Make the gate mechanical and exhaustive: every network `impl` method, and every `use`
  statement that exists only to support one, gated the same way, not a partial cleanup.

**Non-Goals:**
- Changing any model struct's fields, `#[serde(rename(...))]` mappings, or any network
  method's signature or behavior.
- Adding a non-`reqwest` HTTP client option (e.g. gating in a second client
  implementation) — this change only makes the existing one optional.
- Verifying `Plugins/Bundesliga` actually cross-compiles to `wasm32-wasip2` end-to-end
  once this ships — that's follow-up work in that repo (switching its `openligadb`
  dependency to `default-features = false` and re-running the build), not part of this
  change.

## Decisions

**One feature, `http-client`, default-on, gates `reqwest`, `async-trait`, `url`, and every
network method — not a finer-grained per-endpoint split.**
Rationale: there's exactly one consumer motivating this (a types-only, no-network use
case) and it needs zero network methods, not a subset. A single feature is the simplest
thing that satisfies it; finer granularity (e.g. one feature per model) would add
maintenance surface with no current consumer asking for it. `url` becomes optional too
(currently unconditional in `Cargo.toml`) because every one of its uses (`Url::parse` to
build a request) lives inside a network method being gated — nothing else in the crate
touches it.

**Gate at the `impl` method and `use` statement level, not by splitting files into
`*_types.rs`/`*_client.rs` modules.**
Rationale: this crate's existing consumers (`Apps/rust`, and any external crates.io users)
import from the current module paths (`openligadb::models::team::Team`, etc.); splitting
modules would be a breaking rename for no behavioral benefit. `#[cfg(feature =
"http-client")]` on the affected `impl` blocks/methods and their supporting `use`
statements keeps every public path stable.

**Exhaustive list of what gets `#[cfg(feature = "http-client")]` (or moves into a
gated `[dependencies]` entry):**

| File | Gated |
|---|---|
| `Cargo.toml` | `reqwest`, `async-trait`, `url` become `optional = true`, required by `http-client` |
| `src/lib.rs` | `mod util;` |
| `src/util.rs` | entire file (only reachable from gated methods once done) |
| `src/models/league.rs` | `impl League { list }`; its `use crate::util;`/`use std::error::Error;`/`use url::Url;` |
| `src/models/match.rs` (`r#match.rs`) | `impl Match { get, by_teams, by_league, by_league_group, next_match_by_league_team, last_match_by_league_team, next_match_by_league, last_match_by_league, by_league_team, by_team_range, by_team_id_range }` and matching `use`s |
| `src/models/table.rs` | `impl TableTeam { get_bl_table, get_group_table }` and matching `use`s |
| `src/models/team.rs` | `impl Team { available }` and matching `use`s |
| `src/models/group.rs` | `impl Group { current, available }` (calls `reqwest::get` directly, not via `util`) and matching `use`s |
| `src/models/sport.rs` | `impl Sport { list }` and matching `use`s |
| `src/models/result.rs` | `impl ResultInfo { list }` and matching `use`s (`GlobalResultInfo`, `MatchResult`, `ResultInfo`'s struct definitions are ungated — pure data) |
| `src/models/goal.rs` | `impl GoalGetter { list }` and matching `use`s (`Goal`'s struct definition is ungated — pure data, never fetched standalone) |
| `src/models/location.rs` | nothing — pure data struct, never has network methods |

Every `#[cfg(test)] mod tests` block that exercises a gated network method also needs
`#[cfg(feature = "http-client")]` alongside `#[cfg(test)]`, so `cargo test
--no-default-features` doesn't fail to compile against methods that no longer exist.
`test_deserialize_match` in `src/models/match.rs` (deserializes a fixture file, no network
call) stays ungated — it's exactly the kind of usage this change is meant to support.

**`constants::API_BASE_URL` stays ungated.**
Rationale: it's a plain `&str` constant with no dependency footprint; gating it would only
force every gated `use crate::constants::API_BASE_URL;` to also be conditional for no
benefit, since the constant itself compiles fine either way.

## Risks / Trade-offs

- [A future network method added to a model `impl` block ships ungated by mistake,
  silently reintroducing the `reqwest` dependency for `--no-default-features` builds] →
  CI runs `cargo check --no-default-features` (task 3) so any ungated network method fails
  the build immediately with an unresolved-import error, not silently.
- [Downstream consumers relying on `openligadb`'s current implicit "always has networking"
  assumption could be surprised if they ever set `default-features = false` without
  meaning to] → `http-client` defaults on; nothing changes unless a consumer opts out
  explicitly.
- [This change doesn't itself prove `Plugins/Bundesliga` builds for `wasm32-wasip2`] →
  Explicitly out of scope (see Non-Goals); that verification belongs to that repo's own
  follow-up once this ships.

## Migration Plan

Not applicable — additive, opt-in change; no existing consumer's `Cargo.toml` needs to
change. `Plugins/Bundesliga` adopts `default-features = false` as its own follow-up once
this is released.

## Open Questions

None — the feature-gate boundary (network methods + their exclusive `use`s vs. everything
else) is unambiguous from the current file structure; every file's disposition is listed
in the Decisions table above.
