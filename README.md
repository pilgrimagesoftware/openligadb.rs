# OpenLigaDB Rust client

[![CI][ci-badge]][ci]
[![Crates.io][crates-badge]][crates-url]
[![MIT licensed][mit-badge]][mit-url]
[![Documentation][docs-badge]][docs]
[![Rust][rust-badge]][rust]

[ci-badge]: https://github.com/pilgrimagesoftware/openligadb.rs/actions/workflows/rust-ci.yaml/badge.svg
[ci]: https://github.com/pilgrimagesoftware/openligadb.rs/actions/workflows/rust-ci.yaml
[crates-badge]: https://img.shields.io/crates/v/openligadb.svg
[crates-url]: https://crates.io/crates/openligadb
[mit-badge]: https://img.shields.io/badge/license-MIT-blue.svg
[mit-url]: https://github.com/pilgrimagesoftware/openligadb.rs/blob/master/LICENSE.md
[docs-badge]: https://docs.rs/openligadb/badge.svg
[docs]: https://docs.rs/openligadb
[rust-badge]: https://img.shields.io/badge/rust-1.82.0%2B-blue.svg?maxAge=3600
[rust]: https://github.com/pilgrimagesoftware/openligadb

This is a Rust client for accessing the [OpenLigaDB API](https://www.openligadb.de/).

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
openligadb = "0.0.9"
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
openligadb = { version = "0.0.9", default-features = false }
```

With `http-client` off, the crate has no networking dependencies and cross-compiles to
targets `reqwest` can't reach (e.g. `wasm32-wasip2`).

## Change log

[CHANGELOG](CHANGELOG.md)

## License

Licensed under:

 * MIT license ([LICENSE-MIT](LICENSE.md) or https://opensource.org/licenses/MIT)

## Contribution

Unless explicitly stated otherwise, any contribution intentionally submitted for inclusion in the work
by you shall be licensed as above.
