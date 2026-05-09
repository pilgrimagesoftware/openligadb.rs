#![doc = r"A Rust library for accessing the [OpenLigaDB] [API].

[OpenLigaDB]: https://www.openligadb.de/
[API]: https://api.openligadb.de/index.html"]

#![allow(
    clippy::doc_markdown,
    clippy::module_name_repetitions,
    clippy::must_use_candidate,
    clippy::option_if_let_else
)]
#![warn(missing_docs)]
#![warn(rust_2018_idioms)]

mod constants;
pub mod models;
mod util;
