# OpenLigaDB Rust client

[![CI][ci-badge]][ci]
[![Crates.io][crates-badge]][crates-url]
[![MIT licensed][mit-badge]][mit-url]
[![Documentation][docs-badge]][docs]
[![Rust][rust-badge]][rust]

[ci-badge]: https://github.com/pilgrimagesoftware/openligadb.rs/actions/workflows/rust-ci.yaml/badge.svg
[ci]: https://github.com/pilgrimagesoftware/openligadb.rs/actions/workflows/rust-ci.yaml
[crates-badge]: https://img.shields.io/crates/v/badge-maker.svg
[crates-url]: https://crates.io/crates/openligadb
[mit-badge]: https://img.shields.io/badge/license-MIT-blue.svg
[mit-url]: https://github.com/pilgrimagesoftware/openligadb.rs/blob/master/LICENSE
[docs-badge]: https://docs.rs/openligadb/badge.svg
[docs]: https://docs.rs/openligadb
[rust-badge]: https://img.shields.io/badge/rust-1.82.0%2B-blue.svg?maxAge=3600
[rust]: https://github.com/pilgrimagesoftware/openligadb

This is a Rust client for accessing the [OpenLigaDB API](https://www.openligadb.de/).

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
openligadb = "0.0.1"
```

and this to your code:

```rust
use openligadb;

let leagues = openligadb::models::league::list()?;
```

## Change log

[CHANGELOG](CHANGELOG.md)

## License

Licensed under:

 * MIT license ([LICENSE-MIT](LICENSE) or https://opensource.org/licenses/MIT)

## Contribution

Unless explicitly stated otherwise, any contribution intentionally submitted for inclusion in the work
by you shall be licensed as above.
