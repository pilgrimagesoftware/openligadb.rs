## [Unreleased]

### Added

- `http-client` Cargo feature (default-on), gating `reqwest`, `async-trait`, `url`, and every
  model's network methods. Consumers that only need the model structs and their `serde` impls
  can depend on this crate with `default-features = false`, pulling in no networking stack —
  this unblocks cross-compiling to targets `reqwest` can't reach (e.g. `wasm32-wasip2`).
