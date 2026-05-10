## Why

Every public method in the `openligadb.rs` library returns `Result<..., Box<dyn Error>>`. This erases the concrete error type, making it impossible for callers to match on specific error variants (e.g., network failure vs. JSON parse failure vs. URL construction failure). It also prevents the error from being `Send + Sync`, which causes problems when the futures are used in multithreaded runtimes. A `thiserror`-derived error enum would give consumers a typed, matchable, `Send + Sync` error with no additional boilerplate.

## What Changes

- Add `thiserror = "1"` to `[dependencies]` in `openligadb.rs/Cargo.toml`
- Define a `pub enum OpenLigaError` in a new `error.rs` module with variants: `Http(reqwest::Error)`, `Json(reqwest::Error)`, `Url(url::ParseError)`
- Update `util::list` and `util::get` to return `Result<_, OpenLigaError>` and use `?` with `#[from]` conversions
- Update all model method signatures to return `Result<_, OpenLigaError>`
- Re-export `OpenLigaError` from `lib.rs`

## Capabilities

**Modified Capabilities**
- All public model methods: return type changes from `Box<dyn Error>` to `OpenLigaError`; this is a **breaking API change**

## Impact

- `openligadb.rs/Cargo.toml`: add `thiserror = "1"`
- `openligadb.rs/src/error.rs`: new file with `OpenLigaError` enum
- `openligadb.rs/src/lib.rs`: add `pub mod error;` and re-export `OpenLigaError`
- `openligadb.rs/src/util.rs`: update return types
- All model files: update return type annotations and remove `map_err(|e| e.to_string())` calls
- `app/src-tauri/src/lib.rs`: update `.map_err(|e| e.to_string())` calls at Tauri command boundaries (no change needed since those already call `.map_err`)
