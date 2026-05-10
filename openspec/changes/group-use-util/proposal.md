## Why

`openligadb.rs/src/models/group.rs` implements `Group::current` and `Group::available` by directly calling `reqwest::get(…).await?.json::<T>().await?` inline — the same pattern that `util::get` and `util::list` exist to abstract. Every other model (`league.rs`, `match.rs`, `team.rs`, `table.rs`, `sport.rs`, `goal.rs`, `result.rs`) delegates to those helpers. `group.rs` is the sole exception, duplicating ~10 lines of boilerplate per method and diverging from the codebase convention.

## What Changes

- Replace the inline `reqwest` call in `Group::current` with `util::get(api_url).await`
- Replace the inline `reqwest` call in `Group::available` with `util::list(api_url).await`
- Remove the direct `reqwest::get` calls and their associated `map_err` chains from `group.rs`
- Add `use crate::util;` import to `group.rs`

## Capabilities

No behaviour change — internal consistency refactor.

## Impact

- `openligadb.rs/src/models/group.rs`: replace inline HTTP logic with `util::get`/`util::list` calls; add import
