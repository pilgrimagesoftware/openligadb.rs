# OpenLigaDB Rust client

[![Crates.io](https://img.shields.io/crates/v/openligadb.svg)](https://crates.io/crates/openligadb)
[![docs.rs](https://img.shields.io/docsrs/openligadb)](https://docs.rs/openligadb)
[![CI](https://github.com/pilgrimagesoftware/openligadb.rs/actions/workflows/ci.yaml/badge.svg)](https://github.com/pilgrimagesoftware/openligadb.rs/actions/workflows/ci.yaml)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE.md)

This is a Rust client for accessing the [OpenLigaDB API](https://www.openligadb.de/).

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
openligadb = "0.0.13"
```

and this to your code:

```rust
use openligadb;

let leagues = openligadb::models::league::list()?;
```

## Features

`http-client` (default-on) gates `reqwest`, `async-trait`, and `url`, along with every
model's network methods (`League::list`, `Match::get`, etc.). Disable it if you only need
the model structs and their `serde` impls — for example, deserializing responses fetched
through a transport other than `reqwest` (a WASM host `fetch` capability, a proxy, etc.):

```toml
[dependencies]
openligadb = { version = "0.0.13", default-features = false }
```

With `http-client` off, the crate has no networking dependencies and cross-compiles to
targets `reqwest` can't reach (e.g. `wasm32-wasip2`).

## Change log

[CHANGELOG](CHANGELOG.md)

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md) for the development workflow and commit conventions, and [RELEASING.md](RELEASING.md) for how versions get cut.
This project follows the [Code of Conduct](CODE_OF_CONDUCT.md).

## Security

See [SECURITY.md](SECURITY.md) to report a vulnerability.

## License

Licensed under:

* MIT license ([LICENSE.md](LICENSE.md) or <https://opensource.org/licenses/MIT>)

## Contribution

Unless explicitly stated otherwise, any contribution intentionally submitted for inclusion in the work by you shall be licensed as above.
