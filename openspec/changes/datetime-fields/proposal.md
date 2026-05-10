## Why

Match date/time fields (`when`, `when_utc`, `last_update`) and the `time_zone` field in `match.rs` are all typed as `Option<String>`. This pushes date parsing onto every caller, introduces silent bugs (timezone handling, format differences), and makes date arithmetic cumbersome. `chrono` is already a transitive dependency of the app crate, and using `chrono::DateTime<chrono::Utc>` for UTC timestamps and `chrono_tz::Tz` (or a plain `String` for now) for the timezone would make the types self-documenting and immediately useful.

## What Changes

- Add `chrono = { version = "0.4", features = ["serde"] }` to `[dependencies]` in `openligadb.rs/Cargo.toml`
- Change `Match::when` from `Option<String>` to `Option<chrono::DateTime<chrono::FixedOffset>>`
- Change `Match::when_utc` from `Option<String>` to `Option<chrono::DateTime<chrono::Utc>>`
- Change `Match::last_update` from `Option<String>` to `Option<chrono::DateTime<chrono::FixedOffset>>`
- Keep `Match::time_zone` as `Option<String>` for now (IANA timezone name string)

## Capabilities

**Modified Capabilities**
- `openligadb` (library crate): `Match` struct field types change — this is a **breaking API change** for any consumer that reads these fields

## Impact

- `openligadb.rs/Cargo.toml`: add `chrono` dependency with `serde` feature
- `openligadb.rs/src/models/match.rs`: update field types and verify serde round-trips with the existing JSON fixture
- `app/src-tauri/src/lib.rs`: update any code that accesses these fields as strings (currently the frontend receives them as ISO strings via Tauri commands, so the serialised form stays the same)
