## Why

`openligadb.rs/src/util.rs` has three small issues that reduce correctness and clarity:
1. `reqwest::get(url.as_str())` unnecessarily converts a `Url` to `&str`; `reqwest::get` accepts `impl IntoUrl` and `url::Url` already implements it.
2. Both `list` and `get` carry an unnecessary `+ 'static` lifetime bound on the type parameter `M`. `DeserializeOwned` already implies the type does not borrow from the deserializer; adding `'static` is redundant.
3. Both functions are `pub` but live in a private module (`mod util`). They should be `pub(crate)` to accurately express that they are internal helpers.

## What Changes

- Pass `url` directly to `reqwest::get()` instead of `url.as_str()`
- Remove the `+ 'static` bound from both function signatures
- Change visibility from `pub` to `pub(crate)` on both functions
- Remove the bare `use reqwest;` import (unnecessary when using the full path `reqwest::get`)

## Capabilities

No behaviour change — internal refactor.

## Impact

- `openligadb.rs/src/util.rs`: all four changes above
