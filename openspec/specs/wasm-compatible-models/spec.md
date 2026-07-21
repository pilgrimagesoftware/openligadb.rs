## Purpose

Consumers that only need this crate's model types and their `serde` implementations,
notably WASM components with no network access such as `Plugins/Bundesliga`, can depend
on the crate without pulling in `reqwest`, `async-trait`, or any code that can't
cross-compile to targets like `wasm32-wasip2`. This capability governs the `http-client`
feature gate that makes that possible.

## Requirements

### Requirement: Optional HTTP Client Feature
The crate SHALL gate `reqwest`, `async-trait`, `url`, and every network-calling method
behind an `http-client` Cargo feature that is enabled by default.

#### Scenario: Default build is unaffected
- **WHEN** a consumer depends on this crate with default features (no explicit
  `default-features = false`)
- **THEN** every model type and every network method (`League::list`, `Match::get`, etc.)
  is available, identical to the crate's behavior before this change

#### Scenario: Consumer opts out of the HTTP client
- **WHEN** a consumer depends on this crate with `default-features = false`
- **THEN** `reqwest`, `async-trait`, and `url` are absent from the resolved dependency
  graph, and no network method is available on any model type

### Requirement: Model Types Remain Available Without the HTTP Client
Every model struct and its `serde::Serialize`/`Deserialize` implementation SHALL compile
and function identically regardless of whether the `http-client` feature is enabled.

#### Scenario: Deserializing a response without the HTTP client feature
- **WHEN** a consumer with `default-features = false` deserializes upstream JSON into a
  model type (e.g. via `serde_json::from_slice`)
- **THEN** the deserialized value is identical to what the same input would produce with
  `http-client` enabled

### Requirement: Cross-Compiles Without the HTTP Client
With `http-client` disabled, the crate SHALL build for targets `reqwest`'s TLS stack
cannot reach, including `wasm32-wasip2`.

#### Scenario: Building for wasm32-wasip2 with the HTTP client disabled
- **WHEN** the crate is built with `--target wasm32-wasip2 --no-default-features`
- **THEN** the build succeeds, since no dependency in the resolved graph requires
  compiling non-wasm-portable code (e.g. `aws-lc-sys`'s C sources)

#### Scenario: Building for wasm32-wasip2 with the HTTP client enabled
- **WHEN** the crate is built with `--target wasm32-wasip2` and default features
- **THEN** the build fails, since `reqwest`'s TLS backend cannot cross-compile for that
  target; this is expected and unchanged by this capability, and only the
  `--no-default-features` path is required to succeed
